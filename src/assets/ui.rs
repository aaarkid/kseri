use image::{RgbImage, Rgb};
use super::{
    generator::AssetGenerator,
    palette::GreekPalette,
    patterns::{PatternGenerator, Corner},
    fonts::BitmapFont5x7,
};

pub struct UIGenerator {
    palette: GreekPalette,
    font: BitmapFont5x7,
}

impl UIGenerator {
    pub fn new() -> Self {
        Self {
            palette: GreekPalette::new(),
            font: BitmapFont5x7::new(),
        }
    }
    
    pub fn generate_score_frame(&self, width: u32, height: u32) -> RgbImage {
        let mut buffer = AssetGenerator::create_canvas(width, height);
        
        // Fill with lavender background
        AssetGenerator::fill(&mut buffer, self.palette.get(GreekPalette::lavender_mist()));
        
        // Draw Greek border
        PatternGenerator::draw_meander_border(&mut buffer, 2, self.palette.get(GreekPalette::goldenrod()));
        
        // Inner frame
        AssetGenerator::draw_rect(&mut buffer, 4, 4, width - 8, height - 8, 
            self.palette.get(GreekPalette::deep_purple()), false);
        
        buffer
    }
    
    pub fn generate_button(&self, width: u32, height: u32, state: ButtonState) -> RgbImage {
        let mut buffer = AssetGenerator::create_canvas(width, height);
        
        let (bg_color, border_color, _text_color) = match state {
            ButtonState::Normal => (
                self.palette.get(GreekPalette::medium_lavender()),
                self.palette.get(GreekPalette::goldenrod()),
                self.palette.get(GreekPalette::deep_purple()),
            ),
            ButtonState::Hover => (
                self.palette.get(GreekPalette::light_lavender()),
                self.palette.get(GreekPalette::goldenrod()),
                self.palette.get(GreekPalette::deep_purple()),
            ),
            ButtonState::Pressed => (
                self.palette.get(GreekPalette::soft_purple()),
                self.palette.get(GreekPalette::dark_goldenrod()),
                self.palette.get(GreekPalette::white()),
            ),
        };
        
        // Fill background
        AssetGenerator::fill(&mut buffer, bg_color);
        
        // Draw border
        AssetGenerator::draw_rect(&mut buffer, 0, 0, width, height, border_color, false);
        
        // Add corner flourishes
        PatternGenerator::draw_corner_flourish(&mut buffer, 1, 1, 3, border_color, Corner::TopLeft);
        PatternGenerator::draw_corner_flourish(&mut buffer, width - 4, 1, 3, border_color, Corner::TopRight);
        PatternGenerator::draw_corner_flourish(&mut buffer, 1, height - 4, 3, border_color, Corner::BottomLeft);
        PatternGenerator::draw_corner_flourish(&mut buffer, width - 4, height - 4, 3, border_color, Corner::BottomRight);
        
        buffer
    }
    
    pub fn generate_turn_indicator(&self) -> RgbImage {
        let size = 16;
        let mut buffer = AssetGenerator::create_canvas(size, size);
        
        // Fill with transparent background (white for now)
        AssetGenerator::fill(&mut buffer, self.palette.get(GreekPalette::white()));
        
        // Draw arrow pointing right
        for y in 0..size {
            let x_start = y / 2;
            let x_end = size - 1 - (y / 2);
            
            if y < size / 2 {
                // Top half of arrow
                for x in x_start..=x_end {
                    buffer.put_pixel(x, y, self.palette.get(GreekPalette::goldenrod()));
                }
            } else {
                // Bottom half of arrow
                let adjusted_y = size - 1 - y;
                let x_start = adjusted_y / 2;
                let x_end = size - 1 - (adjusted_y / 2);
                for x in x_start..=x_end {
                    buffer.put_pixel(x, y, self.palette.get(GreekPalette::goldenrod()));
                }
            }
        }
        
        buffer
    }
    
    pub fn generate_deck_counter_frame(&self) -> RgbImage {
        let mut buffer = AssetGenerator::create_canvas(48, 32);
        
        // Background
        AssetGenerator::fill(&mut buffer, self.palette.get(GreekPalette::lavender_mist()));
        
        // Border
        AssetGenerator::draw_rect(&mut buffer, 0, 0, 48, 32, 
            self.palette.get(GreekPalette::deep_purple()), false);
        
        // Deck icon (simplified cards)
        for i in 0..3 {
            let x = 4 + i * 2;
            let y = 8 + i;
            AssetGenerator::draw_rect(&mut buffer, x, y, 12, 16, 
                self.palette.get(GreekPalette::purple_gray()), true);
            AssetGenerator::draw_rect(&mut buffer, x, y, 12, 16, 
                self.palette.get(GreekPalette::deep_purple()), false);
        }
        
        buffer
    }
    
    pub fn generate_kseri_banner(&self) -> RgbImage {
        let mut buffer = AssetGenerator::create_canvas(128, 48);
        
        // Background with gradient effect using dithering
        for y in 0..48 {
            let color = if y < 24 {
                self.palette.get(GreekPalette::goldenrod())
            } else {
                self.palette.get(GreekPalette::dark_goldenrod())
            };
            for x in 0..128 {
                buffer.put_pixel(x, y, color);
            }
        }
        
        // Greek border
        PatternGenerator::draw_meander_border(&mut buffer, 3, self.palette.get(GreekPalette::deep_purple()));
        
        // "KSERI!" text centered
        self.font.render_text(&mut buffer, "KSERI!", 44, 20, self.palette.get(GreekPalette::white()));
        
        // Decorative stars
        self.draw_star(&mut buffer, 20, 24, self.palette.get(GreekPalette::white()));
        self.draw_star(&mut buffer, 108, 24, self.palette.get(GreekPalette::white()));
        
        buffer
    }
    
    fn draw_star(&self, buffer: &mut RgbImage, x: u32, y: u32, color: Rgb<u8>) {
        // Simple 5-pointed star
        let star_points = [
            (0, -4), (-2, -1), (-4, 0), (-2, 1), (-1, 4),
            (0, 2), (1, 4), (2, 1), (4, 0), (2, -1)
        ];
        
        // Draw star outline
        for i in 0..star_points.len() {
            let (x1, y1) = star_points[i];
            let (x2, y2) = star_points[(i + 2) % star_points.len()]; // Connect every other point
            AssetGenerator::draw_line(
                buffer,
                x as i32 + x1, y as i32 + y1,
                x as i32 + x2, y as i32 + y2,
                color
            );
        }
    }
    
    pub fn generate_table_texture(&self) -> RgbImage {
        PatternGenerator::generate_marble_texture(32, &self.palette)
    }
}

#[derive(Copy, Clone)]
pub enum ButtonState {
    Normal,
    Hover,
    Pressed,
}