use ansi::{abbrev::*, *};

use glium::glutin::event::ModifiersState;

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

fn control(
    color: &str,
    modifier: Option<ModifiersState>,
    key: &[&str],
    description: &str,
    revert: Option<ModifiersState>,
) -> String {
    let (mut key_string, mut len) = key_string(color, key);

    if let Some(modifier) = modifier {
        let (modifier_key_string, modifier_len) = modifier_key_string(modifier);
        key_string.insert_str(0, &modifier_key_string);
        len += modifier_len;
    }

    format!(
        "	{key_string}{D}{space}{description}{revert}",
        space = pad(len),
        revert = if let Some(modifier) = revert {
            revert_string(modifier)
        } else {
            String::new()
        },
    )
}

fn pad(len: usize) -> String {
    if PADDING < len {
        String::from(" ")
    } else {
        " ".repeat(PADDING - len)
    }
}

fn revert_string(modifier: ModifiersState) -> String {
    todo!("make a modifier to string function");
    format!("\t{I}({B}+ {Y}{modifier:?}{D} {I}to revert action){D}")
}

fn key_string(color: &str, key: &[&str]) -> (String, usize) {
    let mut key_string = String::new();
    key_string.push_str(&format!("{B}{color}"));
    for (i, k) in key.iter().enumerate() {
        key_string.push_str(k);
        if i != key.len() - 1 {
            key_string.push_str(&format!("{D}, {B}{color}"));
        }
    }

    (key_string, key.join(", ").chars().collect::<Vec<_>>().len())
}

fn modifier_key_string(modifier: ModifiersState) -> (String, usize) {
    let mut action = Vec::new();

    if modifier.ctrl() {
        action.push("ctrl");
    }
    if modifier.shift() {
        action.push("shift");
    }
    if modifier.alt() {
        action.push("alt");
    }
    if modifier.logo() {
        action.push("super");
    }

    let mut s = String::new();
    let mut len = 0;

    for a in action {
        s.push_str(&format!("{D}{B}{I}{a}{D} {B}+ "));
        len += a.chars().collect::<Vec<_>>().len() + 3;
    }

    (s, len)
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
",
        // {B}{OC}object{N_C}:
        // {obj_prev}
        // {obj_next}

        // {B}{Y}speed{N_C}:
        // {speed_inc}
        // {speed_dec}

        // {B}{BLUE}translation{N_C}:
        // {move_left}
        // {move_right}
        // {move_up}
        // {move_down}
        // {move_forward}
        // {move_backward}

        // {B}{M}rotation{N_C}:
        // {rotate_x}
        // {rotate_y}
        // {rotate_z}

        // {B}{R}c{Y}o{G}l{CYA}o{BLU}r{N_C}:
        // {color_change}

        // {B}{BG_BLACK}{G}texture{D}{B}:
        // {texture_on}
        // {texture_prev}
        // {texture_next}
        // {texture_change}
        // ",
        // quit = control(&format!("{I}{RED}"), &["esc"], "quit", &[]),
        // pause = control(&format!("{I}{Y}"), &["space"], "pause", &[]),
        // reverse = control(&format!("{I}{M}"), &["R"], "reverse", &[]),
        // fps_inc = control(
        //     BLU,
        //     &["↑"],
        //     "increase fps",
        //     &[(ModifiersState::CTRL, "decrease fps")]
        // ),
        // fps_dec = control(BLU, &["↓"], "decrease fps", &[]),
        // //
        // obj_prev = control(OC, &["←"], "previous object", &[]),
        // obj_next = control(OC, &["→"], "next object", &[]),
        // //
        // speed_inc = control(Y, &["+", "="], "increase speed", &[]),
        // speed_dec = control(Y, &["-"], "decrease speed", &[]),
        // //
        // move_left = control(XC, &["A"], "move left", &[]),
        // move_right = control(XC, &["D"], "move right", &[]),
        // move_up = control(YC, &["W"], "move up", &[]),
        // move_down = control(YC, &["S"], "move down", &[]),
        // move_forward = control(&format!("{I}{ZC}"), &["scroll ⤉"], "move forward", &[]),
        // move_backward = control(&format!("{I}{ZC}"), &["scroll ⤈"], "move backward", &[]),
        // //
        // rotate_x = control(XC, &["X"], "rotate around the X axis", &[]),
        // rotate_y = control(YC, &["Y"], "rotate around the Y axis", &[]),
        // rotate_z = control(ZC, &["Z"], "rotate around the Z axis", &[]),
        // //
        // color_change = control(R, &["C"], "change color pattern", &[]),
        // //
        // texture_on = control(
        //     &format!("{BG_BLACK}{G}"),
        //     &["T"],
        //     "toggle texture on/off",
        //     &[]
        // ),
        // texture_prev = control(&format!("{BG_BLACK}{G}"), &["5"], "previous texture", &[]),
        // texture_next = control(&format!("{BG_BLACK}{G}"), &["6"], "next texture", &[]),
        // texture_change = control(
        //     &format!("{BG_BLACK}{G}"),
        //     &["C"],
        //     "change texture pattern",
        //     &[]
        // ),
        quit = control(&format!("{I}{RED}"), None, &["esc"], "quit", None),
        pause = control(&format!("{I}{Y}"), None, &["space"], "pause", None),
        reverse = control(M, None, &["R"], "reverse", None),
        fps_inc = control(
            BLU,
            Some(ModifiersState::CTRL | ModifiersState::SHIFT),
            &["↑"],
            "increase fps",
            None,
        ),
        fps_dec = control(
            BLU,
            None,
            &["↓"],
            "decrease fps",
            Some(ModifiersState::CTRL)
        ),
        //
    );
}
