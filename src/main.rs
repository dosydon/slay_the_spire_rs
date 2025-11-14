use slay_the_spire::battle_cli::BattleCli;

fn main() {
    println!("Welcome to Slay the Spire Battle Simulator!");
    println!("=========================================");
    
    let mut rng = rand::rng();
    let mut battle_cli = BattleCli::new();
    battle_cli.run(&mut rng);
    
    println!("\nThanks for playing!");
}
