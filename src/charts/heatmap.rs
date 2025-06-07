use crate::charts::Chart;
use crate::models::GraphRequest;

pub struct HeatmapChart {
    pub cell_size: f64,
    pub show_values: bool,
    pub color_scale: ColorScale,
}

#[derive(Clone)]
pub enum ColorScale {
    Blues,
    Reds,
    Greens,
    Viridis,
    Custom(Vec<String>),
}

impl Default for HeatmapChart {
    fn default() -> Self {
        Self {
            cell_size: 40.0,
            show_values: true,
            color_scale: ColorScale::Blues,
        }
    }
}

impl HeatmapChart {
    fn get_color(&self, value: f64, min_val: f64, max_val: f64) -> String {
        let normalized = if max_val > min_val {
            (value - min_val) / (max_val - min_val)
        } else {
            0.5
        };

        match &self.color_scale {
            ColorScale::Blues => {
                let intensity = (normalized * 255.0) as u8;
                format!("rgb({}, {}, 255)", 255 - intensity, 255 - intensity)
            }
            ColorScale::Reds => {
                let intensity = (normalized * 255.0) as u8;
                format!("rgb(255, {}, {})", 255 - intensity, 255 - intensity)
            }
            ColorScale::Greens => {
                let intensity = (normalized * 255.0) as u8;
                format!("rgb({}, 255, {})", 255 - intensity, 255 - intensity)
            }
            ColorScale::Viridis => {
                // Viridis color scale approximation
                if normalized < 0.25 {
                    let t = normalized * 4.0;
                    format!("rgb({}, {}, {})", 
                        (68.0 * (1.0 - t) + 59.0 * t) as u8,
                        (1.0 * (1.0 - t) + 82.0 * t) as u8,
                        (84.0 * (1.0 - t) + 139.0 * t) as u8)
                } else if normalized < 0.5 {
                    let t = (normalized - 0.25) * 4.0;
                    format!("rgb({}, {}, {})", 
                        (59.0 * (1.0 - t) + 33.0 * t) as u8,
                        (82.0 * (1.0 - t) + 144.0 * t) as u8,
                        (139.0 * (1.0 - t) + 140.0 * t) as u8)
                } else if normalized < 0.75 {
                    let t = (normalized - 0.5) * 4.0;
                    format!("rgb({}, {}, {})", 
                        (33.0 * (1.0 - t) + 94.0 * t) as u8,
                        (144.0 * (1.0 - t) + 201.0 * t) as u8,
                        (140.0 * (1.0 - t) + 98.0 * t) as u8)
                } else {
                    let t = (normalized - 0.75) * 4.0;
                    format!("rgb({}, {}, {})", 
                        (94.0 * (1.0 - t) + 253.0 * t) as u8,
                        (201.0 * (1.0 - t) + 231.0 * t) as u8,
                        (98.0 * (1.0 - t) + 37.0 * t) as u8)
                }
            }
            ColorScale::Custom(colors) => {
                if colors.is_empty() {
                    "#888888".to_string()
                } else {
                    let index = (normalized * (colors.len() - 1) as f64) as usize;
                    colors[index.min(colors.len() - 1)].clone()
                }
            }
        }
    }
}

impl Chart for HeatmapChart {
    fn generate(&self, request: &GraphRequest) -> String {
        // ヒートマップデータを2次元配列として解釈
        // データが1次元の場合は正方形のグリッドとして配置
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

        // グリッドサイズを計算（正方形に近い形で配置）
        let total_cells = data.len();
        let grid_size = (total_cells as f64).sqrt().ceil() as usize;
        let rows = (total_cells + grid_size - 1) / grid_size;
        let cols = grid_size;

        // データの範囲を計算
        let min_val = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let max_val = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        // SVG生成
        let margin = 80.0;
        let width = margin * 2.0 + cols as f64 * self.cell_size;
        let height = margin * 2.0 + rows as f64 * self.cell_size;

        let mut svg = format!(
            r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
  <defs>
    <style>
      .heatmap-cell {{ stroke: #fff; stroke-width: 1; }}
      .heatmap-cell:hover {{ stroke: #333; stroke-width: 2; }}
      .cell-text {{ font-family: Arial, sans-serif; font-size: 10px; text-anchor: middle; dominant-baseline: middle; }}
      .title {{ font-family: Arial, sans-serif; font-size: 16px; font-weight: bold; fill: #333; text-anchor: middle; }}
      .legend {{ font-family: Arial, sans-serif; font-size: 12px; fill: #333; }}
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

        // ヒートマップセル
        for (i, &value) in data.iter().enumerate() {
            let row = i / cols;
            let col = i % cols;
            
            if row >= rows {
                break;
            }

            let x = margin + col as f64 * self.cell_size;
            let y = margin + row as f64 * self.cell_size;
            let color = self.get_color(value, min_val, max_val);

            svg.push_str(&format!(
                r#"  <rect x="{}" y="{}" width="{}" height="{}" fill="{}" class="heatmap-cell">
    <title>Value: {:.3}</title>
  </rect>"#,
                x, y, self.cell_size, self.cell_size, color, value
            ));

            // 値をテキストで表示
            if self.show_values && self.cell_size > 20.0 {
                let text_color = if (value - min_val) / (max_val - min_val) > 0.5 {
                    "#fff"
                } else {
                    "#000"
                };
                svg.push_str(&format!(
                    r#"  <text x="{}" y="{}" class="cell-text" fill="{}">{:.1}</text>"#,
                    x + self.cell_size / 2.0,
                    y + self.cell_size / 2.0,
                    text_color,
                    value
                ));
            }
        }

        // カラースケール凡例
        let legend_x = margin;
        let legend_y = height - margin + 20.0;
        let legend_width = 200.0;
        let legend_height = 20.0;
        let legend_steps = 10;

        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" class="legend">Color Scale</text>"#,
            legend_x, legend_y - 5.0
        ));

        for i in 0..legend_steps {
            let step_width = legend_width / legend_steps as f64;
            let x = legend_x + i as f64 * step_width;
            let normalized = i as f64 / (legend_steps - 1) as f64;
            let value = min_val + normalized * (max_val - min_val);
            let color = self.get_color(value, min_val, max_val);

            svg.push_str(&format!(
                r#"  <rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="none" />"#,
                x, legend_y, step_width, legend_height, color
            ));
        }

        // 凡例ラベル
        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" class="legend" text-anchor="start">{:.2}</text>"#,
            legend_x, legend_y + legend_height + 15.0, min_val
        ));
        svg.push_str(&format!(
            r#"  <text x="{}" y="{}" class="legend" text-anchor="end">{:.2}</text>"#,
            legend_x + legend_width, legend_y + legend_height + 15.0, max_val
        ));

        svg.push_str("</svg>");
        svg
    }
}