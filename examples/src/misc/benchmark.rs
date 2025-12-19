//! Performance benchmark example

use sina::{Color, Paint, Point, Rect, Surface, CpuSurface};
use std::time::Instant;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚡ Sina Performance Benchmark\n");
    
    let mut surface = CpuSurface::new(1920, 1080);
    
    // Benchmark 1: Simple shapes
    {
        let start = Instant::now();
        let mut paint = Paint::with_color(Color::RED);
        
        for i in 0..1000 {
            let x = (i % 40) as f32 * 48.0;
            let y = (i / 40) as f32 * 43.0;
            surface.canvas().draw_rect(
                Rect::new(x, y, 40.0, 40.0),
                &paint,
            );
        }
        
        let duration = start.elapsed();
        println!("✓ 1000 rectangles: {:?}", duration);
    }
    
    // Benchmark 2: Circles
    {
        surface.canvas().clear(Color::WHITE);
        let start = Instant::now();
        let mut paint = Paint::with_color(Color::BLUE);
        
        for i in 0..500 {
            let x = (i as f32 * 3.0) % 1920.0;
            let y = ((i / 10) as f32 * 20.0) % 1080.0;
            surface.canvas().draw_circle(
                Point::new(x, y),
                10.0,
                &paint,
            );
        }
        
        let duration = start.elapsed();
        println!("✓ 500 circles: {:?}", duration);
    }
    
    // Benchmark 3: Lines
    {
        surface.canvas().clear(Color::WHITE);
        let start = Instant::now();
        let mut paint = Paint::with_color(Color::BLACK);
        paint.set_stroke(sina::StrokeStyle { width: 2.0, ..Default::default() });
        
        for i in 0..2000 {
            let x1 = (i as f32 * 0.96) % 1920.0;
            let y1 = ((i as f32 * 0.54) % 1080.0).abs();
            let x2 = ((i as f32 * 1.2) % 1920.0).abs();
            let y2 = (i as f32 * 0.54) % 1080.0;
            
            surface.canvas().draw_line(
                Point::new(x1, y1),
                Point::new(x2, y2),
                &paint,
            );
        }
        
        let duration = start.elapsed();
        println!("✓ 2000 lines: {:?}", duration);
    }
    
    // Benchmark 4: Clear operation
    {
        let start = Instant::now();
        for _ in 0..100 {
            surface.canvas().clear(Color::WHITE);
        }
        let duration = start.elapsed();
        println!("✓ 100 clear operations: {:?}", duration);
    }
    
    // Benchmark 5: PNG export
    {
        let start = Instant::now();
        let path = "examples/output/misc/benchmark/benchmark_output.png";
        std::fs::create_dir_all("examples/output/misc/benchmark")?;
        surface.save_png(path)?;
        let duration = start.elapsed();
        println!("✓ PNG export (1920x1080): {:?}", duration);
    }
    
    println!("\n✅ Benchmark complete!");
    
    Ok(())
}
