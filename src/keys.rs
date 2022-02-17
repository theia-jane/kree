use std::{os::raw::c_uint};
use x11::xlib;
use x11::keysym::*;
use std::ffi::CString;
use serde_yaml::Value;

pub type ModMaskCode = c_uint;
pub type KeyCode = c_uint;

#[derive(Debug, Clone)]
pub enum Command {
    Spawn(Value),
    Mapping(Value),
    Noop()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct KeyCombo {
    pub mods: ModMaskCode,
    pub key: KeyCode,
    pub event: Event,
}

impl KeyCombo {
    pub fn new(event: Event, mods: &[Mod], key: KeyCode) -> Self {
        let mods = mods.iter().fold(0, |acc, it| acc | **it);
        let key = key as KeyCode;
        Self { mods, key, event }
    }

}

pub fn get_keycode_from_string(key: String) -> c_uint {
    let key = key.to_lowercase();
    unsafe {
        match CString::new(key.as_bytes()) {
            Ok(b) => xlib::XStringToKeysym(b.as_ptr()) as c_uint,
            _ => panic!("Invalid key string!"),
        }
    }
}

pub fn parse_key_chord(key_chord: String) -> (c_uint, Vec<Mod>) {
    let split_chord = key_chord.split("+").collect::<Vec<&str>>();

    println!("Key chord: {:?}", key_chord);

    let mut modifiers: Vec<Mod> = vec![];
    let mut key: c_uint = 0;
    for part in split_chord {
        let part = part.to_lowercase();

        if part == "shift" {
            modifiers.push(Mod::Shift);
        }
        else if part == "control" || part == "ctrl" {
            modifiers.push(Mod::Control);
        }
        else if part == "super" || part == "command" || part == "cmd" || part == "win" {
            modifiers.push(Mod::Super);
        }
        else if part == "hyper" {
            modifiers.push(Mod::Hyper);
        }
        else if part == "alt" {
            modifiers.push(Mod::Alt);
        }
        else if part == "caps" || part == "capslock"  {
            modifiers.push(Mod::Caps);
        }
        else if part == "space" || part == "spc"  {
            key = get_keycode_from_string("space".to_string());
        }
        else if part == "enter" || part == "return" {
            key = XK_Return;
        }
        else {
            key = get_keycode_from_string(part);
        }
    }


    (key, modifiers)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Mod {
    Shift,
    Caps,
    Control,
    Alt,
    Super,
    Hyper,
}

impl std::ops::Deref for Mod {
    type Target = ModMaskCode;
    fn deref(&self) -> &ModMaskCode {
        use Mod::*;
        match *self {
            Shift => &xcb::MOD_MASK_SHIFT,
            Caps => &xcb::MOD_MASK_LOCK,
            Control => &xcb::MOD_MASK_CONTROL,
            Alt => &xcb::MOD_MASK_1,
            Super => &xcb::MOD_MASK_4,
            Hyper => &xcb::MOD_MASK_3,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    KeyDown,
    KeyUp,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_key_chord() {
        let (key, mods) = parse_key_chord(String::from("Shift+a"));
        assert_eq!(key, 97);
        assert_eq!(mods, vec![Mod::Shift]);

        // Order currently matters / is preserved
        let (key, mods) = parse_key_chord(String::from("Super+Shift+x"));
        assert_eq!(key, 120);
        assert_eq!(mods, vec![Mod::Super, Mod::Shift]);
        assert_ne!(mods, vec![Mod::Shift, Mod::Super]);

        // Case of modifiers should not matter
        let (_, mods) = parse_key_chord(String::from("SHIFT+a"));
        assert_eq!(mods, vec![Mod::Shift]);
    }

    #[test]
    fn test_creating_key_combo() {
        let key_combo = KeyCombo::new(Event::KeyUp, &vec![Mod::Shift, Mod::Super], 97);
        assert_eq!(key_combo.mods, 65 as ModMaskCode);
        assert_eq!(key_combo.key, 97 as KeyCode);
        assert_eq!(key_combo.event, Event::KeyUp);
    }

}
