use clap::Parser;

#[derive(Parser)]
#[clap(name = "crab 🦀")]
#[clap(version)]
#[clap(author = "CarlosEduardoL")]
#[clap(about = "Rust cat copy", long_about = None)]
struct CrabArgs {
    #[clap(value_name = "FILES")]
    /// files to be concatenated, when FILE is -, read standard input.
    files: Vec<String>,
    #[clap(short, long = "number")]
    /// add A number on the line start
    number_lines: bool,
    #[clap(short = 'b', long = "number-nonblank")]
    /// number nonempty output lines, overrides -n
    number_non_blank: bool,
    #[clap(short = 'A', long = "show-all")]
    /// equivalent to -vET
    show_all: bool,
    #[clap(short)]
    /// equivalent to -vE
    ev: bool,
    #[clap(short)]
    /// equivalent to -vT
    t: bool,
    #[clap(short = 'E', long = "show-ends")]
    /// show end of lines with $
    show_ends: bool,
    #[clap(short = 'T', long = "show-tabs")]
    /// display TAB characters as ^I
    show_tabs: bool,
    #[clap(short, long = "squeeze-blank")]
    /// suppress repeated empty output lines
    squeeze_blank: bool,
    #[clap(short = 'v', long = "show-nonprinting")]
    /// use ^ and M- notation, except for LFD and TAB
    show_non_printing: bool,
}

pub struct Args {
    pub show_non_printing: bool,
    pub show_ends: bool,
    pub show_tabs: bool,
    pub number_lines: bool,
    pub squeeze_blank: bool,
    pub number_non_blank: bool,
}

pub fn args() -> (Vec<String>, Args) {
    let mut args = CrabArgs::parse();
    args.show_non_printing |= args.ev || args.show_all || args.t;
    args.show_ends |= args.ev || args.show_all;
    args.show_tabs |= args.show_all || args.t;
    args.number_lines |= args.number_non_blank;
    (
        args.files,
        Args {
            show_non_printing: args.show_non_printing,
            show_ends: args.show_ends,
            show_tabs: args.show_tabs,
            number_lines: args.number_lines,
            squeeze_blank: args.squeeze_blank,
            number_non_blank: args.number_non_blank,
        },
    )
}
