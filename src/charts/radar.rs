use super::{get_max_value, Chart};
use crate::models::GraphRequest;
use crate::utils::{self, svg};
use std::f64::consts::PI;

pub struct RadarChart {}

impl Chart for RadarChart {
    fn generate(&self, request: &GraphRequest) -> String {
        let mut svg_content = svg::create_svg_header(
            request.title.as_deref(),
            request.x_label.as_deref(),
            request.y_label.as_deref(),
        );

        let series = if request.series.is_empty() {
            vec![request.data.iter().map(|&v| v).collect::<Vec<f64>>()]
        } else {
            request
                .series
                .iter()
                .map(|s| s.data.iter().map(|d| d.value).collect::<Vec<f64>>())
                .collect()
        };

        let max_value = get_max_value(&request.series);
        let center_x = 400.0;
        let center_y = 300.0;
        let radius = 200.0;
        let num_axes = series[0].len();
        let angle_step = (2.0 * PI) / (num_axes as f64);

        // Draw axis lines and labels
        for i in 0..num_axes {
            let angle = -PI / 2.0 + (i as f64 * angle_step);
            let end_x = center_x + radius * angle.cos();
            let end_y = center_y + radius * angle.sin();

            // Draw axis line
            svg_content.push_str(&format!(
                r#"<line x1="{}" y1="{}" x2="{}" y2="{}" style="stroke:#CCCCCC;stroke-width:1"/>"#,
                center_x, center_y, end_x, end_y
            ));

            // Draw axis label
            let label_x = center_x + (radius + 20.0) * angle.cos();
            let label_y = center_y + (radius + 20.0) * angle.sin();
            svg_content.push_str(&format!(
                r#"<text x="{}" y="{}" text-anchor="middle" font-size="12">{}</text>"#,
                label_x,
                label_y,
                i + 1
            ));

            // Draw concentric circles
            for j in 1..=5 {
                let r = radius * (j as f64 / 5.0);
                svg_content.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="{}" fill="none" style="stroke:#CCCCCC;stroke-width:1"/>"#,
                    center_x, center_y, r
                ));
            }
        }

        // Draw data
        for (series_idx, series_data) in series.iter().enumerate() {
            let mut path = String::new();
            let color = request
                .colors
                .as_ref()
                .and_then(|c| c.get(series_idx))
                .map(String::as_str)
                .unwrap_or(
                    utils::get_default_colors()[series_idx % utils::get_default_colors().len()],
                );

            for (i, &value) in series_data.iter().enumerate() {
                let angle = -PI / 2.0 + (i as f64 * angle_step);
                let r = radius * (value / max_value);
                let x = center_x + r * angle.cos();
                let y = center_y + r * angle.sin();

                if i == 0 {
                    path.push_str(&format!("M {} {}", x, y));
                } else {
                    path.push_str(&format!(" L {} {}", x, y));
                }

                // Draw point
                svg_content.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="4" fill="{}"/>"#,
                    x, y, color
                ));
                svg_content.push_str(&utils::generate_value_text(x, y, value));
            }

            // Close the path
            path.push_str(" Z");

            // Draw filled area with transparency
            svg_content.push_str(&format!(
                r#"<path d="{}" fill="{}" fill-opacity="0.3"/>"#,
                path, color
            ));

            // Draw outline
            svg_content.push_str(&format!(
                r#"<path d="{}" style="stroke:{};stroke-width:2" fill="none"/>"#,
                path, color
            ));
        }

        if !request.series.is_empty() {
            svg_content.push_str(&svg::create_legend(&request.series, 600.0, 50.0));
        }

        svg_content.push_str(svg::create_svg_footer());
        svg_content
    }

    fn needs_axes(&self) -> bool {
        false
    }
}
