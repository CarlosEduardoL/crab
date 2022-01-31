use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name="crab 🦀")]
#[clap(version="v0.1.0")]
#[clap(author = "CarlosEduardoL")]
#[clap(about = "Rust cat copy", long_about = None)]
pub struct CrabArgs {
    #[clap(value_name = "FILES")]
    /// files to be concatenated, when FILE is -, read standard input.
    pub files: Vec<String>,
    #[clap(short, long="number")]
    /// add A number on the line start
    pub number_lines: bool,
    #[clap(short='b', long="number-nonblank")]
    /// number nonempty output lines, overrides -n
    pub number_non_blank: bool,
    #[clap(short='A', long="show-all")]
    /// equivalent to -vET
    pub show_all: bool,
    #[clap(short)]
    /// equivalent to -vE
    pub ev: bool,
    #[clap(short)]
    /// equivalent to -vT
    pub t: bool,
    #[clap(short='E', long="show-ends")]
    /// show end of lines with $
    pub show_ends: bool,
    #[clap(short='T', long="show-tabs")]
    /// display TAB characters as ^I
    pub show_tabs: bool,
    #[clap(short, long="squeeze-blank")]
    /// suppress repeated empty output lines
    pub squeeze_blank: bool,
    #[clap(short='v', long="show-nonprinting")]
    /// use ^ and M- notation, except for LFD and TAB
    pub show_non_printing: bool
}
