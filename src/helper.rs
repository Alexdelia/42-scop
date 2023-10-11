use ansi::{abbrev::*, *};

use glium::glutin::event::ModifiersState;

const OC: &str = "\x1b[1;38;2;0;255;183m";
const LC: &str = "\x1b[1;38;2;255;255;142m";

const XC: &str = "\x1b[1;38;2;0;128;255m";
const YC: &str = "\x1b[1;38;2;255;0;128m";
const ZC: &str = "\x1b[1;38;2;128;255;0m";

const PADDING: usize = 10;

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
        let (modifier_key_string, modifier_len) = modifier_key_string(color, modifier);
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
    if PADDING <= len {
        String::from(" ")
    } else {
        " ".repeat(PADDING - len)
    }
}

fn revert_string(modifier: ModifiersState) -> String {
    format!(
        "\t{I}( {B}+ {Y}{modifier}{D} {I}to revert action){D}",
        modifier = modifier_to_string(modifier).join("{N_C} + {Y}")
    )
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

fn modifier_to_string(modifier: ModifiersState) -> Vec<String> {
    let mut action = Vec::new();

    if modifier.ctrl() {
        action.push(String::from("ctrl"));
    }
    if modifier.shift() {
        action.push(String::from("shift"));
    }
    if modifier.alt() {
        action.push(String::from("alt"));
    }
    if modifier.logo() {
        action.push(String::from("super"));
    }

    action
}

fn modifier_key_string(color: &str, modifier: ModifiersState) -> (String, usize) {
    let mut s = String::new();
    let mut len = 0;

    for action in modifier_to_string(modifier) {
        s.push_str(&format!("{D}{B}{I}{F}{color}{action}{D} {B}+ "));
        len += action.chars().collect::<Vec<_>>().len() + 3;
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

{B}{Y}speed{N_C}:
{speed_inc}
{speed_dec}
{fps_inc}
{fps_dec}

{B}{OC}object{N_C}:
{obj_prev}
{obj_next}

{B}{BLU}traslation{N_C}:
{move_left}
{move_right}
{move_up}
{move_down}
{move_forward}
{move_backward}

{B}{M}rotation{N_C}:
{rotate_x_clockwise}
{rotate_x_counterclockwise}
{rotate_x_stop}
{rotate_y_clockwise}
{rotate_y_counterclockwise}
{rotate_y_stop}
{rotate_z_clockwise}
{rotate_z_counterclockwise}
{rotate_z_stop}

{B}{LC}light{N_C}:
{enlight}

{B}{R}c{Y}o{G}l{CYA}o{BLU}r{N_C}:
{color_change}

{B}{BG_BLACK}{G}texture{D}{B}:
{texture_on}
{texture_prev}
{texture_next}
{texture_change}
",
        // flow
        quit = control(&format!("{I}{RED}"), None, &["esc"], "quit", None),
        pause = control(&format!("{I}{Y}"), None, &["space"], "pause", None),
        reverse = control(M, None, &["R"], "reverse", None),
        // speed
        speed_inc = control(
            Y,
            None,
            &["+", "="],
            "increase speed",
            Some(ModifiersState::CTRL)
        ),
        speed_dec = control(
            Y,
            None,
            &["-"],
            "decrease speed",
            Some(ModifiersState::CTRL)
        ),
        fps_inc = control(
            BLU,
            Some(ModifiersState::ALT),
            &["↑"],
            "increase fps",
            Some(ModifiersState::CTRL)
        ),
        fps_dec = control(
            BLU,
            Some(ModifiersState::ALT),
            &["↓"],
            "decrease fps",
            Some(ModifiersState::CTRL)
        ),
        // object
        obj_prev = control(
            OC,
            None,
            &["←"],
            "previous object",
            Some(ModifiersState::CTRL)
        ),
        obj_next = control(OC, None, &["→"], "next object", Some(ModifiersState::CTRL)),
        // translation
        move_left = control(XC, None, &["A"], "move left", Some(ModifiersState::CTRL)),
        move_right = control(XC, None, &["D"], "move right", Some(ModifiersState::CTRL)),
        move_up = control(YC, None, &["W"], "move up", Some(ModifiersState::CTRL)),
        move_down = control(YC, None, &["S"], "move down", Some(ModifiersState::CTRL)),
        move_forward = control(
            &format!("{I}{ZC}"),
            None,
            &["scroll ⤉"],
            "move forward",
            Some(ModifiersState::CTRL)
        ),
        move_backward = control(
            &format!("{I}{ZC}"),
            None,
            &["scroll ⤈"],
            "move backward",
            Some(ModifiersState::CTRL)
        ),
        // rotation
        rotate_x_clockwise = control(XC, None, &["X"], "rotate clockwise around the X axis", None,),
        rotate_x_counterclockwise = control(
            XC,
            Some(ModifiersState::CTRL),
            &["X"],
            "rotate counterclockwise around the X axis",
            None,
        ),
        rotate_x_stop = control(
            XC,
            Some(ModifiersState::SHIFT),
            &["X"],
            "stop rotation around the X axis",
            None,
        ),
        rotate_y_clockwise = control(YC, None, &["Y"], "rotate clockwise around the Y axis", None,),
        rotate_y_counterclockwise = control(
            YC,
            Some(ModifiersState::CTRL),
            &["Y"],
            "rotate counterclockwise around the Y axis",
            None,
        ),
        rotate_y_stop = control(
            YC,
            Some(ModifiersState::SHIFT),
            &["Y"],
            "stop rotation around the Y axis",
            None,
        ),
        rotate_z_clockwise = control(ZC, None, &["Z"], "rotate clockwise around the Z axis", None,),
        rotate_z_counterclockwise = control(
            ZC,
            Some(ModifiersState::CTRL),
            &["Z"],
            "rotate counterclockwise around the Z axis",
            None,
        ),
        rotate_z_stop = control(
            ZC,
            Some(ModifiersState::SHIFT),
            &["Z"],
            "stop rotation around the Z axis",
            None,
        ),
        // light
        enlight = control(LC, None, &["E", "L"], "toggle enlightment on/off", None,),
        // color
        color_change = control(
            R,
            None,
            &["C"],
            "change color pattern",
            Some(ModifiersState::CTRL),
        ),
        // texture
        texture_on = control(
            &format!("{BG_BLACK}{G}"),
            None,
            &["T"],
            "toggle texture on/off",
            None,
        ),
        texture_prev = control(
            &format!("{BG_BLACK}{G}"),
            Some(ModifiersState::ALT),
            &["←"],
            "previous texture",
            Some(ModifiersState::CTRL)
        ),
        texture_next = control(
            &format!("{BG_BLACK}{G}"),
            Some(ModifiersState::ALT),
            &["→"],
            "next texture",
            Some(ModifiersState::CTRL)
        ),
        texture_change = control(
            &format!("{BG_BLACK}{G}"),
            None,
            &["C"],
            "change texture pattern",
            Some(ModifiersState::CTRL)
        ),
    );
}
