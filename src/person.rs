use bevy::prelude::*;
use bevy_xpbd_3d::prelude::*;

// TODO build a lakitu

#[derive(Clone, Copy)]
#[derive(Component)]
pub struct Person {
    size: f32,
    girth: f32,
}

impl Default for Person {
    fn default() -> Self {
        Self::new(1., 1.)
    }
}

impl Person {
    const BODY_MASS_DENSITY: f32 = 5e5;
    const LIMB_MASS_DENSITY: f32 = 1e4;
    const BASE_HEAD_RADIUS: f32 = 0.09;

    const BASE_TORSO_LENGTH: f32 = 0.5;
    const BASE_TORSO_RADIUS: f32 = 0.1;

    const BASE_LIMB_LENGTH: f32 = 0.6;
    const BASE_LIMB_RADIUS: f32 = 0.05;
    const ARM_TO_LEG_RATIO: f32 = 0.75;

    pub fn new(size: f32, girth: f32) -> Self {
        Self { size, girth }
    }

    pub fn spawn_ragdoll(
        self,
        origin: Vec3,
        commands: &mut Commands,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
    ) {
        /*** body ***/
        // head
        let head_radius = Self::BASE_HEAD_RADIUS * self.girth;
        let head_shape = shape::UVSphere {
            radius: head_radius,
            ..Default::default()
        };
        let head_collider = Collider::ball(head_radius);

        // torso
        let torso_radius = Self::BASE_TORSO_RADIUS * self.girth;
        let torso_height = Self::BASE_TORSO_LENGTH * self.size - torso_radius * 2.;
        let torso_shape = shape::Capsule {
            radius: torso_radius,
            depth: torso_height,
            ..Default::default()
        };
        let torso_collider = Collider::cylinder(torso_height, torso_radius);
        /*** ***/

        /*** limbs ***/
        let limb_radius = Self::BASE_LIMB_RADIUS * self.girth;

        // arm
        let arm_height =
            Self::BASE_LIMB_LENGTH * self.size * Self::ARM_TO_LEG_RATIO - limb_radius * 2.;
        let arm_shape = shape::Capsule {
            radius: limb_radius,
            depth: arm_height,
            ..Default::default()
        };
        let arm_collider = Collider::capsule(arm_height, limb_radius);

        // hand
        let hand_size = Self::BASE_LIMB_RADIUS * 1.1;
        let hand_shape = shape::UVSphere {
            radius: hand_size,
            ..Default::default()
        };
        let hand_collider = Collider::ball(hand_size);

        // leg
        let leg_height = Self::BASE_LIMB_LENGTH * self.size - limb_radius * 2.;
        let leg_shape = shape::Capsule {
            radius: limb_radius,
            depth: leg_height,
            ..Default::default()
        };
        let leg_collider = Collider::capsule(leg_height, limb_radius);
        /*** ***/

        /*** spawn the person ***/
        let leg_total_height = leg_height + limb_radius * 2.;
        let head = commands
            .spawn((
                Head,
                RigidBody::Dynamic,
                Name::new("Head"),
                head_collider,
                ColliderDensity(Self::BODY_MASS_DENSITY),
                PbrBundle {
                    mesh: meshes.add(head_shape.into()),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(
                        Vec3::Y * (torso_height * 0.5 + torso_radius + head_radius * 0.5),
                    ),
                    ..Default::default()
                },
            ))
            .id();

        let left_arm = commands
            .spawn((
                Arm,
                RigidBody::Dynamic,
                Name::new("Left Arm"),
                arm_collider.clone(),
                ColliderDensity(Self::LIMB_MASS_DENSITY),
                PbrBundle {
                    mesh: meshes.add(arm_shape.into()),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(Vec3::new(
                        -torso_radius - limb_radius * 2.,
                        torso_height / 2. - torso_radius * 0.4,
                        0.,
                    )),
                    ..Default::default()
                },
            ))
            .id();
        let right_arm = commands
            .spawn((
                Arm,
                RigidBody::Dynamic,
                Name::new("Right Arm"),
                arm_collider,
                ColliderDensity(Self::LIMB_MASS_DENSITY),
                PbrBundle {
                    mesh: meshes.add(arm_shape.into()),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(Vec3::new(
                        torso_radius + limb_radius * 2.,
                        torso_height / 2. - torso_radius * 0.4,
                        0.,
                    )),
                    ..Default::default()
                },
            ))
            .id();

        let left_hand = commands
            .spawn((
                Hand,
                RigidBody::Dynamic,
                Name::new("Left Hand"),
                hand_collider.clone(),
                ColliderDensity(Self::LIMB_MASS_DENSITY),
                PbrBundle {
                    mesh: meshes.add(hand_shape.into()),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(Vec3::new(
                        -torso_radius - arm_height - limb_radius * 2.,
                        torso_height / 2. - torso_radius * 0.4,
                        0.,
                    )),
                    ..Default::default()
                },
            ))
            .id();
        let right_hand = commands
            .spawn((
                Hand,
                RigidBody::Dynamic,
                Name::new("Right Hand"),
                hand_collider,
                ColliderDensity(Self::LIMB_MASS_DENSITY),
                PbrBundle {
                    mesh: meshes.add(hand_shape.into()),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(Vec3::new(
                        torso_radius + arm_height + limb_radius * 2.,
                        torso_height / 2. - torso_radius * 0.4,
                        0.,
                    )),
                    ..Default::default()
                },
            ))
            .id();

        let left_leg = commands
            .spawn((
                Leg,
                RigidBody::Dynamic,
                Name::new("Left Leg"),
                leg_collider.clone(),
                ColliderDensity(Self::LIMB_MASS_DENSITY),
                PbrBundle {
                    mesh: meshes.add(leg_shape.into()),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(Vec3::new(
                        -torso_radius + limb_radius * 0.8,
                        -torso_height / 2. - torso_radius - leg_total_height / 2.,
                        0.,
                    )),
                    ..Default::default()
                },
            ))
            .id();
        let right_leg = commands
            .spawn((
                Arm,
                RigidBody::Dynamic,
                Name::new("Right Leg"),
                leg_collider,
                ColliderDensity(Self::LIMB_MASS_DENSITY),
                PbrBundle {
                    mesh: meshes.add(leg_shape.into()),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(Vec3::new(
                        torso_radius - limb_radius * 0.8,
                        -torso_height / 2. - torso_radius - leg_total_height / 2.,
                        0.,
                    )),
                    ..Default::default()
                },
            ))
            .id();

        let body = commands
            .spawn((
                self,
                Name::new("Person"),
                RigidBody::Dynamic,
                torso_collider,
                ColliderDensity(Self::BODY_MASS_DENSITY),
                GravityScale(1.5),
                PbrBundle {
                    mesh: meshes.add(torso_shape.into()),
                    material: materials.add(Color::RED.into()),
                    transform: Transform::from_translation(origin),
                    ..Default::default()
                },
            ))
            .add_child(head)
            .add_child(left_arm)
            .add_child(right_arm)
            .add_child(left_hand)
            .add_child(right_hand)
            .add_child(left_leg)
            .add_child(right_leg)
            .id();

        /*** joints ***/
        // head-torso
        let neck_joint = commands
            .spawn((
                Name::new("Neck Joint"),
                SphericalJoint {
                    twist_axis: Vec3::Z,
                    local_anchor1: Vec3::Y * torso_height,
                    local_anchor2: Vec3::NEG_Y * head_radius * 0.4,
                    compliance: 1e-7,
                    swing_limit: Some(AngleLimit::new(
                        -std::f32::consts::FRAC_PI_3,
                        std::f32::consts::FRAC_PI_3,
                    )),
                    ..SphericalJoint::new(body, head)
                },
            ))
            .id();

        // torso-arm
        let left_shoulder_joint = commands
            .spawn((
                Name::new("Left Shoulder Joint"),
                SphericalJoint {
                    swing_axis: Vec3::NEG_X,
                    twist_axis: Vec3::Y,
                    local_anchor1: Vec3::new(-torso_radius - limb_radius, torso_height / 2., 0.),
                    local_anchor2: Vec3::Y * (arm_height / 2. + limb_radius),
                    compliance: 1e-7,
                    swing_limit: Some(AngleLimit::new(
                        -std::f32::consts::FRAC_PI_3,
                        std::f32::consts::FRAC_PI_3,
                    )),
                    ..SphericalJoint::new(body, left_arm)
                },
            ))
            .id();
        let right_shoulder_joint = commands
            .spawn((
                Name::new("Right Shoulder Joint"),
                SphericalJoint {
                    swing_axis: Vec3::X,
                    twist_axis: Vec3::Y,
                    local_anchor1: Vec3::new(torso_radius + limb_radius, torso_height / 2., 0.),
                    local_anchor2: Vec3::NEG_Y * (arm_height / 2. + limb_radius),
                    compliance: 1e-7,
                    swing_limit: Some(AngleLimit::new(
                        -std::f32::consts::FRAC_PI_3,
                        std::f32::consts::FRAC_PI_3,
                    )),
                    ..SphericalJoint::new(body, right_arm)
                },
            ))
            .id();

        // arm-hand
        let left_wrist_joint = commands
            .spawn((
                Name::new("Left Wrist Joint"),
                FixedJoint::new(left_arm, left_hand)
                    .with_local_anchor_1(Vec3::NEG_Y * (arm_height / 2. + limb_radius))
                    .with_local_anchor_2(Vec3::ZERO)
                    .with_compliance(1e-7),
            ))
            .id();
        let right_wrist_joint = commands
            .spawn((
                Name::new("Right Wrist Joint"),
                FixedJoint::new(right_arm, right_hand)
                    .with_local_anchor_1(Vec3::Y * (arm_height / 2. + limb_radius))
                    .with_local_anchor_2(Vec3::ZERO)
                    .with_compliance(1e-7),
            ))
            .id();

        // hip joint
        let left_hip_joint = commands
            .spawn((
                Name::new("Left Hip Joint"),
                SphericalJoint {
                    local_anchor1: Vec3::new(
                        -torso_radius / 1.7,
                        -torso_height / 2. - torso_radius / 1.7,
                        0.,
                    ),
                    local_anchor2: Vec3::Y * leg_height / 2.,
                    compliance: 1e-7,
                    swing_limit: Some(AngleLimit::new(
                        -std::f32::consts::FRAC_PI_3,
                        std::f32::consts::FRAC_PI_3,
                    )),
                    ..SphericalJoint::new(body, left_leg)
                },
            ))
            .id();
        let right_hip_joint = commands
            .spawn((
                Name::new("Right Hip Joint"),
                SphericalJoint {
                    local_anchor1: Vec3::new(
                        torso_radius / 1.7,
                        -torso_height / 2. - torso_radius / 1.7,
                        0.,
                    ),
                    local_anchor2: Vec3::Y * leg_height / 2.,
                    compliance: 1e-7,
                    swing_limit: Some(AngleLimit::new(
                        -std::f32::consts::FRAC_PI_3,
                        std::f32::consts::FRAC_PI_3,
                    )),
                    ..SphericalJoint::new(body, right_leg)
                },
            ))
            .id();

        commands
            .entity(body)
            .add_child(neck_joint)
            .add_child(left_shoulder_joint)
            .add_child(right_shoulder_joint)
            .add_child(left_wrist_joint)
            .add_child(right_wrist_joint)
            .add_child(left_hip_joint)
            .add_child(right_hip_joint);
    }
}

#[derive(Component)]
pub(crate) struct Head;
#[derive(Component)]
pub(crate) struct Arm;
#[derive(Component)]
pub(crate) struct Hand;
#[derive(Component)]
pub(crate) struct Leg;
