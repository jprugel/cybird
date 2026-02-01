use bevy::{color::palettes::basic::*, input_focus::InputFocus, prelude::*};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use clicker_plugin::*;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Resource /* Default */, Reflect)]
struct Score(u32);

#[derive(Message)]
enum Transaction {
    Increase(u32),
    Decrease(u32),
}

#[derive(Resource, Default)]
struct PluginLoader(pub Vec<libloading::Library>);

impl PluginLoader {
    fn add(&mut self, lib: libloading::Library) {
        self.0.push(lib);
    }
}

#[derive(Resource)]
pub struct GameState {
    context: PluginContext,
    stage: u32,
}

#[derive(Component)]
struct UpgradeButton;

#[derive(Component)]
struct UpgradeLevel(UpgradeId);

#[derive(Message)]
struct OnClick;

#[derive(Component, Hash, Eq, PartialEq, Clone)]
struct UpgradeId(String);

#[derive(Message)]
struct OnUpgrade(UpgradeId);

#[derive(Component)]
struct CurrencyText;

impl Default for GameState {
    fn default() -> Self {
        let map = PluginContext::default();

        Self {
            context: map,
            stage: 0,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(ResourceInspectorPlugin::<Score>::default())
        .add_message::<OnClick>()
        .add_message::<OnStage>()
        .add_message::<OnUpgrade>()
        .add_message::<Transaction>()
        .add_message::<Prestige>()
        .init_resource::<InputFocus>()
        .init_resource::<GameState>()
        .init_resource::<Score>()
        .init_resource::<PluginLoader>()
        .register_type::<Score>()
        .add_systems(Startup, setup)
        .add_systems(Startup, register_upgrades)
        .add_systems(Update, score_handler)
        .add_systems(Update, update_upgrade_cost)
        .add_systems(Update, update_upgrade_level)
        .add_systems(Startup, setup_currency)
        .add_systems(Update, button_system)
        .add_systems(Update, increase_score)
        .add_systems(Update, upgrade_button_system)
        .add_systems(Startup, upgrade_view)
        .add_systems(Startup, plugin_loader)
        .add_systems(Update, update_view)
        .add_systems(Update, upgrade_effect)
        .add_systems(Update, upgrade_gamestage)
        .add_systems(Update, stage_handler)
        .add_systems(Update, handle_prestige)
        .run();

    Ok(())
}

#[derive(Message)]
struct OnStage;

impl Default for Score {
    fn default() -> Self {
        Score(0)
    }
}

fn stage_handler(
    score: Res<Score>,
    mut gamestate: ResMut<GameState>,
    mut message_writer: MessageWriter<OnStage>,
) {
    if score.0 >= 10 && gamestate.stage == 0 {
        gamestate.stage += 1;
        message_writer.write(OnStage);
    }
    if score.0 >= 100 && gamestate.stage == 1 {
        gamestate.stage += 1;
        message_writer.write(OnStage);
    }
    if score.0 >= 1000 && gamestate.stage == 2 {
        gamestate.stage += 1;
        message_writer.write(OnStage);
    }
    if score.0 >= 10000 && gamestate.stage == 3 {
        gamestate.stage += 1;
        message_writer.write(OnStage);
    }
    if score.0 >= 100000 && gamestate.stage == 4 {
        gamestate.stage += 1;
        message_writer.write(OnStage);
    }
}

fn plugin_loader(mut gamestate: ResMut<GameState>, mut plugin_loader: ResMut<PluginLoader>) {
    try_load_plugin(
        "./target/debug/upgrade3.dll",
        &mut gamestate,
        &mut plugin_loader,
    )
    .expect("Failed to load plugin");
}

fn upgrade_gamestage(gamestate: Res<GameState>, mut query: Query<(&mut Visibility, &UpgradeId)>) {
    for (mut visibility, upgrade_id) in query.iter_mut() {
        if let Some(Upgrade { stage, .. }) = gamestate
            .context
            .get_registrables::<Upgrade>()
            .iter()
            .find(|u| u.name == *upgrade_id.0)
        {
            if *stage == gamestate.stage {
                *visibility = Visibility::Visible;
            }
        }
    }
}

fn register_upgrades(mut gamestate: ResMut<GameState>) {
    gamestate.context.register(Upgrade {
        name: "Cookie Recycler".to_string(),
        level: 0,

        description: "Increase cookie click yield by 1 per level.".to_string(),
        stage: 1,
        cost: |level| level * 2 + 1,
        effect_type: EffectType::Additive,
        effects: vec![Effect {
            trigger: EffectTrigger::Click,
            value: EffectValue::Add(|level| level),
        }],
    });

    gamestate.context.register(Upgrade {
        name: "Cookie Accelerator".to_string(),
        level: 0,

        description: "Increase cookie click yield by 10 per level.".to_string(),
        stage: 2,
        effect_type: EffectType::Additive,
        cost: |level| level * 10 + 10,
        effects: vec![Effect {
            trigger: EffectTrigger::Click,
            value: EffectValue::Add(|level| level * 10),
        }],
    });

    gamestate.context.register(Upgrade {
        name: "Cookie Prestige".to_string(),
        level: 0,

        stage: 5,
        cost: |_| 100_000,
        description: "Increases all yields by 2x".to_string(),
        effect_type: EffectType::Multiplicative,
        effects: vec![
            Effect {
                trigger: EffectTrigger::Click,
                value: EffectValue::Multiply(|level| level * 2),
            },
            Effect {
                trigger: EffectTrigger::Click,
                value: EffectValue::Prestige,
            },
        ],
    });
}

fn score_handler(mut score: ResMut<Score>, mut message_reader: MessageReader<Transaction>) {
    for msg in message_reader.read() {
        match msg {
            Transaction::Increase(amount) => score.0 += amount,
            Transaction::Decrease(amount) => {
                if score.0 >= *amount {
                    score.0 -= amount;
                } else {
                    score.0 = 0;
                }
            }
        }
    }
}

fn upgrade_effect(
    mut gamestate: ResMut<GameState>,
    score: Res<Score>,
    mut message_writer: MessageWriter<Transaction>,
    mut msg_reader: MessageReader<OnUpgrade>,
    mut prestige_writer: MessageWriter<Prestige>,
) {
    for msg in msg_reader.read() {
        let upgrades = gamestate.context.get_registrables::<Upgrade>();
        let upgrade = upgrades.iter().find(|x| x.name == msg.0.0).unwrap();
        let cost = (upgrade.cost)(upgrade.level);
        let upgrade_effect = upgrade
            .effects
            .iter()
            .filter(|effect| matches!(effect.value, EffectValue::Prestige))
            .collect::<Vec<_>>();
        if upgrade_effect.len() >= 1 {
            info!("Prestige found");
            prestige_writer.write(Prestige);
        }
        if score.0 >= cost {
            message_writer.write(Transaction::Decrease(cost));
            gamestate
                .context
                .get_registrables_mut::<Upgrade>()
                .into_iter()
                .find(|x| x.name == msg.0.0)
                .unwrap()
                .level += 1;
        }
    }
}

fn handle_prestige(
    mut gamestate: ResMut<GameState>,
    mut message_reader: MessageReader<Prestige>,
    mut score: ResMut<Score>,
) {
    for _ in message_reader.read() {
        score.0 = 0;
        info!("Prestige triggered");

        for upgrade in gamestate.context.get_registrables_mut::<Upgrade>() {
            if upgrade.name == "Cookie Prestige" {
                continue;
            }
            info!("Resetting upgrades!");
            upgrade.level = 0;
        }
    }
}

fn setup_currency(mut commands: Commands) {
    commands.spawn((
        CurrencyText,
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Text::new("0"),
    ));
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct UpgradeCost(UpgradeId);

fn update_upgrade_cost(mut query: Query<(&mut Text, &UpgradeCost)>, gamestate: ResMut<GameState>) {
    for (mut text, cost) in query.iter_mut() {
        let cost = (gamestate
            .context
            .get_registrables::<Upgrade>()
            .iter()
            .find(|upgrade| upgrade.name == cost.0.0)
            .unwrap()
            .cost)(
            gamestate
                .context
                .get_registrables()
                .iter()
                .find(|upgrade: &&&Upgrade| upgrade.name == cost.0.0)
                .unwrap()
                .level,
        );
        *text = Text::new(format!("Cost: {}", cost));
    }
}

fn update_upgrade_level(
    mut query: Query<(&mut Text, &UpgradeLevel)>,
    gamestate: ResMut<GameState>,
) {
    for (mut text, cost) in query.iter_mut() {
        let level = gamestate
            .context
            .get_registrables::<Upgrade>()
            .iter()
            .find(|upgrade| upgrade.name == cost.0.0)
            .unwrap()
            .level;
        *text = Text::new(format!("Level: {}", level));
    }
}

fn upgrade_view(mut commands: Commands, gamestate: Res<GameState>) {
    let mut canvas = commands.spawn(Node {
        width: percent(100),
        height: percent(100),
        flex_direction: FlexDirection::Column,
        align_items: AlignItems::End,
        justify_content: JustifyContent::FlexStart,
        ..default()
    });

    let mut upgrades = gamestate
        .context
        .get_registrables::<Upgrade>()
        .into_iter()
        .collect::<Vec<&Upgrade>>();

    upgrades.sort_by(|a, b| {
        a.stage
            .cmp(&b.stage)
            .then((a.cost)(a.level).cmp(&(b.cost)(b.level)))
    });

    for upgrade in upgrades {
        let cost = (upgrade.cost)(upgrade.level);
        canvas.with_children(|b| {
            b.spawn((
                Visibility::Hidden,
                UpgradeId(upgrade.name.clone()),
                UpgradeButton,
                Button,
                Node {
                    width: px(400),
                    height: px(100),
                    border: UiRect::all(px(5)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    border_radius: BorderRadius::all(px(5)),
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BackgroundColor(Color::BLACK),
                children![
                    (
                        ButtonText,
                        Text::new(upgrade.name.clone()),
                        TextFont {
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                        TextShadow::default(),
                    ),
                    (
                        Text::new(format!("Cost: {}", cost)),
                        UpgradeCost(UpgradeId(upgrade.name.clone()))
                    ),
                    (
                        Text::new(format!("Level: {}", upgrade.level)),
                        UpgradeLevel(UpgradeId(upgrade.name.clone())),
                        Node {
                            border: UiRect::all(px(2)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor::all(Color::WHITE),
                    )
                ],
            ));
        });
    }
}

#[derive(Message)]
struct Prestige;

fn increase_score(
    gamestate: ResMut<GameState>,
    mut message_reader: MessageReader<OnClick>,
    mut message_writer: MessageWriter<Transaction>,
) {
    for _ in message_reader.read() {
        let base_rate = gamestate
            .context
            .get_registrables::<Upgrade>()
            .iter()
            .filter(|upgrade| upgrade.effect_type == EffectType::Additive)
            .fold(1, |acc, upgrade| {
                if let EffectValue::Add(f) = upgrade.effects[0].value {
                    acc + f(upgrade.level)
                } else {
                    acc
                }
            });

        info!("Base rate: {}", base_rate);
        info!(
            "Upgrades: {:?}",
            gamestate
                .context
                .get_registrables::<Upgrade>()
                .iter()
                .collect::<Vec<_>>()
        );

        let mut rate = base_rate as f32
            * gamestate
                .context
                .get_registrables::<Upgrade>()
                .iter()
                .filter(|upgrade| upgrade.effect_type == EffectType::Multiplicative)
                .fold(1., |acc, upgrade| {
                    info!("Upgrade: {:?}", upgrade);
                    if let EffectValue::Multiply(f) = upgrade.effects[0].value {
                        acc * f(upgrade.level) as f32
                    } else {
                        acc
                    }
                });

        info!("Rate: {}", rate);

        if base_rate > rate as u32 {
            rate = base_rate as f32;
        }

        let _ = message_writer.write(Transaction::Increase(rate as u32));
    }
}

fn update_view(score: Res<Score>, mut text_query: Query<&mut Text, With<CurrencyText>>) {
    for mut text in text_query.iter_mut() {
        **text = score.0.to_string();
    }
}

fn button_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
            &Children,
        ),
        (With<ClickerButton>, Changed<Interaction>),
    >,
    mut message_writer: MessageWriter<OnClick>,
    mut button_text_query: Query<&mut Text, With<ButtonText>>,
) {
    for (entity, interaction, mut color, mut border_color, mut button, children) in
        &mut interaction_query
    {
        let mut text = button_text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                input_focus.set(entity);
                **text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                *border_color = BorderColor::all(RED);
                message_writer.write(OnClick);

                // The accessibility system's only update the button's state when the `Button` component is marked as changed.
                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                **text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                *border_color = BorderColor::all(Color::WHITE);
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                **text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                *border_color = BorderColor::all(Color::BLACK);
            }
        }
    }
}

fn upgrade_button_system(
    mut input_focus: ResMut<InputFocus>,
    mut interaction_query: Query<
        (
            Entity,
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &mut Button,
            &UpgradeId,
            &Children,
        ),
        (With<UpgradeButton>, Changed<Interaction>),
    >,
    mut message_writer: MessageWriter<OnUpgrade>,
    mut button_text_query: Query<&mut Text, With<ButtonText>>,
) {
    for (entity, interaction, mut color, mut border_color, mut button, upgrade_id, children) in
        &mut interaction_query
    {
        info!("Upgrade button system");
        let mut _text = button_text_query.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                info!("Upgrade button pressed");
                input_focus.set(entity);
                *color = PRESSED_BUTTON.into();
                *border_color = BorderColor::all(RED);
                message_writer.write(OnUpgrade(upgrade_id.clone()));

                // The accessibility system's only update the button's state when the `Button` component is marked as changed.
                button.set_changed();
            }
            Interaction::Hovered => {
                input_focus.set(entity);
                *color = HOVERED_BUTTON.into();
                *border_color = BorderColor::all(Color::WHITE);
                button.set_changed();
            }
            Interaction::None => {
                input_focus.clear();
                *color = NORMAL_BUTTON.into();
                *border_color = BorderColor::all(Color::BLACK);
            }
        }
    }
}

fn setup(mut commands: Commands) {
    // ui camera
    commands.spawn(Camera2d);
    commands.spawn(button());
}

#[derive(Component)]
struct ButtonText;

#[derive(Component)]
struct ClickerButton;

fn button() -> impl Bundle {
    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        children![(
            Button,
            ClickerButton,
            Node {
                width: px(150),
                height: px(65),
                border: UiRect::all(px(5)),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(px(5)),
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BackgroundColor(Color::BLACK),
            children![(
                ButtonText,
                Text::new("Button"),
                TextFont {
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
                TextShadow::default(),
            )]
        )],
    )
}

fn try_load_plugin(
    path: &str,
    gamestate: &mut GameState,
    plugin_loader: &mut PluginLoader,
) -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        println!("  Loading DLL: {}", path);
        let lib = libloading::Library::new(path)?;
        println!("  DLL loaded successfully");

        // Get plugin metadata
        let get_author: libloading::Symbol<unsafe extern "C" fn() -> *const c_char> =
            lib.get(b"get_author")?;
        let get_id: libloading::Symbol<unsafe extern "C" fn() -> *const c_char> =
            lib.get(b"get_id")?;
        let load_plugin: libloading::Symbol<unsafe extern "C" fn(*mut std::ffi::c_void) -> i32> =
            lib.get(b"load_plugin")
                .expect("Failed to load load_plugin symbol");
        let free_string: libloading::Symbol<unsafe extern "C" fn(*mut c_char)> =
            lib.get(b"free_string")?;

        // Call the functions
        let author_ptr = get_author();
        let id_ptr = get_id();

        let author = CStr::from_ptr(author_ptr).to_str()?;
        let id = CStr::from_ptr(id_ptr).to_str()?;

        println!("  Plugin Author: {}", author);
        println!("  Plugin ID: {}", id);

        let result =
            load_plugin(&mut gamestate.context as *mut PluginContext as *mut std::ffi::c_void);
        println!("  Plugin load result: {}", result);

        // Free the allocated strings
        free_string(author_ptr as *mut c_char);
        free_string(id_ptr as *mut c_char);
        plugin_loader.add(lib);

        Ok(())
    }
}
