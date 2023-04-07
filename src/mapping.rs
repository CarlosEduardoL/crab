use lazy_static::lazy_static;
use crate::args::ARGS;

/**
This function returns a reference to a static byte slice representing the tab character.

The tab character can be displayed as a visible tab (^I) or a regular tab (\t), depending
on the value of the show_tabs field of the ARGS global variable.

The TAB constant holds the byte slice representing a regular tab.

The VISIBLE_TAB constant holds the byte slice representing a visible tab (^I).

The FINAL_TAB lazy static variable holds a reference to the byte slice that will be returned by the function.

If show_tabs is true, the visible tab byte slice is returned. Otherwise, the regular tab byte slice is returned.
 */
pub fn tab() -> &'static [u8] {
    const TAB: &[u8; 1] = b"\t";
    const VISIBLE_TAB: &[u8; 2] = b"^I";
    lazy_static! {
        static ref FINAL_TAB: &'static [u8] = if ARGS.show_tabs {
            VISIBLE_TAB
        } else {
            TAB
        };
    }
    &FINAL_TAB
}

/**
This function returns a static reference to a byte slice representing a newline character.
The returned byte slice is either a visible newline character or a regular newline character,
depending on the value of the ARGS.show_ends boolean value.
## Returns
A reference to a static byte slice representing a newline character.
 */
pub fn new_line() -> &'static [u8] {
    const NEW_LINE: &[u8; 1] = b"\n";
    const VISIBLE_NEW_LINE: &[u8; 2] = b"$\n";
    lazy_static! {
        static ref FINAL_NEW_LINE: &'static [u8] = if ARGS.show_ends {
            VISIBLE_NEW_LINE
        } else {
            NEW_LINE
        };
    }
    &FINAL_NEW_LINE
}
