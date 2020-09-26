# Rustのインストールと環境設定

## Download & Install

- brewではなくrustupのサイトからインストールする。（VS Codeのrust拡張機能はrustupを使用するため）

[https://rustup.rs/](https://rustup.rs/)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- VS Codeにrustupのパスを通す

```bash
# パスを確認しコピーする
which rustup
```

`rust-client.rustupPath`に貼り付け
