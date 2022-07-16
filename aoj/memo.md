# memo

## ディレクトリ変更メモ

```sh
find aoj -name \*.rs | awk -F'/' '{ printf "cp -pr %s aoj*wk/src/bin/%s*%s.rs\n", $0, $2, $3}'

cp -pr aoj/GRL_2/A/src/main.rs aoj_wk/src/bin/GRL_2_A.rs
cp -pr aoj/GRL_4/B/src/main.rs aoj_wk/src/bin/GRL_4_B.rs
cp -pr aoj/ALDS1_5/A/src/main.rs aoj_wk/src/bin/ALDS1_5_A.rs
cp -pr aoj/ALDS1_4/B/src/main.rs aoj_wk/src/bin/ALDS1_4_B.rs
cp -pr aoj/ALDS1_11/A/src/main.rs aoj_wk/src/bin/ALDS1_11_A.rs
cp -pr aoj/ALDS1_11/C/src/main.rs aoj_wk/src/bin/ALDS1_11_C.rs
cp -pr aoj/DPL_1/A/src/main.rs aoj_wk/src/bin/DPL_1_A.rs
cp -pr aoj/DPL_1/C/src/main.rs aoj_wk/src/bin/DPL_1_C.rs
cp -pr aoj/DPL_1/D/src/main.rs aoj_wk/src/bin/DPL_1_D.rs
cp -pr aoj/DPL_1/B/src/main.rs aoj_wk/src/bin/DPL_1_B.rs
cp -pr aoj/ALDS1_10/A/src/main.rs aoj_wk/src/bin/ALDS1_10_A.rs
cp -pr aoj/ALDS1_10/C/src/main.rs aoj_wk/src/bin/ALDS1_10_C.rs
cp -pr aoj/template/A/src/main.rs aoj_wk/src/bin/template_A.rs
cp -pr aoj/ITP1_7/B/src/main.rs aoj_wk/src/bin/ITP1_7_B.rs
cp -pr aoj/ITP1_1/A/src/main.rs aoj_wk/src/bin/ITP1_1_A.rs
cp -pr aoj/ITP1_1/C/src/main.rs aoj_wk/src/bin/ITP1_1_C.rs
cp -pr aoj/ITP1_1/D/src/main.rs aoj_wk/src/bin/ITP1_1_D.rs
cp -pr aoj/ITP1_1/B/src/main.rs aoj_wk/src/bin/ITP1_1_B.rs
cp -pr aoj/GRL_1/A/src/main.rs aoj_wk/src/bin/GRL_1_A.rs
cp -pr aoj/GRL_1/C/src/main.rs aoj_wk/src/bin/GRL_1_C.rs
cp -pr aoj/NTL_1/A/src/main.rs aoj_wk/src/bin/NTL_1_A.rs
cp -pr aoj/NTL_1/B/src/main.rs aoj_wk/src/bin/NTL_1_B.rs
cp -pr aoj/DSL_1/A/src/main.rs aoj_wk/src/bin/DSL_1_A.rs
cp -pr aoj/ALDS1_12/A/src/main.rs aoj_wk/src/bin/ALDS1_12_A.rs
```
