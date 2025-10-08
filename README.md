## 概要
このリポジトリは、CodiMDページをhtml形式でダウンロードするツールです。<br>
ページ内のCodiMDのページも再帰でダウンロードされます。<br>
また、CodiMD内で使われているリソースもダウンロードされます。

<img width="669" height="362" alt="image" src="https://github.com/user-attachments/assets/5ea08764-31b9-4bd9-8b10-6a2f9321b071" />

## 使い方
[Rust](https://rust-lang.org/ja/)をインストール後に
```cmd
git clone https://github.com/8bitTD/codimd_downloader
cd codimd_downloader
cargo run --release
```
GUI起動後に必要な設定をして実行ボタンを押してください。<br>
output_path内に*.htmlとリソースがダウンロードされます。<br>

<img width="562" height="251" alt="image" src="https://github.com/user-attachments/assets/7cb1fcc5-5317-4f27-8724-c2de098cbc15" />
