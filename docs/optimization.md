# Cloudflare Workers WebAssemblyサイズ最適化

## 背景
- Cloudflare Workersには4MBのサイズ制限がある
- 当初のビルドサイズは約4.1MB（gzip: 2.9MB）で制限に近い状態だった
- 日本語フォントの必要性からフォントファイルの削除は選択肢から除外

## 最適化方針
1. **未使用コードの削除**
   - 未使用のメソッドやトレイトの削除
   - 未使用のインポートの整理
   - 重複コードの統合

2. **依存関係の最適化**
   - 必要最小限の機能のみを有効化
   - デフォルト機能の無効化
   - コンパイル設定の最適化

3. **コード構造の改善**
   - ライフタイム問題の解決
   - 所有権の適切な管理
   - SVG属性の記述方法の統一

## 実装変更

### 1. 依存関係の最適化
```toml
[dependencies]
worker = { version = "0.5.0", features = ["http"], default-features = false }
worker-macros = { version = "0.5.0", features = ["http"] }
serde = { version = "1.0", features = ["derive"], default-features = false }
serde_json = { version = "1.0", default-features = false }
resvg = { version = "0.35.0", features = ["text"], default-features = false }
tiny-skia = { version = "0.10.0", default-features = false }
usvg = { version = "0.35.0", features = ["text"], default-features = false }
fontdb = { version = "0.14.1", default-features = false }
getrandom = { version = "0.2", features = ["js"], default-features = false }
```

### 2. コンパイル最適化設定
```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = ['-Os']

[profile.release]
opt-level = 's'
lto = true
codegen-units = 1
panic = 'abort'
```

### 3. コード最適化
1. `needs_axes`メソッドの削除
   - 使用されていないトレイトメソッドを削除
   - 各チャートの実装から削除

2. 未使用インポートの削除
   ```rust
   // Before
   use crate::utils::{self, svg};
   use std::cmp::Ordering;

   // After
   use crate::utils;
   ```

3. 文字列の所有権最適化
   ```rust
   // Before
   let color = point.color.as_ref().unwrap_or_else(|| {
       series_item.color.as_ref().unwrap_or(&"#0000FF".to_string())
   });

   // After
   let color = match &point.color {
       Some(c) => c.clone(),
       None => match &series_item.color {
           Some(c) => c.clone(),
           None => "#0000FF".to_string(),
       },
   };
   ```

## 結果
- 初期サイズ: 4.1MB（gzip: 2.9MB）
- 最適化後: 3.4MB（gzip: 1.7MB）
- 削減率: 約20%（gzip: 約41%）

## 今後の課題
1. 残存する警告の対応
   - 未使用の`format_number`関数
   - 未使用の`generate_x_axis_ticks_for_bar`関数
   - 未使用の`generate_x_axis_ticks`関数
   - `DataPoint`の`label`フィールド（機能として必要なため保持）

2. 検討可能な追加最適化
   - フォントのサブセット化（使用頻度の高い文字のみを含める）
   - チャートタイプのフィーチャーフラグ導入（必要なチャートのみをビルドに含める）
   - SVGテンプレートの共通化（重複コードの削減）
