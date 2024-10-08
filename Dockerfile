# ============== Container setup
FROM scottyhardy/docker-wine:stable-9.0 AS setup

VOLUME /namu/NPKI

USER root

RUN apt-get update

# Basic font files for debugging, window manager for dummy x window
RUN apt-get install -y \
    fonts-nanum fonts-nanum-coding

# qvopenapi 어딘가에 브라우저를 띄우는 코드가 숨어있는거로 추정 
# /usr/lib/firefox/firefox http://www.mynamuh.com/tx/wooriwmBoard/boardList.action?sBoard_Id=25&sType_Cd=3000000000&isHTS=Y
# 위와 같이 firefox가 호출되서 cpu, mem 사용량이 폭등한다. 이를 방지하려고 브라우저 제거
RUN apt-get remove -y firefox

# Set correct locale
RUN locale-gen ko_KR.UTF-8 && \
    update-locale LANG=ko_KR.UTF-8 LC_MESSAGES=POSIX

# Create wineuser (entrypoint is inherited from docker-wine image)
# Bunch of user creation, permission setup and etc.
RUN /usr/bin/entrypoint echo Done

RUN mkdir -p /namu /home/wineuser/.wine/drive_c/users/wineuser/AppData/LocalLow && chown -R wineuser:wineuser /namu /home/wineuser
WORKDIR /namu

USER wineuser

RUN wine cmd.exe /c "echo winesetup complete"

# ============== Download DLL
FROM setup AS dll

USER root
RUN apt-get update && apt-get install -y curl
USER wineuser

RUN curl -O https://download.nhqv.com/download/iflgtrading/openapi.qv.zip && \
    unzip openapi.qv.zip -d openapi.qv && \
    cp -R openapi.qv/bin bin && \
    rm -R openapi.qv && rm openapi.qv.zip

# ============== Container
FROM setup

RUN ln -s /namu/NPKI /home/wineuser/.wine/drive_c/users/wineuser/AppData/LocalLow/NPKI

COPY target/i686-pc-windows-gnu/release/qvopenapi-http.exe /namu/bin/qvopenapi-http.exe
COPY --from=dll /namu/bin /namu/bin
COPY docker/entrypoint.sh /namu/entrypoint.sh

# USER root
# ENTRYPOINT /usr/bin/entrypoint /bin/bash
ENTRYPOINT /namu/entrypoint.sh
