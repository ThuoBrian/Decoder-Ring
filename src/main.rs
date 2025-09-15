use clap::Parser;
use decoder_ring::print_stats_analysis;

/// CLI tool to reverse engineer a Caesar cipher
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The message to decrypt
    #[arg(short, long)]
    message: String,

    //statistical information about the message
    #[arg(short, long)]
    stats: bool,

    //guess the shift
    #[arg(short, long)]
    guess: bool,
}

// run it
fn main() {
    let args = Args::parse();
    //stats
    if args.stats {
        print_stats_analysis(&args.message);
    }
    //guess
    if args.guess {
        let (depth, best_shift, decrypted, max_score) =
            decoder_ring::guess_shift(&args.message, 26);
        println!(
            "Best shift: {} (out of {}), score: {}",
            best_shift, depth, max_score
        );
        println!("\n Decrypted message: {} \n", decrypted);
    }
}
