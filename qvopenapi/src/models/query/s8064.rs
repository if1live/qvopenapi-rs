use std::ffi::c_char;
use std::mem::size_of;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::utils::parse_number;
use crate::{error::*, models::*};
use qvopenapi_bindings::{Ts8064InBlock, Ts8064OutBlock, Ts8064OutBlock1};

pub const TR_CODE_S8064: &str = "s8064";

#[derive(Debug, Clone, Deserialize)]
pub struct S8064Request {
    pub account_index: i32,
    pub cur_cdz3: String,
    pub trade_password: String,
}

impl S8064Request {
    pub fn new(account_index: i32, cur_cdz3: String, trade_password: String) -> S8064Request {
        S8064Request {
            account_index,
            cur_cdz3,
            trade_password,
        }
    }

    pub fn into_raw(&self) -> Arc<RawQueryRequest<Ts8064InBlock>> {
        // ‘거래비밀번호1’과 ‘거래비밀번호2’에는 거래비밀번호를 동일하게 입력하시기 바랍니다.
        let trad_pswd_no_1z44 = self.trade_password.to_string();
        let trad_pswd_no_2z44 = self.trade_password.to_string();

        let qut_iqr_dit_cdz1 = "0".to_string();
        let fc_sec_trd_nat_cdz3 = "".to_string();
        let xns_dit_cdz1 = "0".to_string();
        let fee_dit_cdz1 = "0".to_string();

        Arc::new(RawQueryRequest::new(
            &TR_CODE_S8064,
            self.account_index,
            Box::new(Ts8064InBlock {
                act_noz11: [' ' as c_char; 11],
                _act_noz11: ' ' as c_char,
                pwdz44: [' ' as c_char; 44],
                _pwdz44: ' ' as c_char,
                qut_iqr_dit_cdz1: [qut_iqr_dit_cdz1.as_str().as_ptr() as c_char; 1],
                _qut_iqr_dit_cdz1: ' ' as c_char,
                fc_sec_trd_nat_cdz3: [fc_sec_trd_nat_cdz3.as_str().as_ptr() as c_char; 3],
                _fc_sec_trd_nat_cdz3: ' ' as c_char,
                cur_cdz3: [self.cur_cdz3.as_str().as_ptr() as c_char; 3],
                _cur_cdz3: ' ' as c_char,
                xns_dit_cdz1: [xns_dit_cdz1.as_str().as_ptr() as c_char; 1],
                _xns_dit_cdz1: ' ' as c_char,
                fee_dit_cdz1: [fee_dit_cdz1.as_str().as_ptr() as c_char; 1],
                _fee_dit_cdz1: ' ' as c_char,
                trad_pswd_no_1z44: [trad_pswd_no_1z44.as_str().as_ptr() as c_char; 44],
                _trad_pswd_no_1z44: ' ' as c_char,
                trad_pswd_no_2z44: [trad_pswd_no_2z44.as_str().as_ptr() as c_char; 44],
                _trad_pswd_no_2z44: ' ' as c_char,
            }),
        ))
    }
}

pub fn parse_s8064_response(
    block_data: *const c_char,
    _block_len: i32,
) -> Result<Value, QvOpenApiError> {
    unsafe {
        let res = &(*(block_data as *const Ts8064OutBlock));
        Ok(json!(S8064Response {
            krw_dcaz18: parse_number(&res.krw_dcaz18)?,
            krw_ny_stl_xcl_amtz18: parse_number(&res.krw_ny_stl_xcl_amtz18)?,
            fc_dcaz15: parse_number(&res.fc_dcaz15)?,
            fc_ny_stl_xcl_amtz15: parse_number(&res.fc_ny_stl_xcl_amtz15)?,
        }))
    }
}

pub fn parse_s8064_response1_array(
    block_data: *const c_char,
    block_len: i32,
) -> Result<Value, QvOpenApiError> {
    unsafe {
        let block_count = block_len as usize / size_of::<Ts8064OutBlock1>();
        let res: &[Ts8064OutBlock1] =
            core::slice::from_raw_parts(block_data as *const Ts8064OutBlock1, block_count);

        let ret: Result<Vec<S8064Response1>, QvOpenApiError> =
            res.iter().map(parse_s8064_response1).collect();
        Ok(json!(ret?))
    }
}

fn parse_s8064_response1(res: &Ts8064OutBlock1) -> Result<S8064Response1, QvOpenApiError> {
    Ok(S8064Response1 {
        krw_eal_amtz18: parse_number(&res.krw_eal_amtz18)?,
    })
}

#[derive(Debug, Clone, Serialize)]
struct S8064Response {
    pub krw_dcaz18: Option<i64>,
    pub krw_ny_stl_xcl_amtz18: Option<i64>,
    pub fc_dcaz15: Option<i64>,
    pub fc_ny_stl_xcl_amtz15: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
struct S8064Response1 {
    pub krw_eal_amtz18: Option<i64>,
}

pub const BLOCK_NAME_S8064_OUT: &str = "s8064OutBlock";
pub const BLOCK_NAME_S8064_OUT1_ARRAY: &str = "s8064OutBlock1";
