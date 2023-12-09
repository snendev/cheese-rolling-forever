use bevy::prelude::*;

use crate::Cheese;

#[derive(Component)]
pub(super) struct MenuGraphic;
#[derive(Component)]
pub(super) struct MenuGraphicCheese;

pub(super) fn spawn_graphic(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Name::new("Menu Graphic Scene"),
            MenuGraphic,
            SpatialBundle::default(),
        ))
        .with_children(|builder| {
            // cheese
            builder.spawn((
                Name::new("Cheese"),
                MenuGraphicCheese,
                Cheese::graphic(
                    Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
                    &mut meshes,
                    &mut materials,
                ),
            ));
            // lighting
            let spotlight = SpotLight {
                color: Color::rgb(0.93, 0.85, 0.74),
                ..Default::default()
            };
            builder.spawn((
                Name::new("Spotlight Right"),
                SpotLightBundle {
                    spot_light: spotlight.clone(),
                    transform: Transform::from_xyz(2., 3., 2.).looking_at(Vec3::ZERO, Vec3::Y),
                    ..Default::default()
                },
            ));
            builder.spawn((
                Name::new("Spotlight Left"),
                SpotLightBundle {
                    spot_light: spotlight,
                    transform: Transform::from_xyz(-2., 3., 2.).looking_at(Vec3::ZERO, Vec3::Y),
                    ..Default::default()
                },
            ));
            // a plane with the texture to frame the spotlight
            builder.spawn((
                Name::new("Floor"),
                PbrBundle {
                    mesh: meshes.add(
                        shape::Plane {
                            size: 500.,
                            ..Default::default()
                        }
                        .into(),
                    ),
                    material: materials.add(Color::DARK_GREEN.into()),
                    transform: Transform::from_translation(Vec3::Y * -Cheese::RADIUS),
                    ..Default::default()
                },
            ));
        });
}

pub(super) fn spin_graphic(mut query: Query<&mut Transform, With<MenuGraphicCheese>>) {
    const ROTATION_SPEED: f32 = std::f32::consts::FRAC_PI_8 * 0.03;
    for mut transform in query.iter_mut() {
        transform.rotate_y(ROTATION_SPEED);
    }
}
