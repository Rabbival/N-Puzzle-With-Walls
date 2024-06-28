use crate::prelude::KeyCode;

pub fn try_get_string_from_keycode(keycode: KeyCode, shift_pressed: bool) -> Option<String> {
    if is_letter(keycode){
        Some(letter_string(keycode, shift_pressed))
    }else if is_digit(keycode){
        digit_string(keycode, shift_pressed)
    }else if is_numpad_number(keycode){
        Some(String::from(&format!("{:?}", keycode)[6..]))
    }else if is_allowed_special_sign(keycode){
        Some(allowed_special_sign_string(keycode))
    }else{
        None
    }
}

fn letter_string(keycode: KeyCode, shift_pressed: bool) -> String{
    let letter_str = &format!("{:?}", keycode)[3..];
    if !shift_pressed {
        letter_str.to_lowercase()
    }else{
        String::from(letter_str)
    }
}

fn digit_string(keycode: KeyCode, shift_pressed: bool) -> Option<String> {
    if shift_pressed {
        if keycode == KeyCode::Digit9 {
            Some(String::from("("))
        }else if keycode == KeyCode::Digit0{
            Some(String::from(")"))
        }else{
            None
        }
    }else{
        Some(String::from(&format!("{:?}", keycode)[5..]))
    }
}

fn allowed_special_sign_string(keycode: KeyCode) -> String{
    match keycode{
        KeyCode::BracketLeft => String::from("("),
        KeyCode::BracketRight => String::from(")"),
        KeyCode::Minus => String::from("-"),
        KeyCode::NumpadSubtract => String::from("-"),
        KeyCode::Space => String::from(" "),
        _ => String::default()
    }
}

pub fn is_letter(keycode: KeyCode) -> bool{
    keycode == KeyCode::KeyQ || keycode == KeyCode::KeyW || keycode == KeyCode::KeyE || keycode == KeyCode::KeyR ||
    keycode == KeyCode::KeyT || keycode == KeyCode::KeyY || keycode == KeyCode::KeyU || keycode == KeyCode::KeyI ||
    keycode == KeyCode::KeyO || keycode == KeyCode::KeyP || keycode == KeyCode::KeyA || keycode == KeyCode::KeyS ||
    keycode == KeyCode::KeyD || keycode == KeyCode::KeyF || keycode == KeyCode::KeyG || keycode == KeyCode::KeyH ||
    keycode == KeyCode::KeyJ || keycode == KeyCode::KeyK || keycode == KeyCode::KeyL || keycode == KeyCode::KeyZ ||
    keycode == KeyCode::KeyX || keycode == KeyCode::KeyC || keycode == KeyCode::KeyV || keycode == KeyCode::KeyB ||
    keycode == KeyCode::KeyN || keycode == KeyCode::KeyM
}

pub fn is_digit(keycode: KeyCode) -> bool{
    keycode == KeyCode::Digit0 || keycode == KeyCode::Digit1 || keycode == KeyCode::Digit2 ||
    keycode == KeyCode::Digit3 || keycode == KeyCode::Digit4 || keycode == KeyCode::Digit5 ||
    keycode == KeyCode::Digit6 || keycode == KeyCode::Digit7 || keycode == KeyCode::Digit8 ||
    keycode == KeyCode::Digit9
}

pub fn is_numpad_number(keycode: KeyCode) -> bool{
    keycode == KeyCode::Numpad0 || keycode == KeyCode::Numpad1 || keycode == KeyCode::Numpad2 ||
    keycode == KeyCode::Numpad3 || keycode == KeyCode::Numpad4 || keycode == KeyCode::Numpad5 ||
    keycode == KeyCode::Numpad6 || keycode == KeyCode::Numpad7 || keycode == KeyCode::Numpad8 ||
    keycode == KeyCode::Numpad9
}

pub fn is_allowed_special_sign(keycode: KeyCode) -> bool{
    keycode == KeyCode::BracketLeft || keycode == KeyCode::BracketRight ||
    keycode == KeyCode::Minus || keycode == KeyCode::NumpadSubtract || keycode == KeyCode::Space
}