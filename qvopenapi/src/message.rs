use crate::{*, request::end_active_request};
use log::*;
use std::ffi::{c_char, CStr};
use std::time::Instant;

use crate::error::*;
use crate::basic_structs::*;
use encoding::{all::WINDOWS_949, DecoderTrap, Encoding};
use windows::Win32::UI::WindowsAndMessaging::WM_USER;

pub const WM_WMCAEVENT: u32 = WM_USER + 8400;

pub const CA_CONNECTED: u32 = WM_USER + 110;
pub const CA_DISCONNECTED: u32 = WM_USER + 120;
pub const CA_SOCKETERROR: u32 = WM_USER + 130;
pub const CA_RECEIVEDATA: u32 = WM_USER + 210;
pub const CA_RECEIVESISE: u32 = WM_USER + 220;
pub const CA_RECEIVEMESSAGE: u32 = WM_USER + 230;
pub const CA_RECEIVECOMPLETE: u32 = WM_USER + 240;
pub const CA_RECEIVEERROR: u32 = WM_USER + 250;

pub const WM_CUSTOMEVENT: u32 = WM_USER + 8410;

pub const CA_COMMAND: u32 = WM_USER + 110;

/**
 * Handle messages from window message loop
 */
pub fn on_wmca_msg(message_type: usize, lparam: isize) -> std::result::Result<(), QvOpenApiError> {
    match u32::try_from(message_type).unwrap() {
        CA_CONNECTED => on_connect(lparam),
        CA_DISCONNECTED => Err(QvOpenApiError::EventUnimplementedError {
            event: String::from("CA_DISCONNECTED"),
        }),
        CA_SOCKETERROR => Err(QvOpenApiError::EventUnimplementedError {
            event: String::from("CA_SOCKETERROR"),
        }),
        CA_RECEIVEDATA => Err(QvOpenApiError::EventUnimplementedError {
            event: String::from("CA_RECEIVEDATA"),
        }),
        CA_RECEIVESISE => Err(QvOpenApiError::EventUnimplementedError {
            event: String::from("CA_RECEIVESISE"),
        }),
        CA_RECEIVEMESSAGE => on_receive_message(lparam),
        CA_RECEIVECOMPLETE => on_receive_complete(lparam),
        CA_RECEIVEERROR => on_receive_error(lparam),
        _ => Err(QvOpenApiError::WindowUnknownEventError {
            wparam: message_type,
        }),
    }
}

pub fn on_custom_msg(
    message_type: usize,
    lparam: isize,
) -> std::result::Result<(), QvOpenApiError> {
    match u32::try_from(message_type).unwrap() {
        CA_COMMAND => request::execute_active_request(),
        _ => Err(QvOpenApiError::WindowUnknownEventError {
            wparam: message_type,
        }),
    }
}

fn on_connect(lparam: isize) -> std::result::Result<(), QvOpenApiError> {
    let data_block = lparam as *const LoginBlock;
    unsafe {
        let login_info = (*data_block).login_info;
        let login_datetime = from_cp949(&(*login_info).login_datetime); // "20230506203715"
        let server_name = from_cp949(&(*login_info).server_name); // "htsi194        "
        let user_id = from_cp949(&(*login_info).user_id);
        let account_count = from_cp949(&(*login_info).account_count); // "002"
        let account_infoes = Vec::new();

        info!("CA_CONNECT (\"{}\", \"{}\", \"{}\", \"{}\")", login_datetime, server_name, user_id, account_count);

        end_active_request(Ok(Arc::new(ConnectResponse {
            login_datetime: Instant::now(),
            server_name,
            user_id,
            account_count: 0,
            account_infoes,
        })))
    }
}

fn on_receive_message(lparam: isize) -> std::result::Result<(), QvOpenApiError> {
    let data_block = lparam as *const OutDataBlock<MessageHeader>;
    unsafe {
        let msg_header = (*(*data_block).p_data).sz_data;
        let message_code = from_cp949(&(*msg_header).message_code);
        let message = from_cp949(&(*msg_header).message);
        info!("CA_RECEIVEMESSAGE [{}] \"{}\"", message_code, message);

        if message_code != "00000" {
            end_active_request(Err(QvOpenApiError::QvApiMessageError { message_code, message }))?;
        }
    }

    Ok(())
}

fn on_receive_complete(lparam: isize) -> std::result::Result<(), QvOpenApiError> {
    let data_block = lparam as *const OutDataBlock<()>;
    unsafe {
        let tr_index = (*data_block).tr_index;
        info!("CA_RECEIVECOMPLETE (tr_index: {})", tr_index);
        end_active_request(Ok(Arc::new(QueryResponse{})))?;
    }
    Ok(())
}

fn on_receive_error(lparam: isize) -> std::result::Result<(), QvOpenApiError> {
    let data_block = lparam as *const OutDataBlock<c_char>;
    unsafe {
        let error_msg = from_cp949_ptr((*(*data_block).p_data).sz_data);
        info!("CA_RECEIVEERROR \"{}\"", error_msg);
        end_active_request(Err(QvOpenApiError::QvApiMessageError { message_code: String::from("ERROR"), message: error_msg }))?;
    }
    Ok(())
}

/**
 * 문자열 끝에 null이 없을 수도, 있을 수도 있음
 */
fn from_cp949(src: &[c_char]) -> String {
    let mut cloned: Vec<u8> = vec![];
    for s in src.iter() {
        // null을 만나면 여기까지만
        if *s == 0 {
            break;
        }
        cloned.push(*s as u8);
    }
    WINDOWS_949
        .decode(cloned.as_slice(), DecoderTrap::Replace)
        .unwrap()
}

/**
 * 문자열 끝에 null이 있음
 */
fn from_cp949_ptr(src: *const c_char) -> String {
    unsafe {
        let cstr: &CStr = CStr::from_ptr(src);
        WINDOWS_949
            .decode(cstr.to_bytes(), DecoderTrap::Replace)
            .unwrap()
    }
}
