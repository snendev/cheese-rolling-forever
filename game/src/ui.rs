// global ui utilities

use bevy::prelude::*;

pub struct CheeseUIPlugin;

impl Plugin for CheeseUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_button_activity);
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

pub fn button() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        background_color: NORMAL_BUTTON.into(),
        ..Default::default()
    }
}

fn handle_button_activity(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), Changed<Interaction>>,
) {
    for (interaction, mut bg_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                bg_color.0 = PRESSED_BUTTON;
            }
            Interaction::Hovered => {
                bg_color.0 = HOVERED_BUTTON;
            }
            _ => {
                bg_color.0 = NORMAL_BUTTON;
            }
        }
    }
}
