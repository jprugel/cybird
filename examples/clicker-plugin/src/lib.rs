use cybird::prelude::*;

#[derive(Default, Context)]
pub struct PluginContext(Vec<Registrable>);

pub enum Registrable {
    Upgrade(Upgrade),
}

#[derive(Registrable)]
pub struct Upgrade {
    pub name: String,
    pub level: u32,

    pub stage: u32,
    pub cost: fn(u32) -> u32,
    pub description: String,
    pub effect_type: EffectType,

    // Effect needs to be a bit more complex i think
    pub effects: Vec<Effect>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum EffectType {
    Additive,
    Multiplicative,
}

#[derive(PartialEq, Eq, Debug)]
pub enum EffectTrigger {
    Click,
}

#[derive(Debug)]
pub enum EffectValue {
    Add(fn(u32) -> u32),
    Multiply(fn(u32) -> u32),
    Prestige,
}

#[derive(Debug)]
pub struct Effect {
    pub trigger: EffectTrigger,
    pub value: EffectValue,
}

impl Effect {
    pub fn to_string(&self, level: u32) -> String {
        let value = match self.value {
            EffectValue::Add(f) => format!("add: {}", f(level)),
            EffectValue::Multiply(f) => format!("multiply: {}", f(level)),
            EffectValue::Prestige => format!("prestige"),
        };
        format!("trigger: {:?}, value: {:?}", self.trigger, value)
    }
}

// effect needs work so we can implement UpgradeType::Click, UpgradeType::Tick, UpgradeType::Prestige

impl std::fmt::Debug for Upgrade {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Upgrade")
            .field("name", &self.name)
            .field("level", &self.level)
            .field("stage", &self.stage)
            .field("cost", &(self.cost)(self.level))
            .field("effect_type", &self.effect_type)
            .field(
                "effects",
                &self
                    .effects
                    .iter()
                    .map(|effect| effect.to_string(self.level))
                    .collect::<Vec<String>>(),
            )
            .finish()
    }
}
