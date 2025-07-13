use image::{ImageBuffer, Rgb, RgbImage};
use std::fs;
use crate::components::card::{Card, Suit, Rank};
use super::{
    cards::CardGenerator,
    ui::{UIGenerator, ButtonState},
};

pub struct AssetGenerator {
    card_generator: CardGenerator,
    ui_generator: UIGenerator,
}

impl AssetGenerator {
    pub fn new() -> Self {
        Self {
            card_generator: CardGenerator::new(),
            ui_generator: UIGenerator::new(),
        }
    }
    
    pub fn generate_all_assets(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Generating Greek-themed pixel art assets with lavender palette...");
        
        // Create directories if they don't exist
        fs::create_dir_all("assets/cards/individual")?;
        fs::create_dir_all("assets/ui")?;
        fs::create_dir_all("assets/textures")?;
        
        // Generate all 52 cards
        self.generate_all_cards()?;
        
        // Generate card back
        self.generate_card_back()?;
        
        // Generate UI elements
        self.generate_ui_elements()?;
        
        // Generate table texture
        self.generate_table_texture()?;
        
        println!("All assets generated successfully!");
        Ok(())
    }
    
    fn generate_all_cards(&self) -> Result<(), Box<dyn std::error::Error>> {
        let suits = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        let ranks = [
            Rank::Ace, Rank::Two, Rank::Three, Rank::Four, Rank::Five,
            Rank::Six, Rank::Seven, Rank::Eight, Rank::Nine, Rank::Ten,
            Rank::Jack, Rank::Queen, Rank::King,
        ];
        
        for suit in &suits {
            for rank in &ranks {
                let card = Card { suit: *suit, rank: *rank };
                let image = self.card_generator.generate_card(&card);
                
                let suit_name = match suit {
                    Suit::Hearts => "hearts",
                    Suit::Diamonds => "diamonds",
                    Suit::Clubs => "clubs",
                    Suit::Spades => "spades",
                };
                
                let rank_name = match rank {
                    Rank::Ace => "ace",
                    Rank::Two => "2",
                    Rank::Three => "3",
                    Rank::Four => "4",
                    Rank::Five => "5",
                    Rank::Six => "6",
                    Rank::Seven => "7",
                    Rank::Eight => "8",
                    Rank::Nine => "9",
                    Rank::Ten => "10",
                    Rank::Jack => "jack",
                    Rank::Queen => "queen",
                    Rank::King => "king",
                };
                
                let filename = format!("assets/cards/individual/{}_{}.png", suit_name, rank_name);
                
                image.save(&filename)?;
                println!("Generated: {}", filename);
            }
        }
        
        Ok(())
    }
    
    fn generate_card_back(&self) -> Result<(), Box<dyn std::error::Error>> {
        let card_back = self.card_generator.generate_card_back();
        card_back.save("assets/cards/individual/card_back.png")?;
        println!("Generated: assets/cards/individual/card_back.png");
        Ok(())
    }
    
    fn generate_ui_elements(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Score frame
        let score_frame = self.ui_generator.generate_score_frame(120, 60);
        score_frame.save("assets/ui/score_frame.png")?;
        println!("Generated: assets/ui/score_frame.png");
        
        // Buttons in different states
        let button_normal = self.ui_generator.generate_button(64, 24, ButtonState::Normal);
        button_normal.save("assets/ui/button_normal.png")?;
        println!("Generated: assets/ui/button_normal.png");
        
        let button_hover = self.ui_generator.generate_button(64, 24, ButtonState::Hover);
        button_hover.save("assets/ui/button_hover.png")?;
        println!("Generated: assets/ui/button_hover.png");
        
        let button_pressed = self.ui_generator.generate_button(64, 24, ButtonState::Pressed);
        button_pressed.save("assets/ui/button_pressed.png")?;
        println!("Generated: assets/ui/button_pressed.png");
        
        // Turn indicator
        let turn_indicator = self.ui_generator.generate_turn_indicator();
        turn_indicator.save("assets/ui/turn_indicator.png")?;
        println!("Generated: assets/ui/turn_indicator.png");
        
        // Deck counter
        let deck_counter = self.ui_generator.generate_deck_counter_frame();
        deck_counter.save("assets/ui/deck_counter.png")?;
        println!("Generated: assets/ui/deck_counter.png");
        
        // Kseri banner
        let kseri_banner = self.ui_generator.generate_kseri_banner();
        kseri_banner.save("assets/ui/kseri_banner.png")?;
        println!("Generated: assets/ui/kseri_banner.png");
        
        Ok(())
    }
    
    fn generate_table_texture(&self) -> Result<(), Box<dyn std::error::Error>> {
        let table_texture = self.ui_generator.generate_table_texture();
        table_texture.save("assets/textures/table_marble.png")?;
        println!("Generated: assets/textures/table_marble.png");
        Ok(())
    }

    pub fn create_canvas(width: u32, height: u32) -> RgbImage {
        ImageBuffer::new(width, height)
    }

    pub fn put_pixel_safe(buffer: &mut RgbImage, x: i32, y: i32, color: Rgb<u8>) {
        if x >= 0 && y >= 0 && x < buffer.width() as i32 && y < buffer.height() as i32 {
            buffer.put_pixel(x as u32, y as u32, color);
        }
    }

    pub fn draw_line(buffer: &mut RgbImage, x0: i32, y0: i32, x1: i32, y1: i32, color: Rgb<u8>) {
        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;
        let mut x = x0;
        let mut y = y0;
        
        loop {
            Self::put_pixel_safe(buffer, x, y, color);
            
            if x == x1 && y == y1 { break; }
            
            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x += sx;
            }
            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    pub fn draw_rect(buffer: &mut RgbImage, x: u32, y: u32, w: u32, h: u32, color: Rgb<u8>, filled: bool) {
        if filled {
            for py in y..y.saturating_add(h).min(buffer.height()) {
                for px in x..x.saturating_add(w).min(buffer.width()) {
                    buffer.put_pixel(px, py, color);
                }
            }
        } else {
            // Top and bottom edges
            for px in x..x.saturating_add(w).min(buffer.width()) {
                if y < buffer.height() {
                    buffer.put_pixel(px, y, color);
                }
                if y + h - 1 < buffer.height() && h > 0 {
                    buffer.put_pixel(px, y + h - 1, color);
                }
            }
            // Left and right edges
            for py in y + 1..y.saturating_add(h - 1).min(buffer.height()) {
                if x < buffer.width() {
                    buffer.put_pixel(x, py, color);
                }
                if x + w - 1 < buffer.width() && w > 0 {
                    buffer.put_pixel(x + w - 1, py, color);
                }
            }
        }
    }

    pub fn draw_circle(buffer: &mut RgbImage, cx: i32, cy: i32, radius: i32, color: Rgb<u8>, filled: bool) {
        let mut x = 0;
        let mut y = radius;
        let mut d = 3 - 2 * radius;
        
        while x <= y {
            if filled {
                for px in -x..=x {
                    Self::put_pixel_safe(buffer, cx + px, cy + y, color);
                    Self::put_pixel_safe(buffer, cx + px, cy - y, color);
                }
                for px in -y..=y {
                    Self::put_pixel_safe(buffer, cx + px, cy + x, color);
                    Self::put_pixel_safe(buffer, cx + px, cy - x, color);
                }
            } else {
                Self::put_pixel_safe(buffer, cx + x, cy + y, color);
                Self::put_pixel_safe(buffer, cx - x, cy + y, color);
                Self::put_pixel_safe(buffer, cx + x, cy - y, color);
                Self::put_pixel_safe(buffer, cx - x, cy - y, color);
                Self::put_pixel_safe(buffer, cx + y, cy + x, color);
                Self::put_pixel_safe(buffer, cx - y, cy + x, color);
                Self::put_pixel_safe(buffer, cx + y, cy - x, color);
                Self::put_pixel_safe(buffer, cx - y, cy - x, color);
            }
            
            if d < 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }

    pub fn fill(buffer: &mut RgbImage, color: Rgb<u8>) {
        for pixel in buffer.pixels_mut() {
            *pixel = color;
        }
    }

    pub fn draw_border(buffer: &mut RgbImage, thickness: u32, color: Rgb<u8>) {
        let w = buffer.width();
        let h = buffer.height();
        
        for i in 0..thickness {
            // Top edge
            for x in 0..w {
                if i < h {
                    buffer.put_pixel(x, i, color);
                }
            }
            // Bottom edge
            for x in 0..w {
                if h > i && h - 1 - i < h {
                    buffer.put_pixel(x, h - 1 - i, color);
                }
            }
            // Left edge
            for y in i..h.saturating_sub(i) {
                if i < w {
                    buffer.put_pixel(i, y, color);
                }
            }
            // Right edge
            for y in i..h.saturating_sub(i) {
                if w > i && w - 1 - i < w {
                    buffer.put_pixel(w - 1 - i, y, color);
                }
            }
        }
    }
}