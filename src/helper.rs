use ansi::{abbrev::*, *};

const XC: &str = "\x1b[1;38;2;0;128;255m";
const YC: &str = "\x1b[1;38;2;255;0;128m";
const ZC: &str = "\x1b[1;38;2;128;255;0m";

pub fn help() {
    #[cfg(not(debug_assertions))]
    println!(
        "{B}{ASCII_COLORED_THE_ORDER}\n{CLEAR}{D}{DIM}{B}{ASCII_42}\t\t{D}{B}{COLOR_THE_ORDER}scop{D}"
    );

    println!(
        "
{B}{CYA}control{N_C}:

{B}{G}flow{N_C}:
	{I}{RED}esc\t{D}{R}quit
	{B}{I}{Y}space\t{D}{Y}pause

{B}{M}rotation{N_C}:
	{XC}X{D}\trotate around the {XC}X{D} axis
	{YC}Y{D}\trotate around the {YC}Y{D} axis
	{ZC}Z{D}\trotate around the {ZC}Z{D} axis

{B}{BLUE}translation{N_C}:
	{XC}A{D}, {XC}Q{D}\tmove {XC}left{D}
	{XC}D{D}\tmove {XC}right{D}
	{YC}W{D}, {YC}Z{D}\tmove {YC}up{D}
	{YC}S{D}\tmove {YC}down{D}
	{I}{ZC}scroll up{D}\tmove {ZC}forward{D}
	{I}{ZC}scroll down{D}\tmove {ZC}backward{D}

{B}{Y}speed{N_C}:
	{Y}+{D}\tincrease speed
	{Y}-{D}\tdecrease speed

{B}color:

{B}{RV}texture{N_R}:
	"
    );

    // speed
    // fps
    // color type
    // texture on off
    // texture type
    // position
    // pause
}
