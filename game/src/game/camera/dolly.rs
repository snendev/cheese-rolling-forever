// this code is based off
// https://github.com/BlackPhlox/bevy_dolly/commit/dcd1f44df1f853244783e24d19cc338411c44308
// but due to issues with compilation and their ongoing work around the bevy 0.12 release,
// a simplified approach is ported here

use bevy::prelude::*;
use dolly::{
    driver::RigDriverTraits,
    prelude::{CameraRig, RightHanded},
    rig::CameraRigBuilder,
};

pub use dolly;

#[derive(Component, Deref, DerefMut)]
pub struct Rig(CameraRig);

impl Rig {
    pub fn builder() -> RigBuilder {
        RigBuilder(CameraRig::builder())
    }
}

pub struct RigBuilder(CameraRigBuilder<RightHanded>);

impl RigBuilder {
    pub fn with(mut self, driver: impl RigDriverTraits<RightHanded>) -> Self {
        self.0 = self.0.with(driver);
        self
    }

    pub fn build(self) -> Rig {
        Rig(self.0.build())
    }
}

#[derive(Default)]
pub struct DollyPlugin<T: Component> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Component> DollyPlugin<T> {
    #[allow(clippy::type_complexity)]
    pub fn update_active(
        mut cameras: Query<(&mut Transform, &Camera), With<T>>,
        time: Res<Time>,
        mut query: Query<&mut Rig, (Changed<Rig>, With<T>)>,
    ) {
        for mut rig in &mut query {
            let transform = rig.update(time.delta_seconds());

            cameras.for_each_mut(|(mut t, camera)| {
                if camera.is_active {
                    let (translation, rotation) = transform.into_position_rotation();
                    *t = Transform {
                        translation,
                        rotation,
                        ..Default::default()
                    };
                }
            });
        }
    }
}

impl<T: Component> Plugin for DollyPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::update_active);
    }
}
