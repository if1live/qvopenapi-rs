use std::ffi::c_char;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::utils::{parse_number, parse_ratio, parse_ratio_prec, parse_ratio_str, parse_string};
use crate::{error::*, models::*};
use qvopenapi_bindings::{Ts8202InBlock, Ts8202OutBlock, Ts8202OutBlock1};

pub const TR_CODE_S8202: &str = "s8202";

#[derive(Debug, Clone, Deserialize)]
pub struct S8202Request {
    pub account_index: i32,
}

impl S8202Request {
    pub fn new(account_index: i32) -> S8202Request {
        S8202Request { account_index }
    }

    pub fn into_raw(&self) -> Arc<RawQueryRequest<Ts8202InBlock>> {
        // 어떤 값을 넣어야하는지 몰라서 하드코딩으로 땜빵
        let balance_type = '1';
        let aet_bsez1 = '1';
        let iem_llf_cdz2: [i8; 2] = [b'0' as i8, b'0' as i8];

        Arc::new(RawQueryRequest::new(
            TR_CODE_S8202,
            self.account_index,
            Box::new(Ts8202InBlock {
                pswd_noz8: [' ' as c_char; 44],
                _pswd_noz8: ' ' as c_char,
                bnc_bse_cdz1: [balance_type as c_char],
                _bnc_bse_cdz1: ' ' as c_char,
                iem_llf_cdz2: iem_llf_cdz2,
                _iem_llf_cdz2: ' ' as c_char,
                aet_bsez1: [aet_bsez1 as c_char],
                _aet_bsez1: ' ' as c_char,
            }),
        ))
    }
}

pub fn parse_s8202_response(
    block_data: *const c_char,
    _block_len: i32,
) -> Result<Value, QvOpenApiError> {
    unsafe {
        let res = &(*(block_data as *const Ts8202OutBlock));
        Ok(json!(S8202Response {
            dpsit_amtz18: parse_number(&res.dpsit_amtz18)?,
            mrgn_amtz18: parse_number(&res.mrgn_amtz18)?,
            mgint_npaid_amtz18: parse_number(&res.mgint_npaid_amtz18)?,
            chgm_pos_amtz18: parse_number(&res.chgm_pos_amtz18)?,
            cash_mrgn_amtz18: parse_number(&res.cash_mrgn_amtz18)?,
            subst_mgamt_amtz18: parse_number(&res.subst_mgamt_amtz18)?,
            coltr_ratez11: parse_number(&res.coltr_ratez11)?,
            rcble_amtz18: parse_number(&res.rcble_amtz18)?,
            order_pos_csamtz18: parse_number(&res.order_pos_csamtz18)?,
            nordm_loan_amtz18: parse_number(&res.nordm_loan_amtz18)?,
            etc_lend_amtz18: parse_number(&res.etc_lend_amtz18)?,
            subst_amtz18: parse_number(&res.subst_amtz18)?,
            sln_sale_amtz18: parse_number(&res.sln_sale_amtz18)?,
            bal_buy_ttamtz18: parse_number(&res.bal_buy_ttamtz18)?,
            bal_ass_ttamtz18: parse_number(&res.bal_ass_ttamtz18)?,
            asset_tot_amtz18: parse_number(&res.asset_tot_amtz18)?,
            actvt_type10: parse_string(&res.actvt_type10)?,
            lend_amtz18: parse_number(&res.lend_amtz18)?,
            sl_mrgn_amtz18: parse_number(&res.sl_mrgn_amtz18)?,
            pos_csamt1z18: parse_number(&res.pos_csamt1z18)?,
            pos_csamt2z18: parse_number(&res.pos_csamt2z18)?,
            pos_csamt3z18: parse_number(&res.pos_csamt3z18)?,
            pos_csamt4z18: parse_number(&res.pos_csamt4z18)?,
            dpsit_amt_d1_z15: parse_number(&res.dpsit_amt_d1_z15)?,
            dpsit_amt_d2_z18: parse_number(&res.dpsit_amt_d2_z18)?,
            tot_eal_plsz15: parse_number(&res.tot_eal_plsz15)?,
            pft_rtz15: parse_ratio(&res.pft_rtz15)?,
            lsnpf_amt_wonz15: parse_number(&res.lsnpf_amt_wonz15)?,
        }))
    }
}

pub fn parse_s8202_response1_array(
    block_data: *const c_char,
    block_len: i32,
) -> Result<Value, QvOpenApiError> {
    unsafe {
        let block_count = block_len as usize / size_of::<Ts8202OutBlock1>();
        let res: &[Ts8202OutBlock1] =
            core::slice::from_raw_parts(block_data as *const Ts8202OutBlock1, block_count);

        let ret: Result<Vec<S8202Response1>, QvOpenApiError> =
            res.iter().map(parse_s8202_response1).collect();
        Ok(json!(ret?))
    }
}

fn parse_s8202_response1(res: &Ts8202OutBlock1) -> Result<S8202Response1, QvOpenApiError> {
    Ok(S8202Response1 {
        issue_codez12: parse_string(&res.issue_codez12)?,
        issue_namez40: parse_string(&res.issue_namez40)?,
        bal_typez6: parse_string(&res.bal_typez6)?,
        loan_datez8: parse_string(&res.loan_datez8)?,
        bal_qtyz18: parse_number(&res.bal_qtyz18)?,
        unstl_qtyz18: parse_number(&res.unstl_qtyz18)?,
        slby_amtz18: parse_number(&res.slby_amtz18)?,
        byn_amtz18: parse_number(&res.byn_amtz18)?,
        prsnt_pricez18: parse_number(&res.prsnt_pricez18)?,
        lsnpf_amtz18: parse_number(&res.lsnpf_amtz18)?,
        lsnpf_amt_wonz18: parse_number(&res.lsnpf_amt_wonz18)?,
        earn_ratez15: parse_ratio_prec(&res.earn_ratez15, 9)?,
        mrgn_codez4: parse_string(&res.mrgn_codez4)?,
        jan_qtyz18: parse_number(&res.jan_qtyz18)?,
        expr_datez8: parse_string(&res.expr_datez8)?,
        ass_amtz18: parse_number(&res.ass_amtz18)?,
        issue_mgamt_ratez10: parse_ratio_str(&res.issue_mgamt_ratez10)?,
        medo_slby_amtz18: parse_number(&res.medo_slby_amtz18)?,
        post_lsnpf_amtz18: parse_number(&res.post_lsnpf_amtz18)?,
        cur_cdz3: parse_string(&res.cur_cdz3)?,
        nat_cd_nmz40: parse_string(&res.nat_cd_nmz40)?,
        pdt_tp_nmz50: parse_string(&res.pdt_tp_nmz50)?,
        iem_mlf_cdz5: parse_string(&res.iem_mlf_cdz5)?,
    })
}

#[derive(Debug, Clone, Serialize)]
struct S8202Response {
    pub dpsit_amtz18: Option<i64>,
    pub mrgn_amtz18: Option<i64>,
    pub mgint_npaid_amtz18: Option<i64>,
    pub chgm_pos_amtz18: Option<i64>,
    pub cash_mrgn_amtz18: Option<i64>,
    pub subst_mgamt_amtz18: Option<i64>,
    pub coltr_ratez11: Option<i64>,
    pub rcble_amtz18: Option<i64>,
    pub order_pos_csamtz18: Option<i64>,
    pub nordm_loan_amtz18: Option<i64>,
    pub etc_lend_amtz18: Option<i64>,
    pub subst_amtz18: Option<i64>,
    pub sln_sale_amtz18: Option<i64>,
    pub bal_buy_ttamtz18: Option<i64>,
    pub bal_ass_ttamtz18: Option<i64>,
    pub asset_tot_amtz18: Option<i64>,
    pub actvt_type10: String,
    pub lend_amtz18: Option<i64>,
    pub sl_mrgn_amtz18: Option<i64>,
    pub pos_csamt1z18: Option<i64>,
    pub pos_csamt2z18: Option<i64>,
    pub pos_csamt3z18: Option<i64>,
    pub pos_csamt4z18: Option<i64>,
    pub dpsit_amt_d1_z15: Option<i64>,
    pub dpsit_amt_d2_z18: Option<i64>,
    pub tot_eal_plsz15: Option<i64>,
    pub pft_rtz15: Option<f64>,
    pub lsnpf_amt_wonz15: Option<i64>,
}

#[derive(Debug, Clone, Serialize)]
struct S8202Response1 {
    pub issue_codez12: String,
    pub issue_namez40: String,
    pub bal_typez6: String,
    pub loan_datez8: String,
    pub bal_qtyz18: Option<i64>,
    pub unstl_qtyz18: Option<i64>,
    pub slby_amtz18: Option<i64>,
    pub byn_amtz18: Option<i64>,
    pub prsnt_pricez18: Option<i64>,
    pub lsnpf_amtz18: Option<i64>,
    pub lsnpf_amt_wonz18: Option<i64>,
    pub earn_ratez15: Option<f64>,
    pub mrgn_codez4: String,
    pub jan_qtyz18: Option<i64>,
    pub expr_datez8: String,
    pub ass_amtz18: Option<i64>,
    pub issue_mgamt_ratez10: Option<f64>,
    pub medo_slby_amtz18: Option<i64>,
    pub post_lsnpf_amtz18: Option<i64>,
    pub cur_cdz3: String,
    pub nat_cd_nmz40: String,
    pub pdt_tp_nmz50: String,
    pub iem_mlf_cdz5: String,
}

pub const BLOCK_NAME_S8202_OUT: &str = "s8202OutBlock";
pub const BLOCK_NAME_S8202_OUT1_ARRAY: &str = "s8202OutBlock1";
