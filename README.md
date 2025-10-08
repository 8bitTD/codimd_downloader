## 概要
このリポジトリは、CodiMDページをhtml形式でダウンロードするツールです。<br>
ページ内のCodiMDのページも再帰でダウンロードされます。<br>
また、CodiMD内で使われているリソースもダウンロードされます。

<img width="669" height="362" alt="image" src="https://github.com/user-attachments/assets/5ea08764-31b9-4bd9-8b10-6a2f9321b071" />

## 使い方
### サーバー側
事前にサーバー側に[こちら](https://github.com/8bitTD/codimd_display)のjsonで取得できるサイトを構築してください
### ツール実行
[Rust](https://rust-lang.org/ja/)をインストール後に
```cmd
git clone https://github.com/8bitTD/codimd_downloader
cd codimd_downloader
cargo run --release
```
GUI起動後に必要な設定をして実行ボタンを押してください。<br>
output_path内に*.htmlとリソースがダウンロードされます。<br>

<img width="562" height="251" alt="image" src="https://github.com/user-attachments/assets/7cb1fcc5-5317-4f27-8724-c2de098cbc15" />
##環境
### サーバー環境
Ubuntu 24.04.3 LTS
rustc 1.90.0
CodiMD 2.6.1
### 実行環境
Windows 10 Home
rustc 1.90.0
Windows10 Home
