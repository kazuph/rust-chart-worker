use crate::charts::Chart;
use crate::models::GraphRequest;

pub struct CandlestickChart;

#[derive(Clone)]
pub struct CandleData {
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub label: Option<String>,
}

impl CandlestickChart {
    fn parse_ohlc_data(&self, request: &GraphRequest) -> Vec<CandleData> {
        // データが4の倍数でOHLC形式として解釈
        if !request.data.is_empty() && request.data.len() % 4 == 0 {
            let mut candles = Vec::new();
            for chunk in request.data.chunks(4) {
                candles.push(CandleData {
                    open: chunk[0],
                    high: chunk[1],
                    low: chunk[2],
                    close: chunk[3],
                    label: None,
                });
            }
            candles
        } else if let Some(series) = request.series.first() {
            // サンプルデータから模擬的なOHLCを生成
            series.data.iter().enumerate().map(|(_i, d)| {
                let base = d.value;
                let variation = base * 0.1;
                CandleData {
                    open: base + variation * 0.2,
                    high: base + variation,
                    low: base - variation,
                    close: base - variation * 0.3,
                    label: d.label.clone(),
                }
            }).collect()
        } else {
            Vec::new()
        }
    }
}

impl Chart for CandlestickChart {
    fn generate(&self, request: &GraphRequest) -> String {
        let candles = self.parse_ohlc_data(request);
        
        if candles.is_empty() {
            return String::new();
        }

        // 価格範囲を計算
        let mut min_price = f64::INFINITY;
        let mut max_price = f64::NEG_INFINITY;
        
        for candle in &candles {
            min_price = min_price.min(candle.low);
            max_price = max_price.max(candle.high);
        }
        
        let price_range = max_price - min_price;
        let padding = price_range * 0.1;
        min_price -= padding;
        max_price += padding;

        // SVG設定
        let width = 800.0;
        let height = 600.0;
        let margin = 60.0;
        let chart_width = width - 2.0 * margin;
        let chart_height = height - 2.0 * margin;
        
        let candle_width = chart_width / candles.len() as f64 * 0.6;
        let candle_spacing = chart_width / candles.len() as f64;

        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <style>
      .candlestick-up {{ fill: #2ecc71; stroke: #27ae60; stroke-width: 1; }}
      .candlestick-down {{ fill: #e74c3c; stroke: #c0392b; stroke-width: 1; }}
      .candlestick-wick {{ stroke: #333; stroke-width: 1; }}
      .axis {{ stroke: #333; stroke-width: 2; }}
      .axis-text {{ font-family: Arial, sans-serif; font-size: 12px; fill: #333; }}
      .title {{ font-family: Arial, sans-serif; font-size: 16px; font-weight: bold; fill: #333; text-anchor: middle; }}
      .grid {{ stroke: #ddd; stroke-width: 1; stroke-dasharray: 2,2; }}
    </style>
  </defs>
  <rect width="100%" height="100%" fill="white"/>"#,
            width, height
        );

        // タイトル
        if let Some(title) = &request.title {
            svg.push_str(&format!(
                r#"  <text x="{}" y="30" class="title">{}</text>"#,
                width / 2.0, title
            ));
        }

        // グリッド線
        for i in 0..=5 {
            let y = margin + (chart_height * i as f64 / 5.0);
            svg.push_str(&format!(
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" class="grid" />"#,
                margin, y, width - margin, y
            ));
        }

        // ローソク足を描画
        for (i, candle) in candles.iter().enumerate() {
            let x = margin + (i as f64 + 0.5) * candle_spacing;
            
            // 価格をY座標に変換
            let open_y = height - margin - ((candle.open - min_price) / (max_price - min_price)) * chart_height;
            let high_y = height - margin - ((candle.high - min_price) / (max_price - min_price)) * chart_height;
            let low_y = height - margin - ((candle.low - min_price) / (max_price - min_price)) * chart_height;
            let close_y = height - margin - ((candle.close - min_price) / (max_price - min_price)) * chart_height;
            
            let is_up = candle.close >= candle.open;
            let body_top = if is_up { close_y } else { open_y };
            let body_bottom = if is_up { open_y } else { close_y };
            let body_height = (body_bottom - body_top).abs().max(1.0);
            
            let class = if is_up { "candlestick-up" } else { "candlestick-down" };

            // 上髭線
            svg.push_str(&format!(
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" class="candlestick-wick" />"#,
                x, high_y, x, body_top
            ));

            // 下髭線
            svg.push_str(&format!(
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" class="candlestick-wick" />"#,
                x, body_bottom, x, low_y
            ));

            // ローソク足本体
            svg.push_str(&format!(
                r#"  <rect x="{}" y="{}" width="{}" height="{}" class="{}">
    <title>Open: {:.2}, High: {:.2}, Low: {:.2}, Close: {:.2}</title>
  </rect>"#,
                x - candle_width / 2.0, body_top, candle_width, body_height, class,
                candle.open, candle.high, candle.low, candle.close
            ));
        }

        // X軸
        svg.push_str(&format!(
            r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" class="axis" />"#,
            margin, height - margin, width - margin, height - margin
        ));

        // Y軸
        svg.push_str(&format!(
            r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" class="axis" />"#,
            margin, margin, margin, height - margin
        ));

        // Y軸ラベル
        for i in 0..=5 {
            let y = height - margin - (chart_height * i as f64 / 5.0);
            let price = min_price + (max_price - min_price) * i as f64 / 5.0;
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" class="axis-text" text-anchor="end">{:.2}</text>"#,
                margin - 10.0, y + 4.0, price
            ));
        }

        // X軸ラベル
        for (i, candle) in candles.iter().enumerate() {
            if i % ((candles.len() / 8).max(1)) == 0 {
                let x = margin + (i as f64 + 0.5) * candle_spacing;
                let default_label = format!("{}", i + 1);
                let label = candle.label.as_ref()
                    .map(|s| s.as_str())
                    .unwrap_or(&default_label);
                svg.push_str(&format!(
                    r#"  <text x="{}" y="{}" class="axis-text" text-anchor="middle">{}</text>"#,
                    x, height - margin + 20.0, label
                ));
            }
        }

        // 軸ラベル
        if let Some(x_label) = &request.x_label {
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" class="axis-text" text-anchor="middle">{}</text>"#,
                width / 2.0, height - 10.0, x_label
            ));
        }

        if let Some(y_label) = &request.y_label {
            svg.push_str(&format!(
                r#"  <text x="20" y="{}" class="axis-text" text-anchor="middle" transform="rotate(-90, 20, {})">{}</text>"#,
                height / 2.0, height / 2.0, y_label
            ));
        }

        svg.push_str("</svg>");
        svg
    }
}