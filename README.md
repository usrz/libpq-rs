# Work in progress

Mainly this is a research project, it'll probably never see the light of the
day, but it's simply my exploration of Rust.

The scope is to bring a statically-compiled version of LibPQ and expose it
as a NodeJS module, using the OpenSSL version embedded with NodeJS itself.

Basically, a zero-dependencies, native, PostgreSQL client written in Rust.

### Visual Studio Code settings

To use VSCode with the OpenSSL include files provided by NodeJS we need to
add few extra bits to `.vscode/setting.json`:

```json
{
  "rust-analyzer.cargo.extraEnv": {
    "RUSTFLAGS": "-C link-args=-Wl,-undefined,dynamic_lookup",
    "OPENSSL_LIB_DIR": "/usr/local/share/node-20.12.2/lib",
    "OPENSSL_INCLUDE_DIR": "/usr/local/share/node-20.12.2/include/node",
    "OPENSSL_STATIC": "0",
    "OPENSSL_LIBS": ""
  }
}
```
