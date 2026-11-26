use crate::ui::{HOVERED_PRESSED_BUTTON, NORMAL_BUTTON, TEXT_COLOR};
use clap::builder::styling::Color;

use crate::{
    enums::{GameState, MenuButtonAction, MenuState},
    ui::{HOVERED_BUTTON, PRESSED_BUTTON},
};
use bevy::{
    color::palettes::basic::*,
    picking::hover::Hovered,
    prelude::*,
    ui::{Checked, Pressed},
    ui_widgets::{Activate, Button, RadioButton, observe},
};
pub struct MenuPlugin;
impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        // At start, the menu is not enabled. This will be changed in `menu_setup` when
        // entering the `GameState::Menu` state.
        // Current screen in the menu is handled by an independent state from `GameState`
        app.add_systems(OnEnter(GameState::Menu), menu_setup)
            // Systems to handle the main menu screen
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            // Systems to handle the settings menu screen
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            // Systems to handle the display settings screen
            .add_systems(
                OnEnter(MenuState::SettingsDisplay),
                display_settings_menu_setup,
            )
            // Systems to handle the sound settings screen
            .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
            .add_observer(
                on_value_change_setting_radio::<Volume>.run_if(in_state(MenuState::SettingsSound)),
            )
            // Observer that handles navigation within the app upon pressing certain buttons.
            .add_observer(on_button_activate_update_states)
            // Observers for button styling (regular buttons and radio buttons)
            .add_observer(on_button_over_style)
            .add_observer(on_button_out_style)
            .add_observer(on_radio_button_press_style)
            .add_observer(on_radio_button_release_style);
    }
}
#[derive(Resource, Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Volume(pub u32);

/// Observer that changes the background color to hovered over buttons / radio buttons to the correct color.
fn on_button_over_style(
    event: On<PointerOver>,
    mut button_q: Query<&mut BackgroundColor, (With<Button>, Without<RadioButton>)>,
    mut radio_button_q: Query<
        (&mut BackgroundColor, Has<Checked>),
        (With<RadioButton>, Without<Button>),
    >,
) {
    if let Ok(mut background_color) = button_q.get_mut(event.entity) {
        *background_color = HOVERED_BUTTON.into();
    }
    if let Ok((mut background_color, is_checked)) = radio_button_q.get_mut(event.entity) {
        if is_checked {
            *background_color = PRESSED_BUTTON.into();
        } else {
            *background_color = HOVERED_BUTTON.into();
        }
    }
}

/// Observer that returns the normal background color to buttons / radio buttons that are hovered out of.
fn on_button_out_style(
    event: On<PointerOut>,
    mut bg_q: Query<&mut BackgroundColor, (With<Button>, Without<RadioButton>)>,
    mut radio_button_q: Query<
        (&mut BackgroundColor, Has<Checked>),
        (With<RadioButton>, Without<Button>),
    >,
) {
    if let Ok(mut background_color) = bg_q.get_mut(event.entity) {
        *background_color = NORMAL_BUTTON.into();
    }
    if let Ok((mut background_color, is_checked)) = radio_button_q.get_mut(event.entity) {
        if is_checked {
            *background_color = PRESSED_BUTTON.into();
        } else {
            *background_color = NORMAL_BUTTON.into();
        }
    }
}

/// Observer that changes the background color of radio buttons that are pressed
fn on_radio_button_press_style(
    event: On<PointerPress>,
    mut radio_button_q: Query<&mut BackgroundColor, With<RadioButton>>,
) {
    if let Ok(mut background_color) = radio_button_q.get_mut(event.entity) {
        *background_color = PRESSED_BUTTON.into();
    }
}

/// Observer that changes the background color of radio buttons that are released
fn on_radio_button_release_style(
    event: On<PointerRelease>,
    mut radio_button_q: Query<(&mut BackgroundColor, Has<Checked>), With<RadioButton>>,
) {
    if let Ok((mut background_color, is_checked)) = radio_button_q.get_mut(event.entity) {
        if is_checked {
            *background_color = HOVERED_PRESSED_BUTTON.into();
        } else {
            *background_color = HOVERED_BUTTON.into();
        }
    }
}

// This system updates the settings when a new value for a setting is selected, and marks
// the radio button as the one currently selected
fn on_value_change_setting_radio<
    T: Resource<Mutability = Mutable> + Component + PartialEq + Copy,
>(
    event: On<ValueChange<Entity>>,
    new_setting_query: Query<(&Setting<T>, Entity), (With<RadioButton>, Without<Checked>)>,
    previous_query: Single<(Entity, &mut BackgroundColor), (With<RadioButton>, With<Checked>)>,
    mut commands: Commands,
    mut setting: ResMut<T>,
) {
    if let Ok((radio_setting, entity)) = new_setting_query.get(event.value)
        && *setting != radio_setting.0
    {
        let (previous, mut previous_button_color) = previous_query.into_inner();
        *previous_button_color = NORMAL_BUTTON.into();
        commands.entity(previous).remove::<Checked>();
        commands.entity(entity).insert(Checked);
        *setting = radio_setting.0;
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_node = Node {
        width: px(300),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: px(30),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: px(10),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: FontSize::Px(33.0),
        ..default()
    };

    let right_icon = asset_server.load("textures/Game Icons/right.png");
    let wrench_icon = asset_server.load("textures/Game Icons/wrench.png");
    let exit_icon = asset_server.load("textures/Game Icons/exitRight.png");

    commands.spawn((
        DespawnOnExit(MenuState::Main),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnMainMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                // Display the game name
                (
                    Text::new("Bevy Game Menu UI"),
                    TextFont {
                        font_size: FontSize::Px(67.0),
                        ..default()
                    },
                    TextColor(TEXT_COLOR),
                    Node {
                        margin: UiRect::all(px(50)),
                        ..default()
                    },
                ),
                // Display three buttons for each action available from the main menu:
                // - new game
                // - settings
                // - quit
                (
                    Button,
                    ActivateOnPress,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Play,
                    children![
                        (ImageNode::new(right_icon), button_icon_node.clone()),
                        (
                            Text::new("New Game"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    ActivateOnPress,
                    button_node.clone(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Settings,
                    children![
                        (ImageNode::new(wrench_icon), button_icon_node.clone()),
                        (
                            Text::new("Settings"),
                            button_text_font.clone(),
                            TextColor(TEXT_COLOR),
                        ),
                    ]
                ),
                (
                    Button,
                    ActivateOnPress,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![
                        (ImageNode::new(exit_icon), button_icon_node),
                        (Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),
                    ]
                ),
            ]
        )],
    ));
}

fn settings_menu_setup(mut commands: Commands) {
    let button_node = Node {
        width: px(200),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = (
        TextFont {
            font_size: FontSize::Px(33.0),
            ..default()
        },
        TextColor(TEXT_COLOR),
    );

    commands.spawn((
        DespawnOnExit(MenuState::Settings),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    ));
}
fn display_settings_menu_setup(mut commands: Commands) {
    fn button_node() -> Node {
        Node {
            width: px(200),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }
    fn button_text_style() -> impl Bundle {
        (
            TextFont {
                font_size: FontSize::Px(33.0),
                ..default()
            },
            TextColor(TEXT_COLOR),
        )
    }
}

fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
    let button_node = Node {
        width: px(200),
        height: px(65),
        margin: UiRect::all(px(20)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = (
        TextFont {
            font_size: FontSize::Px(33.0),
            ..default()
        },
        TextColor(TEXT_COLOR),
    );

    let volume = *volume;
    let button_node_clone = button_node.clone();
    commands.spawn((
        DespawnOnExit(MenuState::SettingsSound),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnSoundSettingsMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                (
                    RadioGroup,
                    Node {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                    Children::spawn((
                        Spawn((Text::new("Volume"), button_text_style.clone())),
                        SpawnWith(move |parent: &mut ChildSpawner| {
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    RadioButton,
                                    Node {
                                        width: px(30),
                                        height: px(65),
                                        ..button_node_clone.clone()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    Setting(Volume(volume_setting)),
                                ));
                                if volume == Volume(volume_setting) {
                                    entity.insert((Checked, BackgroundColor(PRESSED_BUTTON)));
                                }
                            }
                        })
                    ))
                ),
                (
                    Button,
                    ActivateOnPress,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::BackToSettings,
                    children![(Text::new("Back"), button_text_style)]
                )
            ]
        )],
    ));
}

/// Observer that changes the menu and game state depending on the `MenuButtonAction`
/// attached to the activated Button.
fn on_button_activate_update_states(
    event: On<Activate>,
    action_q: Query<&MenuButtonAction, With<Button>>,
    mut app_exit_writer: MessageWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(menu_button_action) = action_q.get(event.entity) {
        match menu_button_action {
            MenuButtonAction::Quit => {
                app_exit_writer.write(AppExit::Success);
            }
            MenuButtonAction::Play => {
                game_state.set(GameState::Game);
                menu_state.set(MenuState::Disabled);
            }
            MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
            MenuButtonAction::SettingsDisplay => {
                menu_state.set(MenuState::SettingsDisplay);
            }
            MenuButtonAction::SettingsSound => {
                menu_state.set(MenuState::SettingsSound);
            }
            MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
            MenuButtonAction::BackToSettings => {
                menu_state.set(MenuState::Settings);
            }
        }
    }
}
