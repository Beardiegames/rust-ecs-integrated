
#![allow(dead_code)]

use std::collections::VecDeque;

use crate::types::{ Entity, Component };
use crate::spawns::Spawn;


#[derive(Default, Clone)]
pub struct GameObject {
    pub position: Position,
    pub agenda: Agenda,
    pub movement: Movement,
    pub health: Health,
    pub focus: Focus,
    pub attack: Attack,
    pub damage: Damage,
    pub defense: Defense,
    pub resist: Resist,
    pub afflictions: Afflictions,
    pub carry: Carry,
}

impl GameObject {
    pub fn has_position(&self) -> bool { *Component::is_active(&self.position) }
    pub fn has_agenda(&self) -> bool { *Component::is_active(&self.agenda) }
    pub fn has_movement(&self) -> bool { *Component::is_active(&self.movement) }
    pub fn has_health(&self) -> bool { *Component::is_active(&self.health) }
    pub fn has_focus(&self) -> bool { *Component::is_active(&self.focus) }
    pub fn has_attack(&self) -> bool { *Component::is_active(&self.attack) }
    pub fn has_damage(&self) -> bool { *Component::is_active(&self.damage) }
    pub fn has_defense(&self) -> bool { *Component::is_active(&self.defense) }
    pub fn has_resist(&self) -> bool { *Component::is_active(&self.resist) }
    pub fn has_afflictions(&self) -> bool { *Component::is_active(&self.afflictions) }
    pub fn has_carry(&self) -> bool { *Component::is_active(&self.carry) }
}

impl Entity for GameObject {

}

// --component types--

#[derive(Default, Clone)]
pub struct Movement {
    active: bool,
    speed: f32,
    move_to: Option<Position>,
}
impl Movement {
    pub fn from_speed(speed: f32) -> Self {
        Movement { active: true, speed, move_to: None, }
    }
    pub fn speed(&self) -> &f32 {
        &self.speed
    }
}
impl Component for Movement {
    fn set_active(&mut self, activate: bool) { self.active = activate; }
    fn is_active(&self) -> &bool { &self.active }
}


#[derive(Default, Clone)]
pub struct Damage {
    active: bool,
    incoming: VecDeque<Attack>,
}
impl Damage {
    pub fn new() -> Self {
        Damage { active: true, incoming: VecDeque::new() }
    }
    pub fn take_damage(&mut self, attack: Attack) {
        self.incoming.push_back(attack)
    }
}
impl Component for Damage {
    fn set_active(&mut self, activate: bool) { self.active = activate; }
    fn is_active(&self) -> &bool { &self.active }
}
impl Iterator for Damage {
    type Item = Attack;
    fn next(&mut self) -> Option<Self::Item> {
        self.incoming.pop_front()
    }
}


#[derive(Default, Clone)]
pub struct Defense {
    active: bool,
    blockers: Vec<ImpactProtection>,
}
impl Defense {
    pub fn from_blockers(blockers: Vec<ImpactProtection>) -> Self {
        Defense { active: true, blockers, }
    }
    pub fn resolve_attack(&self, attack: &Attack) -> u32 {
        let mut power = attack.weapon.power.clone();
        for blocker in &self.blockers {
            if blocker.against == attack.weapon.impact {
                power = (power as f32 * blocker.immunity_factor.as_f32()) as u32;
                power = (power as f32 - blocker.reduction) as u32;
            }
        }
        power
    } 
}
impl Component for Defense {
    fn set_active(&mut self, activate: bool) { self.active = activate; }
    fn is_active(&self) -> &bool { &self.active }
}


#[derive(Default, Clone)]
pub struct Resist {
    active: bool,
    resistances: Vec<AfflictionProtection>,
}
impl Resist {
    pub fn new(resistances: Vec<AfflictionProtection>) -> Self {
        Resist { active: true, resistances, }
    }
}
impl Component for Resist {
    fn set_active(&mut self, activate: bool) { self.active = activate; }
    fn is_active(&self) -> &bool { &self.active }
}

#[derive(Clone)]
pub enum ImmunityFactor {
    None,
    Half,
    Full,
}
impl Default for ImmunityFactor {
    fn default() -> Self { ImmunityFactor::None }
}
impl ImmunityFactor {
    fn as_f32(&self) -> f32 { 
        match self {
            Self::None => 1.0,
            Self::Half => 0.5,
            Self::Full => 0.0,
        }
    }
}

#[derive(Default, Clone)]
pub struct ImpactProtection {
    pub against: Impact,
    pub immunity_factor: ImmunityFactor,
    pub reduction: f32,
}

#[derive(Default, Clone)]
pub struct AfflictionProtection {
    pub against: Affliction,
    pub immunity_factor: ImmunityFactor,
    pub reduction: f32,
}


#[derive(Default, Clone)]
pub struct Health {
    active: bool,
    pub current_hp: u32,
    pub max_hp: u32,
}
impl Health {
    pub fn from_hp(hp: u32) -> Self {
        Health { active: true, current_hp: hp, max_hp: hp }
    }
    pub fn heal(&mut self, hp: u32) {
        self.current_hp += hp;
        if self.current_hp > self.max_hp { self.current_hp = self.max_hp; }
    }
    pub fn damage(&mut self, hp: u32) {
        self.current_hp -= hp;
    }
}
impl Component for Health {
    fn set_active(&mut self, activate: bool) { self.active = activate; }
    fn is_active(&self) -> &bool { &self.active }
}


#[derive(Default, Clone)]
pub struct Focus {
    active: bool,
    focus: Vec<Spawn>,
}
impl Focus {
    pub fn new() -> Self {
        Focus { active: true, focus: Vec::new(), }
    }
    pub fn add(&mut self, spawn: &Spawn) {
        if self.enlisted(spawn).is_none() { 
            self.focus.push(spawn.clone());
        }
    }
    pub fn prime(&self) -> Option<&Spawn> {
        if self.focus.len() > 0 {
            Some(&self.focus[0])
        } else {
            None
        }
    }
    pub fn remove(&mut self, spawn: &Spawn) {
        if let Some(index) = self.enlisted(spawn) { 
            self.focus.remove(index);
        }
    }
    pub fn enlisted(&self, spawn: &Spawn) -> Option<usize>{
        self.focus.iter().position(|x| x == spawn)
    }
    pub fn count(&self) -> usize {
        self.focus.len()
    }
}
impl Component for Focus {
    fn set_active(&mut self, activate: bool) { self.active = activate; }
    fn is_active(&self) -> &bool { &self.active }
}


#[derive(Clone, Default)]
pub struct Attack {
    pub active: bool,
    pub weapon: Weapon,
    pub skill: u32,
    pub range: u32,
}
impl Attack {
    pub fn power(&self) -> u32 {
        self.weapon.power + self.skill
    }
}
impl Component for Attack {
    fn set_active(&mut self, activate: bool) { self.active = activate; }
    fn is_active(&self) -> &bool { &self.active }
}

#[derive(Clone, Default)]
pub struct Weapon {
    pub impact: Impact,
    pub effects: Vec<Affliction>,
    pub power: u32,
    pub range: f32,
}
impl Weapon {
    fn add_effect(&mut self, affliction: Affliction) {
        self.effects.push(affliction)
    }


    pub fn provoke() -> Self { Weapon { 
        impact: Impact::Mental, 
        effects: vec![Affliction::Annoyed], 
        power: 0,
        range: 10.0,
    }}
    pub fn smart_remarks() -> Self { Weapon { 
        impact: Impact::Mental, 
        effects: vec![Affliction::Confused], 
        power: 0,
        range: 10.0,
    }}
    pub fn intamidation() -> Self { Weapon { 
        impact: Impact::Mental, 
        effects: vec![Affliction::Scared], 
        power: 0,
        range: 10.0,
    }}
    pub fn handgun() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![], 
        power: 3,
        range: 40.0,
    }}
    pub fn shotgun() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![], 
        power: 5,
        range: 20.0,
    }}
    pub fn rifle() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![], 
        power: 4,
        range: 60.0,
    }}
    pub fn fists() -> Self { Weapon { 
        impact: Impact::Bashing, 
        effects: vec![], 
        power: 1,
        range: 0.0,
    }}
    pub fn baton() -> Self { Weapon { 
        impact: Impact::Bashing, 
        effects: vec![Affliction::Dazzled], 
        power: 2,
        range: 0.0,
    }}
    pub fn rapier() -> Self { Weapon { 
        impact: Impact::Cutting, 
        effects: vec![Affliction::Wounded], 
        power: 4,
        range: 5.0,
    }}
    pub fn spear() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![], 
        power: 3,
        range: 10.0,
    }}
    pub fn needle() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![Affliction::Poisoned], 
        power: 0,
        range: 0.0,
    }}
    pub fn mortar() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![Affliction::Burning], 
        power: 5,
        range: 40.0,
    }}
    pub fn canon() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![Affliction::Burning], 
        power: 8,
        range: 60.0,
    }}
    pub fn missle() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![Affliction::Burning], 
        power: 10,
        range: 100.0,
    }}
    pub fn mine() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![Affliction::Burning], 
        power: 5, 
        range: 0.0,
    }}
}
#[derive(Clone, PartialEq)]
pub enum Impact {
    Bashing,
    Piercing,
    Cutting,
    Exploding,
    Mental,
}
impl Default for Impact {
    fn default() -> Self { Impact::Bashing }
}
#[derive(Clone, PartialEq)]
pub enum Affliction {
    Annoyed,
    Confused,
    Dazzled,
    Scared,
    Wounded,
    Burning,
    Freezing,
    Melting,
    Poisoned,
    Diseased,
}
impl Default for Affliction {
    fn default() -> Self { Affliction::Annoyed }
}

#[derive(Default, Clone)]
pub struct Afflictions { 
    active: bool,
    list: Vec<Affliction>,
}
impl Component for Afflictions {
    fn set_active(&mut self, activate: bool) { self.active = activate }
    fn is_active(&self) -> &bool { &self.active }
}

#[derive(Default, Clone)]
pub struct Carry {
    active: bool,
    spawns: Vec<Spawn>,
}
impl Component for Carry {
    fn set_active(&mut self, activate: bool) { self.active = activate; }
    fn is_active(&self) -> &bool { &self.active }
}

#[derive(Default, Clone)]
pub struct Agenda {
    active: bool,
    pub faction: Faction,
}
impl Component for Agenda {
    fn set_active(&mut self, activate: bool) { self.active = activate; }
    fn is_active(&self) -> &bool { &(self.active) }
}


#[derive(Clone, PartialEq)]
pub enum Faction {
    None,
    Red,
    Bleu,
}
impl Faction {
    pub fn opposing(&self, other: &Faction) -> bool {
        *self != Self::None 
        && *other != Self::None 
        && other != self
    }
}
impl Default for Faction {
    fn default() -> Self { Faction::None }
}


#[derive(Default, Clone)]
pub struct Position {
    active: bool,
    pub x: f64,
    pub y: f64,
}
impl Position {
    pub fn distance(&self, other: &Position) -> f64 {
        let diff_x = other.x - self.x;
        let diff_y = other.y - self.y;
        ((diff_x * diff_x) + (diff_y * diff_y)).sqrt()
    }
}
impl Component for Position {
    fn set_active(&mut self, activate: bool) { self.active = activate; }
    fn is_active(&self) -> &bool { &self.active }
}
