use specs::prelude::*;

use super::components::*;
use super::{AnimationComponent, SecondaryAnimation};

use crate::after_image::CharacterAnimationState;
use crate::atlas::{EasyECS, EasyMutECS, EasyMutWorld};
use crate::clash::*;

pub fn begin_ranged_cast_animation(ecs: &mut World, target: &Entity) {
    let frame = ecs.get_current_frame();
    let animation = {
        let attacks = ecs.read_storage::<AttackComponent>();
        match attacks.grab(*target).attack.ranged_kind() {
            BoltKind::Fire => CharacterAnimationState::Magic,
            BoltKind::Bullet => CharacterAnimationState::Crouch,
        }
    };

    const RANGED_CAST_LENGTH: u64 = 18;
    let cast_animation =
        AnimationComponent::sprite_state(animation, CharacterAnimationState::Idle, frame, RANGED_CAST_LENGTH).with_secondary(SecondaryAnimation::StartBolt);
    ecs.shovel(*target, cast_animation);
}

pub fn begin_melee_animation(ecs: &mut World, target: &Entity) {
    let frame = ecs.get_current_frame();
    let animation = {
        let attacks = ecs.read_storage::<AttackComponent>();
        match attacks.grab(*target).attack.melee_kind() {
            WeaponKind::Sword => CharacterAnimationState::AttackTwo,
        }
    };

    const MELEE_ATTACK_LENGTH: u64 = 18;
    let attack_animation = AnimationComponent::sprite_state(animation, CharacterAnimationState::Idle, frame, MELEE_ATTACK_LENGTH);
    ecs.shovel(*target, attack_animation);

    let attack_effect = DelayedEffectComponent::init(DelayedEffect::init(DelayedEffectKind::ApplyMelee, frame, MELEE_ATTACK_LENGTH));
    ecs.shovel(*target, attack_effect);
}

pub fn begin_ranged_bolt_animation(ecs: &mut World, target: &Entity) {
    let frame = ecs.get_current_frame();
    let bolt = start_bolt(ecs, &target);
    let sprite = {
        let attacks = ecs.write_storage::<AttackComponent>();
        match attacks.grab(bolt).attack.ranged_kind() {
            BoltKind::Fire => SpriteKinds::FireBolt,
            BoltKind::Bullet => SpriteKinds::BulletBolt,
        }
    };
    ecs.shovel(bolt, RenderComponent::init(sprite));

    let source_position = ecs.get_position(&bolt);
    let target_position = ecs.read_storage::<AttackComponent>().grab(bolt).attack.target;

    let path_length = source_position.distance_to(target_position).unwrap() as u64;
    let animation_length = if frame < 4 { 4 * path_length } else { 2 * path_length };

    let animation = AnimationComponent::movement(source_position.origin, target_position, frame, animation_length);
    ecs.shovel(bolt, animation);

    let bolt_effect = DelayedEffectComponent::init(DelayedEffect::init(DelayedEffectKind::ApplyBolt, frame, animation_length));
    ecs.shovel(bolt, bolt_effect);
}

pub fn move_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if matches!(kind, EventKind::Move(state) if state.is_begin()) {
        let target = target.unwrap();
        let new_position = {
            let movements = ecs.read_storage::<MovementComponent>();
            movements.grab(target).new_position
        };

        const MOVE_LENGTH: u64 = 8;
        let frame = ecs.get_current_frame();
        let position = ecs.get_position(&target);

        ecs.shovel(target, AnimationComponent::movement(position.origin, new_position.origin, frame, MOVE_LENGTH));
        ecs.raise_event(EventKind::Move(MoveState::Complete), Some(target));
    }
}
