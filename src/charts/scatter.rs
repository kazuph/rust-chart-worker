use super::Chart;
use crate::models::GraphRequest;
use crate::utils::{self, svg};

pub struct ScatterChart {}

impl Chart for ScatterChart {
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

        let raw_max = super::get_max_value(&request.series);
        let max_value = svg::nice_max(raw_max);
        let segment_width = 640.0 / (series[0].len() as f64 - 1.0);

        svg_content.push_str(&utils::svg::generate_y_axis_ticks(max_value));
        svg_content.push_str(&utils::svg::generate_x_axis_ticks_for_line(series[0].len()));

        for (series_idx, series_data) in series.iter().enumerate() {
            let color = request
                .colors
                .as_ref()
                .and_then(|c| c.get(series_idx))
                .map(String::as_str)
                .unwrap_or(
                    utils::get_default_colors()[series_idx % utils::get_default_colors().len()],
                );

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
            svg_content.push_str(&svg::create_legend(&request.series, 660.0, 50.0));
        }

        svg_content.push_str(svg::create_svg_footer());
        svg_content
    }
}
