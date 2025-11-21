use std::env;
use slay_the_spire::battle_cli::BattleCli;

fn main() {
    println!("Welcome to Slay the Spire Battle Simulator!");
    println!("=========================================");

    // Collect command line arguments (skip program name)
    let args: Vec<String> = env::args().skip(1).collect();

    let mut rng = rand::rng();
    let mut battle_cli = BattleCli::new_with_cards(args);
    battle_cli.run(&mut rng);

    println!("\nThanks for playing!");
}
