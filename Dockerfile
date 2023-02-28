FROM rust:buster
COPY . /opt/bot
WORKDIR /opt/bot
RUN cargo install --path .

FROM debian:buster
LABEL corgo.language="rust"
COPY --from=0 /usr/local/cargo/bin/golden-god-bot /opt/golden-god-bot
COPY ./json/ /
RUN apt-get update && apt-get upgrade
ENTRYPOINT /opt/golden-god-bot