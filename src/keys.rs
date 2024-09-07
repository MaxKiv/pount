use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct KeyMap(pub HashMap<String, KeyCode>);

impl KeyMap {
    fn new() -> Self {
        KeyMap(HashMap::<String, KeyCode>::new())
    }
}

impl Default for KeyMap {
    fn default() -> Self {
        let mut keymap = KeyMap::new();
        keymap.0.insert("quit".to_string(), KeyCode::Escape);
        keymap.0.insert("restart".to_string(), KeyCode::Enter);
        keymap.0.insert("undo".to_string(), KeyCode::KeyU);
        keymap.0.insert("spawn".to_string(), KeyCode::Space);
        keymap
    }
}

pub fn print_keymap(keymap: Res<KeyMap>) {
    if keymap.is_changed() || keymap.is_added() {
        info!("Keymap:");
        for (description, keycode) in keymap.0.iter() {
            info!("{:?} -> {}", keycode, description);
        }
    }
}
