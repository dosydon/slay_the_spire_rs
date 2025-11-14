use slay_the_spire::game_cli::GameCli;

fn main() {
    println!("🎮 Slay the Spire - Full Game");
    println!("==============================");
    
    let mut game_cli = GameCli::new();
    game_cli.run();
}