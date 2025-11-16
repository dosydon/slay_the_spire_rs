use clap::Parser;
use slay_the_spire::battle_cli::BattleCli;

#[derive(Parser)]
#[command(name = "Slay the Spire Battle")]
#[command(about = "Battle simulator for Slay the Spire card game")]
struct Args {
    /// Use implemented cards test deck instead of starter deck
    #[arg(short, long)]
    test_deck: bool,
}

fn main() {
    let args = Args::parse();

    println!("⚔️  Slay the Spire - Battle Simulator");
    println!("=====================================");

    let mut rng = rand::rng();
    let mut battle_cli = BattleCli::new_with_deck_choice(args.test_deck);
    battle_cli.run(&mut rng);
}