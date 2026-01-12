use clicker_plugin::*;
use cybird::*;

#[cybird::plugin]
#[derive(Default)]
pub struct Upgrade3;

impl Plugin<Upgrades> for Upgrade3 {
    fn author(&self) -> &str {
        "jprugel"
    }

    fn id(&self) -> &str {
        "upgrade3"
    }

    fn load(&self, ctx: &mut Upgrades) -> Result<()> {
        println!("Loading Upgrade3 plugin...");
        ctx.register(Upgrade {
            name: "Crumble Decintigrator".to_string(),
            level: 0,
            stage: 3,
            cost: |level| level * 100 + 1,
            effect: |level| level * 10 + 1,
        });

        ctx.register(Upgrade {
            name: "Uber Oven".to_string(),
            level: 0,
            stage: 4,
            cost: |level| level * level,
            effect: |level| level * level,
        });

        println!("Loading Upgrade3 Plugin end...");
        Ok(())
    }
}
