
#[derive(Clone)]

use crate::scene::Scene;
use crate::scene::Pointer;


pub type Group = usize;


pub struct Name([u8; 16]);

impl PartialEq for Name {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..16 { 
            if i < other.0.len() { 
                if self.0[i] != other.0[i] { return false; }
            } else {
                if self.0[i] != 0 { return false; } 
                else { return true; }
            }
        }
        true
    }
}

impl Default for Name {
    fn default() -> Self { Name([0; 16]) }
}


#[derive(Clone, Default)]
pub struct Spawn {
    pointer: Pointer,
    group: Group,
    name: Name,
}

impl Spawn {
    fn pointer(&self) -> &Pointer { &self.pointer }
    fn group(&self) -> &Group { &self.group }

    fn name(&self) -> &str { 
        std::str::from_utf8(&self.name.0).unwrap()
    }

    fn new_name(&mut self, name: &str) {
        let bytes = name.as_bytes();
        for i in 0..16 { 
            if i < name.len() { 
                self.name.0[i] = bytes[i]; 
            } else {
                self.name.0[i] = 0 
            } 
        }
    }
}

impl PartialEq for Spawn {
    fn eq(&self, other: &Spawn) -> bool {
        self.pointer == other.pointer
    }
}