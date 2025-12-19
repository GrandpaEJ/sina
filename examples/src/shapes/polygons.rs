//! Polygon and star shapes example

use sina::{Color, CpuSurface, Paint, Path, Point, Surface};
use std::f32::consts::PI;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("⭐ Polygons and Stars Example\n");

    let mut surface = CpuSurface::new(1200, 900);
    surface.canvas().clear(Color::rgb(245, 245, 250));

    // Pentagon
    println!("Drawing pentagon...");
    draw_regular_polygon(
        &mut surface,
        Point::new(200.0, 200.0),
        80.0,
        5,
        Color::rgb(255, 100, 100),
    );

    // Hexagon
    println!("Drawing hexagon...");
    draw_regular_polygon(
        &mut surface,
        Point::new(450.0, 200.0),
        80.0,
        6,
        Color::rgb(100, 255, 100),
    );

    // Octagon
    println!("Drawing octagon...");
    draw_regular_polygon(
        &mut surface,
        Point::new(700.0, 200.0),
        80.0,
        8,
        Color::rgb(100, 100, 255),
    );

    // Decagon
    println!("Drawing decagon...");
    draw_regular_polygon(
        &mut surface,
        Point::new(950.0, 200.0),
        80.0,
        10,
        Color::rgb(255, 255, 100),
    );

    // 5-pointed star
    println!("Drawing 5-pointed star...");
    draw_star(
        &mut surface,
        Point::new(200.0, 500.0),
        80.0,
        40.0,
        5,
        Color::rgb(255, 215, 0),
    );

    // 6-pointed star
    println!("Drawing 6-pointed star...");
    draw_star(
        &mut surface,
        Point::new(450.0, 500.0),
        80.0,
        40.0,
        6,
        Color::rgb(200, 150, 255),
    );

    // 8-pointed star
    println!("Drawing 8-pointed star...");
    draw_star(
        &mut surface,
        Point::new(700.0, 500.0),
        80.0,
        40.0,
        8,
        Color::rgb(100, 200, 255),
    );

    // Triangle (3-sided polygon)
    println!("Drawing triangle...");
    draw_regular_polygon(
        &mut surface,
        Point::new(200.0, 750.0),
        80.0,
        3,
        Color::rgb(255, 150, 150),
    );

    // Diamond (4-sided polygon rotated)
    println!("Drawing diamond...");
    let center = Point::new(450.0, 750.0);
    let diamond = Path::builder()
        .move_to(Point::new(center.x, center.y - 90.0))
        .line_to(Point::new(center.x + 60.0, center.y))
        .line_to(Point::new(center.x, center.y + 90.0))
        .line_to(Point::new(center.x - 60.0, center.y))
        .close();

    let paint = Paint::with_color(Color::rgb(255, 100, 255));
    surface.canvas().draw_path(&diamond.build(), &paint);

    let path = "examples/output/shapes/polygons.png";
    std::fs::create_dir_all("examples/output/shapes")?;
    surface.save_png(path)?;
    println!("\n✅ Saved to {}", path);

    Ok(())
}

fn draw_regular_polygon(
    surface: &mut CpuSurface,
    center: Point,
    radius: f32,
    sides: u32,
    color: Color,
) {
    let mut path = Path::builder();

    for i in 0..sides {
        let angle = (i as f32 * 2.0 * PI / sides as f32) - PI / 2.0;
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();

        if i == 0 {
            path = path.move_to(Point::new(x, y));
        } else {
            path = path.line_to(Point::new(x, y));
        }
    }
    path = path.close();

    let paint = Paint::with_color(color);
    surface.canvas().draw_path(&path.build(), &paint);
}

fn draw_star(
    surface: &mut CpuSurface,
    center: Point,
    outer_radius: f32,
    inner_radius: f32,
    points: u32,
    color: Color,
) {
    let mut path = Path::builder();

    for i in 0..(points * 2) {
        let angle = (i as f32 * PI / points as f32) - PI / 2.0;
        let radius = if i % 2 == 0 {
            outer_radius
        } else {
            inner_radius
        };
        let x = center.x + radius * angle.cos();
        let y = center.y + radius * angle.sin();

        if i == 0 {
            path = path.move_to(Point::new(x, y));
        } else {
            path = path.line_to(Point::new(x, y));
        }
    }
    path = path.close();

    let paint = Paint::with_color(color);
    surface.canvas().draw_path(&path.build(), &paint);
}
