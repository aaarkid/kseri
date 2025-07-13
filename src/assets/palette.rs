use image::{Rgb, Rgba};

#[derive(Clone, Debug)]
pub struct GreekPalette {
    colors: [Rgb<u8>; 16],
}

impl GreekPalette {
    pub fn new() -> Self {
        Self {
            colors: [
                Rgb([255, 255, 255]), // 0: White (background)
                Rgb([48, 25, 52]),    // 1: Deep Purple (outlines)
                Rgb([225, 215, 240]), // 2: Light Lavender
                Rgb([150, 123, 182]), // 3: Medium Lavender
                Rgb([219, 112, 147]), // 4: Dusty Rose (hearts/diamonds)
                Rgb([150, 75, 100]),  // 5: Deep Rose
                Rgb([102, 51, 153]),  // 6: Royal Purple (spades/clubs)
                Rgb([75, 0, 130]),    // 7: Indigo
                Rgb([218, 165, 32]),  // 8: Goldenrod (borders)
                Rgb([184, 134, 11]),  // 9: Dark Goldenrod
                Rgb([143, 124, 161]), // 10: Soft Purple
                Rgb([181, 126, 220]), // 11: Medium Purple
                Rgb([245, 240, 250]), // 12: Lavender Mist
                Rgb([147, 112, 219]), // 13: Medium Purple (Aegean inspired)
                Rgb([106, 90, 121]),  // 14: Purple Gray
                Rgb([104, 52, 108]),  // 15: Byzantium Purple
            ],
        }
    }

    pub fn get(&self, index: u8) -> Rgb<u8> {
        self.colors[(index & 0x0F) as usize]
    }

    pub fn get_rgba(&self, index: u8, alpha: u8) -> Rgba<u8> {
        let rgb = self.get(index);
        Rgba([rgb[0], rgb[1], rgb[2], alpha])
    }

    pub fn white() -> u8 { 0 }
    pub fn deep_purple() -> u8 { 1 }
    pub fn light_lavender() -> u8 { 2 }
    pub fn medium_lavender() -> u8 { 3 }
    pub fn dusty_rose() -> u8 { 4 }
    pub fn deep_rose() -> u8 { 5 }
    pub fn royal_purple() -> u8 { 6 }
    pub fn indigo() -> u8 { 7 }
    pub fn goldenrod() -> u8 { 8 }
    pub fn dark_goldenrod() -> u8 { 9 }
    pub fn soft_purple() -> u8 { 10 }
    pub fn medium_purple() -> u8 { 11 }
    pub fn lavender_mist() -> u8 { 12 }
    pub fn aegean_purple() -> u8 { 13 }
    pub fn purple_gray() -> u8 { 14 }
    pub fn byzantium() -> u8 { 15 }
    
    // Aliases for compatibility
    pub fn black() -> u8 { Self::deep_purple() }
    pub fn light_gray() -> u8 { Self::light_lavender() }
    pub fn dark_gray() -> u8 { Self::medium_lavender() }
    pub fn bright_red() -> u8 { Self::dusty_rose() }
    pub fn dark_red() -> u8 { Self::deep_rose() }
    pub fn bright_blue() -> u8 { Self::royal_purple() }
    pub fn dark_blue() -> u8 { Self::indigo() }
    pub fn gold() -> u8 { Self::goldenrod() }
    pub fn bronze() -> u8 { Self::dark_goldenrod() }
    pub fn marble() -> u8 { Self::lavender_mist() }
}

impl Default for GreekPalette {
    fn default() -> Self {
        Self::new()
    }
}