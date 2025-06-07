use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: String,
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub text: String,
    pub grid: String,
    pub axis: String,
    pub colors: Vec<String>,
}

impl Theme {
    pub fn light() -> Self {
        Self {
            name: "Light".to_string(),
            background: "#ffffff".to_string(),
            primary: "#3498db".to_string(),
            secondary: "#2ecc71".to_string(),
            accent: "#e74c3c".to_string(),
            text: "#333333".to_string(),
            grid: "#dddddd".to_string(),
            axis: "#333333".to_string(),
            colors: vec![
                "#3498db".to_string(),
                "#e74c3c".to_string(),
                "#2ecc71".to_string(),
                "#f39c12".to_string(),
                "#9b59b6".to_string(),
                "#1abc9c".to_string(),
                "#34495e".to_string(),
                "#e67e22".to_string(),
            ],
        }
    }

    pub fn dark() -> Self {
        Self {
            name: "Dark".to_string(),
            background: "#2c3e50".to_string(),
            primary: "#3498db".to_string(),
            secondary: "#2ecc71".to_string(),
            accent: "#e74c3c".to_string(),
            text: "#ecf0f1".to_string(),
            grid: "#34495e".to_string(),
            axis: "#ecf0f1".to_string(),
            colors: vec![
                "#3498db".to_string(),
                "#e74c3c".to_string(),
                "#2ecc71".to_string(),
                "#f39c12".to_string(),
                "#9b59b6".to_string(),
                "#1abc9c".to_string(),
                "#95a5a6".to_string(),
                "#e67e22".to_string(),
            ],
        }
    }

    pub fn material() -> Self {
        Self {
            name: "Material".to_string(),
            background: "#fafafa".to_string(),
            primary: "#2196f3".to_string(),
            secondary: "#4caf50".to_string(),
            accent: "#ff5722".to_string(),
            text: "#212121".to_string(),
            grid: "#e0e0e0".to_string(),
            axis: "#424242".to_string(),
            colors: vec![
                "#2196f3".to_string(),
                "#f44336".to_string(),
                "#4caf50".to_string(),
                "#ff9800".to_string(),
                "#9c27b0".to_string(),
                "#00bcd4".to_string(),
                "#607d8b".to_string(),
                "#795548".to_string(),
            ],
        }
    }

    pub fn minimal() -> Self {
        Self {
            name: "Minimal".to_string(),
            background: "#ffffff".to_string(),
            primary: "#000000".to_string(),
            secondary: "#666666".to_string(),
            accent: "#999999".to_string(),
            text: "#000000".to_string(),
            grid: "#f0f0f0".to_string(),
            axis: "#000000".to_string(),
            colors: vec![
                "#000000".to_string(),
                "#666666".to_string(),
                "#999999".to_string(),
                "#cccccc".to_string(),
                "#333333".to_string(),
                "#555555".to_string(),
                "#777777".to_string(),
                "#aaaaaa".to_string(),
            ],
        }
    }

    pub fn get_color(&self, index: usize) -> &String {
        &self.colors[index % self.colors.len()]
    }

    pub fn generate_css(&self) -> String {
        format!(
            r#"
    .chart-background {{ fill: {}; }}
    .chart-text {{ font-family: Arial, sans-serif; fill: {}; }}
    .chart-title {{ font-family: Arial, sans-serif; font-size: 16px; font-weight: bold; fill: {}; text-anchor: middle; }}
    .chart-axis {{ stroke: {}; stroke-width: 2; }}
    .chart-axis-text {{ font-family: Arial, sans-serif; font-size: 12px; fill: {}; }}
    .chart-grid {{ stroke: {}; stroke-width: 1; stroke-dasharray: 2,2; }}
    .chart-primary {{ fill: {}; stroke: {}; }}
    .chart-secondary {{ fill: {}; stroke: {}; }}
    .chart-accent {{ fill: {}; stroke: {}; }}
    "#,
            self.background,
            self.text,
            self.text,
            self.axis,
            self.text,
            self.grid,
            self.primary, self.primary,
            self.secondary, self.secondary,
            self.accent, self.accent
        )
    }

    pub fn apply_to_svg(&self, svg_content: &str) -> String {
        let mut result = svg_content.to_string();
        
        // テーマ色の置換文字列を事前作成
        let bg_fill = format!("fill=\"{}\"", self.background);
        let text_fill = format!("fill=\"{}\"", self.text);
        let axis_stroke = format!("stroke=\"{}\"", self.axis);
        let grid_stroke = format!("stroke=\"{}\"", self.grid);
        let color0_fill = format!("fill=\"{}\"", self.get_color(0));
        let color1_fill = format!("fill=\"{}\"", self.get_color(1));
        let color2_fill = format!("fill=\"{}\"", self.get_color(2));
        
        // 色の置換を実行
        result = result.replace("fill=\"white\"", &bg_fill);
        result = result.replace("fill=\"#ffffff\"", &bg_fill);
        result = result.replace("fill=\"#333333\"", &text_fill);
        result = result.replace("fill=\"#333\"", &text_fill);
        result = result.replace("stroke=\"black\"", &axis_stroke);
        result = result.replace("stroke=\"#333333\"", &axis_stroke);
        result = result.replace("stroke=\"#333\"", &axis_stroke);
        result = result.replace("stroke=\"#dddddd\"", &grid_stroke);
        result = result.replace("stroke=\"#ddd\"", &grid_stroke);
        result = result.replace("fill=\"#3498db\"", &color0_fill);
        result = result.replace("fill=\"#e74c3c\"", &color1_fill);
        result = result.replace("fill=\"#2ecc71\"", &color2_fill);
        
        // ダークテーマの場合、より包括的な色変更
        if self.name == "Dark" {
            // テキスト色
            result = result.replace("fill=\"#000\"", &text_fill);
            result = result.replace("fill=\"black\"", &text_fill);
            // SVGテキスト要素のfillが明示されていない場合の対応
            result = result.replace("<text ", &format!("<text fill=\"{}\" ", self.text));
            
            // 線の色も白系に変更
            result = result.replace("stroke:black", &format!("stroke:{}", self.axis));
            result = result.replace("stroke:#333", &format!("stroke:{}", self.axis));
            result = result.replace("stroke:#000", &format!("stroke:{}", self.axis));
            
            // その他の暗い色も明るく
            result = result.replace("fill=\"#000000\"", &text_fill);
            result = result.replace("stroke=\"none\"", "stroke=\"none\""); // noneはそのまま
        }
        
        result
    }
}

pub struct ThemeManager {
    themes: Vec<Theme>,
    current_theme: String,
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self {
            themes: vec![
                Theme::light(),
                Theme::dark(),
                Theme::material(),
                Theme::minimal(),
            ],
            current_theme: "Light".to_string(),
        }
    }
}

impl ThemeManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_theme(&mut self, theme: Theme) {
        self.themes.push(theme);
    }

    pub fn get_theme(&self, name: &str) -> Option<&Theme> {
        self.themes.iter().find(|t| t.name == name)
    }

    pub fn get_current_theme(&self) -> Option<&Theme> {
        self.get_theme(&self.current_theme)
    }

    pub fn set_current_theme(&mut self, name: String) {
        if self.themes.iter().any(|t| t.name == name) {
            self.current_theme = name;
        }
    }

    pub fn list_themes(&self) -> Vec<&String> {
        self.themes.iter().map(|t| &t.name).collect()
    }

    pub fn from_name(name: &str) -> Theme {
        match name.to_lowercase().as_str() {
            "dark" => Theme::dark(),
            "material" => Theme::material(),
            "minimal" => Theme::minimal(),
            _ => Theme::light(),
        }
    }
}