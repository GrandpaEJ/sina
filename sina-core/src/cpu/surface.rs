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
        if cfg!(debug_assertions) && text.starts_with("Hello") {
             // println!("DEBUG: Layout '{}' -> {} glyphs", text, positioned_glyphs.len());
        }
        
        // Create glyph cache
        let mut glyph_cache = GlyphCache::default();
        
        // Get the paint color and target dimensions
        let color = paint.color;
        let target_width = self.draw_target.width();
        let target_height = self.draw_target.height();
        
        // Render each glyph
        for (glyph_pos, shaped_glyph) in positioned_glyphs {
            // Rasterize glyph using its index (important for ligatures/complex scripts)
            let rasterized_opt = glyph_cache.get_or_rasterize_indexed(
                font,
                shaped_glyph.glyph_index,
                font_size
            );

            if rasterized_opt.is_none() {
                if cfg!(debug_assertions) {
                    println!("DEBUG: Failed to rasterize glyph {}", shaped_glyph.glyph_index);
                }
                continue;
            }
            let rasterized = rasterized_opt.unwrap();

            if rasterized.width == 0 || rasterized.height == 0 {
                 continue;
            }
            
            // Calculate position with bearings
            let x = (glyph_pos.x + rasterized.bearing_x).round() as i32;
            let y = (glyph_pos.y - rasterized.bearing_y - rasterized.height as f32).round() as i32;

            if cfg!(debug_assertions) && text.starts_with("Hello") && shaped_glyph.glyph_index < 1000 {
                // println!("DEBUG: DrawGlyph {} at ({},{}) size={}x{} fmt={:?}", shaped_glyph.glyph_index, x, y, rasterized.width, rasterized.height, rasterized.format);
            }
                
                // Skip if completely off screen
                if x + (rasterized.width as i32) < 0 || y + (rasterized.height as i32) < 0 {
                    continue;
                }
                if x >= target_width || y >= target_height {
                    continue;
                }
                
                // For RGBA glyphs (Emojis), use raqote's native image drawing for correct blending
                if rasterized.format == crate::text::GlyphFormat::Rgba {
                     let mut argb_pixels = Vec::with_capacity(rasterized.width * rasterized.height);
                     for chunk in rasterized.pixels.chunks(4) {
                         let r = chunk[0];
                         let g = chunk[1];
                         let b = chunk[2];
                         let a = chunk[3];
                         
                         // Modulate alpha by paint opacity
                         let final_a = ((a as u16 * color.a as u16) / 255) as u8;
                         
                         // Premultiply alpha (standard for ARGB32)
                         let r = ((r as u16 * final_a as u16) / 255) as u8;
                         let g = ((g as u16 * final_a as u16) / 255) as u8;
                         let b = ((b as u16 * final_a as u16) / 255) as u8;
                         
                         // Pack into 0xAARRGGBB u32
                         let pixel = ((final_a as u32) << 24) | 
                                     ((r as u32) << 16) | 
                                     ((g as u32) << 8) | 
                                     (b as u32);
                         argb_pixels.push(pixel);
                     }
                     
                     let image = raqote::Image {
                         width: rasterized.width as i32,
                         height: rasterized.height as i32,
                         data: &argb_pixels,
                     };
                     
                     self.draw_target.draw_image_at(x as f32, y as f32, &image, &DrawOptions::default());
                     continue;
                } else if rasterized.format == crate::text::GlyphFormat::Alpha {
                    // Manual optimized path for standard text to ensure reliability
                    // This avoids Raqote image creation overhead and potential stride bugs for simple masks
                    
                    let target_data = self.draw_target.get_data_mut();
                    let target_width_usize = target_width as usize;
                    
                    let src_r = color.r;
                    let src_g = color.g;
                    let src_b = color.b;
                    
                    for gy in 0..rasterized.height {
                        let target_y = y + (gy as i32);
                        if target_y < 0 || target_y >= target_height {
                            continue;
                        }
                        
                        // Per-row bounds check for X is in inner loop
                        let row_start_idx = (target_y as usize) * target_width_usize;
                        
                        for gx in 0..rasterized.width {
                            let target_x = x + (gx as i32);
                            if target_x < 0 || target_x >= target_width {
                                continue;
                            }
                            
                            let glyph_alpha = rasterized.pixels[gy * rasterized.width + gx];
                            if glyph_alpha == 0 {
                                continue;
                            }
                            
                            // Blend paint alpha with glyph alpha
                            let src_a = ((glyph_alpha as u16 * color.a as u16) / 255) as u8;
                            if src_a == 0 {
                                continue;
                            }
                             
                             // Calculate target pixel index
                             let target_idx = row_start_idx + (target_x as usize);
                             
                             // DEBUG: Log first pixel of first glyph
                             // if gy == rasterized.height / 2 && gx == rasterized.width / 2 && x < 100 {
                             //    println!("DEBUG: AlphaPixel at ({},{}) src_a={} color={:?}", target_x, target_y, src_a, color);
                             // }

                             // Get existing pixel (ARGB format in raqote)
                             let existing = target_data[target_idx];
                             let dst_a = ((existing >> 24) & 0xFF) as u8;
                             let dst_r = ((existing >> 16) & 0xFF) as u8;
                             let dst_g = ((existing >> 8) & 0xFF) as u8;
                             let dst_b = (existing & 0xFF) as u8;
                             
                             // Blend using standard alpha compositing
                             // dst' = src * alpha + dst * (1 - alpha)
                             let inv_alpha = 255 - src_a;
                             
                             let out_r = ((src_r as u16 * src_a as u16 + dst_r as u16 * inv_alpha as u16) / 255) as u8;
                             let out_g = ((src_g as u16 * src_a as u16 + dst_g as u16 * inv_alpha as u16) / 255) as u8;
                             let out_b = ((src_b as u16 * src_a as u16 + dst_b as u16 * inv_alpha as u16) / 255) as u8;
                             let out_a = dst_a.saturating_add(src_a) - ((dst_a as u16 * src_a as u16) / 255) as u8;
                             
                             // Write back in ARGB format
                             target_data[target_idx] = 
                                 ((out_a as u32) << 24) | 
                                 ((out_r as u32) << 16) | 
                                 ((out_g as u32) << 8) | 
                                 (out_b as u32);
                        }
                    }
                    continue;
                }
        }
    }
}
