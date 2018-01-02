#[cfg(not(target_arch="wasm32"))]
extern crate glutin;

use input::{ButtonState};
use std::ops::Index;

#[derive(Copy)]
///A structure that stores each key's state
pub struct Keyboard {
    pub(crate) keys: [ButtonState; 256],
}

impl Keyboard {
    #[cfg(not(target_arch="wasm32"))]
    pub(crate) fn process_event(&mut self, event: &glutin::KeyboardInput) {
        if let Some(keycode) = event.virtual_keycode {
            let index = keycode as usize;
            let previous_state = self.keys[index];
            self.keys[index] = match event.state {
                glutin::ElementState::Pressed => {
                    if previous_state.is_down() {
                        ButtonState::Held
                    } else {
                        ButtonState::Pressed
                    }
                }
                glutin::ElementState::Released => {
                    if previous_state.is_down() {
                        ButtonState::Released
                    } else {
                        ButtonState::NotPressed
                    }
                }
            };
        }
    }

    pub(crate) fn clear_temporary_states(&mut self) {
        for index in 0..self.keys.len() {
            self.keys[index] = self.keys[index].clear_temporary();
        }
    }
}

impl Clone for Keyboard {
    fn clone(&self) -> Keyboard {
        *self
    }
}
//TODO: Create some way of indexing the keyboard that works the same on WASM and native
/*
#[cfg(not(target_arch="wasm32"))]
impl Index<glutin::Key> for Keyboard {
    type Output = ButtonState;

    fn index(&self, index: Key) -> &ButtonState {
        &self.keys[index as usize]
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypress() {
        let mut keyboard = Keyboard {
            keys: [ButtonState::NotPressed; 256]
        };
        keyboard.process_event(&glutin::KeyboardInput {
            scancode: 0,
            state: glutin::ElementState::Pressed,
            virtual_keycode: Some(Key::A),
            modifiers: glutin::ModifiersState::default()
        });
        assert_eq!(keyboard[Key::A], ButtonState::Pressed);
        keyboard.process_event(&glutin::KeyboardInput {
            scancode: 0,
            state: glutin::ElementState::Pressed,
            virtual_keycode: Some(Key::A),
            modifiers: glutin::ModifiersState::default()
        });
        assert_eq!(keyboard[Key::A], ButtonState::Held);
        keyboard.process_event(&glutin::KeyboardInput {
            scancode: 0,
            state: glutin::ElementState::Released,
            virtual_keycode: Some(Key::A),
            modifiers: glutin::ModifiersState::default()
        });
        assert_eq!(keyboard[Key::A], ButtonState::Released);
        keyboard.process_event(&glutin::KeyboardInput {
            scancode: 0,
            state: glutin::ElementState::Released,
            virtual_keycode: Some(Key::A),
            modifiers: glutin::ModifiersState::default()
        });
        assert_eq!(keyboard[Key::A], ButtonState::NotPressed);
    }

    #[test]
    fn clear_states() {
        let mut keyboard = Keyboard {
            keys: [ButtonState::NotPressed; 256].clone()
        };
        keyboard.process_event(&glutin::KeyboardInput {
            scancode: 0,
            state: glutin::ElementState::Pressed,
            virtual_keycode: Some(Key::A),
            modifiers: glutin::ModifiersState::default()
        });
        keyboard.clear_temporary_states();
        assert_eq!(keyboard[Key::A], ButtonState::Held);
        keyboard.process_event(&glutin::KeyboardInput {
            scancode: 0,
            state: glutin::ElementState::Released,
            virtual_keycode: Some(Key::A),
            modifiers: glutin::ModifiersState::default()
        });
        keyboard.clear_temporary_states();
        assert_eq!(keyboard[Key::A], ButtonState::NotPressed);
    }
}
