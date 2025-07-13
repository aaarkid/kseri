use image::{RgbImage, Rgb};
use super::generator::AssetGenerator;
use super::palette::GreekPalette;

pub struct PatternGenerator;

impl PatternGenerator {
    pub fn generate_meander_pattern(width: u32, height: u32) -> Vec<Vec<bool>> {
        let mut pattern = vec![vec![false; width as usize]; height as usize];
        
        // Classic Greek key/meander pattern unit (6x6)
        let meander_unit = [
            [1,1,1,1,0,0],
            [1,0,0,1,0,0],
            [1,0,1,1,1,1],
            [1,0,1,0,0,1],
            [1,1,1,0,0,1],
            [0,0,0,0,0,1],
        ];
        
        // Tile the pattern
        for y in 0..height {
            for x in 0..width {
                let ux = (x % 6) as usize;
                let uy = (y % 6) as usize;
                pattern[y as usize][x as usize] = meander_unit[uy][ux] == 1;
            }
        }
        
        pattern
    }
    
    pub fn draw_meander_border(buffer: &mut RgbImage, thickness: u32, color: Rgb<u8>) {
        let width = buffer.width();
        let height = buffer.height();
        let pattern = Self::generate_meander_pattern(width, thickness);
        
        // Top border
        for y in 0..thickness.min(height) {
            for x in 0..width {
                if pattern[y as usize][x as usize] {
                    buffer.put_pixel(x, y, color);
                }
            }
        }
        
        // Bottom border
        for y in 0..thickness.min(height) {
            for x in 0..width {
                let py = height - 1 - y;
                if py < height && pattern[y as usize][x as usize] {
                    buffer.put_pixel(x, py, color);
                }
            }
        }
        
        // Left border
        let pattern_v = Self::generate_meander_pattern(thickness, height);
        for y in thickness..height.saturating_sub(thickness) {
            for x in 0..thickness.min(width) {
                if pattern_v[y as usize][x as usize] {
                    buffer.put_pixel(x, y, color);
                }
            }
        }
        
        // Right border
        for y in thickness..height.saturating_sub(thickness) {
            for x in 0..thickness.min(width) {
                let px = width - 1 - x;
                if px < width && pattern_v[y as usize][x as usize] {
                    buffer.put_pixel(px, y, color);
                }
            }
        }
    }
    
    pub fn generate_column_pattern() -> [[u8; 8]; 16] {
        // Simplified Greek column design for card backs
        [
            [1,1,1,1,1,1,1,1], // Capital
            [1,0,0,0,0,0,0,1],
            [1,1,1,1,1,1,1,1],
            [0,1,0,0,0,0,1,0], // Shaft
            [0,1,0,0,0,0,1,0],
            [0,1,0,0,0,0,1,0],
            [0,1,0,0,0,0,1,0],
            [0,1,0,0,0,0,1,0],
            [0,1,0,0,0,0,1,0],
            [0,1,0,0,0,0,1,0],
            [0,1,0,0,0,0,1,0],
            [0,1,0,0,0,0,1,0],
            [0,1,0,0,0,0,1,0],
            [1,1,1,1,1,1,1,1], // Base
            [1,0,0,0,0,0,0,1],
            [1,1,1,1,1,1,1,1],
        ]
    }
    
    pub fn generate_amphora_motif() -> [[u8; 12]; 16] {
        // Simplified amphora/vase design
        [
            [0,0,0,1,1,1,1,1,1,0,0,0], // Rim
            [0,0,1,0,0,0,0,0,0,1,0,0],
            [0,0,1,0,0,0,0,0,0,1,0,0], // Neck
            [0,0,0,1,0,0,0,0,1,0,0,0],
            [0,1,1,0,1,0,0,1,0,1,1,0], // Handles
            [1,0,0,0,0,1,1,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,1], // Body
            [1,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,1],
            [1,0,0,0,0,0,0,0,0,0,0,1],
            [0,1,0,0,0,0,0,0,0,0,1,0],
            [0,0,1,0,0,0,0,0,0,1,0,0],
            [0,0,0,1,1,0,0,1,1,0,0,0], // Base
            [0,0,0,0,0,1,1,0,0,0,0,0],
        ]
    }
    
    pub fn draw_corner_flourish(buffer: &mut RgbImage, x: u32, y: u32, size: u32, color: Rgb<u8>, corner: Corner) {
        // Simple 4x4 Greek corner decoration
        let flourish = [
            [1,1,1,0],
            [1,0,0,1],
            [1,0,0,0],
            [0,1,0,0],
        ];
        
        for dy in 0..size.min(4) {
            for dx in 0..size.min(4) {
                let pixel = match corner {
                    Corner::TopLeft => flourish[dy as usize][dx as usize],
                    Corner::TopRight => flourish[dy as usize][3 - dx as usize],
                    Corner::BottomLeft => flourish[3 - dy as usize][dx as usize],
                    Corner::BottomRight => flourish[3 - dy as usize][3 - dx as usize],
                };
                
                if pixel == 1 {
                    let px = x + dx;
                    let py = y + dy;
                    if px < buffer.width() && py < buffer.height() {
                        buffer.put_pixel(px, py, color);
                    }
                }
            }
        }
    }
    
    pub fn generate_marble_texture(size: u32, palette: &GreekPalette) -> RgbImage {
        let mut buffer = AssetGenerator::create_canvas(size, size);
        
        // Base marble color
        AssetGenerator::fill(&mut buffer, palette.get(GreekPalette::marble()));
        
        // Add subtle veining pattern
        for y in 0..size {
            for x in 0..size {
                // Create marble-like pattern using simple noise
                let noise = ((x * 7 + y * 11) % 17) as f32 / 17.0;
                if noise < 0.15 {
                    buffer.put_pixel(x, y, palette.get(GreekPalette::light_gray()));
                } else if noise > 0.85 {
                    buffer.put_pixel(x, y, palette.get(GreekPalette::white()));
                }
            }
        }
        
        // Add diagonal veins
        for i in 0..size/8 {
            let offset = i * 8;
            for j in 0..size {
                let x = (j + offset) % size;
                let y = j;
                if x < size && y < size && (j % 4) < 2 {
                    buffer.put_pixel(x, y, palette.get(GreekPalette::light_gray()));
                }
            }
        }
        
        buffer
    }
}

#[derive(Copy, Clone)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}