//! Vector paths example demonstrating Bezier curves and complex shapes

use sina::{Color, Paint, Point, Surface, CpuSurface, Path};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Sina Vector Paths Example");
    
    let mut surface = CpuSurface::new(800, 600);
    surface.canvas().clear(Color::WHITE);
    
    // Example 1: Drawing a triangle with a path
    println!("Drawing triangle...");
    let triangle = Path::builder()
        .move_to(Point::new(100.0, 100.0))
        .line_to(Point::new(200.0, 100.0))
        .line_to(Point::new(150.0, 50.0))
        .close()
        .build();
    
    let mut paint = Paint::with_color(Color::rgb(255, 100, 100));
    surface.canvas().draw_path(&triangle, &paint);
    
    // Example 2: Quadratic Bezier curve (smooth arc)
    println!("Drawing quadratic curve...");
    let quad_curve = Path::builder()
        .move_to(Point::new(250.0, 100.0))
        .quad_to(Point::new(350.0, 50.0), Point::new(450.0, 100.0))
        .line_to(Point::new(450.0, 100.0))
        .close()
        .build();
    
    paint.set_color(Color::rgb(100, 100, 255));
    paint.set_stroke(sina::StrokeStyle { width: 3.0, ..Default::default() });
    surface.canvas().draw_path(&quad_curve, &paint);
    
    // Example 3: Cubic Bezier curve (S-shape)
    println!("Drawing cubic curve...");
    let cubic_curve = Path::builder()
        .move_to(Point::new(100.0, 200.0))
        .cubic_to(
            Point::new(150.0, 150.0),  // Control point 1
            Point::new(250.0, 250.0),  // Control point 2
            Point::new(300.0, 200.0)   // End point
        )
        .line_to(Point::new(300.0, 200.0))
        .close()
        .build();
    
    paint.set_color(Color::rgb(100, 200, 100));
    surface.canvas().draw_path(&cubic_curve, &paint);
    
    // Example 4: Complex path - star shape
    println!("Drawing star...");
    let star = Path::builder()
        .move_to(Point::new(550.0, 250.0))
        .line_to(Point::new(570.0, 300.0))
        .line_to(Point::new(625.0, 310.0))
        .line_to(Point::new(585.0, 345.0))
        .line_to(Point::new(600.0, 400.0))
        .line_to(Point::new(550.0, 370.0))
        .line_to(Point::new(500.0, 400.0))
        .line_to(Point::new(515.0, 345.0))
        .line_to(Point::new(475.0, 310.0))
        .line_to(Point::new(530.0, 300.0))
        .close()
        .build();
    
    paint.set_color(Color::rgb(255, 215, 0)); // Gold
    surface.canvas().draw_path(&star, &paint);
    
    // Example 5: Multi-path shape (heart)
    println!("Drawing heart...");
    let heart = Path::builder()
        .move_to(Point::new(400.0, 450.0))
        .cubic_to(
            Point::new(400.0, 425.0),
            Point::new(380.0, 410.0),
            Point::new(360.0, 410.0)
        )
        .cubic_to(
            Point::new(330.0, 410.0),
            Point::new(310.0, 430.0),
            Point::new(310.0, 460.0)
        )
        .cubic_to(
            Point::new(310.0, 480.0),
            Point::new(320.0, 495.0),
            Point::new(400.0, 550.0)
        )
        .cubic_to(
            Point::new(480.0, 495.0),
            Point::new(490.0, 480.0),
            Point::new(490.0, 460.0)
        )
        .cubic_to(
            Point::new(490.0, 430.0),
            Point::new(470.0, 410.0),
            Point::new(440.0, 410.0)
        )
        .cubic_to(
            Point::new(420.0, 410.0),
            Point::new(400.0, 425.0),
            Point::new(400.0, 450.0)
        )
        .close()
        .build();
    
    paint.set_color(Color::rgb(255, 50, 100)); // Pink-red
    surface.canvas().draw_path(&heart, &paint);
    
    // Save output
    let path = "examples/output/shapes/vector_paths.png";
    std::fs::create_dir_all("examples/output/shapes")?;
    surface.save_png(path)?;
    println!("✅ Saved to {}", path);
    
    Ok(())
}
