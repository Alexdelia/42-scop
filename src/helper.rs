use ansi::{abbrev::*, *};

const OC: &str = "\x1b[1;38;2;0;255;183m";

const XC: &str = "\x1b[1;38;2;0;128;255m";
const YC: &str = "\x1b[1;38;2;255;0;128m";
const ZC: &str = "\x1b[1;38;2;128;255;0m";

const PADDING: usize = 9;

#[cfg(not(debug_assertions))]
pub fn header() {
    println!(
        "{B}{ASCII_COLORED_THE_ORDER}\n{CLEAR}{D}{DIM}{B}{ASCII_42}\t\t{D}{B}{COLOR_THE_ORDER}scop{D}"
    );
}

fn control(color: &str, key: &[&str], description: &str) -> String {
    let mut len = key.join(", ").chars().collect::<Vec<_>>().len();
    if len < PADDING {
        len = PADDING - len;
    } else {
        len = 1;
    }

    let mut key_string = String::new();
    key_string.push_str(B);
    key_string.push_str(color);
    for (i, k) in key.iter().enumerate() {
        key_string.push_str(k);
        if i != key.len() - 1 {
            key_string.push_str(&format!("{D}, {B}{color}"));
        }
    }

    format!(
        "{key_string}{D}{space}{description}",
        space = " ".repeat(len),
    )
}

pub fn help() {
    println!(
        "
{B}{CYA}control{N_C}:

{B}{G}flow{N_C}:
	{quit}
	{pause}
	{reverse}
	{fps_inc}
	{fps_dec}

{B}{OC}object{N_C}:
	{obj_prev}
	{obj_next}

{B}{Y}speed{N_C}:
	{speed_inc}
	{speed_dec}
	
{B}{BLUE}translation{N_C}:
	{move_left}
	{move_right}
	{move_up}
	{move_down}
	{move_forward}
	{move_backward}

{B}{M}rotation{N_C}:
	{rotate_x}
	{rotate_y}
	{rotate_z}

{B}{R}c{Y}o{G}l{CYA}o{BLU}r{N_C}:
	{color_change}

{B}{BG_BLACK}{G}texture{D}{B}:
	{texture_on}
	{texture_prev}
	{texture_next}
	{texture_change}
",
        quit = control(&format!("{I}{RED}"), &["esc"], "quit"),
        pause = control(&format!("{I}{Y}"), &["space"], "pause"),
        reverse = control(&format!("{I}{M}"), &["R"], "reverse"),
        fps_inc = control(BLU, &["↑"], "increase fps"),
        fps_dec = control(BLU, &["↓"], "decrease fps"),
        //
        obj_prev = control(OC, &["←"], "previous object"),
        obj_next = control(OC, &["→"], "next object"),
        //
        speed_inc = control(Y, &["+", "="], "increase speed"),
        speed_dec = control(Y, &["-"], "decrease speed"),
        //
        move_left = control(XC, &["A"], "move left"),
        move_right = control(XC, &["D"], "move right"),
        move_up = control(YC, &["W"], "move up"),
        move_down = control(YC, &["S"], "move down"),
        move_forward = control(&format!("{I}{ZC}"), &["scroll ⤉"], "move forward"),
        move_backward = control(&format!("{I}{ZC}"), &["scroll ⤈"], "move backward"),
        //
        rotate_x = control(XC, &["X"], "rotate around the X axis"),
        rotate_y = control(YC, &["Y"], "rotate around the Y axis"),
        rotate_z = control(ZC, &["Z"], "rotate around the Z axis"),
        //
        color_change = control(R, &["C"], "change color pattern"),
        //
        texture_on = control(&format!("{BG_BLACK}{G}"), &["T"], "toggle texture on/off"),
        texture_prev = control(&format!("{BG_BLACK}{G}"), &["5"], "previous texture"),
        texture_next = control(&format!("{BG_BLACK}{G}"), &["6"], "next texture"),
        texture_change = control(&format!("{BG_BLACK}{G}"), &["C"], "change texture pattern"),
    );
}
