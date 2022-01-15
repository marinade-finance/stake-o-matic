FROM alpine:3.14

RUN apk update
RUN apk upgrade
RUN apk add --no-cache bash sqlite rust cargo openssl-dev eudev-dev linux-headers

WORKDIR /usr/local/

ADD program program/
ADD cli cli/
ADD bot bot/
ADD sql sql/
ADD Cargo.lock .
ADD Cargo.toml .
ADD score-all-mainnet.sh .
ADD clean-score-all-mainnet.sh .
ADD import-into-sqlite.sh .

RUN cargo build

CMD sqlite3 --version

CMD ./clean-score-all-mainnet.sh
