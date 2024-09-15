// X10a8285.fdf[실시간 해외증권잔고]
// 계좌명/비밀번호조회(c8010)
typedef struct tagc8010InBlock // 기본입력
{
    char pswd_noz44[44];
    char _pswd_noz44; // 비밀번호
} Tc8010InBlock;

typedef struct tagc8010OutBlock // 화면출력
{
    // accnt_namez40
    char accnt_namez40[40];
    char _accnt_namez40;

    // pass_wdz8
    char pass_wdz8[44];
    char _pass_wdz8;

    // pass2_wdz8
    char pass2_wdz8[44];
    char _pass2_wdz8;

    // 0:정상1:오류
    char valid_codez1[1];
    char _valid_codez1;
} Tc8010OutBlock;

typedef struct tagc8010
{
    Tc8010InBlock c8010inblock;   // 기본입력
    Tc8010OutBlock c8010outblock; // 화면출력
} Tc8010;

// X08a8220.fdf[잔고평가(종목별합산)]
typedef struct tags8202InBlock // 기본입력
{
    // 계좌번호
    // char accnt_noz11[11];
    // char _accnt_noz11;

    // 비밀번호
    char pswd_noz8[44];
    char _pswd_noz8;

    // 잔고구분
    char bnc_bse_cdz1[1];
    char _bnc_bse_cdz1;

    // 종목대분류
    char iem_llf_cdz2[2];
    char _iem_llf_cdz2;

    // 자산기준
    char aet_bsez1[1];
    char _aet_bsez1;

} Ts8202InBlock;

typedef struct tags8202OutBlock // 화면출력
{
    // 예수금
    char dpsit_amtz18[18];
    char _dpsit_amtz18;

    // 신용융자금
    char mrgn_amtz18[18];
    char _mrgn_amtz18;

    // 이자미납금
    char mgint_npaid_amtz18[18];
    char _mgint_npaid_amtz18;

    // 출금가능금액
    char chgm_pos_amtz18[18];
    char _chgm_pos_amtz18;

    // 현금증거금
    char cash_mrgn_amtz18[18];
    char _cash_mrgn_amtz18;

    // 대용증거금
    char subst_mgamt_amtz18[18];
    char _subst_mgamt_amtz18;

    // 담보비율
    char coltr_ratez11[11];
    char _coltr_ratez11;

    // 현금미수금
    char rcble_amtz18[18];
    char _rcble_amtz18;

    // 주문가능액
    char order_pos_csamtz18[18];
    char _order_pos_csamtz18;

    // 미상환금
    char nordm_loan_amtz18[18];
    char _nordm_loan_amtz18;

    // 기타대여금
    char etc_lend_amtz18[18];
    char _etc_lend_amtz18;

    // 대용금액
    char subst_amtz18[18];
    char _subst_amtz18;

    // 대주담보금
    char sln_sale_amtz18[18];
    char _sln_sale_amtz18;

    // 매입원가(계좌합산)
    char bal_buy_ttamtz18[18];
    char _bal_buy_ttamtz18;

    // 평가금액(계좌합산)
    char bal_ass_ttamtz18[18];
    char _bal_ass_ttamtz18;

    // 순자산액(계좌합산)
    char asset_tot_amtz18[18];
    char _asset_tot_amtz18;

    // 활동유형
    char actvt_type10[10];
    char _actvt_type10;

    // 대출금
    char lend_amtz18[18];
    char _lend_amtz18;

    // 매도증거금
    char sl_mrgn_amtz18[18];
    char _sl_mrgn_amtz18;

    // 20%주문가능금액
    char pos_csamt1z18[18];
    char _pos_csamt1z18;

    // 30%주문가능금액
    char pos_csamt2z18[18];
    char _pos_csamt2z18;

    // 40%주문가능금액
    char pos_csamt3z18[18];
    char _pos_csamt3z18;

    // 100%주문가능금액
    char pos_csamt4z18[18];
    char _pos_csamt4z18;

    // D1예수금
    char dpsit_amt_d1_z15[15];
    char _dpsit_amt_d1_z15;

    // D2예수금
    char dpsit_amt_d2_z18[18];
    char _dpsit_amt_d2_z18;

    // 총평가손익
    char tot_eal_plsz15[15];
    char _tot_eal_plsz15;

    // 수익율
    char pft_rtz15[15];
    char _pft_rtz15;

    // 손익(원)
    char lsnpf_amt_wonz15[15];
    char _lsnpf_amt_wonz15;
} Ts8202OutBlock;

typedef struct tags8202OutBlock1 // 화면출력1, [반복]
{
    // 종목번호
    char issue_codez12[12];
    char _issue_codez12;

    // 종목명
    char issue_namez40[40];
    char _issue_namez40;

    // 잔고유형
    char bal_typez6[6];
    char _bal_typez6;

    // 대출일
    char loan_datez8[8];
    char _loan_datez8;

    // 잔고수량
    char bal_qtyz18[18];
    char _bal_qtyz18;

    // 미결제량
    char unstl_qtyz18[18];
    char _unstl_qtyz18;

    // 평균매입가
    char slby_amtz18[18];
    char _slby_amtz18;

    // 매입금액
    char byn_amtz18[18];
    char _byn_amtz18;

    // 현재가
    char prsnt_pricez18[18];
    char _prsnt_pricez18;

    // 손익(천원)
    char lsnpf_amtz18[18];
    char _lsnpf_amtz18;

    // 손익(원)
    char lsnpf_amt_wonz18[18];
    char _lsnpf_amt_wonz18;

    // 손익율
    char earn_ratez15[15];
    char _earn_ratez15;

    // 신용유형
    char mrgn_codez4[4];
    char _mrgn_codez4;

    // 잔량
    char jan_qtyz18[18];
    char _jan_qtyz18;

    // 만기일
    char expr_datez8[8];
    char _expr_datez8;

    // 평가금액
    char ass_amtz18[18];
    char _ass_amtz18;

    // 종목증거금율
    char issue_mgamt_ratez10[10];
    char _issue_mgamt_ratez10;

    // 평균매도가
    char medo_slby_amtz18[18];
    char _medo_slby_amtz18;

    // 매도손익
    char post_lsnpf_amtz18[18];
    char _post_lsnpf_amtz18;

    // 통화코드
    char cur_cdz3[3];
    char _cur_cdz3;

    // 국가코드명
    char nat_cd_nmz40[40];
    char _nat_cd_nmz40;

    // 상품유형명
    char pdt_tp_nmz50[50];
    char _pdt_tp_nmz50;

    // 종목중분류코드
    char iem_mlf_cdz5[5];
    char _iem_mlf_cdz5;
} Ts8202OutBlock1;

typedef struct tags8202
{
    Ts8202InBlock s8202inblock;         // 기본입력
    Ts8202OutBlock s8202outblock;       // 화면출력
    Ts8202OutBlock1 s8202outblock1[20]; // 화면출력1 , [반복]
} Ts8202;
