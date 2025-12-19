//! Font format support example
//!
//! Demonstrates various font format features including variable fonts and collections.

use sina::{BitmapFontRenderer, Font, VariableFontManager};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔤 Sina Font Format Support Demo\n");

    // This example demonstrates the API for advanced font features
    // Note: You'll need appropriate font files to see actual results

    println!("📋 Supported Font Formats:");
    println!("   ✓ TrueType (.ttf)");
    println!("   ✓ OpenType (.otf)");
    println!("   ✓ TrueType Collections (.ttc)");
    println!("   ✓ Variable Fonts (fvar)");
    println!("   ✓ Bitmap Fonts (CBDT/SBIX)");
    println!("   ✓ Color Emoji (COLR/CPAL) - stub\n");

    // Example 1: Loading from a TrueType Collection
    println!("💡 Example 1: TrueType Collections (.ttc)");
    println!("   // Load first font from collection");
    println!("   let font = Font::from_collection(\"font.ttc\", 0)?;");
    println!("   ");
    println!("   // Check collection size");
    println!("   let data = std::fs::read(\"font.ttc\")?;");
    println!("   if let Some(count) = Font::collection_size(&data) {{");
    println!("       println!(\"Collection has {{}} fonts\", count);");
    println!("   }}\n");

    // Example 2: Variable Fonts
    println!("💡 Example 2: Variable Fonts");
    println!("   let font = Font::from_file(\"variable-font.ttf\")?;");
    println!("   ");
    println!("   if VariableFontManager::is_variable(&font) {{");
    println!("       let axes = VariableFontManager::axes(&font);");
    println!("       for axis in axes {{");
    println!("           println!(\"Axis: {{}} ({{}})\", axis.tag, axis.name);");
    println!("           println!(\"  Range: {{}} - {{}}\", axis.min_value, axis.max_value);");
    println!("       }}");
    println!("   }}\n");

    // Example 3: Bitmap Fonts
    println!("💡 Example 3: Bitmap Fonts");
    println!("   let font = Font::from_file(\"emoji-font.ttf\")?;");
    println!("   ");
    println!("   if BitmapFontRenderer::has_bitmaps(&font) {{");
    println!("       let sizes = BitmapFontRenderer::available_sizes(&font);");
    println!("       println!(\"Available bitmap sizes: {{:?}}\", sizes);");
    println!("   }}\n");

    // Example 4: Color Emoji
    println!("💡 Example 4: Color Emoji (COLR/CPAL)");
    println!("   use sina::ColorEmojiRenderer;");
    println!("   ");
    println!("   let mut emoji_renderer = ColorEmojiRenderer::new();");
    println!("   let font = Font::from_file(\"color-emoji.ttf\")?;");
    println!("   ");
    println!("   if emoji_renderer.has_color_layers(&font, glyph_id) {{");
    println!("       let layers = emoji_renderer.get_color_layers(&font, glyph_id, 0);");
    println!("       // Render each layer with its color");
    println!("   }}\n");

    println!("📚 Note: These examples show the API structure.");
    println!("   To see them in action, provide appropriate font files for each format.\n");

    println!("🎯 Try it yourself:");
    println!("   1. Find a variable font (many available at fonts.google.com)");
    println!("   2. Try macOS system fonts for .ttc collections");
    println!("   3. emoji fonts often have CBDT or COLR tables\n");

    Ok(())
}
