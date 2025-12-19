//! CPU-based surface implementation using raqote

use crate::{Color, Paint, Path, Point, Rect, Surface, Canvas};
use raqote::{DrawTarget, DrawOptions, Source, SolidSource, PathBuilder as RaqotePathBuilder};

pub struct CpuSurface {
    draw_target: DrawTarget,
}

impl CpuSurface {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            draw_target: DrawTarget::new(width, height),
        }
    }
}

impl Surface for CpuSurface {
    fn width(&self) -> u32 {
        self.draw_target.width() as u32
    }

    fn height(&self) -> u32 {
        self.draw_target.height() as u32
    }

    fn canvas(&mut self) -> &mut dyn Canvas {
        self
    }

    fn save_png(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.draw_target.write_png(path)?;
        Ok(())
    }
}

impl Canvas for CpuSurface {
    fn clear(&mut self, color: Color) {
        let source = SolidSource {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        };
        self.draw_target.clear(source);
    }

    fn save(&mut self) {
        // TODO: Implement state stack
    }

    fn restore(&mut self) {
        // TODO: Implement state stack
    }

    fn draw_path(&mut self, path: &Path, paint: &Paint) {
        // Convert lyon path to raqote path
        let mut raqote_path = RaqotePathBuilder::new();
        
        for event in path.lyon_path().iter() {
            match event {
                lyon_path::Event::Begin { at } => {
                    raqote_path.move_to(at.x, at.y);
                }
                lyon_path::Event::Line { to, .. } => {
                    raqote_path.line_to(to.x, to.y);
                }
                lyon_path::Event::Quadratic { ctrl, to, .. } => {
                    raqote_path.quad_to(ctrl.x, ctrl.y, to.x, to.y);
                }
                lyon_path::Event::Cubic { ctrl1, ctrl2, to, .. } => {
                    raqote_path.cubic_to(ctrl1.x, ctrl1.y, ctrl2.x, ctrl2.y, to.x, to.y);
                }
                lyon_path::Event::End { close, .. } => {
                    if close {
                        raqote_path.close();
                    }
                }
            }
        }

        let raqote_path = raqote_path.finish();
        let source = Source::Solid(SolidSource {
            r: paint.color.r,
            g: paint.color.g,
            b: paint.color.b,
            a: paint.color.a,
        });

        let draw_options = DrawOptions::default();

        if paint.is_fill() {
            self.draw_target.fill(&raqote_path, &source, &draw_options);
        } else if let Some(stroke) = &paint.stroke {
            let stroke_style = raqote::StrokeStyle {
                width: stroke.width,
                ..Default::default()
            };
            self.draw_target.stroke(&raqote_path, &source, &stroke_style, &draw_options);
        }
    }

    fn draw_rect(&mut self, rect: Rect, paint: &Paint) {
        let mut path = RaqotePathBuilder::new();
        path.rect(rect.x, rect.y, rect.width, rect.height);
        let raqote_path = path.finish();

        let source = Source::Solid(SolidSource {
            r: paint.color.r,
            g: paint.color.g,
            b: paint.color.b,
            a: paint.color.a,
        });

        let draw_options = DrawOptions::default();

        if paint.is_fill() {
            self.draw_target.fill(&raqote_path, &source, &draw_options);
        } else if let Some(stroke) = &paint.stroke {
            let stroke_style = raqote::StrokeStyle {
                width: stroke.width,
                ..Default::default()
            };
            self.draw_target.stroke(&raqote_path, &source, &stroke_style, &draw_options);
        }
    }

    fn draw_circle(&mut self, center: Point, radius: f32, paint: &Paint) {
        let mut path = RaqotePathBuilder::new();
        path.arc(center.x, center.y, radius, 0.0, 2.0 * std::f32::consts::PI);
        let raqote_path = path.finish();

        let source = Source::Solid(SolidSource {
            r: paint.color.r,
            g: paint.color.g,
            b: paint.color.b,
            a: paint.color.a,
        });

        let draw_options = DrawOptions::default();

        if paint.is_fill() {
            self.draw_target.fill(&raqote_path, &source, &draw_options);
        } else if let Some(stroke) = &paint.stroke {
            let stroke_style = raqote::StrokeStyle {
                width: stroke.width,
                ..Default::default()
            };
            self.draw_target.stroke(&raqote_path, &source, &stroke_style, &draw_options);
        }
    }

    fn draw_line(&mut self, from: Point, to: Point, paint: &Paint) {
        let mut path = RaqotePathBuilder::new();
        path.move_to(from.x, from.y);
        path.line_to(to.x, to.y);
        let raqote_path = path.finish();

        let source = Source::Solid(SolidSource {
            r: paint.color.r,
            g: paint.color.g,
            b: paint.color.b,
            a: paint.color.a,
        });

        let stroke_style = raqote::StrokeStyle {
            width: paint.stroke.as_ref().map(|s| s.width).unwrap_or(1.0),
            ..Default::default()
        };

        let draw_options = DrawOptions::default();
        self.draw_target.stroke(&raqote_path, &source, &stroke_style, &draw_options);
    }

    fn draw_text(&mut self, text: &str, position: Point, font: &crate::text::Font, font_size: f32, paint: &Paint) {
        use crate::text::{TextLayout, TextAlign, GlyphCache};
        
        // Create layout engine
        let layout = TextLayout::new(font.clone(), font_size);
        
        // Layout the text
        let positioned_glyphs = layout.layout(text, position, TextAlign::Left);
        
        // Create glyph cache
        let mut glyph_cache = GlyphCache::default();
        
        // Render each glyph
        for (glyph_pos, shaped_glyph) in positioned_glyphs {
            // Rasterize glyph
            if let Some(rasterized) = glyph_cache.get_or_rasterize(
                font,
                shaped_glyph.character,
                font_size
            ) {
                if rasterized.width == 0 || rasterized.height == 0 {
                    continue;
                }
                
                // Create a path for the glyph by drawing it as a filled rectangle
                // This is a simplified approach - for better quality, we could rasterize the actual glyph outlines
                
                // Convert to RGBA and composite manually
                let rgba_pixels = rasterized.to_rgba(paint.color);
                
                // Create an image from the RGBA pixels
                let mut image_data = vec![0u32; rasterized.width * rasterized.height];
                for y in 0..rasterized.height {
                    for x in 0..rasterized.width {
                        let idx = (y * rasterized.width + x) * 4;
                        let r = rgba_pixels[idx];
                        let g = rgba_pixels[idx + 1];
                        let b = rgba_pixels[idx + 2];
                        let a = rgba_pixels[idx + 3];
                        
                        // Pack into ARGB format (raqote uses ARGB)
                        image_data[y * rasterized.width + x] = 
                            ((a as u32) << 24) | ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
                    }
                }
                
                // Create image
                let image = raqote::Image {
                    width: rasterized.width as i32,
                    height: rasterized.height as i32,
                    data: &image_data,
                };
                
                // Calculate position with bearings
                let x = (glyph_pos.x + rasterized.bearing_x).round();
                let y = (glyph_pos.y - rasterized.bearing_y - rasterized.height as f32).round();
                
                // Draw the glyph image onto main surface
                self.draw_target.draw_image_at(
                    x,
                    y,
                    &image,
                    &DrawOptions::default()
                );
            }
        }
    }
}
