FROM rust:1.50

COPY ./src/server/ /rust-app

ENTRYPOINT ["sh"]