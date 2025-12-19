//! Complex shapes and patterns example

use sina::{Color, Paint, Point, Rect, Surface, CpuSurface};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Sina Complex Shapes Example");
    
    let mut surface = CpuSurface::new(800, 600);
    surface.canvas().clear(Color::rgb(245, 245, 250));
    
    // Example 1: Simple grid pattern
    println!("Drawing grid pattern...");
    let paint = Paint::with_color(Color::rgba(200, 200, 200, 200));
    for x in (0..800).step_by(100) {
        surface.canvas().draw_line(
            Point::new(x as f32, 0.0),
            Point::new(x as f32, 600.0),
            &paint,
        );
    }
    for y in (0..600).step_by(100) {
        surface.canvas().draw_line(
            Point::new(0.0, y as f32),
            Point::new(800.0, y as f32),
            &paint,
        );
    }
    
    // Example 2: Concentric circles
    println!("Drawing concentric circles...");
    let center = Point::new(200.0, 200.0);
    for i in 1..=5 {
        let radius = i as f32 * 20.0;
        let paint = Paint::with_color(Color::rgb(100, 150, 255));
        surface.canvas().draw_circle(center, radius, &paint);
    }
    
    // Example 3: Overlapping rectangles
    println!("Drawing overlapping rectangles...");
    let colors = [
        Color::rgb(255, 100, 100),
        Color::rgb(100, 255, 100),
        Color::rgb(100, 100, 255),
    ];
    for (i, color) in colors.iter().enumerate() {
        let offset = i as f32 * 30.0;
        let paint = Paint::with_color(*color);
        surface.canvas().draw_rect(
            Rect::new(450.0 + offset, 100.0 + offset, 120.0, 120.0),
            &paint,
        );
    }
    
    // Example 4: Checkerboard pattern
    println!("Drawing checkerboard...");
    for row in 0..4 {
        for col in 0..4 {
            let color = if (row + col) % 2 == 0 {
                Color::rgb(80, 80, 80)
            } else {
                Color::rgb(220, 220, 220)
            };
            let paint = Paint::with_color(color);
            surface.canvas().draw_rect(
                Rect::new(
                    50.0 + (col as f32 * 40.0),
                    400.0 + (row as f32 * 40.0),
                    40.0,
                    40.0
                ),
                &paint,
            );
        }
    }
    
    let path = "examples/output/shapes/complex_shapes.png";
    std::fs::create_dir_all("examples/output/shapes")?;
    surface.save_png(path)?;
    println!("✅ Saved to {}", path);
    
    Ok(())
}
