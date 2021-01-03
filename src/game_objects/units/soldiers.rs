
use crate::ecs::{ Scene };
use crate::game_objects::{ GameObject, Weapons, Factions };

pub trait Soldier {
    fn convert(&mut self) -> &mut Self;
    fn movement(&mut self);
    fn attack(&mut self, scene: &mut Scene<GameObject>);
    fn take_damage(&mut self, from_weapon: &Weapons);
}

impl Soldier for GameObject {

    fn convert(&mut self) -> &mut Self { &mut self }

    fn movement(&mut self) {
        self.position.x += 1;
    }

    fn attack(&mut self, s: &mut Scene<GameObject>) {
        if self.target.is_none() {
            self.target = s.find(|x| { 
                x.faction != Factions::None
                && x.faction != self.faction
                && self.position.x - x.position.x < 10 
            });
        }
    }

    fn take_damage(&mut self, from_weapon: &Weapons) {
        self.health -= match from_weapon.clone() {
            Weapons::AwefulPuns(hp)     => hp,
            Weapons::SmartRemarks(hp)   => hp,
            Weapons::Fists(hp)          => hp,
            Weapons::Handgun(hp)        => hp * 2.0,
            Weapons::Rifle(hp)          => hp * 2.0,
            Weapons::Canon(hp)          => hp / 2.0,
        }
    }
}