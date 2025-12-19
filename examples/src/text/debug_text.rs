//! Debug text rendering to see what's happening

use sina::{Color, Paint, Point, Rect, Surface, CpuSurface, Font};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Debug Text Rendering\n");
    
    let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
    if !std::path::Path::new(font_path).exists() {
        return Err("Font not found".into());
    }
    
    let font = Font::from_file(font_path)?;
    
    // Create surface with colored background to see contrast
    let mut surface = CpuSurface::new(800, 400);
    
    // Try different backgrounds
    println!("Test 1: White background, black text");
    surface.canvas().clear(Color::WHITE);
    
    let paint = Paint::with_color(Color::BLACK);
    surface.canvas().draw_text(
        "Black on White",
        Point::new(50.0, 100.0),
        &font,
        48.0,
        &paint,
    );
    
    println!("Test 2: Red background, white text");
    let red_bg = Paint::with_color(Color::RED);
    surface.canvas().draw_rect(
        Rect::new(0.0, 150.0, 800.0, 100.0),
        &red_bg,
    );
    
    let white_paint = Paint::with_color(Color::WHITE);
    surface.canvas().draw_text(
        "White on Red",
        Point::new(50.0, 220.0),
        &font,
        48.0,
        &white_paint,
    );
    
    println!("Test 3: Blue background, yellow text");
    let blue_bg = Paint::with_color(Color::BLUE);
    surface.canvas().draw_rect(
        Rect::new(0.0, 300.0, 800.0, 100.0),
        &blue_bg,
    );
    
    let yellow_paint = Paint::with_color(Color::rgb(255, 255, 0));
    surface.canvas().draw_text(
        "Yellow on Blue",
        Point::new(50.0, 350.0),
        &font,
        48.0,
        &yellow_paint,
    );
    
    surface.save_png("examples/output/text/debug_text.png")?;
    println!("\n✅ Saved debug output");
    
    Ok(())
}
