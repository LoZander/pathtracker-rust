use std::{cmp::Ordering, fmt::Display};
use egui::WidgetText;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize, Hash)]
pub struct Health {
    pub current: u32,
    pub max: u32,
    pub temp: u32,
}

impl Health {
    #[must_use]
    pub const fn new(max: u32) -> Self {
        Self { max, current: max, temp: 0 }
    }

    fn heal(&mut self, x: u32) {
        self.current = self.max.min(self.current + x);
    }

    fn damage(&mut self, x: u32) {
        let temp_left= self.temp.checked_sub(x);
        match temp_left {
            Some(new_temp) => self.set_temp(new_temp),
            None => {
                self.set_current(self.current.saturating_sub(x - self.temp));
                self.set_temp(0);
            }
        };
    }

    fn set_temp(&mut self, hp: u32) {
        self.temp = hp
    }

    fn set_current(&mut self, hp: u32) {
        self.current = hp.min(self.max)
    }

    fn set_max(&mut self, max: u32) {
        self.max = max;
        self.current = self.current.min(max);
    }

    fn add_temp(&mut self, hp: u32) {
        self.temp = self.temp + hp;
    } 
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[derive(Serialize, Deserialize, Hash)]
pub struct ChrName(String);

impl PartialOrd for ChrName {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for ChrName {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = &self.0;
        let y = &other.0;
        x.cmp(y)
    }
}

impl ChrName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl AsRef<str> for ChrName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Display for ChrName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl PartialEq<&Self> for ChrName {
    fn eq(&self, other: &&Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<ChrName> for &ChrName {
    fn eq(&self, other: &ChrName) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<&str> for ChrName {
    fn eq(&self, other: &&str) -> bool {
        &self.0 == other
    }
}

impl PartialEq<ChrName> for &str {
    fn eq(&self, other: &ChrName) -> bool {
        other == self
    }
}

impl PartialEq<str> for ChrName {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<ChrName> for str {
    fn eq(&self, other: &ChrName) -> bool {
        other == self
    }
}

impl PartialEq<String> for ChrName {
    fn eq(&self, other: &String) -> bool {
        self.0 == other.as_ref()
    }
}

impl PartialEq<ChrName> for String {
    fn eq(&self, other: &ChrName) -> bool {
        other == self
    }
}

impl From<ChrName> for String {
    fn from(value: ChrName) -> Self {
        value.0
    }
}

impl From<ChrName> for WidgetText {
    fn from(value: ChrName) -> Self {
        value.0.into()
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize, Hash)]
pub struct Chr {
    pub name: ChrName,
    pub init: i32,
    pub player: bool,
    pub health: Option<Health>,
}

impl PartialOrd for Chr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Chr {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.init != other.init { return other.init.cmp(&self.init) }
        match (self.player, other.player) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            _ => Ordering::Equal
        }
    }
}

impl Chr {
    pub fn builder(name: impl Into<String>, init: i32, player: bool) -> ChrBuilder {
        ChrBuilder::new(ChrName(name.into()), init, player)
    }

    pub fn heal(&mut self, x: u32) -> bool {
        if let Some(health) = self.health.as_mut() {
            health.heal(x);
            return true
        }
        false
    }

    pub fn damage(&mut self, x: u32) -> bool {
        if let Some(health) = self.health.as_mut() {
            health.damage(x);
            return true
        }
        false
    }

    pub fn set_temp_health(&mut self, hp: u32) -> bool {
        if let Some(health) = &mut self.health {
            health.set_temp(hp);
            return true
        }
        false
    }

    pub fn set_health(&mut self, health: Health) {
        self.health = Some(health);
    }

    pub fn set_current_health(&mut self, hp: u32) -> bool {
        if let Some(health) = &mut self.health {
            health.set_current(hp);
            return true
        }
        false
    }

    pub fn set_max_health(&mut self, max: u32) {
        if let Some(hp) = &mut self.health {
            hp.set_max(max)
        } else {
            self.health = Some(Health::new(max));
        }
    }

    pub fn add_temp_health(&mut self, hp: u32) -> bool {
        if let Some(health) = &mut self.health {
            health.add_temp(hp);
            return true
        }
        false
    }
}

pub struct ChrBuilder {
    name: ChrName,
    init: i32,
    player: bool,
    health: Option<Health>,
}

impl ChrBuilder {
    #[must_use]
    pub fn new(name: ChrName, init: i32, player: bool) -> Self {
        Self {
            name,
            init,
            player,
            health: None,
        }
    }

    #[must_use]
    pub fn build(self) -> Chr {
        Chr {
            name: self.name,
            init: self.init,
            player: self.player,
            health: self.health,
        }
    }
    
    #[must_use]
    pub fn with_health(self, health: Health) -> Self {
        Self { health: Some(health), ..self }
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::character::Health;

    use super::Chr;

    #[test]
    fn chr_order_greater_initiative_is_less_order() {
        let c1 = Chr::builder("a", 20, true).build();
        let c2 = Chr::builder("b", 10, true).build();

        assert_eq!(Ordering::Less, c1.cmp(&c2));
    }

    #[test]
    fn chr_order_less_initiative_is_greater_order() {
        let c1 = Chr::builder("a", 10, true).build();
        let c2 = Chr::builder("b", 20, true).build();

        assert_eq!(Ordering::Greater, c1.cmp(&c2));
    }

    #[test]
    fn chr_order_same_init_enemy_less_than_player() {
        let c1 = Chr::builder("a", 10, false).build();
        let c2 = Chr::builder("b", 10, true).build();

        assert_eq!(Ordering::Less, c1.cmp(&c2));
    }

    #[test]
    fn chr_order_same_init_enemy_equal_enemy() {
        let c1 = Chr::builder("a", 10, false).build();
        let c2 = Chr::builder("b", 10, false).build();

        assert_eq!(Ordering::Equal, c1.cmp(&c2));
    }

    #[test]
    fn damage_less_than_all() {
        let mut health = Health::new(100);
        health.damage(23);

        assert_eq!(77, health.current);
    }


    #[test]
    fn damage_overkill() {
        let mut health = Health::new(100);
        health.damage(101);

        assert_eq!(0, health.current);
    }

    #[test]
    fn damage_temp_absorbs_less_than_all() {
        let mut health = Health::new(100);
        health.set_temp(20);
        health.damage(6);

        assert_eq!(100, health.current);
        assert_eq!(14, health.temp)
    }

    #[test]
    fn damage_more_than_temp_rolls_over() {
        let mut health = Health::new(100);
        health.set_temp(10);
        health.damage(12);

        assert_eq!(98, health.current);
        assert_eq!(0, health.temp);
    }
}
