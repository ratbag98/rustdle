use clap::{Args, Parser};

use super::PuzzleType;

#[derive(Args, Debug)]
#[group(multiple = false)]
pub struct LetterSelection {
    /// User-specified grid of letters
    pub letters: Option<String>,

    /// Create a random square grid of x by x letters
    #[arg(short = 'x', long)]
    square: Option<u8>,

    /// download puzzle, either standard or express
    #[arg(value_enum, short = 'w', long, default_value_t = PuzzleType::Standard)]
    download: PuzzleType,
}

#[derive(Parser, Debug)]
#[command(author = "Rob Rainthorpe", version, name = "Rustdle", about = "Solve Squaredles with the power of Rust!", long_about = None)]
pub struct RustdleArgs {
    #[command(flatten)]
    pub letter_selection: LetterSelection,

    /// display results as a single column
    #[arg(short = 'c', long)]
    single_column: bool,

    /// display letters grid
    #[arg(short, long)]
    grid: bool,

    /// group solutions by length
    #[arg(short, long)]
    length: bool,

    /// display headers for length-grouped solution lists
    #[arg(short = 'H')]
    headers: bool,

    /// show all solutions for a word in GUI
    #[arg(short, long)]
    multiple: bool,

    /// randomise letter order, maybe useful for setting puzzles
    #[arg(short, long)]
    random: bool,

    /// sort solutions alphabetically
    #[arg(short, long, default_value_t = true)]
    sort: bool,

    /// run in GUI mode
    #[arg(short = 'u')]
    gui: bool,

    /// add extra letters to ensure puzzle is square
    #[arg(short = 't', long = "auto-extend")]
    auto_extend: bool,

    /// debug mode, mainly shows neighbour list
    #[arg(short, long)]
    debug: bool,

    /// specify custom word list
    #[arg(short, long)]
    file: Option<String>,

    /// show progress of trie walk
    #[arg(short = 'z', long = "slow-mode")]
    slow_mode: bool,
}
