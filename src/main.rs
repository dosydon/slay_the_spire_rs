use slay_the_spire::cli::BattleCli;

fn main() {
    println!("Welcome to Slay the Spire Battle Simulator!");
    println!("=========================================");
    
    let mut battle_cli = BattleCli::new();
    battle_cli.run();
    
    println!("\nThanks for playing!");
}
