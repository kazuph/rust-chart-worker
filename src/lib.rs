use serde::Deserialize;
use std::io::Cursor;
use worker::*;

use image::{ImageBuffer, Rgba};
use plotters::prelude::*;

#[derive(Deserialize)]
struct GraphRequest {
    graph_type: String,
    data: Vec<f64>,
}

#[event(fetch)]
async fn fetch(mut req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    if req.method() != Method::Post {
        return Response::error("Method not allowed", 405);
    }

    let graph_req: GraphRequest = match req.json().await {
        Ok(val) => val,
        Err(_) => return Response::error("Invalid JSON", 400),
    };

    if graph_req.data.is_empty() {
        return Response::error("Data array is empty", 400);
    }

    let width = 800;
    let height = 600;

    let mut buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    if let Err(e) = try_draw_chart(&mut buffer, &graph_req) {
        return Response::error(format!("Failed to draw chart: {}", e), 500);
    }

    let mut png_bytes = Vec::new();
    {
        let mut cursor = Cursor::new(&mut png_bytes);
        match image::DynamicImage::ImageRgba8(buffer).write_to(&mut cursor, image::ImageFormat::Png)
        {
            Ok(_) => {}
            Err(_) => return Response::error("Failed to encode image", 500),
        }
    }

    let mut headers = Headers::new();
    headers.set("Content-Type", "image/png")?;

    Ok(Response::from_bytes(png_bytes)?.with_headers(headers))
}

fn try_draw_chart(
    buffer: &mut ImageBuffer<Rgba<u8>, Vec<u8>>,
    graph_req: &GraphRequest,
) -> std::result::Result<(), String> {
    let (width, height) = buffer.dimensions();
    let root = BitMapBackend::with_buffer(buffer, (width, height)).into_drawing_area();
    root.fill(&WHITE).map_err(|e| e.to_string())?;

    let max_val = graph_req.data.iter().cloned().fold(f64::MIN, f64::max);
    let min_val = graph_req.data.iter().cloned().fold(f64::MAX, f64::min);

    // Add some padding to the value range
    let value_margin = (max_val - min_val) * 0.1;
    let y_range = (min_val - value_margin)..(max_val + value_margin);

    let mut chart = ChartBuilder::on(&root)
        .margin(50)
        .build_cartesian_2d(0..(graph_req.data.len() as i32), y_range)
        .map_err(|e| e.to_string())?;

    // Configure mesh with better styling
    chart
        .configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .axis_style(&BLACK.mix(0.8))
        .draw()
        .map_err(|e| e.to_string())?;

    match graph_req.graph_type.as_str() {
        "bar" => {
            chart
                .draw_series(graph_req.data.iter().enumerate().map(|(i, &v)| {
                    let bar_width = 0.8;
                    let x0 = i as f64 - bar_width / 2.0;
                    let x1 = i as f64 + bar_width / 2.0;
                    Rectangle::new([(x0 as i32, 0.0), (x1 as i32, v)], BLUE.mix(0.8).filled())
                }))
                .map_err(|e| e.to_string())?;
        }
        "scatter" => {
            chart
                .draw_series(
                    graph_req
                        .data
                        .iter()
                        .enumerate()
                        .map(|(i, &v)| Circle::new((i as i32, v), 3, BLUE.mix(0.8).filled())),
                )
                .map_err(|e| e.to_string())?;
        }
        _ => {
            // Default to line chart
            chart
                .draw_series(LineSeries::new(
                    graph_req
                        .data
                        .iter()
                        .enumerate()
                        .map(|(i, &v)| (i as i32, v)),
                    &BLUE.mix(0.8),
                ))
                .map_err(|e| e.to_string())?;
        }
    }

    root.present().map_err(|e| e.to_string())?;
    Ok(())
}
