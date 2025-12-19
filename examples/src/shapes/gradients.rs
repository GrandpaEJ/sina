//! Gradient effects example (simulated with multiple colors)

use sina::{Color, Paint, Point, Rect, Surface, CpuSurface};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Gradient Effects Example\n");
    
    let mut surface = CpuSurface::new(1000, 800);
    surface.canvas().clear(Color::WHITE);
    
    // Linear gradient effect (horizontal)
    println!("Creating horizontal gradient...");
    for i in 0..100 {
        let progress = i as f32 / 100.0;
        let r = (255.0 * (1.0 - progress)) as u8;
        let b = (255.0 * progress) as u8;
        let paint = Paint::with_color(Color::rgb(r, 0, b));
        
        surface.canvas().draw_rect(
            Rect::new(50.0 + i as f32 * 9.0, 50.0, 10.0, 150.0),
            &paint,
        );
    }
    
    // Vertical gradient
    println!("Creating vertical gradient...");
    for i in 0..100 {
        let progress = i as f32 / 100.0;
        let g = (255.0 * progress) as u8;
        let paint = Paint::with_color(Color::rgb(50, g, 50));
        
        surface.canvas().draw_rect(
            Rect::new(50.0, 250.0 + i as f32 * 2.5, 900.0, 3.0),
            &paint,
        );
    }
    
    // Radial gradient effect (circular)
    println!("Creating radial gradient...");
    let center = Point::new(500.0, 600.0);
    for i in (0..80).rev() {
        let radius = i as f32 * 2.0;
        let intensity = (255.0 * (i as f32 / 80.0)) as u8;
        let paint = Paint::with_color(Color::rgb(255, intensity, 0));
        
        surface.canvas().draw_circle(
            center,
            radius,
            &paint,
        );
    }
    
    // Multi-color gradient
    println!("Creating multi-color gradient...");
    let colors = vec![
        Color::rgb(255, 0, 0),    // Red
        Color::rgb(255, 127, 0),  // Orange
        Color::rgb(255, 255, 0),  // Yellow
        Color::rgb(0, 255, 0),    // Green
        Color::rgb(0, 0, 255),    // Blue
        Color::rgb(75, 0, 130),   // Indigo
        Color::rgb(148, 0, 211),  // Violet
    ];
    
    for (i, color) in colors.iter().enumerate() {
        let paint = Paint::with_color(*color);
        surface.canvas().draw_rect(
            Rect::new(50.0 + i as f32 * 130.0, 500.0, 130.0, 60.0),
            &paint,
        );
    }
    
    surface.save_png("examples/output/shapes/gradients.png")?;
    println!("\n✅ Saved to examples/output/shapes/gradients.png");
    
    Ok(())
}
