FROM ekidd/rust-musl-builder:latest as builder
WORKDIR /home/rust/src
COPY . .
RUN cargo build --locked --release --verbose
RUN cp /home/rust/src/target/x86_64-unknown-linux-musl/release/battleship .
RUN strip /home/rust/src/battleship

FROM scratch
COPY --from=builder /home/rust/src/battleship .
USER 1000:1000
ENV BATTLESHIP_SOCKET=0.0.0.0:1234
EXPOSE 1234
CMD ["./battleship"]
