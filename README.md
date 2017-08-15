# tandem-repeat
*Note: this repositry is not for production use.*

このプログラムは「配列解析アルゴリズム特論」レポート課題の一環で作成されたものです．

# 使用方法

## Rust開発環境の準備
このプログラムはRust言語で書かれています．まだRust開発環境がインストールされていない場合は，以下のページからRust開発環境をインストールしてください．

https://www.rust-lang.org/

## ベンチマーク
実装された4種類のアルゴリズムの動作時間を比較するには，本プログラムのディレクトリ内で以下のコマンドを実行してください．
```sh
cargo bench
```

なお、入力文字列の長さなどを変更したい場合は，ベンチマーク用ソースコードである`benches/bench.rs`を編集してください．

## 繰り返し文字列発見器の利用
実装されたアルゴリズムを使用するには，次のコマンドでプロセスを起動してください．
```sh
cargo run --release
```
標準文字列に入力された文字列から繰り返し文字列を探します．

また、次のように`--algorithm`オプション（または`-a`オプション）で使用アルゴリズムを指定することができます．
```sh
cargo run --release -- --algorithm ultranaive
```
アルゴリズムは以下の文字列または数値で指定してください．
* 1. `ultranaive` : 超ナイーブ法
* 2. `divide` : 分割統治法
* 3. `lcp` : LCP + 超ナイーブ法
* 4. `lcp_divide` : LCP + 分割統治法

アルゴリズムを指定しなかった場合，実装されている中で最速の`lcp_divide`が選択されます．

# LICENSE
The MIT License