//! Simple text rendering example

use sina::{Color, Paint, Point, Surface, CpuSurface, Font};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Sina Text Example\n");
    
    // Try to find a system font
    let font_paths = vec![
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
    ];
    
    let font_path = font_paths.iter()
        .find(|p| std::path::Path::new(p).exists())
        .ok_or("No system fonts found")?;
    
    println!("Using font: {}", font_path);
    let font = Font::from_file(font_path)?;
    
    // Create a simple surface
    let mut surface = CpuSurface::new(800, 400);
    surface.canvas().clear(Color::WHITE);
    
    // Draw title
    let paint = Paint::with_color(Color::rgb(40, 40, 60));
    surface.canvas().draw_text(
        "Hello from Sina!",
        Point::new(50.0, 100.0),
        &font,
        48.0,
        &paint,
    );
    
    // Draw subtitle  
    let paint2 = Paint::with_color(Color::rgb(80, 100, 120));
    surface.canvas().draw_text(
        "Pure Rust 2D Graphics",
        Point::new(50.0, 160.0),
        &font,
        28.0,
        &paint2,
    );
    
    // Save
    surface.save_png("examples/output/text_samples/simple_text.png")?;
    println!("✅ Saved to examples/output/text_samples/simple_text.png");
    
    Ok(())
}
