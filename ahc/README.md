# AHC

## インタラクティブでの入力の受け取り方

`source` を指定すれば `proconio` でも大丈夫。

```rust
use proconio::{input, source::line::LineSource};
use std::io::{stdin, BufReader};

let stdin = stdin();
let mut source = LineSource::new(BufReader::new(stdin.lock()));

input! {
    from &mut source,
    n: usize,
}
```

https://atcoder.jp/contests/abc244/editorial/3625
