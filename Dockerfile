FROM rust:1.67.1 AS builder

WORKDIR /opt/boolean
COPY . .

RUN cargo install --path .

FROM builder AS development

RUN rustup component add rustfmt
CMD ["cargo", "run", "--", "boolean"]

FROM builder AS production

CMD ["boolean"]
