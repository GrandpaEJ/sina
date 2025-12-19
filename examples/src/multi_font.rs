//! Multi-font comparison example

use sina::{Color, Paint, Point, Surface, CpuSurface, Font};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔤 Sina Multi-Font Comparison\n");
    
    // Try to load multiple fonts
    let font_configs = vec![
        ("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf", "DejaVu Sans"),
        ("/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf", "DejaVu Serif"),
        ("/usr/share/fonts/truetype/dejavu/DejaVuSansMono.ttf", "DejaVu Mono"),
        ("/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf", "Liberation Sans"),
        ("/usr/share/fonts/truetype/liberation/LiberationSerif-Regular.ttf", "Liberation Serif"),
    ];
    
    let mut loaded_fonts = Vec::new();
    
    for (path, name) in font_configs {
        if std::path::Path::new(path).exists() {
            match Font::from_file(path) {
                Ok(font) => {
                    println!("✓ Loaded: {}", name);
                    loaded_fonts.push((font, name));
                }
                Err(e) => println!("✗ Failed to load {}: {}", name, e),
            }
        }
    }
    
    if loaded_fonts.is_empty() {
        return Err("No fonts could be loaded".into());
    }
    
    // Create surface
    let height = 100 + (loaded_fonts.len() as i32 * 120);
    let mut surface = CpuSurface::new(1000, height);
    surface.canvas().clear(Color::WHITE);
    
    // Title
    let paint = Paint::with_color(Color::rgb(30, 30, 30));
    surface.canvas().draw_text(
        "Font Comparison",
        Point::new(50.0, 60.0),
        &loaded_fonts[0].0,
        36.0,
        &paint,
    );
    
    // Draw sample text with each font
    let sample_text = "The quick brown fox jumps over the lazy dog";
    
    for (i, (font, name)) in loaded_fonts.iter().enumerate() {
        let y = 150.0 + (i as f32 * 120.0);
        
        // Font name
        let mut label_paint = Paint::with_color(Color::rgb(100, 100, 100));
        surface.canvas().draw_text(
            name,
            Point::new(50.0, y - 10.0),
            font,
            16.0,
            &label_paint,
        );
        
        //Sample text
        let mut text_paint = Paint::with_color(Color::rgb(20, 20, 20));
        surface.canvas().draw_text(
            sample_text,
            Point::new(50.0, y + 30.0),
            font,
            28.0,
            &text_paint,
        );
        
        // Size variations
        surface.canvas().draw_text(
            "ABC abc 123",
            Point::new(50.0, y + 70.0),
            font,
            20.0,
            &text_paint,
        );
    }
    
    // Save output
    surface.save_png("examples/output/text_samples/font_comparison.png")?;
    println!("\n✅ Saved to examples/output/text_samples/font_comparison.png");
    
    Ok(())
}
