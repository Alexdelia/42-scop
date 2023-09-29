use ansi::{abbrev::*, *};

pub fn help() {
    #[cfg(not(debug_assertions))]
    println!(
        "{B}{ASCII_COLORED_THE_ORDER}\n{CLEAR}{D}{DIM}{B}{ASCII_42}\t\t{D}{B}{COLOR_THE_ORDER}scop{D}"
    );

    println!(
        "{B}{CYA}control{N_C}:
{RED}ESC\t\t{D}{R}quit

{M}rotation{N_C}:
	{M}X\t\t{D}{M}rotate{D} around the {B}{M}X{D} axis
	{M}Y\t\t{D}{M}rotate{D} around the {B}{M}Y{D} axis
	{M}Z\t\t{D}{M}rotate{D} around the {B}{M}Z{D} axis
"
    );
}
