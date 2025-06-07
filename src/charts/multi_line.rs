use crate::charts::Chart;
use crate::models::GraphRequest;

pub struct MultiLineChart {
    pub show_points: bool,
    pub smooth_lines: bool,
    pub line_width: f64,
}

impl Default for MultiLineChart {
    fn default() -> Self {
        Self {
            show_points: true,
            smooth_lines: false,
            line_width: 2.0,
        }
    }
}

impl Chart for MultiLineChart {
    fn generate(&self, request: &GraphRequest) -> String {
        if request.series.is_empty() {
            return String::new();
        }

        // データ範囲を計算
        let mut min_x = 0.0;
        let mut max_x: f64 = 0.0;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for series in &request.series {
            for (i, data_point) in series.data.iter().enumerate() {
                let x = i as f64;
                let y = data_point.value;
                
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }

        if max_x <= min_x || max_y <= min_y {
            return String::new();
        }

        // SVG設定
        let width = 800.0;
        let height = 600.0;
        let margin = 60.0;
        let chart_width = width - 2.0 * margin;
        let chart_height = height - 2.0 * margin;

        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <style>
      .multi-line {{ fill: none; stroke-width: {}; }}
      .multi-line:hover {{ stroke-width: {}; }}
      .line-point {{ stroke: #fff; stroke-width: 1; }}
      .line-point:hover {{ r: 6; }}
      .axis {{ stroke: #333; stroke-width: 2; }}
      .axis-text {{ font-family: Arial, sans-serif; font-size: 12px; fill: #333; }}
      .title {{ font-family: Arial, sans-serif; font-size: 16px; font-weight: bold; fill: #333; text-anchor: middle; }}
      .grid {{ stroke: #ddd; stroke-width: 1; stroke-dasharray: 2,2; }}
      .legend {{ font-family: Arial, sans-serif; font-size: 12px; fill: #333; }}
    </style>
  </defs>
  <rect width="100%" height="100%" fill="white"/>"#,
            width, height, self.line_width, self.line_width + 1.0
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
            "#34495e".to_string(), "#e67e22".to_string(),
        ]);

        // 各系列の線を描画
        for (series_index, series) in request.series.iter().enumerate() {
            if series.data.is_empty() {
                continue;
            }

            let color = series.color.as_ref()
                .unwrap_or(&colors[series_index % colors.len()]);

            // パスデータを構築
            let mut path_data = String::new();
            let mut points = Vec::new();

            for (i, data_point) in series.data.iter().enumerate() {
                let x = margin + ((i as f64 - min_x) / (max_x - min_x)) * chart_width;
                let y = height - margin - ((data_point.value - min_y) / (max_y - min_y)) * chart_height;
                
                points.push((x, y, data_point.value));

                if i == 0 {
                    path_data.push_str(&format!("M {} {}", x, y));
                } else if self.smooth_lines && i > 0 {
                    // スムーズライン（簡易ベジェ曲線）
                    let prev_x = margin + (((i - 1) as f64 - min_x) / (max_x - min_x)) * chart_width;
                    let control_x = (prev_x + x) / 2.0;
                    path_data.push_str(&format!(" Q {} {} {} {}", control_x, y, x, y));
                } else {
                    path_data.push_str(&format!(" L {} {}", x, y));
                }
            }

            // 線を描画
            svg.push_str(&format!(
                r#"  <path d="{}" stroke="{}" class="multi-line" />"#,
                path_data, color
            ));

            // ポイントを描画
            if self.show_points {
                for (x, y, value) in points {
                    svg.push_str(&format!(
                        r#"  <circle cx="{}" cy="{}" r="4" fill="{}" class="line-point">
    <title>Series: {}, Value: {:.2}</title>
  </circle>"#,
                        x, y, color,
                        series.name.as_ref().unwrap_or(&format!("Series {}", series_index + 1)),
                        value
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
                r#"  <text x="{}" y="{}" class="axis-text" text-anchor="middle">{:.0}</text>"#,
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

        // 凡例
        let legend_x = width - 150.0;
        let mut legend_y = margin;
        
        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" class="legend" font-weight="bold">Legend</text>"#,
            legend_x, legend_y
        ));
        legend_y += 20.0;

        for (series_index, series) in request.series.iter().enumerate() {
            let color = series.color.as_ref()
                .unwrap_or(&colors[series_index % colors.len()]);
            let default_name = format!("Series {}", series_index + 1);
            let name = series.name.as_ref()
                .unwrap_or(&default_name);

            svg.push_str(&format!(
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="{}" stroke-width="3" />"#,
                legend_x, legend_y - 6.0, legend_x + 20.0, legend_y - 6.0, color
            ));
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" class="legend">{}</text>"#,
                legend_x + 25.0, legend_y, name
            ));
            legend_y += 18.0;
        }

        svg.push_str("</svg>");
        svg
    }
}