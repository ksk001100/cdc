# cdc - CSV Duplicate Checker

CSVファイル内の指定したカラムで重複している値を検出するCLIツールです。

## 特徴

- シンプルで使いやすいコマンドラインインターフェース
- 重複している値の行番号と出現回数を表示
- 高速なCSV処理

## インストール

### ビルドから

```bash
cargo build --release
```

ビルドされたバイナリは `target/release/cdc` に生成されます。

### 使用方法

```bash
cdc <ヘッダー名> <ファイルパス>
```

### 引数

- `<ヘッダー名>`: 重複チェックを行うCSVのカラム名
- `<ファイルパス>`: チェック対象のCSVファイルのパス

## 使用例

### 基本的な使い方

```bash
./target/release/cdc email users.csv
```

このコマンドは `users.csv` ファイル内の `email` カラムで重複をチェックします。

### 出力例

重複が見つかった場合:

```
Duplicates found in column 'email':

Value: 'test@example.com'
  Lines: [2, 5, 8]
  Count: 3

Value: 'admin@example.com'
  Lines: [10, 15]
  Count: 2
```

重複が見つからなかった場合:

```
No duplicates found in column 'email'
```

## 技術スタック

- [Rust](https://www.rust-lang.org/) - プログラミング言語
- [seahorse](https://github.com/ksk001100/seahorse) - CLIフレームワーク
- [csv](https://github.com/BurntSushi/rust-csv) - CSV処理ライブラリ

## ライセンス

MIT
