FROM ubuntu:latest

ADD target/release/gtm-api /opt/gtm/gtm-api

EXPOSE 8000

ENTRYPOINT ["/opt/gtm/gtm-api"]
