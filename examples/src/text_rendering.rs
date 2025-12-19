//! Text rendering example demonstrating TrueType/OpenType font support

use sina::{Color, Paint, Point, Surface, CpuSurface, Font};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Sina Text Rendering Example");

    // Note: This example requires a font file
    // You can use any .ttf or .otf font file
    // For example, download a free font from Google Fonts
    
    println!("⚠️  To run this example, you need a font file.");
    println!("   Download a font (e.g., from fonts.google.com) and update the path below.");
    
    // Example paths (uncomment and modify for your system):
    // let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";  // Linux
    // let font_path = "/System/Library/Fonts/Helvetica.ttc";              // macOS
    // let font_path = "C:\\Windows\\Fonts\\arial.ttf";                    // Windows
    
    // For demonstration, we'll show what the code would look like:
    /*
    // Load font
    let font = Font::from_file(font_path)?;
    println!("✓ Loaded font: {:?}", font.family_name());
    
    // Create surface
    let mut surface = CpuSurface::new(800, 400);
    
    // Clear with white background
    surface.canvas().clear(Color::WHITE);
    
    // Draw title text
    let mut paint = Paint::with_color(Color::rgb(40, 40, 40));
    surface.canvas().draw_text(
        "Sina Rendering Engine",
        Point::new(50.0, 100.0),
        &font,
        48.0,
        &paint,
    );
    
    // Draw subtitle
    paint.set_color(Color::rgb(100, 100, 100));
    surface.canvas().draw_text(
        "Pure Rust 2D Graphics with TrueType/OpenType Support",
        Point::new(50.0, 160.0),
        &font,
        24.0,
        &paint,
    );
    
    // Draw multi-line text
    paint.set_color(Color::rgb(50, 120, 200));
    let lines = vec![
        "✓ TrueType (.ttf) fonts",
        "✓ OpenType (.otf) fonts",
        "✓ Complex script shaping",
        "✓ Glyph caching",
    ];
    
    for (i, line) in lines.iter().enumerate() {
        surface.canvas().draw_text(
            line,
            Point::new(50.0, 220.0 + i as f32 * 35.0),
            &font,
            20.0,
            &paint,
        );
    }
    
    // Save to file
    surface.save_png("text_rendering.png")?;
    
    println!("✅ Saved to text_rendering.png");
    */
    
    println!("\n💡 Tip: Uncomment the code above and provide a font path to test text rendering!");
    
    Ok(())
}
