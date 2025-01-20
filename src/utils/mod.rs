pub mod png;
pub mod svg;

pub fn get_default_colors() -> Vec<&'static str> {
    vec![
        "#0000FF", "#FFB3B3", "#B3E0FF", "#FFE6B3", "#B3FFB3", "#E6B3FF", "#FFD9B3",
    ]
}

pub fn generate_y_axis_ticks(max_value: f64) -> String {
    let num_ticks = 5;
    let tick_interval = max_value / (num_ticks as f64);
    let mut ticks = String::new();

    for i in 0..=num_ticks {
        let y = max_value - (i as f64 * tick_interval);
        let y_pos = 500.0 - (y / max_value * 400.0);
        ticks.push_str(&format!(
            r#"<line x1="50" y1="{0}" x2="750" y2="{0}" style="stroke:#CCCCCC;stroke-width:1"/>"#,
            y_pos
        ));
        ticks.push_str(&format!(
            r#"<text x="45" y="{}" text-anchor="end" font-size="12">{:.1}</text>"#,
            y_pos + 4.0,
            y
        ));
    }
    ticks
}

pub fn generate_x_axis_ticks(num_bars: usize) -> String {
    let mut ticks = String::new();
    let bar_width = 700.0 / (num_bars as f64);

    for i in 0..num_bars {
        let x = 50.0 + (i as f64 * bar_width) + (bar_width / 2.0);
        ticks.push_str(&format!(
            r#"<text x="{}" y="520" text-anchor="middle" font-size="12">{}</text>"#,
            x,
            i + 1
        ));
    }
    ticks
}

pub fn generate_value_text(x: f64, y: f64, value: f64) -> String {
    format!(
        r#"<text x="{}" y="{}" text-anchor="middle" font-size="12">{:.1}</text>"#,
        x,
        y - 10.0,
        value
    )
}
