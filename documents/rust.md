# Rust

公式ドキュメントのメモ、まとめ

## はじめに

### コンパイルと実行

```rust
fn main(){
    println!("Hello World!");
}
```



```bash
rustc hello.rs
hello
> Hello World!
```

### main()

- main関数は特別な関数である。
- C言語のmain関数のようにプログラムで実行される関数。

## その他

- インデントはスペース4
- 行の最後にはセミコロン
- `!`を使用すると関数ではなく**マクロ**として呼び出される。

## Cargo

Rustのパッケージ管理。Pythonでいうpip？

### 新しいプロジェクトの作成

```bash
cargo new hello_cargo
```

- tree

```text
hello_carg
├── Cargo.toml
└── src
   └── main.rs
```

- Cargo.toml

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
authors = ["Yuto Watanabe <yuto.w51942@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

### Build & Run

```bash
# build
cargo build

# run select binary file
./target/debug/hello_cargo

# build & run
cargo run
```

### Check

バイナリファイナルにしないでエラーをチェックする。

```bash
cargo check
```

