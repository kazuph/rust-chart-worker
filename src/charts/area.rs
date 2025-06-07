use super::{get_max_value, Chart};
use crate::models::GraphRequest;
use crate::utils;

pub struct AreaChart {}

impl Chart for AreaChart {
    fn generate(&self, request: &GraphRequest) -> String {
        let mut svg_content = String::from(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">
<rect width="800" height="600" fill="white"/>"#,
        );

        // Draw title and labels first
        if let Some(title) = &request.title {
            svg_content.push_str(&format!(
                r#"<text x="400" y="30" text-anchor="middle" font-size="20">{}</text>"#,
                title
            ));
        }

        if let Some(x_label) = request.x_label.as_deref() {
            svg_content.push_str(&format!(
                r#"<text x="400" y="580" text-anchor="middle" font-size="14">{}</text>"#,
                x_label
            ));
        }

        if let Some(y_label) = request.y_label.as_deref() {
            svg_content.push_str(&format!(
                r#"<text x="30" y="300" text-anchor="middle" font-size="14" transform="rotate(-90, 30, 300)">{}</text>"#,
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
        let segment_width = 640.0 / (series[0].len() as f64 - 1.0);

        // Draw grid lines and axes
        svg_content.push_str(
            r#"<g transform="translate(80, 50)">
<line x1="0" y1="450" x2="640" y2="450" stroke="black" stroke-width="2"/>
<line x1="0" y1="0" x2="0" y2="450" stroke="black" stroke-width="2"/>"#,
        );

        // Draw areas
        for (series_idx, series_data) in series.iter().enumerate() {
            let color = request
                .colors
                .as_ref()
                .and_then(|c| c.get(series_idx))
                .map(String::as_str)
                .unwrap_or(
                    utils::get_default_colors()[series_idx % utils::get_default_colors().len()],
                );

            // Create path for area
            let mut path = String::new();
            path.push_str(&format!("M 0 450")); // Start at bottom-left

            // Draw top line
            for (i, &value) in series_data.iter().enumerate() {
                let x = i as f64 * segment_width;
                let y = 450.0 - ((value / max_value) * 400.0);
                if i == 0 {
                    path.push_str(&format!(" L {:.1} {:.1}", x, y));
                } else {
                    path.push_str(&format!(" L {:.1} {:.1}", x, y));
                }
            }

            // Complete the path back to the bottom
            path.push_str(&format!(
                " L {:.1} 450",
                (series_data.len() - 1) as f64 * segment_width
            ));
            path.push_str(" Z"); // Close the path

            // Add the area with transparency
            svg_content.push_str(&format!(
                r#"<path d="{}" fill="{}" fill-opacity="0.3" stroke="{}" stroke-width="2"/>"#,
                path, color, color
            ));

            // Add data points
            for (i, &value) in series_data.iter().enumerate() {
                let x = i as f64 * segment_width;
                let y = 450.0 - ((value / max_value) * 400.0);
                svg_content.push_str(&format!(
                    r#"<circle cx="{}" cy="{}" r="4" fill="{}"/>"#,
                    x, y, color
                ));
                svg_content.push_str(&utils::svg::generate_value_text(x, y, value));
            }
        }

        svg_content.push_str("</g></svg>");
        svg_content
    }
}
