use kseri::assets::AssetGenerator;

fn main() {
    let generator = AssetGenerator::new();
    
    match generator.generate_all_assets() {
        Ok(_) => {
            println!("\n✨ All Greek-themed pixel art assets with lavender palette generated successfully!");
            println!("Assets saved to:");
            println!("  - assets/cards/individual/ (52 cards + card back)");
            println!("  - assets/ui/ (buttons, frames, indicators)");
            println!("  - assets/textures/ (table texture)");
        }
        Err(e) => {
            eprintln!("❌ Error generating assets: {}", e);
            std::process::exit(1);
        }
    }
}