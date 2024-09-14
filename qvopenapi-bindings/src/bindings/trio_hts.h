// X08a8594.fdf[조회용 해외증권잔고]
// X10a8285.fdf[실시간 해외증권잔고]
typedef struct tags8064InBlock // 기본입력
{
    // 계좌번호
    char act_noz11[11];
    char _act_noz11;

    // 비밀번호
    char pwdz44[44];
    char _pwdz44;

    // 시세조회구분코드
    char qut_iqr_dit_cdz1[1];
    char _qut_iqr_dit_cdz1;

    // 외화증권거래국가코드
    char fc_sec_trd_nat_cdz3[3];
    char _fc_sec_trd_nat_cdz3;

    // 통화코드
    char cur_cdz3[3];
    char _cur_cdz3;

    // 비용구분코드
    char xns_dit_cdz1[1];
    char _xns_dit_cdz1;

    // 수수료구분코드
    char fee_dit_cdz1[1];
    char _fee_dit_cdz1;

    // 거래비밀번호1
    char trad_pswd_no_1z44[44];
    char _trad_pswd_no_1z44;

    // 거래비밀번호2
    char trad_pswd_no_2z44[44];
    char _trad_pswd_no_2z44;
} Ts8064InBlock;

typedef struct tags8064OutBlock // 화면출력
{
    // krw_dca
    char krw_dcaz18[18];
    char _krw_dcaz18;

    // krw_ny_stl_xcl_amt
    char krw_ny_stl_xcl_amtz18[18];
    char _krw_ny_stl_xcl_amtz18;

    // fc_dca
    char fc_dcaz15[15];
    char _fc_dcaz15;

    // fc_ny_stl_xcl_amt
    char fc_ny_stl_xcl_amtz15[15];
    char _fc_ny_stl_xcl_amtz15;
} Ts8064OutBlock;

typedef struct tags8064OutBlock1 // 화면출력, [반복]
{
    char krw_eal_amtz18[18];
    char _krw_eal_amtz18;
} Ts8064OutBlock1;

typedef struct tags8064
{
    Ts8064InBlock s8064inblock;         // 기본입력
    Ts8064OutBlock s8064outblock;       // 화면출력
    Ts8064OutBlock1 s8064outblock1[20]; // 화면출력 , [반복]
} Ts8064;
