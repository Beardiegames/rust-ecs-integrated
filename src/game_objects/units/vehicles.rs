
use crate::game_objects::{ GameObject, Weapons };

pub trait Truck {
    fn movement(&mut self);
    fn take_damage(&mut self, from_weapon: &Weapons);
}

impl Truck for GameObject {

    fn movement(&mut self) { self.position.x += 2; }

    fn take_damage(&mut self, from_weapon: &Weapons) {
        self.health -= match from_weapon {
            Weapons::AwefulPuns(_hp)    => 0.0,
            Weapons::SmartRemarks(_hp)  => 0.0,
            Weapons::Fists(_hp)         => 0.0,
            Weapons::Handgun(hp)        => hp * 1.0,
            Weapons::Rifle(hp)          => hp * 1.0,
            Weapons::Canon(hp)          => hp * 2.0,
        }
    }
}