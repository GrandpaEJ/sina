use sina::{Color, Paint, Point, Surface, CpuSurface, Font};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤡 Sina Emoji Rendering Test");
    
    // Find an emoji font
    let font_path = find_emoji_font();
    
    let font = if let Some(path) = font_path {
        println!("Loading emoji font: {}", path);
        Font::from_file(path)?
    } else {
        println!("⚠️ No dedicated emoji font found. Using default.");
        Font::from_file("/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf")?
    };

    // Create surface
    let mut surface = CpuSurface::new(1400, 600);
    surface.canvas().clear(Color::rgb(255, 255, 255));
    
    let mut paint = Paint::with_color(Color::BLACK);
    
    // Draw some text with emojis
    surface.canvas().draw_text(
        "Hello World! 😀 🌍 🚀",
        Point::new(50.0, 300.0),
        &font,
        60.0,
        &paint,
    );

    // Ensure output directory exists
    std::fs::create_dir_all("examples/output/text")?;
    surface.save_png("examples/output/text/emoji_test.png")?;
    println!("✅ Saved to examples/output/text/emoji_test.png");
    
    Ok(())
}

fn find_emoji_font() -> Option<String> {
    // List of common emoji fonts and their typical paths across OSs
    let candidates = [
        // Linux / Android
        "/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf",
        "/usr/share/fonts/opentype/noto/NotoColorEmoji.ttf",
        "/usr/share/fonts/google-noto-emoji/NotoColorEmoji.ttf",
        // macOS
        "/System/Library/Fonts/Apple Color Emoji.ttc",
        "/System/Library/Fonts/sbix/Apple Color Emoji.ttc",
        // Windows (typical mount or WSL path)
        "/mnt/c/Windows/Fonts/seguiemj.ttf",
        "C:\\Windows\\Fonts\\seguiemj.ttf",
        // Fallbacks
        "/usr/share/fonts/truetype/android/AndroidEmoji.ttf",
    ];

    for path_str in candidates {
        let path = std::path::Path::new(path_str);
        if path.exists() {
            return Some(path.to_string_lossy().to_string());
        }
    }
    
    // Generic search for "emoji" in standard directories using `find`
    // This catches variations like "TwitterColorEmoji" or custom installed ones
    if cfg!(target_os = "linux") {
        let output = std::process::Command::new("find")
            .args(["/usr/share/fonts", "/home/grandpa/.local/share/fonts", "-name", "*Emoji*.ttf"])
            .output()
            .ok()?;
            
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Some(line) = stdout.lines().next() {
            return Some(line.to_string());
        }
    }

    None
}
