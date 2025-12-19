use sina::{Color, Paint, Point, Surface, CpuSurface, Font};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤡 Sina Emoji Rendering Test");
    
    // Create surface
    let mut surface = CpuSurface::new(1400, 600);
    surface.canvas().clear(Color::rgb(255, 255, 255));

    // 1. Load Text Font (DejaVu Sans or FreeSans)
    let text_font_name = "DejaVuSans.ttf";
    let text_font_path = find_font(text_font_name).or_else(|| find_font("FreeSans.ttf"));
    
    // 2. Load Emoji Font
    let emoji_font_path = find_emoji_font();

    if let (Some(text_path), Some(emoji_path)) = (text_font_path, emoji_font_path) {
        println!("Loaded Text Font: {}", text_path);
        println!("Loaded Emoji Font: {}", emoji_path);

        let text_font = Font::from_file(&text_path).expect("Failed to load text font");
        let emoji_font = Font::from_file(&emoji_path).expect("Failed to load emoji font");

        // 3. Draw Standard Text
        let paint = Paint::with_color(Color::rgb(20, 20, 20)); // Dark Gray
        surface.canvas().draw_text(
            "Hello World! Text + Emoji:",
            Point::new(50.0, 200.0),
            &text_font,
            60.0,
            &paint,
        );

        // 4. Draw Emojis (using Emoji Font)
        // Positioned after the text (ap
        // prox width)
        let emoji_text = "😀🌍🦀🚀⚡📍🔎";
        surface.canvas().draw_text(
            emoji_text,
            Point::new(850.0, 200.0), // Hardcoded relative position for simplicity
            &emoji_font,
            60.0,
            &paint, // Color is ignored for RGBA bitmaps, but logic uses alpha
        );
        
        // Ensure output directory exists
        std::fs::create_dir_all("examples/output/text")?;
        surface.save_png("examples/output/text/emoji_test.png")?;
        println!("✅ Saved to examples/output/text/emoji_test.png");
    } else {
        println!("❌ Could not find required fonts.");
        println!("Text font found: {:?}", find_font("DejaVuSans.ttf").is_some());
        println!("Emoji font found: {:?}", find_emoji_font().is_some());
    }

    Ok(())
}

fn find_font(name: &str) -> Option<String> {
    // Try common locations
    let paths = [
        format!("/usr/share/fonts/truetype/dejavu/{}", name),
        format!("/usr/share/fonts/truetype/freefont/{}", name),
        format!("/System/Library/Fonts/{}", name),
    ];
    
    for p in paths {
        if std::path::Path::new(&p).exists() {
            return Some(p);
        }
    }
    
    // Fallback to `find`
    if cfg!(target_os = "linux") {
        let output = std::process::Command::new("find")
            .args(["/usr/share/fonts", "-name", name])
            .output()
            .ok()?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        stdout.lines().next().map(|s| s.to_string())
    } else {
        None
    }
}

fn find_emoji_font() -> Option<String> {
    if cfg!(target_os = "macos") {
         if std::path::Path::new("/System/Library/Fonts/Apple Color Emoji.ttc").exists() {
             return Some("/System/Library/Fonts/Apple Color Emoji.ttc".to_string());
         }
    }
    
    // Linux/Other
    if let Some(path) = find_font("NotoColorEmoji.ttf") {
        return Some(path);
    }
    
    // Fallbacks
    let candidates = [
        "/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf",
        "/usr/share/fonts/google-noto-emoji/NotoColorEmoji.ttf",
        "/usr/share/fonts/truetype/android/AndroidEmoji.ttf",
    ];
    
    for p in candidates {
        if std::path::Path::new(p).exists() {
            return Some(p.to_string());
        }
    }
    None
}
