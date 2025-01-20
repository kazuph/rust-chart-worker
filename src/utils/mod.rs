pub mod png;
pub mod svg;

pub fn get_default_colors() -> Vec<&'static str> {
    vec![
        "#0000FF", "#FFB3B3", "#B3E0FF", "#FFE6B3", "#B3FFB3", "#E6B3FF", "#FFD9B3",
    ]
}

pub fn generate_value_text(x: f64, y: f64, value: f64) -> String {
    format!(
        r#"<text x="{}" y="{}" text-anchor="middle" font-size="12">{:.1}</text>"#,
        x,
        y - 10.0,
        value
    )
}

pub fn format_number(num: f64) -> String {
    if num.fract() == 0.0 {
        format!("{:.0}", num)
    } else {
        format!("{:.1}", num)
    }
}
