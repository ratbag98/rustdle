use clap::{Args, Parser};
// use regex::Regex;

#[derive(Args, Debug)]
#[group(multiple = false)]
/// ArgGroup to ensure at least one kind of letter selection is made by the user
pub struct LetterSelection {
    /// User-specified grid of letters.
    ///
    /// LETTERS can be 3 or more of A-Z or _ (ie no spaces). A '_' represents
    /// a gap in the puzzle. Consider this example puzzle:
    ///
    ///   BC
    ///  DEFG
    ///  HIJK
    ///   LM
    ///
    /// The following command would solve the puzzle:
    ///
    /// rustdle _BC_DEFGHIJK_LM_
    ///
    /// Since all grids are square, the number of letters in LETTERS must
    /// be a square number (eg3x3 = 9, 4x4 = 16 etc)
    ///
    ///
    /// If no letters are provided, today's Standard puzzle will be
    /// downloaded.
    ///
    /// Specify --express to download today's Express puzzle instead
    #[arg(value_name = "LETTERS", verbatim_doc_comment)]
    //, value_parser = just_squaredle_letters)]
    pub letters: Option<String>,

    /// Create a random square grid of X by X letters
    ///
    /// Letter distribution is inspired by a popular word game whose name rhymes with "scrabble".
    #[arg(short = 'x', long, value_name = "X")]
    square: Option<u8>,

    /// download express rather than standard puzzle
    #[arg(short, long)]
    express: bool,
}
// fn just_squaredle_letters(arg: &str) -> Result<&str, String> {
//     let acceptable = Regex::new(r"^[_A-Z]+$").unwrap();
//
//     if acceptable.is_match(arg) {
//         Ok(arg)
//     } else {
//         Err(String::from("invalid characters in letters"))
//     }
// }

#[derive(Parser, Debug)]
#[command(author, version, name = "Rustdle")]
/// Rustdle uses the power of Rust to solve Squaredles
pub struct RustdleArgs {
    #[command(flatten)]
    /// letter selection group
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
    #[arg(short = 'H', long, requires = "length")]
    headers: bool,

    /// run in GUI mode
    #[arg(short = 'u', long)]
    gui: bool,

    /// show all solutions for a word in GUI
    #[arg(short, long, requires = "gui")]
    multiple: bool,

    /// randomise letter order, maybe useful for setting puzzles
    #[arg(short, long, requires = "letters")]
    random: bool,

    /// sort solutions alphabetically
    #[arg(short, long, default_value_t = true)]
    sort: bool,

    /// add extra letters to LETTERS to ensure puzzle is square
    #[arg(short = 't', long = "auto-extend", requires = "letters")]
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
