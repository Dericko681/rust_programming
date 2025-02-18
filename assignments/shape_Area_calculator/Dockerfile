FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/shape_area_calculator"]