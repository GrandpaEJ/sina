//! Multi-font comparison example

use sina::{Color, CpuSurface, Font, Paint, Point, Rect, Surface};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔤 Multi-Font Comparison\n");

    // Try to load multiple fonts
    let font_configs = vec![
        (
            "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            "DejaVu Sans",
        ),
        (
            "/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf",
            "DejaVu Serif",
        ),
        (
            "/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf",
            "DejaVu Mono",
        ),
        (
            "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
            "Liberation Sans",
        ),
        (
            "/usr/share/fonts/truetype/liberation/LiberationSerif-Regular.ttf",
            "Liberation Serif",
        ),
        (
            "/usr/share/fonts/truetype/liberation/LiberationMono-Regular.ttf",
            "Liberation Mono",
        ),
    ];

    let mut loaded_fonts = Vec::new();

    for (path, name) in font_configs {
        if std::path::Path::new(path).exists() {
            match Font::from_file(path) {
                Ok(font) => {
                    println!("✓ Loaded: {}", name);
                    loaded_fonts.push((font, name.to_string()));
                }
                Err(e) => println!("✗ Failed to load {}: {}", name, e),
            }
        }
    }

    if loaded_fonts.is_empty() {
        return Err("No fonts could be loaded".into());
    }

    println!("\nGenerating comparison...\n");

    // Create surface
    let height = 120 + (loaded_fonts.len() as i32 * 140);
    let mut surface = CpuSurface::new(1200, height);
    surface.canvas().clear(Color::WHITE);

    // Title
    let paint = Paint::with_color(Color::rgb(30, 30, 30));
    surface.canvas().draw_text(
        "Font Family Comparison",
        Point::new(50.0, 70.0),
        &loaded_fonts[0].0,
        48.0,
        &paint,
    );

    // Divider
    let divider = Paint::with_color(Color::rgb(200, 200, 200));
    surface
        .canvas()
        .draw_rect(Rect::new(50.0, 100.0, 1100.0, 2.0), &divider);

    // Sample text with each font
    let sample_text = "The quick brown fox jumps over the lazy dog";
    let numbers = "0123456789";

    for (i, (font, name)) in loaded_fonts.iter().enumerate() {
        let y_base = 180.0 + (i as f32 * 140.0);

        // Font name
        let label_paint = Paint::with_color(Color::rgb(120, 120, 120));
        surface.canvas().draw_text(
            name,
            Point::new(50.0, y_base - 20.0),
            font,
            18.0,
            &label_paint,
        );

        // Sample text
        let text_paint = Paint::with_color(Color::rgb(20, 20, 20));
        surface.canvas().draw_text(
            sample_text,
            Point::new(50.0, y_base + 30.0),
            font,
            32.0,
            &text_paint,
        );

        // Numbers and special chars
        surface.canvas().draw_text(
            &format!("{} @#$ ABC abc", numbers),
            Point::new(50.0, y_base + 75.0),
            font,
            24.0,
            &text_paint,
        );

        // Light divider
        surface.canvas().draw_rect(
            Rect::new(50.0, y_base + 105.0, 1100.0, 1.0),
            &Paint::with_color(Color::rgb(230, 230, 230)),
        );
    }

    // Save
    let path = "examples/output/text/font_comparison.png";
    std::fs::create_dir_all("examples/output/text")?;
    surface.save_png(path)?;
    println!("✅ Saved to {}", path);

    Ok(())
}
