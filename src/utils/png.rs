use resvg::usvg::{self, TreeParsing};
use usvg_text_layout::TreeTextToPath;
use worker::console_log;

pub fn svg_to_png(svg_str: &str) -> Result<Vec<u8>, String> {
    // フォントデータベースを初期化
    let mut fontdb = fontdb::Database::new();

    // フォントデータをバイナリとして直接埋め込み
    static FONT_DATA: &[u8] = include_bytes!("../../assets/MPLUS1p-Regular.ttf");
    let font_data = FONT_DATA.to_vec();
    fontdb.load_font_data(font_data);

    // 読み込まれたフォントの情報をログ出力
    for face in fontdb.faces() {
        if let Some(families) = face.families.first() {
            console_log!("Loaded font family: {}", families.0);
        }
    }

    // SVGパース用のオプション設定
    let opt = usvg::Options {
        font_family: "M PLUS 1p".to_string(),
        font_size: 12.0,
        dpi: 96.0,
        ..usvg::Options::default()
    };

    // SVGをパース
    let mut tree =
        usvg::Tree::from_str(svg_str, &opt).map_err(|e| format!("Failed to parse SVG: {}", e))?;

    // テキストをパスに変換
    tree.convert_text(&fontdb);

    // resvgツリーを作成
    let rtree = resvg::Tree::from_usvg(&tree);

    // レンダリングサイズを取得
    let width = rtree.size.width() as u32;
    let height = rtree.size.height() as u32;

    // ピクスマップを作成
    let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or("Failed to create pixmap")?;

    // SVGをレンダリング
    rtree.render(tiny_skia::Transform::default(), &mut pixmap.as_mut());

    // PNGにエンコード
    Ok(pixmap
        .encode_png()
        .map_err(|e| format!("Failed to encode PNG: {}", e))?)
}
