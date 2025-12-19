//! Complex shapes and patterns example

use sina::{Color, Paint, Point, Rect, Surface, CpuSurface};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Sina Complex Shapes Example");
    
    let mut surface = CpuSurface::new(800, 600);
    surface.canvas().clear(Color::rgb(245, 245, 250));
    
    // Example 1: Grid pattern
    println!("Drawing grid pattern...");
    let mut paint = Paint::with_color(Color::rgba(200, 200, 200, 128));
    for x in (0..800).step_by(50) {
        surface.canvas().draw_line(
            Point::new(x as f32, 0.0),
            Point::new(x as f32, 600.0),
            &paint,
        );
    }
    for y in (0..600).step_by(50) {
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
        let alpha = 255 - (i * 40) as u8;
        paint.set_color(Color::rgba(100, 150, 255, alpha));
        surface.canvas().draw_circle(center, radius, &paint);
    }
    
    // Example 3: Overlapping rectangles with transparency
    println!("Drawing overlapping rectangles...");
    let colors = [
        Color::rgba(255, 100, 100, 150),
        Color::rgba(100, 255, 100, 150),
        Color::rgba(100, 100, 255, 150),
    ];
    for (i, color) in colors.iter().enumerate() {
        let offset = i as f32 * 30.0;
        paint.set_color(*color);
        surface.canvas().draw_rect(
            Rect::new(450.0 + offset, 100.0 + offset, 120.0, 120.0),
            &paint,
        );
    }
    
    // Example 4: Spiral pattern
    println!("Drawing spiral...");
    let spiral_center = Point::new(600.0, 400.0);
    let mut angle = 0.0_f32;
    let mut last_point = spiral_center;
    
    paint.set_color(Color::rgb(150, 50, 200));
    paint.set_stroke(sina::StrokeStyle { width: 2.0, ..Default::default() });
    
    for i in 0..100 {
        angle += 0.3;
        let radius = (i as f32) * 1.5;
        let x = spiral_center.x + angle.cos() * radius;
        let y = spiral_center.y + angle.sin() * radius;
        let point = Point::new(x, y);
        
        surface.canvas().draw_line(last_point, point, &paint);
        last_point = point;
    }
    
    // Example 5: Gradient-like effect with multiple circles
    println!("Drawing gradient effect...");
    for i in 0..20 {
        let x = 200.0 + (i as f32) * 15.0;
        let gray = 50 + (i * 10) as u8;
        paint.set_color(Color::rgb(gray, gray, gray + 50));
        surface.canvas().draw_circle(
            Point::new(x, 450.0),
            8.0,
            &paint,
        );
    }
    
    // Example 6: Checkerboard pattern
    println!("Drawing checkerboard...");
    for row in 0..4 {
        for col in 0..4 {
            if (row + col) % 2 == 0 {
                paint.set_color(Color::rgb(80, 80, 80));
            } else {
                paint.set_color(Color::rgb(220, 220, 220));
            }
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
    
    surface.save_png("complex_shapes.png")?;
    println!("✅ Saved to complex_shapes.png");
    
    Ok(())
}
