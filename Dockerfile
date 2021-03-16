FROM ubuntu:latest

RUN apt-get update
RUN apt-get install -y libpq-dev
RUN apt-get update && apt-get -y install ca-certificates libssl-dev

VOLUME /gtm/gtm-api

ADD target/release/gtm-api /opt/gtm/gtm-api

EXPOSE 8000

WORKDIR /gtm/gtm-api

ENTRYPOINT ["/opt/gtm/gtm-api"]
