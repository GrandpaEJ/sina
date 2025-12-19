//! Test to debug glyph positioning

use sina::Font;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let font_path = "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf";
    let font = Font::from_file(font_path)?;

    let raw_face = font.face().raw_face();
    let face = rustybuzz::Face::from_slice(raw_face.data, 0).ok_or("Failed to create face")?;

    let mut buffer = rustybuzz::UnicodeBuffer::new();
    buffer.push_str("Hello");

    let output = rustybuzz::shape(&face, &[], buffer);

    println!("Font units per EM: {}", font.units_per_em());
    println!("\nGlyph info for 'Hello':");

    for (i, (pos, info)) in output
        .glyph_positions()
        .iter()
        .zip(output.glyph_infos())
        .enumerate()
    {
        println!("\nGlyph {}:", i);
        println!("  glyph_id: {}", info.glyph_id);
        println!("  cluster: {}", info.cluster);
        println!("  x_advance: {} (raw)", pos.x_advance);
        println!("  y_advance: {} (raw)", pos.y_advance);
        println!("  x_offset: {}", pos.x_offset);
        println!("  y_offset: {}", pos.y_offset);
    }

    Ok(())
}
