//! Multilingual text rendering example

use sina::{Color, Paint, Point, Surface, CpuSurface, Font};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌍 Sina Multilingual Text Example\n");
    
    // Create surface
    let mut surface = CpuSurface::new(1000, 1000);
    surface.canvas().clear(Color::rgb(245, 248, 250));
    
    // Default font for titles
    let default_font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
    let default_font = if std::path::Path::new(default_font_path).exists() {
        Font::from_file(default_font_path)?
    } else {
        // Fallback to FreeSans if DejaVu is missing
        Font::from_file("/usr/share/fonts/truetype/freefont/FreeSans.ttf")?
    };

    // Title
    let mut paint = Paint::with_color(Color::rgb(20, 30, 50));
    surface.canvas().draw_text(
        "Multilingual Text Rendering",
        Point::new(50.0, 70.0),
        &default_font,
        40.0,
        &paint,
    );
    
    // Subtitle
    paint.set_color(Color::rgb(80, 90, 110));
    surface.canvas().draw_text(
        "Pure Rust • Unicode Support • Complex Scripts",
        Point::new(50.0, 115.0),
        &default_font,
        20.0,
        &paint,
    );
    
    // Different languages with their specific fonts
    let samples = vec![
        ("English", "Hello, World! The quick brown fox jumps.", "DejaVuSans.ttf"),
        ("Spanish", "¡Hola, Mundo! El rápido zorro marrón salta.", "DejaVuSans.ttf"),
        ("French", "Bonjour, Monde! Le renard brun rapide saute.", "DejaVuSans.ttf"),
        ("German", "Hallo, Welt! Der schnelle braune Fuchs springt.", "DejaVuSans.ttf"),
        ("Portuguese", "Olá, Mundo! A rápida raposa marrom pula.", "DejaVuSans.ttf"),
        ("Arabic", "مرحبا بالعالم", "NotoSansArabic-Regular.ttf"),
        ("Hebrew", "שלום עולם", "DejaVuSans.ttf"),
        ("Russian", "Привет, мир!", "DejaVuSans.ttf"),
        ("Greek", "Γεια σου κόσμε!", "DejaVuSans.ttf"),
        ("Japanese", "こんにちは世界", "NotoSansCJK-Regular.ttc"),
        ("Korean", "안녕하세요 세계", "NotoSansCJK-Regular.ttc"),
        ("Chinese", "你好世界", "NotoSansCJK-Regular.ttc"),
        ("Thai", "สวัสดีชาวโลก", "NotoSansThai-Regular.ttf"),
        ("Hindi", "नमस्ते दुनिया", "NotoSansDevanagari-Regular.ttf"),
        ("Bangla", "হ্যালো বিশ্ব! দ্রুত বাদামী শিয়াল লাফ দেয়।", "NotoSansBengali-Regular.ttf"),
    ];
    
    println!("Rendering {} languages...\n", samples.len());
    
    for (i, (lang, text, font_name)) in samples.iter().enumerate() {
        let y = 180.0 + (i as f32 * 50.0);
        
        // Language label (using default font)
        let mut label_paint = Paint::with_color(Color::rgb(100, 110, 130));
        surface.canvas().draw_text(
            &format!("{}:", lang),
            Point::new(50.0, y),
            &default_font,
            16.0,
            &label_paint,
        );
        
        // Find the best font for this language
        let font_path = find_font(font_name);
        
        if let Some(path) = font_path {
            let font = if path.ends_with(".ttc") {
                // For CJK collection, index 0 is usually fine for general use
                Font::from_collection(&path, 0).ok()
            } else {
                Font::from_file(&path).ok()
            };
            
            if let Some(font) = font {
                 // Text sample
                let mut text_paint = Paint::with_color(Color::rgb(30, 40, 60));
                surface.canvas().draw_text(
                    text,
                    Point::new(200.0, y),
                    &font,
                    22.0,
                    &text_paint,
                );
                println!("  ✓ Rendered: {} ({})", lang, font_name);
            } else {
                 println!("  ✗ Failed to load font for: {}", lang);
            }
        } else {
             println!("  ✗ Font not found for: {} ({})", lang, font_name);
             // Verify fallback? No, just warn for now to avoid tofu.
        }
    }
    
    // Save output
    surface.save_png("examples/output/text/multilingual.png")?;
    println!("\n✅ Saved to examples/output/text/multilingual.png");
    
    Ok(())
}

fn find_font(name: &str) -> Option<String> {
    let search_paths = [
        "/usr/share/fonts/truetype/dejavu",
        "/usr/share/fonts/truetype/freefont",
        "/usr/share/fonts/opentype/noto", 
        "/usr/share/fonts/truetype/noto",
    ];
    
    for folder in search_paths {
        let path = std::path::Path::new(folder).join(name);
        if path.exists() {
            return Some(path.to_string_lossy().to_string());
        }
    }
    
    // Try generic search in /usr/share/fonts if not found in specific folders
    let output = std::process::Command::new("find")
        .args(["/usr/share/fonts", "-name", name])
        .output()
        .ok()?;
        
    let stdout = String::from_utf8_lossy(&output.stdout);
    if let Some(line) = stdout.lines().next() {
        return Some(line.to_string());
    }

    None
}
