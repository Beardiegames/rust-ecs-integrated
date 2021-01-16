
use std::collections::VecDeque;

use crate::ecs::{ Spawn, Entity, Component };


#[derive(Default, Clone)]
pub struct GameObject {
    pub position: Position,
    pub faction: Faction,
    pub movement: Movement,
    pub health: Health,
    pub focus: Focus,
    pub attack: Attack,
    pub damage: Damage,
    pub brace: Brace,
    pub resist: Resist,
    pub afflictions: Afflictions,
    pub carry: Carry,
}

impl GameObject {
    pub fn has_position(&self) -> bool { *Component::is_active(&self.position) }
    pub fn has_faction(&self) -> bool { *Component::is_active(&self.faction) }
    pub fn has_movement(&self) -> bool { *Component::is_active(&self.movement) }
    pub fn has_health(&self) -> bool { *Component::is_active(&self.health) }
    pub fn has_focus(&self) -> bool { *Component::is_active(&self.focus) }
    pub fn has_attack(&self) -> bool { *Component::is_active(&self.attack) }
    pub fn has_damage(&self) -> bool { *Component::is_active(&self.damage) }
    pub fn has_brace(&self) -> bool { *Component::is_active(&self.brace) }
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
    pub fn new(speed: f32) -> Self {
        Movement { active: true, speed, move_to: None, }
    }
}
impl Component for Movement {
    fn is_active(&self) -> &bool { &self.active }
}


#[derive(Default, Clone)]
pub struct Damage {
    active: bool,
    incoming: VecDeque<Attack>,
}
impl Damage {
    pub fn new(hp: i32) -> Self {
        Damage { active: true, incoming: VecDeque::new() }
    }
    pub fn take_damage(&mut self, attack: Attack) {
        self.incoming.push_back(attack)
    }
}
impl Component for Damage {
    fn is_active(&self) -> &bool { &self.active }
}
impl Iterator for Damage {
    type Item = Attack;
    fn next(&mut self) -> Option<Self::Item> {
        self.incoming.pop_front()
    }
}


#[derive(Default, Clone)]
pub struct Brace {
    active: bool,
    blockers: Vec<ImpactProtection>,
}
impl Brace {
    pub fn new(hp: i32) -> Self {
        Brace { active: true, blockers: Vec::new(), }
    }
    pub fn resolve_attack(&self, attack: &Attack) -> u32 {
        let mut power = attack.weapon.power.clone();
        for blocker in &self.blockers {
            if blocker.against == attack.weapon.impact {
                power = (power as f32 * blocker.immunity_factor) as u32;
                power = (power as f32 - blocker.reduction) as u32;
            }
        }
        if power < 0 { power = 0; }
        power
    } 
}
impl Component for Brace {
    fn is_active(&self) -> &bool { &self.active }
}


#[derive(Default, Clone)]
pub struct Resist {
    active: bool,
    resistances: Vec<AfflictionProtection>,
}
impl Resist {
    pub fn new(hp: i32) -> Self {
        Resist { active: true, resistances: Vec::new(), }
    }
}
impl Component for Resist {
    fn is_active(&self) -> &bool { &self.active }
}

#[derive(Default, Clone)]
pub struct ImpactProtection {
    against: Impact,
    immunity_factor: f32,
    reduction: f32,
}

#[derive(Default, Clone)]
pub struct AfflictionProtection {
    against: Affliction,
    immunity_factor: f32,
    reduction: f32,
}


#[derive(Default, Clone)]
pub struct Health {
    active: bool,
    current_hp: u32,
    max_hp: u32,
}
impl Health {
    pub fn new(hp: u32) -> Self {
        Health { active: true, current_hp: hp, max_hp: hp }
    }
    pub fn heal(&mut self, hp: u32) {
        self.current_hp += hp;
        if self.current_hp > self.max_hp { self.current_hp = self.max_hp; }
    }
    pub fn damage(&mut self, hp: u32) {
        self.current_hp -= hp;
        if self.current_hp < 0 { self.current_hp = 0; }
    }
}
impl Component for Health {
    fn is_active(&self) -> &bool { &self.active }
}


#[derive(Default, Clone)]
pub struct Focus {
    active: bool,
    focus: Vec<Spawn>,
}
impl Focus {
    pub fn new(attack: Attack) -> Self {
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
    fn is_active(&self) -> &bool { &self.active }
}


#[derive(Clone, Default)]
pub struct Attack {
    active: bool,
    from: Spawn,
    weapon: Weapon,
    skill: u32,
    range: u32,
}
impl Attack {
    pub fn power(&self) -> u32 {
        self.weapon.power + self.skill
    }
}
impl Component for Attack {
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


    fn Provoke() -> Self { Weapon { 
        impact: Impact::Mental, 
        effects: vec![Affliction::Annoyed], 
        power: 0,
        range: 10.0,
    }}
    fn SmartRemarks() -> Self { Weapon { 
        impact: Impact::Mental, 
        effects: vec![Affliction::Confused], 
        power: 0,
        range: 10.0,
    }}
    fn Intimidation() -> Self { Weapon { 
        impact: Impact::Mental, 
        effects: vec![Affliction::Scared], 
        power: 0,
        range: 10.0,
    }}
    fn Handgun() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![], 
        power: 3,
        range: 40.0,
    }}
    fn Shotgun() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![], 
        power: 5,
        range: 20.0,
    }}
    fn Rifle() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![], 
        power: 4,
        range: 60.0,
    }}
    fn Fists() -> Self { Weapon { 
        impact: Impact::Bashing, 
        effects: vec![], 
        power: 1,
        range: 0.0,
    }}
    fn Baton() -> Self { Weapon { 
        impact: Impact::Bashing, 
        effects: vec![Affliction::Dazzled], 
        power: 2,
        range: 0.0,
    }}
    fn Rapier() -> Self { Weapon { 
        impact: Impact::Cutting, 
        effects: vec![Affliction::Wounded], 
        power: 4,
        range: 5.0,
    }}
    fn Spear() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![], 
        power: 3,
        range: 10.0,
    }}
    fn Needle() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![Affliction::Poisoned], 
        power: 0,
        range: 0.0,
    }}
    fn Mortar() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![Affliction::Burning], 
        power: 5,
        range: 40.0,
    }}
    fn Canon() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![Affliction::Burning], 
        power: 8,
        range: 60.0,
    }}
    fn Missle() -> Self { Weapon { 
        impact: Impact::Piercing, 
        effects: vec![Affliction::Burning], 
        power: 10,
        range: 100.0,
    }}
    fn Mine() -> Self { Weapon { 
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

type Afflictions = Vec<Affliction>;
impl Component for Afflictions {
    fn is_active(&self) -> &bool { &(self.len() > 0) }
}

#[derive(Default, Clone)]
pub struct Carry {
    active: bool,
    spawns: Vec<Spawn>,
}
impl Component for Carry {
    fn is_active(&self) -> &bool { &self.active }
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
impl Component for Faction {
    fn is_active(&self) -> &bool { &(*self != Faction::None) }
}


#[derive(Default, Clone)]
pub struct Position {
    active: bool,
    x: f32,
    y: f32,
}
impl Position {
    pub fn distance(&self, other: &Position) -> f32 {
        let diff_x = other.x - self.x;
        let diff_y = other.y - self.y;
        ((diff_x * diff_x) + (diff_y * diff_y)).sqrt()
    }
}
impl Component for Position {
    fn is_active(&self) -> &bool { &self.active }
}
