use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum GraphType {
    Line,
    Bar,
    Scatter,
    Pie,
    Donut,
    Area,
    Radar,
    Histogram,
    Heatmap,
    Candlestick,
    Gauge,
    Bubble,
    StackedBar,
    MultiLine,
}

impl Default for GraphType {
    fn default() -> Self {
        GraphType::Line
    }
}

#[derive(Deserialize, Clone)]
pub struct DataPoint {
    pub value: f64,
    pub label: Option<String>,
    pub color: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Series {
    pub name: Option<String>,
    pub data: Vec<DataPoint>,
    pub color: Option<String>,
}

#[derive(Deserialize)]
pub struct GraphRequest {
    #[serde(default)]
    pub graph_type: GraphType,
    #[serde(default)]
    pub series: Vec<Series>,
    #[serde(default)]
    pub data: Vec<f64>, // 後方互換性のため残す
    pub title: Option<String>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub colors: Option<Vec<String>>,
    pub theme: Option<String>,
    pub bins: Option<usize>, // ヒストグラム用
    pub show_density: Option<bool>, // ヒストグラム用
    pub cell_size: Option<f64>, // ヒートマップ用
    pub show_values: Option<bool>, // ヒートマップ用
}
