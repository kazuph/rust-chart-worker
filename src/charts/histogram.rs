use crate::charts::Chart;
use crate::models::GraphRequest;

pub struct HistogramChart {
    pub bins: usize,
    pub show_density: bool,
}

impl Default for HistogramChart {
    fn default() -> Self {
        Self {
            bins: 10,
            show_density: false,
        }
    }
}

impl Chart for HistogramChart {
    fn generate(&self, request: &GraphRequest) -> String {
        let data = if !request.data.is_empty() {
            &request.data
        } else if let Some(series) = request.series.first() {
            &series.data.iter().map(|d| d.value).collect::<Vec<f64>>()
        } else {
            return String::new();
        };

        if data.is_empty() {
            return String::new();
        }

        // データの範囲を計算
        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let range = max_val - min_val;
        let bin_width = range / self.bins as f64;

        // ヒストグラムのビンを計算
        let mut bins = vec![0; self.bins];
        for &value in data {
            let bin_index = ((value - min_val) / bin_width).floor() as usize;
            let bin_index = bin_index.min(self.bins - 1);
            bins[bin_index] += 1;
        }

        // 最大頻度を取得（スケーリング用）
        let max_count = *bins.iter().max().unwrap_or(&1);
        let scale = if self.show_density {
            data.len() as f64 * bin_width
        } else {
            max_count as f64
        };

        // SVG生成
        let width = 800.0;
        let height = 600.0;
        let margin = 60.0;
        let chart_width = width - 2.0 * margin;
        let chart_height = height - 2.0 * margin;

        let bar_width = chart_width / self.bins as f64;

        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <style>
      .histogram-bar {{ fill: #3498db; stroke: #2980b9; stroke-width: 1; }}
      .histogram-bar:hover {{ fill: #2980b9; }}
      .axis {{ stroke: #333; stroke-width: 2; }}
      .axis-text {{ font-family: Arial, sans-serif; font-size: 12px; fill: #333; }}
      .title {{ font-family: Arial, sans-serif; font-size: 16px; font-weight: bold; fill: #333; text-anchor: middle; }}
      .grid {{ stroke: #ddd; stroke-width: 1; stroke-dasharray: 2,2; }}
    </style>
  </defs>"#,
            width, height
        );

        // タイトル
        if let Some(title) = &request.title {
            svg.push_str(&format!(
                r#"  <text x="{}" y="30" class="title">{}</text>"#,
                width / 2.0,
                title
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

        // ヒストグラムバー
        for (i, &count) in bins.iter().enumerate() {
            let x = margin + (i as f64 * bar_width);
            let bar_height = if scale > 0.0 {
                (count as f64 / scale) * chart_height
            } else {
                0.0
            };
            let y = height - margin - bar_height;

            let bin_start = min_val + (i as f64 * bin_width);
            let bin_end = bin_start + bin_width;

            svg.push_str(&format!(
                r#"  <rect x="{}" y="{}" width="{}" height="{}" class="histogram-bar">
    <title>Range: {:.2}-{:.2}, Count: {}</title>
  </rect>"#,
                x, y, bar_width - 1.0, bar_height, bin_start, bin_end, count
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

        // X軸ラベル
        for i in 0..=self.bins {
            let x = margin + (i as f64 * chart_width / self.bins as f64);
            let value = min_val + (i as f64 * range / self.bins as f64);
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" class="axis-text" text-anchor="middle">{:.1}</text>"#,
                x, height - margin + 20.0, value
            ));
        }

        // Y軸ラベル
        for i in 0..=5 {
            let y = height - margin - (chart_height * i as f64 / 5.0);
            let value = (scale * i as f64 / 5.0) as i32;
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" class="axis-text" text-anchor="end">{}</text>"#,
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

        svg.push_str("</svg>");
        svg
    }
}