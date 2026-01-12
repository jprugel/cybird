use bevy::{color::palettes::basic::*, input_focus::InputFocus, prelude::*};
use clicker_plugin::*;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Resource, Default)]
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
    upgrades: Upgrades,
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
        let map = Upgrades::default();

        Self {
            upgrades: map,
            stage: 0,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_message::<OnClick>()
        .add_message::<OnStage>()
        .add_message::<OnUpgrade>()
        .add_message::<Transaction>()
        .init_resource::<InputFocus>()
        .init_resource::<GameState>()
        .init_resource::<Score>()
        .init_resource::<PluginLoader>()
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
        .run();

    Ok(())
}

#[derive(Message)]
struct OnStage(u32);

fn stage_handler(
    score: Res<Score>,
    mut gamestate: ResMut<GameState>,
    mut message_writer: MessageWriter<OnStage>,
) {
    if score.0 >= 100 && gamestate.stage == 0 {
        gamestate.stage += 1;
        message_writer.write(OnStage(1));
    }
    if score.0 >= 1000 && gamestate.stage == 1 {
        gamestate.stage += 1;
        message_writer.write(OnStage(2));
    }
    if score.0 >= 10000 && gamestate.stage == 2 {
        gamestate.stage += 1;
        message_writer.write(OnStage(3));
    }
    if score.0 >= 100000 && gamestate.stage == 3 {
        gamestate.stage += 1;
        message_writer.write(OnStage(4));
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
        if let Some(Upgrade { stage, .. }) = gamestate.upgrades.get(&upgrade_id.0) {
            if *stage == gamestate.stage {
                *visibility = Visibility::Visible;
            }
        }
    }
}

fn register_upgrades(mut gamestate: ResMut<GameState>) {
    gamestate.upgrades.register(Upgrade {
        name: "Cookie Recycler".to_string(),
        level: 0,

        stage: 1,
        cost: |level| level * 2 + 1,
        effect: |level| level,
    });

    gamestate.upgrades.register(Upgrade {
        name: "Cookie Accelerator".to_string(),
        level: 0,

        stage: 2,
        cost: |level| level * 10 + 10,
        effect: |level| level * 2,
    });
}

fn score_handler(mut score: ResMut<Score>, mut message_reader: MessageReader<Transaction>) {
    for msg in message_reader.read() {
        match msg {
            Transaction::Increase(amount) => score.0 += amount,
            Transaction::Decrease(amount) => score.0 -= amount,
        }
    }
}

fn upgrade_effect(
    mut gamestate: ResMut<GameState>,
    score: Res<Score>,
    mut message_writer: MessageWriter<Transaction>,
    mut msg_reader: MessageReader<OnUpgrade>,
) {
    for msg in msg_reader.read() {
        let upgrade = gamestate.upgrades.get(&msg.0.0.clone()).unwrap();
        let cost = (upgrade.cost)(upgrade.level);
        if score.0 >= cost {
            message_writer.write(Transaction::Decrease(cost));
            gamestate.upgrades.get_mut(&msg.0.0.clone()).unwrap().level += 1;
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
        let cost = (gamestate.upgrades.get(&cost.0.0).unwrap().cost)(
            gamestate.upgrades.get(&cost.0.0).unwrap().level,
        );
        *text = Text::new(format!("Cost: {}", cost));
    }
}

fn update_upgrade_level(
    mut query: Query<(&mut Text, &UpgradeLevel)>,
    gamestate: ResMut<GameState>,
) {
    for (mut text, cost) in query.iter_mut() {
        let level = gamestate.upgrades.get(&cost.0.0).unwrap().level;
        *text = Text::new(format!("Level: {}", level));
    }
}

fn upgrade_view(mut commands: Commands, gamestate: Res<GameState>) {
    let mut canvas = commands.spawn(Node {
        width: percent(100),
        height: percent(100),
        flex_direction: FlexDirection::Column,
        //align_items: AlignItems::Start,
        justify_content: JustifyContent::FlexEnd,
        ..default()
    });
    for (id, upgrade) in gamestate.upgrades.iter() {
        let cost = (upgrade.cost)(upgrade.level);
        canvas.with_children(|b| {
            b.spawn((
                Visibility::Hidden,
                UpgradeId(id.clone()),
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
                    ..default()
                },
                BorderColor::all(Color::WHITE),
                BorderRadius::all(px(5)),
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
                        UpgradeCost(UpgradeId(id.clone()))
                    ),
                    (
                        Text::new(format!("Level: {}", upgrade.level)),
                        UpgradeLevel(UpgradeId(id.clone())),
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

fn increase_score(
    gamestate: ResMut<GameState>,
    mut message_reader: MessageReader<OnClick>,
    mut message_writer: MessageWriter<Transaction>,
) {
    for _ in message_reader.read() {
        info!("Score increased");
        let acc = gamestate
            .upgrades
            .values()
            .fold(0, |acc, upgrade| acc + (upgrade.effect)(upgrade.level));

        message_writer.write(Transaction::Increase(acc));
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
                ..default()
            },
            BorderColor::all(Color::WHITE),
            BorderRadius::MAX,
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

        let result = load_plugin(&mut gamestate.upgrades as *mut Upgrades as *mut std::ffi::c_void);
        println!("  Plugin load result: {}", result);

        // Free the allocated strings
        free_string(author_ptr as *mut c_char);
        free_string(id_ptr as *mut c_char);
        plugin_loader.add(lib);
        println!("loader completed: {:?}", gamestate.upgrades);

        Ok(())
    }
}
