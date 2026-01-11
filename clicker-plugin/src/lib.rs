use std::collections::HashMap;

pub struct Upgrade {
    pub name: String,
    pub level: u32,

    pub cost: fn(u32) -> u32,
    pub effect: fn(u32) -> u32,
}

impl std::fmt::Debug for Upgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Upgrade")
            .field("name", &self.name)
            .field("level", &self.level)
            .finish()
    }
}

#[derive(Default, Debug)]
pub struct Upgrades(HashMap<String, Upgrade>);

impl Upgrades {
    pub fn register(&mut self, upgrade: Upgrade) {
        self.0.insert(upgrade.name.clone(), upgrade);
    }

    pub fn get(&self, name: &str) -> Option<&Upgrade> {
        self.0.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Upgrade> {
        self.0.get_mut(name)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &Upgrade)> {
        self.0.iter()
    }

    pub fn values(&self) -> impl Iterator<Item = &Upgrade> {
        self.0.values()
    }
}
