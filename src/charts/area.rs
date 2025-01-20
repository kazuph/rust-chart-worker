use super::{get_max_value, Chart};
use crate::models::GraphRequest;
use crate::utils::{self, svg};

pub struct AreaChart {}

impl Chart for AreaChart {
    fn generate(&self, request: &GraphRequest) -> String {
        let mut svg_content = String::from(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">
<rect width="800" height="600" fill="white"/>
<g transform="translate(80, 50)">"#,
        );

        // Draw title and labels first
        if let Some(title) = request.title.as_deref() {
            svg_content.push_str(&format!(
                r#"<text x="320" y="30" text-anchor="middle" font-family="M PLUS 1p" font-size="20">{}</text>"#,
                title
            ));
        }

        if let Some(x_label) = request.x_label.as_deref() {
            svg_content.push_str(&format!(
                r#"<text x="320" y="520" text-anchor="middle" font-family="M PLUS 1p" font-size="14">{}</text>"#,
                x_label
            ));
        }

        if let Some(y_label) = request.y_label.as_deref() {
            svg_content.push_str(&format!(
                r#"<text x="-280" y="-50" text-anchor="middle" font-family="M PLUS 1p" font-size="14" transform="rotate(-90)">{}</text>"#,
                y_label
            ));
        }

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

        // Draw grid lines first (behind everything)
        svg_content.push_str(&utils::svg::generate_y_axis_ticks(max_value));
        svg_content.push_str(&utils::svg::generate_x_axis_ticks_for_line(series[0].len()));

        // Draw areas and points
        for (series_idx, series_data) in series.iter().enumerate() {
            let color = request
                .colors
                .as_ref()
                .and_then(|c| c.get(series_idx))
                .map(String::as_str)
                .unwrap_or(
                    utils::get_default_colors()[series_idx % utils::get_default_colors().len()],
                );

            // Draw area
            let mut path = String::new();
            path.push_str(&format!("M 0 450")); // Start at bottom-left

            let segment_width = 640.0 / (series_data.len() as f64 - 1.0);

            // Draw upper line
            for (i, &value) in series_data.iter().enumerate() {
                let x = i as f64 * segment_width;
                let y = 450.0 - ((value / max_value) * (450.0 - 50.0));
                path.push_str(&format!(" L {:.1} {:.1}", x, y));
            }

            // Draw lower line back to start
            path.push_str(&format!(
                " L {:.1} 450 Z",
                (series_data.len() - 1) as f64 * segment_width
            ));

            svg_content.push_str(&format!(
                r#"<path d="{}" fill="{}" fill-opacity="0.2" stroke="{}" stroke-width="2" />"#,
                path, color, color
            ));

            // Draw points and values
            for (i, &value) in series_data.iter().enumerate() {
                let x = i as f64 * segment_width;
                let y = 450.0 - ((value / max_value) * (450.0 - 50.0));
                svg_content.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="4" fill="{}" />"#,
                    x, y, color
                ));
                svg_content.push_str(&utils::svg::generate_value_text(x, y, value));
            }
        }

        if !request.series.is_empty() {
            svg_content.push_str(&svg::create_legend(&request.series, 520.0, 50.0));
        }

        // Draw axes last (in front of everything)
        svg_content.push_str(
            r#"<line x1="0" y1="450" x2="640" y2="450" stroke="black" stroke-width="2"/>
<line x1="0" y1="50" x2="0" y2="450" stroke="black" stroke-width="2"/>"#,
        );

        svg_content.push_str("</g></svg>");
        svg_content
    }

    fn needs_axes(&self) -> bool {
        true
    }
}
