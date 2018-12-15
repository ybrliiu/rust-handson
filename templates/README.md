# rust-handson

2018/12/15開催のRust入門者向けハンズオン #5向けリポジトリになります。

必要に応じてご利用ください。

## 事前準備

```bash
# 過去にwasm-bindgen-cliをインストールしている方はSKIPしてください
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-bindgen-cli
```

## build

```bash
$ yarn dev:build
```

## run

``bash
$ yarn dev:run
```

## 実行に必要なもの

* Rustのコンパイラ
  * [こちら](https://www.rust-lang.org/learn/get-started)を参考にしてください
* Node.js
  * [こちら](https://nodejs.org/ja/download/)を参考にしてください
