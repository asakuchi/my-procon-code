# procon

プロコン用リポジトリ

## テンプレート

```sh
cargo generate --git https://github.com/rust-lang-ja/atcoder-rust-base --branch ja
```

## target

target は root ディレクトリに実態を置いて、各ディレクトリの target はそれを参照するシンボリックリンクにしている。

## プロコン前に

テンプレートからコピーする

```sh
cp -ar abc001/ welcome
```

## 問題を解く前に

target を共有しているので解く前にバイナリを削除する。

```sh
cargo clean -p procon
cargo test
```
