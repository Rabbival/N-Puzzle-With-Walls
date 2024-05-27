use crate::prelude::KeyCode;

fn try_get_string_from_keycode(keycode: KeyCode) -> Option<String> {
    //TODO: get the debug way and remove Key or Digit or Numpad
    // if special, convert to ( or ) or _ 
}

fn is_letter(keycode: KeyCode) -> bool{
    keycode == KeyCode::KeyQ || keycode == KeyCode::KeyW || keycode == KeyCode::KeyE || keycode == KeyCode::KeyR ||
    keycode == KeyCode::KeyT || keycode == KeyCode::KeyY || keycode == KeyCode::KeyU || keycode == KeyCode::KeyI ||
    keycode == KeyCode::KeyO || keycode == KeyCode::KeyP || keycode == KeyCode::KeyA || keycode == KeyCode::KeyS ||
    keycode == KeyCode::KeyD || keycode == KeyCode::KeyF || keycode == KeyCode::KeyG || keycode == KeyCode::KeyH ||
    keycode == KeyCode::KeyJ || keycode == KeyCode::KeyK || keycode == KeyCode::KeyL || keycode == KeyCode::KeyZ ||
    keycode == KeyCode::KeyX || keycode == KeyCode::KeyC || keycode == KeyCode::KeyV || keycode == KeyCode::KeyB ||
    keycode == KeyCode::KeyN || keycode == KeyCode::KeyM
}

fn is_digit(keycode: KeyCode) -> bool{
    keycode == KeyCode::Digit0 || keycode == KeyCode::Digit1 || keycode == KeyCode::Digit2 ||
    keycode == KeyCode::Digit3 || keycode == KeyCode::Digit4 || keycode == KeyCode::Digit5 ||
    keycode == KeyCode::Digit6 || keycode == KeyCode::Digit7 || keycode == KeyCode::Digit8 ||
    keycode == KeyCode::Digit9
}

fn is_numpad_number(keycode: KeyCode) -> bool{
    keycode == KeyCode::Numpad0 || keycode == KeyCode::Numpad1 || keycode == KeyCode::Numpad2 ||
    keycode == KeyCode::Numpad3 || keycode == KeyCode::Numpad4 || keycode == KeyCode::Numpad5 ||
    keycode == KeyCode::Numpad6 || keycode == KeyCode::Numpad7 || keycode == KeyCode::Numpad8 ||
    keycode == KeyCode::Numpad9
}

fn is_allowed_special_sign(keycode: KeyCode) -> bool{
    keycode == KeyCode::BracketLeft || keycode == KeyCode::BracketRight ||
    //TODO: find the keycode for    _
}