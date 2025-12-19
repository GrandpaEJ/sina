//! Multilingual text rendering example

use sina::{Color, Paint, Point, Surface, CpuSurface, Font};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 Sina Multilingual Text Example\n");
    
    // Find a font with good Unicode support
    let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
    
    if !std::path::Path::new(font_path).exists() {
        return Err("DejaVu Sans font not found. Please install dejavu-fonts.".into());
    }
    
    println!("Loading font: {}", font_path);
    let font = Font::from_file(font_path)?;
    
    // Create surface
    let mut surface = CpuSurface::new(1000, 900);
    surface.canvas().clear(Color::rgb(245, 248, 250));
    
    // Title
    let mut paint = Paint::with_color(Color::rgb(20, 30, 50));
    surface.canvas().draw_text(
        "Multilingual Text Rendering",
        Point::new(50.0, 70.0),
        &font,
        40.0,
        &paint,
    );
    
    // Subtitle
    paint.set_color(Color::rgb(80, 90, 110));
    surface.canvas().draw_text(
        "Pure Rust • Unicode Support • Complex Scripts",
        Point::new(50.0, 115.0),
        &font,
        20.0,
        &paint,
    );
    
    // Different languages
    let samples = vec![
        ("English", "Hello, World! The quick brown fox jumps."),
        ("Spanish", "¡Hola, Mundo! El rápido zorro marrón salta."),
        ("French", "Bonjour, Monde! Le renard brun rapide saute."),
        ("German", "Hallo, Welt! Der schnelle braune Fuchs springt."),
        ("Portuguese", "Olá, Mundo! A rápida raposa marrom pula."),
        ("Arabic", "مرحبا بالعالم"),
        ("Hebrew", "שלום עולם"),
        ("Russian", "Привет, мир!"),
        ("Greek", "Γεια σου κόσμε!"),
        ("Japanese", "こんにちは世界"),
        ("Korean", "안녕하세요 세계"),
        ("Chinese", "你好世界"),
        ("Thai", "สวัสดีชาวโลก"),
        ("Hindi", "नमस्ते दुनिया"),
    ];
    
    println!("Rendering {} languages...\n", samples.len());
    
    for (i, (lang, text)) in samples.iter().enumerate() {
        let y = 180.0 + (i as f32 * 50.0);
        
        // Language label
        let mut label_paint = Paint::with_color(Color::rgb(100, 110, 130));
        surface.canvas().draw_text(
            &format!("{}:", lang),
            Point::new(50.0, y),
            &font,
            16.0,
            &label_paint,
        );
        
        // Text sample
        let mut text_paint = Paint::with_color(Color::rgb(30, 40, 60));
        surface.canvas().draw_text(
            text,
            Point::new(200.0, y),
            &font,
            22.0,
            &text_paint,
        );
        
        println!("  ✓ Rendered: {}", lang);
    }
    
    // Save output
    surface.save_png("examples/output/text/multilingual.png")?;
    println!("\n✅ Saved to examples/output/text/multilingual.png");
    
    Ok(())
}
