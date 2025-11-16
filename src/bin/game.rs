use clap::Parser;
use slay_the_spire::game_cli::GameCli;

#[derive(Parser)]
#[command(name = "Slay the Spire")]
#[command(about = "A Rust implementation of Slay the Spire card game")]
struct Args {
    /// Use implemented cards test deck instead of starter deck
    #[arg(short, long)]
    test_deck: bool,
}

fn main() {
    let args = Args::parse();

    println!("🎮 Slay the Spire - Full Game");
    println!("==============================");

    let mut game_cli = GameCli::new_with_deck_choice(args.test_deck);
    game_cli.run();
}