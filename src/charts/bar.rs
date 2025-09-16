use crate::models::{GraphRequest, Series};
use crate::utils::{self, svg};

pub struct BarChart {}

impl super::Chart for BarChart {
    fn generate(&self, request: &GraphRequest) -> String {
        let default_series = if request.series.is_empty() {
            let default_color = utils::get_default_colors()[0].to_string();
            let series_data = request
                .data
                .iter()
                .map(|&value| crate::models::DataPoint {
                    value,
                    label: None,
                    color: None,
                })
                .collect();
            vec![Series {
                name: None,
                data: series_data,
                color: Some(default_color),
            }]
        } else {
            request.series.clone()
        };

        let raw_max = default_series
            .iter()
            .flat_map(|s| s.data.iter().map(|d| d.value))
            .fold(f64::NEG_INFINITY, f64::max);
        let max_value = svg::nice_max(raw_max);

        // Match the drawable width used by axes (0..640)
        let segment_width = 640.0 / (default_series[0].data.len() as f64);
        // Group width inside each segment; center the group at the tick.
        let bar_group_width = if default_series.len() > 1 {
            segment_width * 0.7 // multi-series: slightly wider group
        } else {
            segment_width * 0.8 // single series: comfortable width
        };

        let mut svg_content = format!(
            r#"<svg width="800" height="600" xmlns="http://www.w3.org/2000/svg">
            <rect width="100%" height="100%" fill="white"/>
            <g transform="translate(80, 50)">"#
        );

        // Add title if provided
        if let Some(title) = &request.title {
            svg_content.push_str(&format!(
                r#"<text x="320" y="30" text-anchor="middle" font-family="M PLUS 1p" font-size="20">{}</text>"#,
                title
            ));
        }

        // Add x-axis label if provided
        if let Some(x_label) = &request.x_label {
            svg_content.push_str(&format!(
                r#"<text x="320" y="520" text-anchor="middle" font-family="M PLUS 1p" font-size="14">{}</text>"#,
                x_label
            ));
        }

        // Add y-axis label if provided
        if let Some(y_label) = &request.y_label {
            svg_content.push_str(&format!(
                r#"<text x="-280" y="-50" text-anchor="middle" font-family="M PLUS 1p" font-size="14" transform="rotate(-90)">{}</text>"#,
                y_label
            ));
        }

        // Draw axes
        svg_content.push_str(
            r#"<line x1="0" y1="450" x2="640" y2="450" stroke="black" stroke-width="2"/>
            <line x1="0" y1="50" x2="0" y2="450" stroke="black" stroke-width="2"/>"#,
        );

        // Draw y-axis ticks and values
        for i in 0..=5 {
            let y = 450.0 - (i as f64 * 80.0);
            let value = (i as f64 * max_value / 5.0).round();
            svg_content.push_str(&format!(
                r#"<line x1="-5" y1="{}" x2="0" y2="{}" stroke="black" stroke-width="1"/>
                <text x="-10" y="{}" text-anchor="end" font-family="M PLUS 1p" font-size="12">{}</text>"#,
                y, y, y + 4.0, utils::format_number(value)
            ));
        }

        // Draw x-axis ticks and values
        svg_content.push_str(&utils::svg::generate_x_axis_ticks_for_bar(default_series[0].data.len()));

        // Draw bars
        for (series_idx, series_item) in default_series.iter().enumerate() {
            let bar_each_width = bar_group_width / default_series.len() as f64;

            for (i, point) in series_item.data.iter().enumerate() {
                // Group centered at segment center
                let group_left = (i as f64 * segment_width) + (segment_width - bar_group_width) / 2.0;
                let x = group_left + series_idx as f64 * bar_each_width;
                let height = (point.value / max_value) * 400.0;
                let y = 450.0 - height;
                let color = match &point.color {
                    Some(c) => c.clone(),
                    None => match &series_item.color {
                        Some(c) => c.clone(),
                        None => "#0000FF".to_string(),
                    },
                };

                svg_content.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}"/>"#,
                    x,
                    y,
                    bar_each_width,
                    height,
                    color
                ));

                svg_content.push_str(&utils::svg::generate_value_text(
                    x + bar_each_width / 2.0,
                    y,
                    point.value,
                ));
            }
        }

        // Add legend if there are multiple series
        if default_series.len() > 1 {
            // Place legend outside plotting area to avoid overlap with tall bars
            svg_content.push_str(&svg::create_legend(&default_series, 660.0, 50.0));
        }

        svg_content.push_str("</g></svg>");
        svg_content
    }
}
