// X10a8285.fdf[실시간 해외증권잔고]
// 계좌명/비밀번호조회(c8010)
typedef struct tagc8010InBlock // 기본입력
{
    char _blank;
} Tc8010InBlock;

typedef struct tagc8010OutBlock // 화면출력
{
    // accnt_namez40
    char accnt_namez40[40];
    char _accnt_namez40;

    // pass_wdz8
    char pass_wdz8[8];
    char _pass_wdz8;
    
    // pass2_wdz8
    char pass2_wdz8[8];
    char _pass2_wdz8;

    // 0:정상1:오류
    char valid_codez1[1];
    char _valid_codez1;
} Tc8010OutBlock;

typedef struct tagc8010
{
    Tc8010InBlock c8010inblock;         // 기본입력
    Tc8010OutBlock c8010outblock;       // 화면출력
} Tc8010;
