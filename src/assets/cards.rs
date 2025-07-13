use image::{RgbImage, Rgb};
use crate::components::card::{Card, Suit, Rank};
use super::{
    generator::AssetGenerator,
    palette::GreekPalette,
    patterns::{PatternGenerator, Corner},
    fonts::{BitmapFont5x7, BitmapFont3x5},
};

pub const CARD_WIDTH: u32 = 64;
pub const CARD_HEIGHT: u32 = 96;

pub struct CardGenerator {
    palette: GreekPalette,
    _font_5x7: BitmapFont5x7,
    font_3x5: BitmapFont3x5,
}

impl CardGenerator {
    pub fn new() -> Self {
        Self {
            palette: GreekPalette::new(),
            _font_5x7: BitmapFont5x7::new(),
            font_3x5: BitmapFont3x5::new(),
        }
    }
    
    pub fn generate_card(&self, card: &Card) -> RgbImage {
        let mut buffer = AssetGenerator::create_canvas(CARD_WIDTH, CARD_HEIGHT);
        
        // Fill with white background
        AssetGenerator::fill(&mut buffer, self.palette.get(GreekPalette::white()));
        
        // Draw Greek meander border
        PatternGenerator::draw_meander_border(&mut buffer, 2, self.palette.get(GreekPalette::gold()));
        
        // Draw corner flourishes
        PatternGenerator::draw_corner_flourish(&mut buffer, 2, 2, 4, 
            self.palette.get(GreekPalette::gold()), Corner::TopLeft);
        PatternGenerator::draw_corner_flourish(&mut buffer, CARD_WIDTH - 6, 2, 4, 
            self.palette.get(GreekPalette::gold()), Corner::TopRight);
        PatternGenerator::draw_corner_flourish(&mut buffer, 2, CARD_HEIGHT - 6, 4, 
            self.palette.get(GreekPalette::gold()), Corner::BottomLeft);
        PatternGenerator::draw_corner_flourish(&mut buffer, CARD_WIDTH - 6, CARD_HEIGHT - 6, 4, 
            self.palette.get(GreekPalette::gold()), Corner::BottomRight);
        
        // Get suit color
        let suit_color = match card.suit {
            Suit::Hearts | Suit::Diamonds => self.palette.get(GreekPalette::bright_red()),
            Suit::Spades | Suit::Clubs => self.palette.get(GreekPalette::black()),
        };
        
        // Draw rank
        let rank_str = match card.rank {
            Rank::Ace => "A",
            Rank::King => "K",
            Rank::Queen => "Q",
            Rank::Jack => "J",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
        };
        
        // Top-left rank and suit
        self.font_3x5.render_text(&mut buffer, rank_str, 5, 5, suit_color);
        self.draw_small_suit_symbol(&mut buffer, card.suit, 5, 11, suit_color);
        
        // Bottom-right rank and suit (rotated)
        self.font_3x5.render_text_rotated(&mut buffer, rank_str, CARD_WIDTH - 5, CARD_HEIGHT - 5, suit_color);
        self.draw_small_suit_symbol_rotated(&mut buffer, card.suit, CARD_WIDTH - 5, CARD_HEIGHT - 11, suit_color);
        
        // Draw center content based on card type
        match card.rank {
            Rank::Jack | Rank::Queen | Rank::King => {
                // Face cards - will be handled in a separate method
                self.draw_face_card_placeholder(&mut buffer, card);
            }
            _ => {
                // Number cards - draw suit symbols
                self.draw_number_card_suits(&mut buffer, card);
            }
        }
        
        buffer
    }
    
    fn draw_suit_symbol(&self, buffer: &mut RgbImage, suit: Suit, x: u32, y: u32, color: Rgb<u8>) {
        match suit {
            Suit::Hearts => self.draw_heart(buffer, x, y, color),
            Suit::Diamonds => self.draw_diamond(buffer, x, y, color),
            Suit::Clubs => self.draw_club(buffer, x, y, color),
            Suit::Spades => self.draw_spade(buffer, x, y, color),
        }
    }
    
    fn draw_small_suit_symbol(&self, buffer: &mut RgbImage, suit: Suit, x: u32, y: u32, color: Rgb<u8>) {
        // 4x4 pixel suit symbols for corners
        match suit {
            Suit::Hearts => {
                let pattern = [
                    [0,1,0,1],
                    [1,1,1,1],
                    [0,1,1,0],
                    [0,0,1,0],
                ];
                self.draw_pattern(buffer, &pattern, x, y, color);
            }
            Suit::Diamonds => {
                let pattern = [
                    [0,0,1,0],
                    [0,1,1,1],
                    [1,1,1,1],
                    [0,1,0,0],
                ];
                self.draw_pattern(buffer, &pattern, x, y, color);
            }
            Suit::Clubs => {
                let pattern = [
                    [0,1,1,0],
                    [1,1,1,1],
                    [0,1,1,0],
                    [0,0,1,0],
                ];
                self.draw_pattern(buffer, &pattern, x, y, color);
            }
            Suit::Spades => {
                let pattern = [
                    [0,0,1,0],
                    [0,1,1,1],
                    [1,1,1,1],
                    [0,0,1,0],
                ];
                self.draw_pattern(buffer, &pattern, x, y, color);
            }
        }
    }
    
    fn draw_small_suit_symbol_rotated(&self, buffer: &mut RgbImage, suit: Suit, x: u32, y: u32, color: Rgb<u8>) {
        // Draw upside down for bottom right corner
        match suit {
            Suit::Hearts => {
                let pattern = [
                    [0,0,1,0],
                    [0,1,1,0],
                    [1,1,1,1],
                    [0,1,0,1],
                ];
                self.draw_pattern_rotated(buffer, &pattern, x, y, color);
            }
            Suit::Diamonds => {
                let pattern = [
                    [0,1,0,0],
                    [1,1,1,1],
                    [0,1,1,1],
                    [0,0,1,0],
                ];
                self.draw_pattern_rotated(buffer, &pattern, x, y, color);
            }
            Suit::Clubs => {
                let pattern = [
                    [0,0,1,0],
                    [0,1,1,0],
                    [1,1,1,1],
                    [0,1,1,0],
                ];
                self.draw_pattern_rotated(buffer, &pattern, x, y, color);
            }
            Suit::Spades => {
                let pattern = [
                    [0,0,1,0],
                    [1,1,1,1],
                    [0,1,1,1],
                    [0,0,1,0],
                ];
                self.draw_pattern_rotated(buffer, &pattern, x, y, color);
            }
        }
    }
    
    fn draw_heart(&self, buffer: &mut RgbImage, x: u32, y: u32, color: Rgb<u8>) {
        // 8x8 heart
        let heart = [
            [0,1,1,0,0,1,1,0],
            [1,1,1,1,1,1,1,1],
            [1,1,1,1,1,1,1,1],
            [1,1,1,1,1,1,1,1],
            [0,1,1,1,1,1,1,0],
            [0,0,1,1,1,1,0,0],
            [0,0,0,1,1,0,0,0],
            [0,0,0,0,0,0,0,0],
        ];
        self.draw_pattern(buffer, &heart, x, y, color);
    }
    
    fn draw_diamond(&self, buffer: &mut RgbImage, x: u32, y: u32, color: Rgb<u8>) {
        // 8x8 diamond
        let diamond = [
            [0,0,0,1,1,0,0,0],
            [0,0,1,1,1,1,0,0],
            [0,1,1,1,1,1,1,0],
            [1,1,1,1,1,1,1,1],
            [1,1,1,1,1,1,1,1],
            [0,1,1,1,1,1,1,0],
            [0,0,1,1,1,1,0,0],
            [0,0,0,1,1,0,0,0],
        ];
        self.draw_pattern(buffer, &diamond, x, y, color);
    }
    
    fn draw_club(&self, buffer: &mut RgbImage, x: u32, y: u32, color: Rgb<u8>) {
        // 8x8 club
        let club = [
            [0,0,1,1,1,1,0,0],
            [0,1,1,1,1,1,1,0],
            [0,1,1,1,1,1,1,0],
            [1,1,1,1,1,1,1,1],
            [1,1,1,1,1,1,1,1],
            [0,1,1,1,1,1,1,0],
            [0,0,0,1,1,0,0,0],
            [0,0,1,1,1,1,0,0],
        ];
        self.draw_pattern(buffer, &club, x, y, color);
    }
    
    fn draw_spade(&self, buffer: &mut RgbImage, x: u32, y: u32, color: Rgb<u8>) {
        // 8x8 spade
        let spade = [
            [0,0,0,1,1,0,0,0],
            [0,0,1,1,1,1,0,0],
            [0,1,1,1,1,1,1,0],
            [1,1,1,1,1,1,1,1],
            [1,1,1,1,1,1,1,1],
            [0,1,1,1,1,1,1,0],
            [0,0,0,1,1,0,0,0],
            [0,0,1,1,1,1,0,0],
        ];
        self.draw_pattern(buffer, &spade, x, y, color);
    }
    
    fn draw_pattern<const W: usize, const H: usize>(&self, buffer: &mut RgbImage, 
                                                     pattern: &[[u8; W]; H], 
                                                     x: u32, y: u32, color: Rgb<u8>) {
        for (dy, row) in pattern.iter().enumerate() {
            for (dx, &pixel) in row.iter().enumerate() {
                if pixel == 1 {
                    let px = x + dx as u32;
                    let py = y + dy as u32;
                    if px < buffer.width() && py < buffer.height() {
                        buffer.put_pixel(px, py, color);
                    }
                }
            }
        }
    }
    
    fn draw_pattern_rotated<const W: usize, const H: usize>(&self, buffer: &mut RgbImage, 
                                                            pattern: &[[u8; W]; H], 
                                                            x: u32, y: u32, color: Rgb<u8>) {
        for (dy, row) in pattern.iter().enumerate() {
            for (dx, &pixel) in row.iter().enumerate() {
                if pixel == 1 {
                    let px = x - dx as u32;
                    let py = y - dy as u32;
                    AssetGenerator::put_pixel_safe(buffer, px as i32, py as i32, color);
                }
            }
        }
    }
    
    fn draw_number_card_suits(&self, buffer: &mut RgbImage, card: &Card) {
        let suit_color = match card.suit {
            Suit::Hearts | Suit::Diamonds => self.palette.get(GreekPalette::bright_red()),
            Suit::Spades | Suit::Clubs => self.palette.get(GreekPalette::black()),
        };
        
        // Position suits based on card value
        let positions = match card.rank {
            Rank::Ace => vec![(28, 44)], // Center
            Rank::Two => vec![(28, 20), (28, 68)],
            Rank::Three => vec![(28, 20), (28, 44), (28, 68)],
            Rank::Four => vec![(18, 20), (38, 20), (18, 68), (38, 68)],
            Rank::Five => vec![(18, 20), (38, 20), (28, 44), (18, 68), (38, 68)],
            Rank::Six => vec![(18, 20), (38, 20), (18, 44), (38, 44), (18, 68), (38, 68)],
            Rank::Seven => vec![(18, 20), (38, 20), (28, 32), (18, 44), (38, 44), (18, 68), (38, 68)],
            Rank::Eight => vec![(18, 20), (38, 20), (18, 32), (38, 32), 
                                (18, 56), (38, 56), (18, 68), (38, 68)],
            Rank::Nine => vec![(18, 20), (38, 20), (18, 32), (38, 32), (28, 44),
                               (18, 56), (38, 56), (18, 68), (38, 68)],
            Rank::Ten => vec![(18, 20), (38, 20), (18, 32), (38, 32), 
                              (28, 26), (28, 62),
                              (18, 56), (38, 56), (18, 68), (38, 68)],
            _ => vec![],
        };
        
        for (x, y) in positions {
            self.draw_suit_symbol(buffer, card.suit, x, y, suit_color);
        }
    }
    
    fn draw_face_card_placeholder(&self, buffer: &mut RgbImage, card: &Card) {
        match (card.rank, card.suit) {
            // Jacks - Young Warriors
            (Rank::Jack, Suit::Hearts) => self.draw_achilles(buffer),
            (Rank::Jack, Suit::Diamonds) => self.draw_perseus(buffer),
            (Rank::Jack, Suit::Clubs) => self.draw_theseus(buffer),
            (Rank::Jack, Suit::Spades) => self.draw_jason(buffer),
            
            // Queens - Goddesses
            (Rank::Queen, Suit::Hearts) => self.draw_aphrodite(buffer),
            (Rank::Queen, Suit::Diamonds) => self.draw_athena(buffer),
            (Rank::Queen, Suit::Clubs) => self.draw_hera(buffer),
            (Rank::Queen, Suit::Spades) => self.draw_artemis(buffer),
            
            // Kings - Major Gods
            (Rank::King, Suit::Hearts) => self.draw_zeus(buffer),
            (Rank::King, Suit::Diamonds) => self.draw_apollo(buffer),
            (Rank::King, Suit::Clubs) => self.draw_poseidon(buffer),
            (Rank::King, Suit::Spades) => self.draw_hades(buffer),
            
            _ => {}
        }
    }
    
    // Jack of Hearts - Achilles with spear
    fn draw_achilles(&self, buffer: &mut RgbImage) {
        let center_x = CARD_WIDTH / 2;
        let center_y = CARD_HEIGHT / 2;
        
        // Head (circular)
        AssetGenerator::draw_circle(buffer, center_x as i32, (center_y - 12) as i32, 4, 
            self.palette.get(GreekPalette::deep_purple()), true);
        
        // Body
        AssetGenerator::draw_rect(buffer, center_x - 4, center_y - 8, 8, 12, 
            self.palette.get(GreekPalette::dusty_rose()), true);
        
        // Arms and spear
        AssetGenerator::draw_line(buffer, (center_x - 6) as i32, (center_y - 4) as i32, 
            (center_x + 8) as i32, (center_y - 12) as i32, self.palette.get(GreekPalette::dark_goldenrod()));
        // Spear tip
        AssetGenerator::draw_line(buffer, (center_x + 8) as i32, (center_y - 12) as i32, 
            (center_x + 10) as i32, (center_y - 14) as i32, self.palette.get(GreekPalette::goldenrod()));
        
        // Legs
        AssetGenerator::draw_line(buffer, (center_x - 2) as i32, (center_y + 4) as i32, 
            (center_x - 3) as i32, (center_y + 10) as i32, self.palette.get(GreekPalette::deep_purple()));
        AssetGenerator::draw_line(buffer, (center_x + 2) as i32, (center_y + 4) as i32, 
            (center_x + 3) as i32, (center_y + 10) as i32, self.palette.get(GreekPalette::deep_purple()));
    }
    
    // Jack of Diamonds - Perseus with shield
    fn draw_perseus(&self, buffer: &mut RgbImage) {
        let center_x = CARD_WIDTH / 2;
        let center_y = CARD_HEIGHT / 2;
        
        // Head
        AssetGenerator::draw_circle(buffer, center_x as i32, (center_y - 12) as i32, 4, 
            self.palette.get(GreekPalette::deep_purple()), true);
        
        // Body
        AssetGenerator::draw_rect(buffer, center_x - 4, center_y - 8, 8, 12, 
            self.palette.get(GreekPalette::dusty_rose()), true);
        
        // Shield (circular)
        AssetGenerator::draw_circle(buffer, (center_x - 8) as i32, (center_y - 4) as i32, 5, 
            self.palette.get(GreekPalette::goldenrod()), false);
        AssetGenerator::draw_circle(buffer, (center_x - 8) as i32, (center_y - 4) as i32, 4, 
            self.palette.get(GreekPalette::dark_goldenrod()), true);
        
        // Sword arm
        AssetGenerator::draw_line(buffer, (center_x + 4) as i32, (center_y - 4) as i32, 
            (center_x + 8) as i32, (center_y - 8) as i32, self.palette.get(GreekPalette::deep_purple()));
        
        // Legs
        AssetGenerator::draw_line(buffer, (center_x - 2) as i32, (center_y + 4) as i32, 
            (center_x - 3) as i32, (center_y + 10) as i32, self.palette.get(GreekPalette::deep_purple()));
        AssetGenerator::draw_line(buffer, (center_x + 2) as i32, (center_y + 4) as i32, 
            (center_x + 3) as i32, (center_y + 10) as i32, self.palette.get(GreekPalette::deep_purple()));
    }
    
    // Jack of Clubs - Theseus with sword
    fn draw_theseus(&self, buffer: &mut RgbImage) {
        let center_x = CARD_WIDTH / 2;
        let center_y = CARD_HEIGHT / 2;
        
        // Head
        AssetGenerator::draw_circle(buffer, center_x as i32, (center_y - 12) as i32, 4, 
            self.palette.get(GreekPalette::deep_purple()), true);
        
        // Body
        AssetGenerator::draw_rect(buffer, center_x - 4, center_y - 8, 8, 12, 
            self.palette.get(GreekPalette::royal_purple()), true);
        
        // Sword
        AssetGenerator::draw_line(buffer, (center_x + 6) as i32, (center_y - 10) as i32, 
            (center_x + 6) as i32, (center_y + 2) as i32, self.palette.get(GreekPalette::goldenrod()));
        // Sword hilt
        AssetGenerator::draw_line(buffer, (center_x + 4) as i32, (center_y - 6) as i32, 
            (center_x + 8) as i32, (center_y - 6) as i32, self.palette.get(GreekPalette::dark_goldenrod()));
        
        // Arms
        AssetGenerator::draw_line(buffer, center_x as i32, (center_y - 6) as i32, 
            (center_x + 6) as i32, (center_y - 6) as i32, self.palette.get(GreekPalette::deep_purple()));
        
        // Legs
        AssetGenerator::draw_line(buffer, (center_x - 2) as i32, (center_y + 4) as i32, 
            (center_x - 3) as i32, (center_y + 10) as i32, self.palette.get(GreekPalette::deep_purple()));
        AssetGenerator::draw_line(buffer, (center_x + 2) as i32, (center_y + 4) as i32, 
            (center_x + 3) as i32, (center_y + 10) as i32, self.palette.get(GreekPalette::deep_purple()));
    }
    
    // Jack of Spades - Jason with helmet
    fn draw_jason(&self, buffer: &mut RgbImage) {
        let center_x = CARD_WIDTH / 2;
        let center_y = CARD_HEIGHT / 2;
        
        // Helmet
        AssetGenerator::draw_rect(buffer, center_x - 5, center_y - 14, 10, 6, 
            self.palette.get(GreekPalette::goldenrod()), true);
        // Helmet plume
        AssetGenerator::draw_line(buffer, center_x as i32, (center_y - 14) as i32, 
            center_x as i32, (center_y - 18) as i32, self.palette.get(GreekPalette::dusty_rose()));
        
        // Head
        AssetGenerator::draw_circle(buffer, center_x as i32, (center_y - 10) as i32, 3, 
            self.palette.get(GreekPalette::deep_purple()), true);
        
        // Body
        AssetGenerator::draw_rect(buffer, center_x - 4, center_y - 8, 8, 12, 
            self.palette.get(GreekPalette::royal_purple()), true);
        
        // Arms
        AssetGenerator::draw_line(buffer, (center_x - 4) as i32, (center_y - 6) as i32, 
            (center_x - 8) as i32, (center_y - 2) as i32, self.palette.get(GreekPalette::deep_purple()));
        AssetGenerator::draw_line(buffer, (center_x + 4) as i32, (center_y - 6) as i32, 
            (center_x + 8) as i32, (center_y - 2) as i32, self.palette.get(GreekPalette::deep_purple()));
        
        // Legs
        AssetGenerator::draw_line(buffer, (center_x - 2) as i32, (center_y + 4) as i32, 
            (center_x - 3) as i32, (center_y + 10) as i32, self.palette.get(GreekPalette::deep_purple()));
        AssetGenerator::draw_line(buffer, (center_x + 2) as i32, (center_y + 4) as i32, 
            (center_x + 3) as i32, (center_y + 10) as i32, self.palette.get(GreekPalette::deep_purple()));
    }
    
    // Queen of Hearts - Aphrodite with roses
    fn draw_aphrodite(&self, buffer: &mut RgbImage) {
        let center_x = CARD_WIDTH / 2;
        let center_y = CARD_HEIGHT / 2;
        
        // Hair
        AssetGenerator::draw_rect(buffer, center_x - 6, center_y - 14, 12, 8, 
            self.palette.get(GreekPalette::goldenrod()), true);
        
        // Head
        AssetGenerator::draw_circle(buffer, center_x as i32, (center_y - 10) as i32, 4, 
            self.palette.get(GreekPalette::light_lavender()), true);
        
        // Dress
        AssetGenerator::draw_rect(buffer, center_x - 6, center_y - 6, 12, 16, 
            self.palette.get(GreekPalette::dusty_rose()), true);
        
        // Rose in hand
        AssetGenerator::draw_circle(buffer, (center_x + 8) as i32, (center_y - 4) as i32, 2, 
            self.palette.get(GreekPalette::deep_rose()), true);
        // Stem
        AssetGenerator::draw_line(buffer, (center_x + 8) as i32, (center_y - 2) as i32, 
            (center_x + 8) as i32, (center_y + 2) as i32, self.palette.get(GreekPalette::purple_gray()));
    }
    
    // Queen of Diamonds - Athena with owl
    fn draw_athena(&self, buffer: &mut RgbImage) {
        let center_x = CARD_WIDTH / 2;
        let center_y = CARD_HEIGHT / 2;
        
        // Helmet
        AssetGenerator::draw_rect(buffer, center_x - 5, center_y - 14, 10, 6, 
            self.palette.get(GreekPalette::goldenrod()), true);
        
        // Head
        AssetGenerator::draw_circle(buffer, center_x as i32, (center_y - 10) as i32, 4, 
            self.palette.get(GreekPalette::light_lavender()), true);
        
        // Dress
        AssetGenerator::draw_rect(buffer, center_x - 6, center_y - 6, 12, 16, 
            self.palette.get(GreekPalette::dusty_rose()), true);
        
        // Owl on shoulder
        AssetGenerator::draw_circle(buffer, (center_x - 8) as i32, (center_y - 6) as i32, 3, 
            self.palette.get(GreekPalette::purple_gray()), true);
        // Owl eyes
        AssetGenerator::put_pixel_safe(buffer, (center_x - 9) as i32, (center_y - 7) as i32, 
            self.palette.get(GreekPalette::goldenrod()));
        AssetGenerator::put_pixel_safe(buffer, (center_x - 7) as i32, (center_y - 7) as i32, 
            self.palette.get(GreekPalette::goldenrod()));
    }
    
    // Queen of Clubs - Hera with peacock feather
    fn draw_hera(&self, buffer: &mut RgbImage) {
        let center_x = CARD_WIDTH / 2;
        let center_y = CARD_HEIGHT / 2;
        
        // Crown
        AssetGenerator::draw_rect(buffer, center_x - 4, center_y - 15, 8, 3, 
            self.palette.get(GreekPalette::goldenrod()), true);
        
        // Hair
        AssetGenerator::draw_rect(buffer, center_x - 6, center_y - 12, 12, 6, 
            self.palette.get(GreekPalette::deep_purple()), true);
        
        // Head
        AssetGenerator::draw_circle(buffer, center_x as i32, (center_y - 10) as i32, 4, 
            self.palette.get(GreekPalette::light_lavender()), true);
        
        // Dress
        AssetGenerator::draw_rect(buffer, center_x - 6, center_y - 6, 12, 16, 
            self.palette.get(GreekPalette::royal_purple()), true);
        
        // Peacock feather
        AssetGenerator::draw_line(buffer, (center_x + 8) as i32, (center_y - 8) as i32, 
            (center_x + 8) as i32, (center_y + 4) as i32, self.palette.get(GreekPalette::aegean_purple()));
        // Feather eye
        AssetGenerator::draw_circle(buffer, (center_x + 8) as i32, (center_y - 8) as i32, 2, 
            self.palette.get(GreekPalette::medium_purple()), true);
    }
    
    // Queen of Spades - Artemis with bow
    fn draw_artemis(&self, buffer: &mut RgbImage) {
        let center_x = CARD_WIDTH / 2;
        let center_y = CARD_HEIGHT / 2;
        
        // Hair
        AssetGenerator::draw_rect(buffer, center_x - 6, center_y - 14, 12, 8, 
            self.palette.get(GreekPalette::deep_purple()), true);
        
        // Head
        AssetGenerator::draw_circle(buffer, center_x as i32, (center_y - 10) as i32, 4, 
            self.palette.get(GreekPalette::light_lavender()), true);
        
        // Dress
        AssetGenerator::draw_rect(buffer, center_x - 6, center_y - 6, 12, 16, 
            self.palette.get(GreekPalette::royal_purple()), true);
        
        // Bow
        AssetGenerator::draw_line(buffer, (center_x - 10) as i32, (center_y - 8) as i32, 
            (center_x - 10) as i32, (center_y + 2) as i32, self.palette.get(GreekPalette::dark_goldenrod()));
        // Bow string
        AssetGenerator::draw_line(buffer, (center_x - 10) as i32, (center_y - 8) as i32, 
            (center_x - 8) as i32, (center_y - 3) as i32, self.palette.get(GreekPalette::purple_gray()));
        AssetGenerator::draw_line(buffer, (center_x - 8) as i32, (center_y - 3) as i32, 
            (center_x - 10) as i32, (center_y + 2) as i32, self.palette.get(GreekPalette::purple_gray()));
    }
    
    // King of Hearts - Zeus with lightning
    fn draw_zeus(&self, buffer: &mut RgbImage) {
        let center_x = CARD_WIDTH / 2;
        let center_y = CARD_HEIGHT / 2;
        
        // Crown
        AssetGenerator::draw_rect(buffer, center_x - 5, center_y - 16, 10, 4, 
            self.palette.get(GreekPalette::goldenrod()), true);
        
        // Hair and beard
        AssetGenerator::draw_rect(buffer, center_x - 6, center_y - 12, 12, 10, 
            self.palette.get(GreekPalette::light_lavender()), true);
        
        // Head
        AssetGenerator::draw_circle(buffer, center_x as i32, (center_y - 10) as i32, 4, 
            self.palette.get(GreekPalette::light_lavender()), true);
        
        // Robe
        AssetGenerator::draw_rect(buffer, center_x - 8, center_y - 4, 16, 18, 
            self.palette.get(GreekPalette::dusty_rose()), true);
        
        // Lightning bolt
        AssetGenerator::draw_line(buffer, (center_x + 10) as i32, (center_y - 10) as i32, 
            (center_x + 8) as i32, (center_y - 4) as i32, self.palette.get(GreekPalette::goldenrod()));
        AssetGenerator::draw_line(buffer, (center_x + 8) as i32, (center_y - 4) as i32, 
            (center_x + 10) as i32, (center_y - 2) as i32, self.palette.get(GreekPalette::goldenrod()));
        AssetGenerator::draw_line(buffer, (center_x + 10) as i32, (center_y - 2) as i32, 
            (center_x + 8) as i32, (center_y + 4) as i32, self.palette.get(GreekPalette::goldenrod()));
    }
    
    // King of Diamonds - Apollo with lyre
    fn draw_apollo(&self, buffer: &mut RgbImage) {
        let center_x = CARD_WIDTH / 2;
        let center_y = CARD_HEIGHT / 2;
        
        // Laurel crown
        AssetGenerator::draw_circle(buffer, center_x as i32, (center_y - 14) as i32, 6, 
            self.palette.get(GreekPalette::purple_gray()), false);
        
        // Hair
        AssetGenerator::draw_rect(buffer, center_x - 6, center_y - 12, 12, 6, 
            self.palette.get(GreekPalette::goldenrod()), true);
        
        // Head
        AssetGenerator::draw_circle(buffer, center_x as i32, (center_y - 10) as i32, 4, 
            self.palette.get(GreekPalette::light_lavender()), true);
        
        // Robe
        AssetGenerator::draw_rect(buffer, center_x - 8, center_y - 4, 16, 18, 
            self.palette.get(GreekPalette::dusty_rose()), true);
        
        // Lyre (U-shaped)
        AssetGenerator::draw_line(buffer, (center_x - 8) as i32, (center_y - 4) as i32, 
            (center_x - 8) as i32, (center_y + 4) as i32, self.palette.get(GreekPalette::dark_goldenrod()));
        AssetGenerator::draw_line(buffer, (center_x - 8) as i32, (center_y + 4) as i32, 
            (center_x - 4) as i32, (center_y + 4) as i32, self.palette.get(GreekPalette::dark_goldenrod()));
        AssetGenerator::draw_line(buffer, (center_x - 4) as i32, (center_y + 4) as i32, 
            (center_x - 4) as i32, (center_y - 4) as i32, self.palette.get(GreekPalette::dark_goldenrod()));
        // Lyre strings
        for i in 0..3 {
            AssetGenerator::draw_line(buffer, (center_x - 7 + i * 2) as i32, (center_y - 4) as i32, 
                (center_x - 7 + i * 2) as i32, (center_y + 3) as i32, self.palette.get(GreekPalette::goldenrod()));
        }
    }
    
    // King of Clubs - Poseidon with trident
    fn draw_poseidon(&self, buffer: &mut RgbImage) {
        let center_x = CARD_WIDTH / 2;
        let center_y = CARD_HEIGHT / 2;
        
        // Crown
        AssetGenerator::draw_rect(buffer, center_x - 5, center_y - 16, 10, 4, 
            self.palette.get(GreekPalette::aegean_purple()), true);
        
        // Hair and beard
        AssetGenerator::draw_rect(buffer, center_x - 6, center_y - 12, 12, 10, 
            self.palette.get(GreekPalette::purple_gray()), true);
        
        // Head
        AssetGenerator::draw_circle(buffer, center_x as i32, (center_y - 10) as i32, 4, 
            self.palette.get(GreekPalette::light_lavender()), true);
        
        // Robe
        AssetGenerator::draw_rect(buffer, center_x - 8, center_y - 4, 16, 18, 
            self.palette.get(GreekPalette::royal_purple()), true);
        
        // Trident
        AssetGenerator::draw_line(buffer, (center_x + 10) as i32, (center_y - 12) as i32, 
            (center_x + 10) as i32, (center_y + 6) as i32, self.palette.get(GreekPalette::dark_goldenrod()));
        // Trident prongs
        AssetGenerator::draw_line(buffer, (center_x + 8) as i32, (center_y - 10) as i32, 
            (center_x + 8) as i32, (center_y - 12) as i32, self.palette.get(GreekPalette::dark_goldenrod()));
        AssetGenerator::draw_line(buffer, (center_x + 12) as i32, (center_y - 10) as i32, 
            (center_x + 12) as i32, (center_y - 12) as i32, self.palette.get(GreekPalette::dark_goldenrod()));
    }
    
    // King of Spades - Hades with helm
    fn draw_hades(&self, buffer: &mut RgbImage) {
        let center_x = CARD_WIDTH / 2;
        let center_y = CARD_HEIGHT / 2;
        
        // Dark helmet
        AssetGenerator::draw_rect(buffer, center_x - 6, center_y - 16, 12, 8, 
            self.palette.get(GreekPalette::indigo()), true);
        
        // Face (partially visible)
        AssetGenerator::draw_rect(buffer, center_x - 4, center_y - 10, 8, 4, 
            self.palette.get(GreekPalette::purple_gray()), true);
        
        // Dark robe
        AssetGenerator::draw_rect(buffer, center_x - 8, center_y - 6, 16, 20, 
            self.palette.get(GreekPalette::byzantium()), true);
        
        // Cerberus head at feet (simplified)
        AssetGenerator::draw_circle(buffer, (center_x - 6) as i32, (center_y + 12) as i32, 2, 
            self.palette.get(GreekPalette::deep_purple()), true);
        AssetGenerator::draw_circle(buffer, center_x as i32, (center_y + 12) as i32, 2, 
            self.palette.get(GreekPalette::deep_purple()), true);
        AssetGenerator::draw_circle(buffer, (center_x + 6) as i32, (center_y + 12) as i32, 2, 
            self.palette.get(GreekPalette::deep_purple()), true);
    }
    
    pub fn generate_card_back(&self) -> RgbImage {
        let mut buffer = AssetGenerator::create_canvas(CARD_WIDTH, CARD_HEIGHT);
        
        // Fill with marble background
        AssetGenerator::fill(&mut buffer, self.palette.get(GreekPalette::marble()));
        
        // Draw Greek border
        PatternGenerator::draw_meander_border(&mut buffer, 3, self.palette.get(GreekPalette::gold()));
        
        // Draw column pattern in center
        let column = PatternGenerator::generate_column_pattern();
        let start_x = (CARD_WIDTH - 8) / 2;
        let start_y = (CARD_HEIGHT - 16) / 2;
        
        for (y, row) in column.iter().enumerate() {
            for (x, &pixel) in row.iter().enumerate() {
                if pixel == 1 {
                    let px = start_x + x as u32;
                    let py = start_y + y as u32;
                    if px < buffer.width() && py < buffer.height() {
                        buffer.put_pixel(px, py, self.palette.get(GreekPalette::bronze()));
                    }
                }
            }
        }
        
        // Add corner flourishes
        PatternGenerator::draw_corner_flourish(&mut buffer, 3, 3, 4, 
            self.palette.get(GreekPalette::gold()), Corner::TopLeft);
        PatternGenerator::draw_corner_flourish(&mut buffer, CARD_WIDTH - 7, 3, 4, 
            self.palette.get(GreekPalette::gold()), Corner::TopRight);
        PatternGenerator::draw_corner_flourish(&mut buffer, 3, CARD_HEIGHT - 7, 4, 
            self.palette.get(GreekPalette::gold()), Corner::BottomLeft);
        PatternGenerator::draw_corner_flourish(&mut buffer, CARD_WIDTH - 7, CARD_HEIGHT - 7, 4, 
            self.palette.get(GreekPalette::gold()), Corner::BottomRight);
        
        buffer
    }
}