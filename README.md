# AtCoder Random Picker 🚀

AtCoder Random Pickerは、AtCoder の問題の中から指定した難易度範囲に合致する問題をランダムに提示するWebサービスです。フロントエンドとバックエンドが独立しつつ連携して動作しています。

---

## 概要

- 指定した最低Diffと最高Diffの範囲内でランダムなAtCoder問題を取得
- 表示された問題のURLから直接AtCoderの問題ページへアクセス可能
- 問題のDiff確認およびレート制限（1IPあたり1秒/リクエスト）を実施

---

## 使い方

1. **最低 Diff / 最高 Diff の入力**  
   - 最低 Diffは0以上、最高 Diffは最低 Diff以上で入力してください
2. **「Pick」ボタンを押す**  
   - ボタン押下で問題がランダムに取得され、押した直後は約1秒間ボタンが無効化され、ロード中アイコンが表示されます
3. **問題表示**  
   - 問題記号、名前、URLが表示され、また「Show Difficulty」ボタンでDiffが確認できます
4. **直接アクセス**  
   - 表示された問題URLをクリックすると、AtCoderの問題ページに直接アクセス可能です

---

## ディレクトリ構成

### バックエンド

- **技術**: Rust, Hyper  
- **主なファイル**:
  - `Cargo.toml`: Rustの依存関係定義
  - `Dockerfile`、`fly.toml`: コンテナ化・デプロイ関連設定
  - `src/`: API実装、ルーティング(`utils/routing.rs`)、レートリミッター(`utils/ratelimiter.rs`)等
- **API仕様**:
  - エンドポイント: GET `/`
  - クエリパラメータ: `?min=<最低Diff>&max=<最高Diff>`
  - レスポンス: JSON形式で問題情報（URL、Diffなど）を返却

### フロントエンド

- **技術**: Svelte, Vite, Rabee UI, Tailwind CSS, PostCSS  
- **主なファイル**:
  - 環境設定: `.env.development`, `.env.production`
  - エントリーポイント: `src/main.ts`, `src/App.svelte`
  - UI関連コンポーネント: `src/components/` 以下（Button, Input, Dialog, Label, Message等）
- **動作概要**:
  - ユーザーはDiffを入力し「Pick」ボタンを押すと問題が取得される
  - ローディング状態や入力チェックにより、UIが適切に制御される

---

## セットアップ

### バックエンド

1. Rustがインストールされていることを確認
2. リポジトリルートで以下のコマンドを実行：
  ```sh
  cd backend
  cargo build
  cargo run
  ```
3. ブラウザで [https://localhost:3000](https://localhost:3000) にアクセスして動作確認
4. DockerやFly.dev経由でのデプロイも利用可能（`Dockerfile`, `fly.toml`参照）

### フロントエンド

1. Bunがインストールされていることを確認
2. リポジトリルートで以下のコマンドを実行：
  ```sh
  cd frontend
  bun install
  bun run dev
  ```
3. ブラウザで [http://localhost:5173](http://localhost:5173) にアクセスして動作確認

### 公開環境の例:
  - フロントエンド: https://twil3akine/atrp
  - バックエンド API: https://atcoder-random-picker-be.fly.dev
  これらは参考の公開先であり、ローカル開発時は上記手順でローカルサーバーを起動してください。

---

## CI/CD

- GitHub Actionsを利用したCI/CDパイプラインを構築しており、`.github/workflows/` 配下に設定ファイルがあります。

---

## 注意点

- **入力チェック**: 最低 Diffが負数、または最高 Diffが最低 Diffより小さい場合はエラーとなります
- **レート制限**: 1IPにつき1秒間に1リクエストと制限されています

---

## 開発者向け情報

- **APIエンドポイント**: `GET /?min=<最低Diff>&max=<最高Diff>`  
  JSON形式で問題情報を取得できます

- クエリパラメータ:
  - min (整数) — 最低 Diff。0 以上の整数。
  - max (整数) — 最高 Diff。min 以上 3854 以下の整数。
- バリデーション:
  - min が負数、または max < min の場合は 400 Bad Request を返します。
- レート制限:
  - 1 IP あたり 1秒に1リクエスト（429 Too Many Requests を返す場合あり）。
- レスポンス例 (成功, 200):
  ```json
  {
    "id": "abc001_a",
    "title": "Sample Problem",
    "url": "https://atcoder.jp/contests/abc001/tasks/abc001_1",
    "difficulty": 1200
  }
  ```
- エラー例:
  - 400 Bad Request: バリデーションエラー（メッセージ付与）
  - 429 Too Many Requests: レート制限に抵触した場合
- 使用例:
  - curl:
    ```sh
    curl "https://atcoder-random-picker-be.fly.dev/?min=0&max=2000"
    ```
  - ブラウザ: https://atcoder-random-picker-be.fly.dev/?min=400&max=1200

---