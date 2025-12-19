//! Text samples example showcasing different fonts and sizes

use sina::{Color, Paint, Point, Surface, CpuSurface, Font};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 Sina Text Samples Example\n");
    
    // Try to find system fonts
    let font_paths = vec![
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf",
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf",
        "/usr/share/fonts/truetype/noto/NotoSans-Regular.ttf",
    ];
    
    // Find first available font
    let font_path = font_paths.iter()
        .find(|p| std::path::Path::new(p).exists())
        .ok_or("No system fonts found. Please install DejaVu or Liberation fonts.")?;
    
    println!("Using font: {}", font_path);
    
    let font = Font::from_file(font_path)?;
    
    // Create surface
    let mut surface = CpuSurface::new(1200, 800);
    surface.canvas().clear(Color::rgb(250, 250, 255));
    
    // Title
    let mut paint = Paint::with_color(Color::rgb(20, 20, 40));
    surface.canvas().draw_text(
        "Sina Text Rendering",
        Point::new(50.0, 80.0),
        &font,
        48.0,
        &paint,
    );
    
    // Subtitle
    paint.set_color(Color::rgb(80, 80, 100));
    surface.canvas().draw_text(
        "Pure Rust 2D Graphics Engine",
        Point::new(50.0, 130.0),
        &font,
        24.0,
        &paint,
    );
    
    // Different sizes
    println!("Drawing different font sizes...");
    paint.set_color(Color::rgb(40, 40, 60));
    
    let sizes = vec![12.0, 16.0, 20.0, 24.0, 32.0, 40.0];
    for (i, size) in sizes.iter().enumerate() {
        let y = 200.0 + (i as f32 * 60.0);
        surface.canvas().draw_text(
            &format!("Font size {} pixels - The quick brown fox", size),
            Point::new(50.0, y),
            &font,
            *size,
            &paint,
        );
    }
    
    // Color variations
    println!("Drawing colored text...");
    let colors = vec![
        (Color::rgb(255, 50, 50), "Red Text"),
        (Color::rgb(50, 200, 50), "Green Text"),
        (Color::rgb(50, 100, 255), "Blue Text"),
        (Color::rgb(200, 50, 200), "Purple Text"),
    ];
    
    for (i, (color, text)) in colors.iter().enumerate() {
        paint.set_color(*color);
        surface.canvas().draw_text(
            text,
            Point::new(650.0, 200.0 + (i as f32 * 50.0)),
            &font,
            28.0,
            &paint,
        );
    }
    
    // Unicode support
    println!("Drawing Unicode text...");
    paint.set_color(Color::rgb(20, 20, 40));
    surface.canvas().draw_text(
        "Unicode: © ® ™ § ¶ † ‡",
        Point::new(50.0, 600.0),
        &font,
        24.0,
        &paint,
    );
    
    // Numbers and symbols
    surface.canvas().draw_text(
        "Numbers: 0123456789",
        Point::new(50.0, 650.0),
        &font,
        24.0,
        &paint,
    );
    
    surface.canvas().draw_text(
        "Symbols: !@#$%^&*()_+-=[]{}|;:,.<>?",
        Point::new(50.0, 700.0),
        &font,
        20.0,
        &paint,
    );
    
    // Save output
    surface.save_png("examples/output/text_samples/font_showcase.png")?;
    println!("\n✅ Saved to examples/output/text_samples/font_showcase.png");
    
    Ok(())
}
