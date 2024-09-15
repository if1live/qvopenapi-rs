use std::ffi::c_char;
use std::mem::size_of;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::utils::parse_number;
use crate::{error::*, models::*, wmca_lib};
use qvopenapi_bindings::{Ts8064InBlock, Ts8064OutBlock, Ts8064OutBlock1};

pub const TR_CODE_S8064: &str = "s8064";

#[derive(Debug, Clone, Deserialize)]
pub struct S8064Request {
    pub account_index: i32,
    pub account_password: String,
    pub trade_password: String,
    pub cur_cdz3: String,
    pub qut_iqr_dit_cdz1: Option<String>,
    pub fc_sec_trd_nat_cdz3: Option<String>,
    pub xns_dit_cdz1: Option<String>,
    pub fee_dit_cdz1: Option<String>,
}

impl S8064Request {
    pub fn new(
        account_index: i32,
        account_password: String,
        trade_password: String,
        cur_cdz3: String,
    ) -> S8064Request {
        S8064Request {
            account_index,
            account_password,
            cur_cdz3,
            trade_password,
            qut_iqr_dit_cdz1: Some("0".to_string()),
            fc_sec_trd_nat_cdz3: Some("".to_string()),

            xns_dit_cdz1: Some("0".to_string()),
            fee_dit_cdz1: Some("0".to_string()),
        }
    }

    pub fn into_raw(&self) -> Arc<RawQueryRequest<Ts8064InBlock>> {
        // TODO: 돌아가는거 확인되면 뜯어서 옮기고싶은데
        let mut buffer_0: [u8; 44] = [0; 44];
        wmca_lib::set_account_index_pwd(&mut buffer_0, self.account_index, &self.account_password)
            .unwrap();
        let pwdz44 = String::from_utf8_lossy(&buffer_0).into_owned();

        // ‘거래비밀번호1’과 ‘거래비밀번호2’에는 거래비밀번호를 동일하게 입력하시기 바랍니다.
        let mut buffer_1: [u8; 44] = [0; 44];
        let mut buffer_2: [u8; 44] = [0; 44];
        wmca_lib::set_order_pwd(&mut buffer_1, &self.trade_password).unwrap();
        wmca_lib::set_order_pwd(&mut buffer_2, &self.trade_password).unwrap();
        let trad_pswd_no_1z44 = String::from_utf8_lossy(&buffer_1).into_owned();
        let trad_pswd_no_2z44 = String::from_utf8_lossy(&buffer_2).into_owned();

        println!("pwdz44: {}", pwdz44);
        println!("trad_pswd_no_1z44: {}", trad_pswd_no_1z44);
        println!("trad_pswd_no_2z44: {}", trad_pswd_no_2z44);

        let qut_iqr_dit_cdz1 = self.qut_iqr_dit_cdz1.clone().unwrap_or("0".to_string());
        let fc_sec_trd_nat_cdz3 = self.fc_sec_trd_nat_cdz3.clone().unwrap_or("".to_string());
        let xns_dit_cdz1 = self.xns_dit_cdz1.clone().unwrap_or("0".to_string());
        let fee_dit_cdz1 = self.fee_dit_cdz1.clone().unwrap_or("0".to_string());

        // TODO:
        let act_noz11 = "20101938911".to_string();

        Arc::new(RawQueryRequest::new(
            &TR_CODE_S8064,
            self.account_index,
            Box::new(Ts8064InBlock {
                act_noz11: [act_noz11.as_ptr() as c_char; 11],
                _act_noz11: ' ' as c_char,
                pwdz44: [pwdz44.as_ptr() as c_char; 44],
                _pwdz44: ' ' as c_char,
                qut_iqr_dit_cdz1: [qut_iqr_dit_cdz1.as_ptr() as c_char; 1],
                _qut_iqr_dit_cdz1: ' ' as c_char,
                fc_sec_trd_nat_cdz3: [fc_sec_trd_nat_cdz3.as_ptr() as c_char; 3],
                _fc_sec_trd_nat_cdz3: ' ' as c_char,
                cur_cdz3: [self.cur_cdz3.as_ptr() as c_char; 3],
                _cur_cdz3: ' ' as c_char,
                xns_dit_cdz1: [xns_dit_cdz1.as_ptr() as c_char; 1],
                _xns_dit_cdz1: ' ' as c_char,
                fee_dit_cdz1: [fee_dit_cdz1.as_ptr() as c_char; 1],
                _fee_dit_cdz1: ' ' as c_char,
                trad_pswd_no_1z44: [trad_pswd_no_1z44.as_ptr() as c_char; 44],
                _trad_pswd_no_1z44: ' ' as c_char,
                trad_pswd_no_2z44: [trad_pswd_no_2z44.as_ptr() as c_char; 44],
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
