# Rust Chart Worker

Cloudflare WorkersでRustを使用して動的にグラフを生成するサービスです。
plottersライブラリを使用して、折れ線グラフ、棒グラフ、散布図などを生成できます。

## 必要条件

- Rust
- wrangler (Cloudflare Workers CLI)
- curl (テスト用)

## セットアップ

```bash
# wranglerのインストール
npm install -g wrangler

# 依存関係のインストール
cargo install worker-build
```

## 実行方法

ローカル開発サーバーの起動:

```bash
npx wrangler dev
```

デフォルトでは `http://localhost:8787` でサービスが起動します。

## テスト用curlコマンド

### 1. 折れ線グラフ（デフォルト）
```bash
curl -X POST http://localhost:8787 \
  -H "Content-Type: application/json" \
  -d '{"graph_type": "line", "data": [10, 20, 15, 25, 30, 20, 35, 40, 30, 45]}' \
  -o line_chart.png
```

### 2. 棒グラフ
```bash
curl -X POST http://localhost:8787 \
  -H "Content-Type: application/json" \
  -d '{"graph_type": "bar", "data": [10, 20, 15, 25, 30, 20, 35, 40, 30, 45]}' \
  -o bar_chart.png
```

### 3. 散布図
```bash
curl -X POST http://localhost:8787 \
  -H "Content-Type: application/json" \
  -d '{"graph_type": "scatter", "data": [10, 20, 15, 25, 30, 20, 35, 40, 30, 45]}' \
  -o scatter_plot.png
```

### 4. カスタマイズオプションの使用例
```bash
# タイトルと軸ラベルを指定
curl -X POST http://localhost:8787 \
  -H "Content-Type: application/json" \
  -d '{
    "graph_type": "bar",
    "data": [10, 20, 15, 25, 30],
    "title": "Monthly Sales 2024",
    "x_label": "Month",
    "y_label": "Sales (millions)"
  }' \
  -o custom_chart.png
```

### 5. サイン波データのテスト
```bash
curl -X POST http://localhost:8787 \
  -H "Content-Type: application/json" \
  -d "{\"graph_type\": \"line\", \"data\": $(python3 -c 'import math; print([math.sin(x/10)*10 + 20 for x in range(50)])')}" \
  -o sine_wave.png
```

### 6. ランダムデータのテスト
```bash
curl -X POST http://localhost:8787 \
  -H "Content-Type: application/json" \
  -d "{\"graph_type\": \"line\", \"data\": $(python3 -c 'import random; print([random.uniform(0, 100) for _ in range(20)])')}" \
  -o random_data.png
```

## APIの仕様

### エンドポイント
- POST /

### リクエストボディ
```json
{
  "graph_type": string,  // "line", "bar", "scatter"のいずれか
  "data": number[],      // 描画するデータポイントの配列
  "title": string,       // (オプション) グラフのタイトル
  "x_label": string,     // (オプション) X軸のラベル（デフォルト: "Index"）
  "y_label": string      // (オプション) Y軸のラベル（デフォルト: "Value"）
}
```

### レスポンス
- Content-Type: image/png
- 生成されたグラフ画像がPNG形式で返却されます

### エラーレスポンス
- 405: Method Not Allowed - POSTメソッド以外でアクセスした場合
- 400: Bad Request - 不正なJSONまたは空のデータ配列
- 500: Internal Server Error - グラフ生成時のエラー

## デプロイ

Cloudflare Workersへのデプロイ:

```bash
npx wrangler deploy
```

## ライセンス

MIT
