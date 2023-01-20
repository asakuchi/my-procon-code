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

## crate memo

### itertools;

https://docs.rs/itertools/0.9.0/itertools/index.html

```rust

//
// permutations 順列
// combinations 組み合わせ
// combinations_with_replacement 重複組み合わせ
//
use itertools::Itertools;


for permutation in (0..3).permutations(2) {
    println!("{:?}", permutation);
}

for combination in (0..3).combinations(2) {
    println!("{:?}", combination);
}

// 要素3つ以上のzip
use itertools::izip;

let number_list = [1, 2, 3, 4, 5];
let text_list = ["hello", "world"];
let char_list = ['a', 'b', 'c'];

for (&number_value, &text_value, &char_value) in
    izip!(number_list.iter(), text_list.iter(), char_list.iter())
{
    println!("{} {} {}", number_value, text_value, char_value);
}
```

## superslice

```rust
use superslice::*;

// lower_bound
// upper_bound
```

## work でプログラム書いた後、別のディレクトリにコピーする

```sh
find work -name \*.rs | awk -F'/' '{ printf "cp -pr %s abc/src/bin/abc260_%s\n", $0, $4}' | zsh
```

<!-- ## ACL 利用

```sh
cd ac-library-rs
python3 expand.py modint | pbcopy
``` -->

## ACL や自作ライブラリを使うとき

`a.rs` を提出する場合

```sh
cargo equip --exclude-atcoder-crates --minify libs --remove docs --remove comments --bin a | pbcopy
```
