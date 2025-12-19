//! Pattern generation examples

use sina::{Color, CpuSurface, Paint, Path, Point, Rect, Surface};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔲 Pattern Generation Example\n");

    let mut surface = CpuSurface::new(1200, 1000);
    surface.canvas().clear(Color::rgb(240, 240, 240));

    // Checkerboard pattern
    println!("Creating checkerboard...");
    for row in 0..10 {
        for col in 0..15 {
            if (row + col) % 2 == 0 {
                let paint = Paint::with_color(Color::rgb(60, 60, 80));
                surface.canvas().draw_rect(
                    Rect::new(col as f32 * 80.0, row as f32 * 50.0, 80.0, 50.0),
                    &paint,
                );
            }
        }
    }

    // Dot pattern
    println!("Creating dot pattern...");
    for row in 0..8 {
        for col in 0..15 {
            let paint = Paint::with_color(Color::rgb(200, 50, 100));
            surface.canvas().draw_circle(
                Point::new(col as f32 * 80.0 + 40.0, row as f32 * 50.0 + 525.0),
                8.0,
                &paint,
            );
        }
    }

    // Diagonal lines pattern
    println!("Creating diagonal pattern...");
    let line_paint = Paint::with_color(Color::rgba(100, 150, 255, 180));
    for i in 0..30 {
        let x = i as f32 * 40.0 - 200.0;
        surface.canvas().draw_line(
            Point::new(x, 500.0),
            Point::new(x + 400.0, 100.0),
            &line_paint,
        );
    }

    // Honeycomb hex pattern (simplified with circles)
    println!("Creating honeycomb...");
    for row in 0..5 {
        for col in 0..10 {
            let x = 100.0 + col as f32 * 50.0 + (row % 2) as f32 * 25.0;
            let y = 50.0 + row as f32 * 45.0;

            let paint = Paint::with_color(Color::rgb(255, 200, 50));
            surface.canvas().draw_circle(Point::new(x, y), 20.0, &paint);

            // Draw outline
            let outline = Paint::with_color(Color::rgb(200, 150, 30));
            surface
                .canvas()
                .draw_circle(Point::new(x, y), 21.0, &outline);
        }
    }

    let path = "examples/output/shapes/patterns.png";
    std::fs::create_dir_all("examples/output/shapes")?;
    surface.save_png(path)?;
    println!("\n✅ Saved to {}", path);

    Ok(())
}
