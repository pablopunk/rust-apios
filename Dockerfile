FROM zeithq/rust-nightly
EXPOSE 80
CMD ROCKET_ENV=production cargo run --release
