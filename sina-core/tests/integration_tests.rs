//! Integration tests for Sina rendering engine

use sina::*;

#[test]
fn test_surface_creation() {
    let surface = CpuSurface::new(800, 600);
    assert_eq!(surface.width(), 800);
    assert_eq!(surface.height(), 600);
}

#[test]
fn test_drawing_shapes() {
    let mut surface = CpuSurface::new(100, 100);
    let paint = Paint::with_color(Color::RED);

    // Test rectangle
    surface
        .canvas()
        .draw_rect(Rect::new(10.0, 10.0, 50.0, 50.0), &paint);

    // Test circle
    surface
        .canvas()
        .draw_circle(Point::new(50.0, 50.0), 20.0, &paint);

    // Test line
    surface
        .canvas()
        .draw_line(Point::new(0.0, 0.0), Point::new(100.0, 100.0), &paint);

    // Should not panic
}

#[test]
fn test_path_building() {
    let path = Path::builder()
        .move_to(Point::new(0.0, 0.0))
        .line_to(Point::new(100.0, 0.0))
        .line_to(Point::new(50.0, 100.0))
        .close()
        .build();

    // Path should be created successfully
    let mut surface = CpuSurface::new(200, 200);
    let paint = Paint::with_color(Color::BLUE);
    surface.canvas().draw_path(&path, &paint);
}

#[test]
fn test_bezier_curves() {
    // Paths with curves need to be closed or they should just draw without close
    let path = Path::builder()
        .move_to(Point::new(0.0, 50.0))
        .quad_to(Point::new(50.0, 0.0), Point::new(100.0, 50.0))
        .line_to(Point::new(100.0, 50.0)) // End with line_to before close
        .close()
        .build();

    let mut surface = CpuSurface::new(200, 200);
    let paint = Paint::with_color(Color::GREEN);
    surface.canvas().draw_path(&path, &paint);
}

#[test]
fn test_color_operations() {
    let red = Color::RED;
    assert_eq!(red.r, 255);
    assert_eq!(red.g, 0);
    assert_eq!(red.b, 0);
    assert_eq!(red.a, 255);

    let transparent = Color::rgba(100, 100, 100, 128);
    assert_eq!(transparent.a, 128);

    let premul = Color::rgba(255, 255, 255, 128).premultiply();
    assert!(premul.r <= 128);
}

#[test]
fn test_rect_operations() {
    let rect1 = Rect::new(0.0, 0.0, 100.0, 100.0);
    let rect2 = Rect::new(50.0, 50.0, 100.0, 100.0);

    assert!(rect1.contains(Point::new(50.0, 50.0)));
    assert!(!rect1.contains(Point::new(150.0, 150.0)));

    // Test intersection method
    assert!(rect1.intersects(&rect2));
}

#[test]
fn test_paint_configuration() {
    let mut paint = Paint::with_color(Color::BLUE);
    paint.set_stroke(StrokeStyle {
        width: 5.0,
        ..Default::default()
    });
    assert!(paint.stroke.is_some());
    assert_eq!(paint.stroke.as_ref().unwrap().width, 5.0);

    paint.set_color(Color::RED);
    assert_eq!(paint.color.r, 255);
}

#[test]
fn test_png_export() -> Result<(), Box<dyn std::error::Error>> {
    let mut surface = CpuSurface::new(100, 100);
    surface.canvas().clear(Color::WHITE);

    let temp_file = std::env::temp_dir().join("sina_test.png");
    surface.save_png(temp_file.to_str().unwrap())?;

    // Verify file exists
    assert!(temp_file.exists());

    // Clean up
    std::fs::remove_file(&temp_file)?;

    Ok(())
}

#[test]
fn test_large_surface() {
    let surface = CpuSurface::new(4096, 4096);
    assert_eq!(surface.width(), 4096);
    assert_eq!(surface.height(), 4096);
}

#[test]
fn test_font_collection_size() {
    // Test with invalid data
    let data = vec![0u8; 100];
    let result = Font::collection_size(&data);
    assert!(result.is_none());
}

#[test]
fn test_variable_font_manager() {
    // Test that the API exists and compiles
    let axis = VariationAxis {
        tag: "wght".to_string(),
        name: "Weight".to_string(),
        min_value: 100.0,
        default_value: 400.0,
        max_value: 900.0,
    };

    assert_eq!(axis.tag, "wght");
    assert_eq!(axis.default_value, 400.0);
}
