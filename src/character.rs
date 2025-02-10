use std::cmp::Ordering;
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
        self.current = self.current.saturating_sub(x);
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

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize, Hash)]
pub struct Chr {
    pub name: String,
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
        ChrBuilder::new(name, init, player)
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
    name: String,
    init: i32,
    player: bool,
    health: Option<Health>,
}

impl ChrBuilder {
    #[must_use]
    pub fn new(name: impl Into<String>, init: i32, player: bool) -> Self {
        Self {
            name: name.into(),
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
}
