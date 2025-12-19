//! Basic shapes example for Sina rendering engine

use sina::{Color, CpuSurface, Paint, Point, Rect, Surface};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Sina Basic Shapes Example");

    // Create a 800x600 surface
    let mut surface = CpuSurface::new(800, 600);

    // Clear with white background
    surface.canvas().clear(Color::WHITE);

    // Draw a filled red rectangle
    let mut paint = Paint::with_color(Color::RED);
    surface
        .canvas()
        .draw_rect(Rect::new(50.0, 50.0, 200.0, 150.0), &paint);

    // Draw a filled blue circle
    paint.set_color(Color::BLUE);
    surface
        .canvas()
        .draw_circle(Point::new(450.0, 150.0), 75.0, &paint);

    // Draw a green line
    paint.set_color(Color::GREEN);
    surface
        .canvas()
        .draw_line(Point::new(100.0, 400.0), Point::new(700.0, 400.0), &paint);

    // Draw a semi-transparent purple rectangle
    paint.set_color(Color::rgba(128, 0, 128, 128));
    surface
        .canvas()
        .draw_rect(Rect::new(300.0, 300.0, 300.0, 200.0), &paint);

    // Save to PNG
    let path = "examples/output/shapes/basic_shapes.png";
    std::fs::create_dir_all("examples/output/shapes")?;
    surface.save_png(path)?;

    println!("✅ Saved to {}", path);
    Ok(())
}
