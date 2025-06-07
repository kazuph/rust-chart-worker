use crate::charts::Chart;
use crate::models::GraphRequest;

pub struct BubbleChart {
    pub min_bubble_size: f64,
    pub max_bubble_size: f64,
    pub show_labels: bool,
}

impl Default for BubbleChart {
    fn default() -> Self {
        Self {
            min_bubble_size: 5.0,
            max_bubble_size: 30.0,
            show_labels: false,
        }
    }
}

#[derive(Clone)]
pub struct BubbleData {
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub label: Option<String>,
    pub color: Option<String>,
}

impl BubbleChart {
    fn parse_bubble_data(&self, request: &GraphRequest) -> Vec<BubbleData> {
        if let Some(series) = request.series.first() {
            // データポイントから3次元データ（x, y, size）を生成
            series.data.iter().enumerate().map(|(i, d)| {
                BubbleData {
                    x: i as f64,
                    y: d.value,
                    size: d.value.abs() * 0.5 + 10.0, // サイズはY値に基づく
                    label: d.label.clone(),
                    color: d.color.clone(),
                }
            }).collect()
        } else if !request.data.is_empty() {
            // データが3の倍数の場合はx, y, sizeとして解釈
            if request.data.len() % 3 == 0 {
                request.data.chunks(3).map(|chunk| {
                    BubbleData {
                        x: chunk[0],
                        y: chunk[1],
                        size: chunk[2],
                        label: None,
                        color: None,
                    }
                }).collect()
            } else {
                // そうでなければY値として扱い、Xは連番、サイズはY値ベース
                request.data.iter().enumerate().map(|(i, &y)| {
                    BubbleData {
                        x: i as f64,
                        y,
                        size: y.abs() * 0.5 + 10.0,
                        label: None,
                        color: None,
                    }
                }).collect()
            }
        } else {
            Vec::new()
        }
    }
}

impl Chart for BubbleChart {
    fn generate(&self, request: &GraphRequest) -> String {
        let bubbles = self.parse_bubble_data(request);
        
        if bubbles.is_empty() {
            return String::new();
        }

        // データ範囲を計算
        let min_x = bubbles.iter().map(|b| b.x).fold(f64::INFINITY, f64::min);
        let max_x = bubbles.iter().map(|b| b.x).fold(f64::NEG_INFINITY, f64::max);
        let min_y = bubbles.iter().map(|b| b.y).fold(f64::INFINITY, f64::min);
        let max_y = bubbles.iter().map(|b| b.y).fold(f64::NEG_INFINITY, f64::max);
        let min_size = bubbles.iter().map(|b| b.size).fold(f64::INFINITY, f64::min);
        let max_size = bubbles.iter().map(|b| b.size).fold(f64::NEG_INFINITY, f64::max);

        // SVG設定
        let width = 800.0;
        let height = 600.0;
        let margin = 80.0; // バブルが切れないよう大きめのマージン
        let chart_width = width - 2.0 * margin;
        let chart_height = height - 2.0 * margin;

        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <style>
      .bubble {{ fill-opacity: 0.7; stroke: #333; stroke-width: 1; }}
      .bubble:hover {{ fill-opacity: 0.9; stroke-width: 2; }}
      .bubble-label {{ font-family: Arial, sans-serif; font-size: 10px; fill: #333; text-anchor: middle; dominant-baseline: middle; }}
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
            // 水平線
            let y = margin + (chart_height * i as f64 / 5.0);
            svg.push_str(&format!(
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" class="grid" />"#,
                margin, y, width - margin, y
            ));
            
            // 垂直線
            let x = margin + (chart_width * i as f64 / 5.0);
            svg.push_str(&format!(
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" class="grid" />"#,
                x, margin, x, height - margin
            ));
        }

        // カラーパレット
        let colors = request.colors.as_ref().map(|c| c.clone()).unwrap_or_else(|| vec![
            "#3498db".to_string(), "#e74c3c".to_string(), "#2ecc71".to_string(),
            "#f39c12".to_string(), "#9b59b6".to_string(), "#1abc9c".to_string(),
        ]);

        // バブルを描画
        for (i, bubble) in bubbles.iter().enumerate() {
            // 座標を変換
            let x = margin + ((bubble.x - min_x) / (max_x - min_x).max(1.0)) * chart_width;
            let y = height - margin - ((bubble.y - min_y) / (max_y - min_y).max(1.0)) * chart_height;
            
            // サイズを正規化
            let normalized_size = if max_size > min_size {
                (bubble.size - min_size) / (max_size - min_size)
            } else {
                0.5
            };
            let radius = self.min_bubble_size + normalized_size * (self.max_bubble_size - self.min_bubble_size);
            
            // 色を決定
            let color = bubble.color.as_ref()
                .unwrap_or(&colors[i % colors.len()]);

            svg.push_str(&format!(
                r#"  <circle cx="{}" cy="{}" r="{}" fill="{}" class="bubble">
    <title>X: {:.2}, Y: {:.2}, Size: {:.2}</title>
  </circle>"#,
                x, y, radius, color, bubble.x, bubble.y, bubble.size
            ));

            // ラベル表示
            if self.show_labels {
                if let Some(label) = &bubble.label {
                    svg.push_str(&format!(
                        r#"  <text x="{}" y="{}" class="bubble-label">{}</text>"#,
                        x, y, label
                    ));
                }
            }
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

        // X軸ラベル
        for i in 0..=5 {
            let x = margin + (chart_width * i as f64 / 5.0);
            let value = min_x + (max_x - min_x) * i as f64 / 5.0;
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" class="axis-text" text-anchor="middle">{:.1}</text>"#,
                x, height - margin + 20.0, value
            ));
        }

        // Y軸ラベル
        for i in 0..=5 {
            let y = height - margin - (chart_height * i as f64 / 5.0);
            let value = min_y + (max_y - min_y) * i as f64 / 5.0;
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" class="axis-text" text-anchor="end">{:.1}</text>"#,
                margin - 10.0, y + 4.0, value
            ));
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

        // サイズ凡例
        let legend_x = width - 150.0;
        let legend_y = margin + 20.0;
        
        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" class="axis-text">Bubble Size</text>"#,
            legend_x, legend_y
        ));

        // 小さいバブル
        svg.push_str(&format!(
            r#"  <circle cx="{}" cy="{}" r="{}" fill="gray" opacity="0.5" />"#,
            legend_x + 20.0, legend_y + 20.0, self.min_bubble_size
        ));
        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" class="axis-text" font-size="10px">{:.1}</text>"#,
            legend_x + 45.0, legend_y + 25.0, min_size
        ));

        // 大きいバブル
        svg.push_str(&format!(
            r#"  <circle cx="{}" cy="{}" r="{}" fill="gray" opacity="0.5" />"#,
            legend_x + 20.0, legend_y + 60.0, self.max_bubble_size
        ));
        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" class="axis-text" font-size="10px">{:.1}</text>"#,
            legend_x + 60.0, legend_y + 65.0, max_size
        ));

        svg.push_str("</svg>");
        svg
    }
}