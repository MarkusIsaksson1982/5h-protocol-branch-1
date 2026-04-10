FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo build --release
CMD ["cargo", "run", "--release", "--example", "full_flow"]
