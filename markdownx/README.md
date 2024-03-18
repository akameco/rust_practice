## 目的
小さくはじめる、テストを含めて


## 実装

マークダウンパーサの実装

1. 字句解析 tokenizer, lexer テキストを字句の列(トークン)に分解
2. 構文解析 トークンをASTに変換, parser
3. 意味解析 ASTの意味を解析
4. 最適化 コード最適化
5. コード生成 generator ASTを入力としコード生成

`1→2→5` を実装する

### 実装
- [x] Tokenを定義する
- [x] Parseを実装する
- [x] Generatorを実装する

## 1. 字句解析 

以下の記法に限定して実装する

見出し h1~h6
テキスト

## 2. 構文解析 Parser

## 5. コード生成

## WASM実装
`wasm-pack` をインストールし、`wasm-pack build --target bunlder`を叩く。足りない要素が表示されたら、それを貼り付けて再実行。
`target`に`.wasm`があるので完成
webアプリにつなぎこむ。

`vite-plugin-wasm` を利用すると`import`するだけで使えて便利
