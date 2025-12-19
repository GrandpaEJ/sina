//! Comprehensive text samples showcasing font rendering capabilities

use sina::{Color, Paint, Point, Surface, CpuSurface, Font};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Text Rendering Showcase\n");
    
    // Find system font
    let font_paths = vec![
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
    ];
    
    let font_path = font_paths.iter()
        .find(|p| std::path::Path::new(p).exists())
        .ok_or("No system fonts found")?;
    
    println!("Using font: {}\n", font_path);
    let font = Font::from_file(font_path)?;
    
    // Create canvas
    let mut surface = CpuSurface::new(1200, 900);
    surface.canvas().clear(Color::rgb(250, 250, 255));
    
    // Title
    let mut paint = Paint::with_color(Color::rgb(20, 30, 60));
    surface.canvas().draw_text(
        "Sina Text Rendering Engine",
        Point::new(50.0, 80.0),
        &font,
        56.0,
        &paint,
    );
    
    // Subtitle  
    paint.set_color(Color::rgb(80, 90, 120));
    surface.canvas().draw_text(
        "Pure Rust - TrueType/OpenType - Complex Scripts",
        Point::new(50.0, 140.0),
        &font,
        22.0,
        &paint,
    );
    
    // Section 1: Font Sizes
    println!("Rendering font sizes...");
    paint.set_color(Color::rgb(40, 50, 80));
    surface.canvas().draw_text(
        "Variable Font Sizes",
        Point::new(50.0, 200.0),
        &font,
        32.0,
        &paint,
    );
    
    let sizes = vec![12.0, 16.0, 20.0, 24.0, 32.0];
    for (i, size) in sizes.iter().enumerate() {
        paint.set_color(Color::rgb(60, 70, 100));
        surface.canvas().draw_text(
            &format!("{}px - The quick brown fox jumps over the lazy dog", *size as i32),
            Point::new(70.0, 240.0 + (i as f32 * 45.0)),
            &font,
            *size,
            &paint,
        );
    }
    
    // Section 2: Colors
    println!("Rendering colors...");
    paint.set_color(Color::rgb(40, 50, 80));
    surface.canvas().draw_text(
        "Color Variations",
        Point::new(50.0, 480.0),
        &font,
        32.0,
        &paint,
    );
    
    let colors = vec![
        (Color::rgb(220, 50, 50), "Red"),
        (Color::rgb(50, 180, 50), "Green"),
        (Color::rgb(50, 120, 255), "Blue"),
        (Color::rgb(180, 50, 180), "Purple"),
        (Color::rgb(255, 165, 0), "Orange"),
    ];
    
    for (i, (color, name)) in colors.iter().enumerate() {
        paint.set_color(*color);
        surface.canvas().draw_text(
            &format!("{} Text Sample", name),
            Point::new(70.0, 530.0 + (i as f32 * 40.0)),
            &font,
            26.0,
            &paint,
        );
    }
    
    // Section 3: Special Characters
    println!("Rendering special characters...");
    paint.set_color(Color::rgb(40, 50, 80));
    surface.canvas().draw_text(
        "Unicode Support",
        Point::new(650.0, 200.0),
        &font,
        32.0,
        &paint,
    );
    
    paint.set_color(Color::rgb(60, 70, 100));
    surface.canvas().draw_text(
        "Numbers: 0123456789",
        Point::new(670.0, 250.0),
        &font,
        20.0,
        &paint,
    );
    
    surface.canvas().draw_text(
        "Punctuation: !@#$%^&*()_+-=[]{}|;:,.<>?",
        Point::new(670.0, 290.0),
        &font,
        18.0,
        &paint,
    );
    
    surface.canvas().draw_text(
        "Quotes: \"Hello\" 'World'",
        Point::new(670.0, 330.0),
        &font,
        20.0,
        &paint,
    );
    
    surface.canvas().draw_text(
        "Accented: Cafe Resume Naive",
        Point::new(670.0, 370.0),
        &font,
        20.0,
        &paint,
    );
    
    // Footer
    paint.set_color(Color::rgb(100, 110, 140));
    surface.canvas().draw_text(
        "Powered by: ttf-parser + fontdue + rustybuzz",
        Point::new(50.0, 850.0),
        &font,
        16.0,
        &paint,
    );
    
    // Save
    surface.save_png("examples/output/text/showcase.png")?;
    println!("\nSaved to examples/output/text/showcase.png");
    
    Ok(())
}
