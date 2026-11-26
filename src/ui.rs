use bevy::{
    color::palettes::basic::*,
    picking::hover::Hovered,
    prelude::*,
    ui::Pressed,
    ui_widgets::{Activate, Button, observe},
};

use crate::enums::GameState;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup);
        app.add_systems(
            Update,
            update_button_appearance.run_if(in_state(GameState::Game)),
        );
    }
}

pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    // A full-screen container that centers the button.
    commands.spawn((
        DespawnOnExit(GameState::Game),
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::End,
            justify_content: JustifyContent::End,
            ..default()
        },
        children![
            (
                button(&assets, "Menu"),
                // React to the button being clicked. `Button` fires an `Activate` event on a
                // completed click, so we attach an observer for it right here on the button entity.
                //
                // Try it: this is where you'd run your own logic (start a game, open a menu, etc.).
                observe(|_activate: On<Activate>| {
                    info!("Button clicked!");
                }),
            ),
            (
                button(&assets, "1"),
                // React to the button being clicked. `Button` fires an `Activate` event on a
                // completed click, so we attach an observer for it right here on the button entity.
                //
                // Try it: this is where you'd run your own logic (start a game, open a menu, etc.).
                observe(|_activate: On<Activate>| {
                    info!("Button clicked!");
                }),
            ),
            (
                button(&assets, "2"),
                // React to the button being clicked. `Button` fires an `Activate` event on a
                // completed click, so we attach an observer for it right here on the button entity.
                //
                // Try it: this is where you'd run your own logic (start a game, open a menu, etc.).
                observe(|_activate: On<Activate>| {
                    info!("Button clicked!");
                }),
            )
        ],
    ));
}

fn button(asset_server: &AssetServer, text: &str) -> impl Bundle {
    (
        // The headless button widget. It handles pointer/keyboard input and pressed-state
        // tracking for us; we only provide the look below.
        Button,
        // `Hovered` is used by the picking backend to track if the pointer is over the button.
        Hovered::default(),
        Node {
            width: px(150),
            height: px(65),
            border: UiRect::all(px(5)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            border_radius: BorderRadius::MAX,
            ..default()
        },
        BorderColor::all(Color::BLACK),
        BackgroundColor(NORMAL_BUTTON),
        children![(
            Text::new(text),
            TextFont {
                font: asset_server.load("fonts/Veyrwood.ttf").into(),
                font_size: FontSize::Px(33.0),
                ..default()
            },
            TextColor(Color::srgb(0.9, 0.9, 0.9)),
            TextShadow::default(),
        )],
    )
}

/// Restyle the button and update its label to reflect its current state.
///
/// The `Button` widget maintains a `Pressed` component while the button is held down, and the
/// picking backend keeps `Hovered` up to date. We simply read those each frame and pick a look.
fn update_button_appearance(
    mut buttons: Query<
        (
            &Hovered,
            Has<Pressed>,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        With<Button>,
    >,
) {
    for (hovered, pressed, mut color, mut border_color, children) in &mut buttons {
        match (hovered.get(), pressed) {
            // Pressed (and, since you can only press what you're hovering, also hovered).
            (_, true) => {
                *color = PRESSED_BUTTON.into();
                border_color.set_all(RED);
            }
            // Hovered but not pressed.
            (true, false) => {
                *color = HOVERED_BUTTON.into();
                border_color.set_all(WHITE);
            }
            // Neither hovered nor pressed.
            (false, false) => {
                *color = NORMAL_BUTTON.into();
                border_color.set_all(BLACK);
            }
        }
    }
}
