use std::ffi::c_char;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::utils::{parse_number, parse_string};
use crate::{error::*, models::*};
use qvopenapi_bindings::{Tc8010InBlock, Tc8010OutBlock};

pub const TR_CODE_C8010: &str = "c8010";

#[derive(Debug, Clone, Deserialize)]
pub struct C8010Request {
    pub account_index: i32,
}

impl C8010Request {
    pub fn new(account_index: i32) -> C8010Request {
        C8010Request { account_index }
    }

    pub fn into_raw(&self) -> Arc<RawQueryRequest<Tc8010InBlock>> {
        Arc::new(RawQueryRequest::new(
            TR_CODE_C8010,
            self.account_index,
            Box::new(Tc8010InBlock {
                _blank: ' ' as c_char,
            }),
        ))
    }
}

pub fn parse_c8010_response(
    block_data: *const c_char,
    _block_len: i32,
) -> Result<Value, QvOpenApiError> {
    unsafe {
        let res = &(*(block_data as *const Tc8010OutBlock));
        Ok(json!(C8010Response {
            accnt_namez40: parse_string(&res.accnt_namez40)?,
            pass_wdz8: parse_string(&res.pass_wdz8)?,
            pass2_wdz8: parse_string(&res.pass2_wdz8)?,
            valid_codez1: parse_string(&res.valid_codez1)?,
        }))
    }
}

#[derive(Debug, Clone, Serialize)]
struct C8010Response {
    pub accnt_namez40: String,
    pub pass_wdz8: String,
    pub pass2_wdz8: String,
    pub valid_codez1: String,
}

pub const BLOCK_NAME_C8010_OUT: &str = "c8010OutBlock";
