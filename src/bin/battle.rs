use clap::Parser;
use slay_the_spire::battle_cli::BattleCli;

#[derive(Parser)]
#[command(name = "Slay the Spire Battle")]
#[command(about = "Battle simulator for Slay the Spire card game")]
#[command(version)]
struct Args {
    /// Cards to add to the top of the starting deck
    ///
    /// Specify card names to place on top of the deck before the battle starts.
    /// Cards will be drawn in the order specified.
    ///
    /// Examples:
    ///   cargo run --bin battle -- armaments strike whirlwind
    ///   cargo run --bin battle -- bash defend defend
    #[arg(value_name = "CARDS")]
    cards: Vec<String>,
}

fn main() {
    let args = Args::parse();

    println!("⚔️  Slay the Spire - Battle Simulator");
    println!("=====================================");

    let mut rng = rand::rng();
    let mut battle_cli = BattleCli::new_with_cards(args.cards);
    battle_cli.run(&mut rng);
}