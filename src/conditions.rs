use std::{collections::{HashMap, HashSet}, hash::Hash};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[derive(Deserialize, Serialize)]
pub struct ConditionManager {
    cond_map: HashMap<String, HashSet<Condition>>
}

impl ConditionManager {
    pub fn new() -> Self {
        ConditionManager { cond_map: HashMap::new() }
    }
    pub fn add(&mut self, character: &str, cond: Condition) {
        match self.cond_map.get_mut(character) {
            None => { self.cond_map.insert(character.to_string(), HashSet::from([cond])); },
            Some(conds) => { conds.insert(cond); }
        }
    }

    pub fn remove(&mut self, character: &str, cond_name: &str) {
        let cond_name: String = cond_name.into();
        if let Some(conds) = self.cond_map
            .get_mut(character) { conds.retain(|cond| cond.name != cond_name) }
    }

    pub fn get<'a>(&'a self, character: &str) -> Option<&'a HashSet<Condition>> {
        self.cond_map.get(character)
    }

    pub fn handle_cond_trigger(&mut self, character: &str, trigger: CondTrigger) {
        self.cond_map = self.cond_map.values().map(|conds| {
            let filtered_conds: HashSet<Condition> = conds.clone()
                .into_iter()
                .filter_map(cond_filter(trigger.clone()))
                .collect();

            (character.to_string(), filtered_conds)
        }).collect();
    }

}

fn cond_filter(trigger: CondTrigger) -> impl FnMut(Condition) -> Option<Condition> {
    move |cond: Condition| {   
        match cond.clone() {
            Condition { trigger : t, .. } if t != trigger => Some(cond),
            Condition { reduction : None, .. } => None,
            Condition { reduction : Some(amount), level, .. } => 
                Some(Condition { level: level.saturating_sub(amount), ..cond })
        }
    }
}

#[derive(Debug, Clone, Eq)]
#[derive(Serialize, Deserialize)]
pub struct Condition {
    pub name: String,
    pub level: u8,
    pub trigger: CondTrigger,
    pub reduction: Option<u8>,
}

impl Hash for Condition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(self.name.as_bytes())
    }
}

impl PartialEq for Condition {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Hash)]
#[derive(Serialize, Deserialize)]
pub enum CondTrigger {
    Manual(String),
    StartOfTurn,
    EndOfTurn,
}
