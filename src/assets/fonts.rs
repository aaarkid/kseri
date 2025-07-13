use std::collections::HashMap;
use image::{RgbImage, Rgb};
use super::generator::AssetGenerator;

pub struct BitmapFont5x7 {
    glyphs: HashMap<char, [[u8; 5]; 7]>,
}

impl BitmapFont5x7 {
    pub fn new() -> Self {
        let mut glyphs = HashMap::new();
        
        // Letters
        glyphs.insert('A', [
            [0,1,1,1,0],
            [1,0,0,0,1],
            [1,0,0,0,1],
            [1,1,1,1,1],
            [1,0,0,0,1],
            [1,0,0,0,1],
            [0,0,0,0,0],
        ]);
        
        glyphs.insert('K', [
            [1,0,0,0,1],
            [1,0,0,1,0],
            [1,0,1,0,0],
            [1,1,0,0,0],
            [1,0,1,0,0],
            [1,0,0,1,0],
            [1,0,0,0,1],
        ]);
        
        glyphs.insert('Q', [
            [0,1,1,1,0],
            [1,0,0,0,1],
            [1,0,0,0,1],
            [1,0,0,0,1],
            [1,0,1,0,1],
            [1,0,0,1,0],
            [0,1,1,0,1],
        ]);
        
        glyphs.insert('J', [
            [0,0,1,1,1],
            [0,0,0,1,0],
            [0,0,0,1,0],
            [0,0,0,1,0],
            [0,0,0,1,0],
            [1,0,0,1,0],
            [0,1,1,0,0],
        ]);
        
        // Numbers
        glyphs.insert('1', [
            [0,0,1,0,0],
            [0,1,1,0,0],
            [0,0,1,0,0],
            [0,0,1,0,0],
            [0,0,1,0,0],
            [0,0,1,0,0],
            [0,1,1,1,0],
        ]);
        
        glyphs.insert('2', [
            [0,1,1,1,0],
            [1,0,0,0,1],
            [0,0,0,0,1],
            [0,0,0,1,0],
            [0,0,1,0,0],
            [0,1,0,0,0],
            [1,1,1,1,1],
        ]);
        
        glyphs.insert('3', [
            [0,1,1,1,0],
            [1,0,0,0,1],
            [0,0,0,0,1],
            [0,0,1,1,0],
            [0,0,0,0,1],
            [1,0,0,0,1],
            [0,1,1,1,0],
        ]);
        
        glyphs.insert('4', [
            [0,0,0,1,0],
            [0,0,1,1,0],
            [0,1,0,1,0],
            [1,0,0,1,0],
            [1,1,1,1,1],
            [0,0,0,1,0],
            [0,0,0,1,0],
        ]);
        
        glyphs.insert('5', [
            [1,1,1,1,1],
            [1,0,0,0,0],
            [1,1,1,1,0],
            [0,0,0,0,1],
            [0,0,0,0,1],
            [1,0,0,0,1],
            [0,1,1,1,0],
        ]);
        
        glyphs.insert('6', [
            [0,1,1,1,0],
            [1,0,0,0,1],
            [1,0,0,0,0],
            [1,1,1,1,0],
            [1,0,0,0,1],
            [1,0,0,0,1],
            [0,1,1,1,0],
        ]);
        
        glyphs.insert('7', [
            [1,1,1,1,1],
            [0,0,0,0,1],
            [0,0,0,1,0],
            [0,0,1,0,0],
            [0,0,1,0,0],
            [0,0,1,0,0],
            [0,0,1,0,0],
        ]);
        
        glyphs.insert('8', [
            [0,1,1,1,0],
            [1,0,0,0,1],
            [1,0,0,0,1],
            [0,1,1,1,0],
            [1,0,0,0,1],
            [1,0,0,0,1],
            [0,1,1,1,0],
        ]);
        
        glyphs.insert('9', [
            [0,1,1,1,0],
            [1,0,0,0,1],
            [1,0,0,0,1],
            [0,1,1,1,1],
            [0,0,0,0,1],
            [1,0,0,0,1],
            [0,1,1,1,0],
        ]);
        
        glyphs.insert('0', [
            [0,1,1,1,0],
            [1,0,0,0,1],
            [1,0,0,1,1],
            [1,0,1,0,1],
            [1,1,0,0,1],
            [1,0,0,0,1],
            [0,1,1,1,0],
        ]);
        
        Self { glyphs }
    }
    
    pub fn render_text(&self, buffer: &mut RgbImage, text: &str, x: u32, y: u32, color: Rgb<u8>) {
        let mut offset_x = 0;
        
        for ch in text.chars() {
            if let Some(glyph) = self.glyphs.get(&ch) {
                for (dy, row) in glyph.iter().enumerate() {
                    for (dx, &pixel) in row.iter().enumerate() {
                        if pixel == 1 {
                            let px = x + offset_x + dx as u32;
                            let py = y + dy as u32;
                            if px < buffer.width() && py < buffer.height() {
                                buffer.put_pixel(px, py, color);
                            }
                        }
                    }
                }
                offset_x += 6; // 5 pixels + 1 space
            }
        }
    }
}

pub struct BitmapFont3x5 {
    glyphs: HashMap<char, [[u8; 3]; 5]>,
}

impl BitmapFont3x5 {
    pub fn new() -> Self {
        let mut glyphs = HashMap::new();
        
        // Compact letters for corner indicators
        glyphs.insert('A', [
            [0,1,0],
            [1,0,1],
            [1,1,1],
            [1,0,1],
            [1,0,1],
        ]);
        
        glyphs.insert('K', [
            [1,0,1],
            [1,1,0],
            [1,0,0],
            [1,1,0],
            [1,0,1],
        ]);
        
        glyphs.insert('Q', [
            [0,1,0],
            [1,0,1],
            [1,0,1],
            [1,1,1],
            [0,1,1],
        ]);
        
        glyphs.insert('J', [
            [0,1,1],
            [0,0,1],
            [0,0,1],
            [1,0,1],
            [0,1,0],
        ]);
        
        // Compact numbers
        glyphs.insert('2', [
            [1,1,0],
            [0,0,1],
            [0,1,0],
            [1,0,0],
            [1,1,1],
        ]);
        
        glyphs.insert('3', [
            [1,1,0],
            [0,0,1],
            [0,1,0],
            [0,0,1],
            [1,1,0],
        ]);
        
        glyphs.insert('4', [
            [1,0,1],
            [1,0,1],
            [1,1,1],
            [0,0,1],
            [0,0,1],
        ]);
        
        glyphs.insert('5', [
            [1,1,1],
            [1,0,0],
            [1,1,0],
            [0,0,1],
            [1,1,0],
        ]);
        
        glyphs.insert('6', [
            [0,1,1],
            [1,0,0],
            [1,1,0],
            [1,0,1],
            [1,1,0],
        ]);
        
        glyphs.insert('7', [
            [1,1,1],
            [0,0,1],
            [0,1,0],
            [0,1,0],
            [0,1,0],
        ]);
        
        glyphs.insert('8', [
            [0,1,0],
            [1,0,1],
            [0,1,0],
            [1,0,1],
            [0,1,0],
        ]);
        
        glyphs.insert('9', [
            [0,1,0],
            [1,0,1],
            [0,1,1],
            [0,0,1],
            [1,1,0],
        ]);
        
        glyphs.insert('0', [
            [0,1,0],
            [1,0,1],
            [1,0,1],
            [1,0,1],
            [0,1,0],
        ]);
        
        Self { glyphs }
    }
    
    pub fn render_text(&self, buffer: &mut RgbImage, text: &str, x: u32, y: u32, color: Rgb<u8>) {
        let mut offset_x = 0;
        
        for ch in text.chars() {
            if let Some(glyph) = self.glyphs.get(&ch) {
                for (dy, row) in glyph.iter().enumerate() {
                    for (dx, &pixel) in row.iter().enumerate() {
                        if pixel == 1 {
                            let px = x + offset_x + dx as u32;
                            let py = y + dy as u32;
                            if px < buffer.width() && py < buffer.height() {
                                buffer.put_pixel(px, py, color);
                            }
                        }
                    }
                }
                offset_x += 4; // 3 pixels + 1 space
            }
        }
    }
    
    pub fn render_text_rotated(&self, buffer: &mut RgbImage, text: &str, x: u32, y: u32, color: Rgb<u8>) {
        let mut offset_x = 0;
        
        for ch in text.chars() {
            if let Some(glyph) = self.glyphs.get(&ch) {
                // Render upside down
                for (dy, row) in glyph.iter().enumerate() {
                    for (dx, &pixel) in row.iter().enumerate() {
                        if pixel == 1 {
                            let px = x - offset_x - dx as u32;
                            let py = y - dy as u32;
                            AssetGenerator::put_pixel_safe(buffer, px as i32, py as i32, color);
                        }
                    }
                }
                offset_x += 4;
            }
        }
    }
}