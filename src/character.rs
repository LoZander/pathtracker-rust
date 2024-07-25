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
    pub fn new(max: u32) -> Self {
        Self { max, current: max, temp: 0 }
    }

    fn heal(&mut self, x: u32) {
        self.current = self.max.min(self.current - x)
    }

    fn damage(&mut self, x: u32) {
        self.current = self.max.min(self.current - x)
    } 
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Serialize, Deserialize, Hash)]
pub struct Chr {
    pub name: String,
    pub init: i32,
    pub player: bool,
    pub dex: Option<i32>,
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

        if let (Some(dex1), Some(dex2)) = (self.dex, other.dex) {
            if dex1 != dex2 { return dex2.cmp(&dex1) }
        }

        Ordering::Equal
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
}

pub struct ChrBuilder {
    name: String,
    init: i32,
    player: bool,
    dex: Option<i32>,
    health: Option<Health>,
}

impl ChrBuilder {
    pub fn new(name: impl Into<String>, init: i32, player: bool) -> Self {
        Self {
            name: name.into(),
            init,
            player,
            dex: None,
            health: None,
        }
    }

    pub fn build(self) -> Chr {
        Chr {
            name: self.name,
            init: self.init,
            player: self.player,
            dex: self.dex,
            health: self.health,
        }
    }
    
    pub fn with_health(mut self, health: Health) -> Self {
        self.health = Some(health);
        self
    }

    pub fn with_dex(mut self, dex: i32) -> Self {
        self.dex = Some(dex);
        self
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

        assert_eq!(Ordering::Less, c1.cmp(&c2))
    }

    #[test]
    fn chr_order_less_initiative_is_greater_order() {
        let c1 = Chr::builder("a", 10, true).build();
        let c2 = Chr::builder("b", 20, true).build();

        assert_eq!(Ordering::Greater, c1.cmp(&c2))
    }

    #[test]
    fn chr_order_same_initiative_greater_dex_is_less_order() {
        let mut c1 = Chr::builder("a", 20, true).build();
        let mut c2 = Chr::builder("b", 20, true).build();
        c1.dex = Some(5);
        c2.dex = Some(2);

        assert_eq!(Ordering::Less, c1.cmp(&c2))
    }

    #[test]
    fn chr_order_same_initiative_less_dex_is_greater_order() {
        let mut c1 = Chr::builder("a", 20, true).build();
        let mut c2 = Chr::builder("b", 20, true).build();
        c1.dex = Some(1);
        c2.dex = Some(4);

        assert_eq!(Ordering::Greater, c1.cmp(&c2))
    }
}
