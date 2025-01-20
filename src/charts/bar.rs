use super::{get_max_value, Chart};
use crate::models::GraphRequest;
use crate::utils::{self, svg};

pub struct BarChart {}

impl Chart for BarChart {
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
        let bar_width = 640.0 / (series[0].len() as f64);
        let bar_spacing = bar_width * 0.1;
        let series_width = bar_width - (2.0 * bar_spacing);
        let series_spacing = series_width / (series.len() as f64);

        svg_content.push_str(&utils::svg::generate_y_axis_ticks(max_value));
        svg_content.push_str(&utils::svg::generate_x_axis_ticks(series[0].len()));

        for (series_idx, series_data) in series.iter().enumerate() {
            for (i, &value) in series_data.iter().enumerate() {
                let x = (i as f64 * bar_width) + bar_spacing + (series_idx as f64 * series_spacing);
                let height = ((value / max_value) * (450.0 - 50.0)).max(0.0);
                let y = 450.0 - height;
                let color = request
                    .colors
                    .as_ref()
                    .and_then(|c| c.get(series_idx))
                    .map(String::as_str)
                    .unwrap_or(
                        utils::get_default_colors()[series_idx % utils::get_default_colors().len()],
                    );

                svg_content.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" />"#,
                    x, y, series_spacing, height, color
                ));
                svg_content.push_str(&utils::svg::generate_value_text(
                    x + series_spacing / 2.0,
                    y,
                    value,
                ));
            }
        }

        if !request.series.is_empty() {
            svg_content.push_str(&svg::create_legend(&request.series, 520.0, 50.0));
        }

        svg_content.push_str(svg::create_svg_footer());
        svg_content
    }

    fn needs_axes(&self) -> bool {
        true
    }
}
