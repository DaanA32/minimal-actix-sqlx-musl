# Minimal actix / sqlx with MUSL

1. Install musl-gcc:
  - eg: sudo apt install musl-tools
2. Setup musl openssl:
  - https://qiita.com/liubin/items/6c94f0b61f746c08b74c
3. Setup place mysql db link in .env
4. Create model table:
    eg:
    ``` sql
    create table musl (demo varchar(255));
    ```
5. Install rustup target
    ``` bash
    rustup target add x86_64-unknown-linux-musl
    ```
6. Build musl (ensure you have x86_64-unknown-linux-musl target):
    ``` bash
    PKG_CONFIG_ALLOW_CROSS=1 OPENSSL_STATIC=true OPENSSL_DIR=/musl cargo build --target x86_64-unknown-linux-musl
    ```
    or with [cargo make](https://github.com/sagiegurari/cargo-make)
    ``` bash
    cargo make debug_musl
    ```
7. Verify linking:
    ``` bash
    $ ldd target/x86_64-unknown-linux-musl/debug/musl_demo                                                                                  11:09:07
    statically linked
    ```