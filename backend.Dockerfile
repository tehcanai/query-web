FROM rust:1.83.0
WORKDIR /backend
COPY backend/Cargo.toml ./
COPY backend ./
RUN cargo build --release
CMD ["./target/release/backend"]
