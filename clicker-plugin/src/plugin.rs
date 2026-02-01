use crate::*;
use cybird::Result;



// Runner defines how to handle Effect registration
impl IntoRegistration for Effect {
    fn register_in_context(self, ctx: &mut PluginContext) -> Result<()> {
        if let Some(upgrades) = ctx.get_service_mut::<Upgrades>() {
            let upgrade = Upgrade {
                name: format!("Effect Upgrade {}", upgrades.0.len()),
                level: 0,
                stage: 1,
                cost: |_| 100,
                effect_type: EffectType::Additive,
                description: "Auto-generated from effect by runner".to_string(),
                effects: vec![self],
            };
            upgrades.register(upgrade);
            println!("runner: Created upgrade from effect");
            Ok(())
        } else {
            Err("runner: Upgrades service not available".into())
        }
    }
}
