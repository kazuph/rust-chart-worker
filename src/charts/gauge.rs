use crate::charts::Chart;
use crate::models::GraphRequest;

pub struct GaugeChart {
    pub min_value: f64,
    pub max_value: f64,
    pub show_labels: bool,
    pub show_needle: bool,
}

impl Default for GaugeChart {
    fn default() -> Self {
        Self {
            min_value: 0.0,
            max_value: 100.0,
            show_labels: true,
            show_needle: true,
        }
    }
}

impl GaugeChart {
    fn get_color_for_value(&self, _value: f64, normalized: f64) -> String {
        // 値に基づいて色を決定（緑→黄→赤）
        if normalized < 0.5 {
            // 緑から黄色へ
            let t = normalized * 2.0;
            format!("rgb({}, 255, 0)", (255.0 * t) as u8)
        } else {
            // 黄色から赤へ
            let t = (normalized - 0.5) * 2.0;
            format!("rgb(255, {}, 0)", (255.0 * (1.0 - t)) as u8)
        }
    }
}

impl Chart for GaugeChart {
    fn generate(&self, request: &GraphRequest) -> String {
        let value = if !request.data.is_empty() {
            request.data[0]
        } else if let Some(series) = request.series.first() {
            if !series.data.is_empty() {
                series.data[0].value
            } else {
                return String::new();
            }
        } else {
            return String::new();
        };

        // 値を正規化
        let normalized = (value - self.min_value) / (self.max_value - self.min_value);
        let normalized = normalized.max(0.0).min(1.0);

        // SVG設定
        let width = 600.0;
        let height = 400.0;
        let center_x = width / 2.0;
        let center_y = height - 50.0;
        let radius = 150.0;

        // 角度計算（180度の半円）
        let start_angle = std::f64::consts::PI; // 180度
        let end_angle = 0.0; // 0度
        let needle_angle = start_angle - (normalized * std::f64::consts::PI);

        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <style>
      .gauge-background {{ fill: #f8f9fa; stroke: #dee2e6; stroke-width: 2; }}
      .gauge-arc {{ fill: none; stroke-width: 20; }}
      .gauge-needle {{ stroke: #333; stroke-width: 3; stroke-linecap: round; }}
      .gauge-center {{ fill: #333; }}
      .gauge-text {{ font-family: Arial, sans-serif; font-size: 14px; fill: #333; text-anchor: middle; }}
      .gauge-value {{ font-family: Arial, sans-serif; font-size: 24px; font-weight: bold; fill: #333; text-anchor: middle; }}
      .title {{ font-family: Arial, sans-serif; font-size: 16px; font-weight: bold; fill: #333; text-anchor: middle; }}
    </style>
  </defs>
  <rect width="100%" height="100%" fill="white"/>"#,
            width, height
        );

        // タイトル
        if let Some(title) = &request.title {
            svg.push_str(&format!(
                r#"  <text x="{}" y="30" class="title">{}</text>"#,
                center_x, title
            ));
        }

        // ゲージの背景円弧
        svg.push_str(&format!(
            r#"  <path d="M {} {} A {} {} 0 0 1 {} {}" fill="none" stroke="gray" />"#,
            center_x - radius, center_y,
            radius, radius,
            center_x + radius, center_y
        ));

        // カラーゾーンを描画（複数の円弧）
        let segments = 50;
        let segment_angle = std::f64::consts::PI / segments as f64;
        
        for i in 0..segments {
            let segment_start = start_angle - (i as f64 * segment_angle);
            let segment_end = start_angle - ((i + 1) as f64 * segment_angle);
            let segment_normalized = i as f64 / (segments - 1) as f64;
            let color = self.get_color_for_value(
                self.min_value + segment_normalized * (self.max_value - self.min_value),
                segment_normalized
            );

            let x1 = center_x + radius * segment_start.cos();
            let y1 = center_y + radius * segment_start.sin();
            let x2 = center_x + radius * segment_end.cos();
            let y2 = center_y + radius * segment_end.sin();

            svg.push_str(&format!(
                r#"  <path d="M {} {} A {} {} 0 0 1 {} {}" fill="none" stroke="{}" stroke-width="20" />"#,
                x1, y1, radius, radius, x2, y2, color
            ));
        }

        // 目盛りとラベル
        if self.show_labels {
            let ticks = 11; // 0, 10, 20, ..., 100
            for i in 0..ticks {
                let tick_normalized = i as f64 / (ticks - 1) as f64;
                let tick_angle = start_angle - (tick_normalized * std::f64::consts::PI);
                let tick_value = self.min_value + tick_normalized * (self.max_value - self.min_value);

                // 外側の目盛り
                let outer_x = center_x + (radius + 15.0) * tick_angle.cos();
                let outer_y = center_y + (radius + 15.0) * tick_angle.sin();
                let inner_x = center_x + (radius + 5.0) * tick_angle.cos();
                let inner_y = center_y + (radius + 5.0) * tick_angle.sin();

                svg.push_str(&format!(
                    r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" stroke="none" />"#,
                    inner_x, inner_y, outer_x, outer_y
                ));

                // ラベル
                let label_x = center_x + (radius + 30.0) * tick_angle.cos();
                let label_y = center_y + (radius + 30.0) * tick_angle.sin();
                svg.push_str(&format!(
                    r#"  <text x="{}" y="{}" class="gauge-text">{:.0}</text>"#,
                    label_x, label_y + 5.0, tick_value
                ));
            }
        }

        // ニードル
        if self.show_needle {
            let needle_length = radius - 10.0;
            let needle_x = center_x + needle_length * needle_angle.cos();
            let needle_y = center_y + needle_length * needle_angle.sin();

            svg.push_str(&format!(
                r#"  <line x1="{}" y1="{}" x2="{}" y2="{}" class="gauge-needle" />"#,
                center_x, center_y, needle_x, needle_y
            ));

            // ニードルの中心円
            svg.push_str(&format!(
                r#"  <circle cx="{}" cy="{}" r="8" class="gauge-center" />"#,
                center_x, center_y
            ));
        }

        // 現在値表示
        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" class="gauge-value">{:.1}</text>"#,
            center_x, center_y + 50.0, value
        ));

        // 単位やラベル
        if let Some(y_label) = &request.y_label {
            svg.push_str(&format!(
                r#"  <text x="{}" y="{}" class="gauge-text">{}</text>"#,
                center_x, center_y + 75.0, y_label
            ));
        }

        svg.push_str("</svg>");
        svg
    }
}