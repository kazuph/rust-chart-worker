use crate::charts::Chart;
use crate::models::GraphRequest;

pub struct StackedBarChart {
    pub show_values: bool,
    pub show_percentage: bool,
}

impl Default for StackedBarChart {
    fn default() -> Self {
        Self {
            show_values: false,
            show_percentage: false,
        }
    }
}

impl Chart for StackedBarChart {
    fn generate(&self, request: &GraphRequest) -> String {
        if request.series.is_empty() {
            return String::new();
        }

        // 各カテゴリごとの合計値を計算
        let max_data_points = request.series.iter()
            .map(|s| s.data.len())
            .max()
            .unwrap_or(0);

        if max_data_points == 0 {
            return String::new();
        }

        // 各カテゴリ（X軸位置）での合計値を計算
        let mut category_totals = vec![0.0; max_data_points];
        for series in &request.series {
            for (i, data_point) in series.data.iter().enumerate() {
                if i < max_data_points {
                    category_totals[i] += data_point.value.max(0.0); // 負の値は0として扱う
                }
            }
        }

        let max_total = category_totals.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        if max_total <= 0.0 {
            return String::new();
        }

        // SVG設定
        let width = 800.0;
        let height = 600.0;
        let margin = 60.0;
        let chart_width = width - 2.0 * margin;
        let chart_height = height - 2.0 * margin;
        
        let bar_width = chart_width / max_data_points as f64 * 0.8;
        let bar_spacing = chart_width / max_data_points as f64;

        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <style>
      .stacked-bar {{ stroke: #fff; stroke-width: 1; }}
      .stacked-bar:hover {{ stroke: #333; stroke-width: 2; }}
      .bar-label {{ font-family: Arial, sans-serif; font-size: 10px; fill: #fff; text-anchor: middle; dominant-baseline: middle; }}
      .axis {{ stroke: #333; stroke-width: 2; }}
      .axis-text {{ font-family: Arial, sans-serif; font-size: 12px; fill: #333; }}
      .title {{ font-family: Arial, sans-serif; font-size: 16px; font-weight: bold; fill: #333; text-anchor: middle; }}
      .grid {{ stroke: #ddd; stroke-width: 1; stroke-dasharray: 2,2; }}
      .legend {{ font-family: Arial, sans-serif; font-size: 12px; fill: #333; }}
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

        // カラーパレット
        let colors = request.colors.as_ref().map(|c| c.clone()).unwrap_or_else(|| vec![
            "#3498db".to_string(), "#e74c3c".to_string(), "#2ecc71".to_string(),
            "#f39c12".to_string(), "#9b59b6".to_string(), "#1abc9c".to_string(),
            "#34495e".to_string(), "#e67e22".to_string(),
        ]);

        // スタックバーを描画
        for category_index in 0..max_data_points {
            let x = margin + (category_index as f64 + 0.5) * bar_spacing - bar_width / 2.0;
            let mut current_y = height - margin;
            
            // このカテゴリの各シリーズを下から積み上げ
            for (series_index, series) in request.series.iter().enumerate() {
                if let Some(data_point) = series.data.get(category_index) {
                    let value = data_point.value.max(0.0);
                    if value > 0.0 {
                        let bar_height = (value / max_total) * chart_height;
                        let bar_y = current_y - bar_height;
                        
                        let color = series.color.as_ref()
                            .or_else(|| data_point.color.as_ref())
                            .unwrap_or(&colors[series_index % colors.len()]);

                        let percentage = if category_totals[category_index] > 0.0 {
                            (value / category_totals[category_index]) * 100.0
                        } else {
                            0.0
                        };

                        svg.push_str(&format!(
                            r#"  <rect x="{}" y="{}" width="{}" height="{}" fill="{}" class="stacked-bar">
    <title>Series: {}, Value: {:.2} ({:.1}%)</title>
  </rect>"#,
                            x, bar_y, bar_width, bar_height, color,
                            series.name.as_ref().unwrap_or(&format!("Series {}", series_index + 1)),
                            value, percentage
                        ));

                        // 値ラベル表示
                        if self.show_values && bar_height > 15.0 {
                            let label_text = if self.show_percentage {
                                format!("{:.1}%", percentage)
                            } else {
                                format!("{:.1}", value)
                            };
                            
                            svg.push_str(&format!(
                                r#"  <text x="{}" y="{}" class="bar-label">{}</text>"#,
                                x + bar_width / 2.0, bar_y + bar_height / 2.0, label_text
                            ));
                        }

                        current_y = bar_y;
                    }
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
        for category_index in 0..max_data_points {
            let x = margin + (category_index as f64 + 0.5) * bar_spacing;
            let default_label = format!("Category {}", category_index + 1);
            let label = request.series.first()
                .and_then(|s| s.data.get(category_index))
                .and_then(|d| d.label.as_ref())
                .map(|s| s.as_str())
                .unwrap_or(&default_label);
            
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" class="axis-text" text-anchor="middle">{}</text>"#,
                x, height - margin + 20.0, label
            ));
        }

        // Y軸ラベル
        for i in 0..=5 {
            let y = height - margin - (chart_height * i as f64 / 5.0);
            let value = max_total * i as f64 / 5.0;
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" class="axis-text" text-anchor="end">{:.0}</text>"#,
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
                r#"  <rect x="{}" y="{}" width="12" height="12" fill="{}" />"#,
                legend_x, legend_y - 10.0, color
            ));
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" class="legend">{}</text>"#,
                legend_x + 18.0, legend_y, name
            ));
            legend_y += 18.0;
        }

        svg.push_str("</svg>");
        svg
    }
}