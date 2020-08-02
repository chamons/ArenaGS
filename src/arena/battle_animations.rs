use specs::prelude::*;

use super::components::*;
use super::AnimationComponent;

use crate::after_image::CharacterAnimationState;
use crate::atlas::{EasyECS, EasyMutECS};
use crate::clash::*;

pub fn begin_ranged_cast_animation(ecs: &mut World, target: &Entity) {
    let frame = ecs.get_current_frame();
    let animation = {
        let attacks = ecs.write_storage::<AttackComponent>();
        match attacks.grab(*target).attack.ranged_kind() {
            BoltKind::Fire => CharacterAnimationState::Magic,
            BoltKind::Bullet => CharacterAnimationState::Crouch,
        }
    };

    let cast_animation = AnimationComponent::sprite_state(animation, CharacterAnimationState::Idle, frame, 18).with_effect(PostAnimationEffect::StartBolt);
    ecs.write_storage::<AnimationComponent>().shovel(*target, cast_animation);
}

pub fn begin_melee_animation(ecs: &mut World, target: &Entity) {
    let frame = ecs.get_current_frame();
    let animation = {
        let attacks = ecs.read_storage::<AttackComponent>();
        match attacks.grab(*target).attack.melee_kind() {
            WeaponKind::Sword => CharacterAnimationState::AttackTwo,
        }
    };

    let mut animations = ecs.write_storage::<AnimationComponent>();
    let attack_animation = AnimationComponent::sprite_state(animation, CharacterAnimationState::Idle, frame, 18).with_effect(PostAnimationEffect::ApplyMelee);
    animations.shovel(*target, attack_animation);
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
    ecs.write_storage::<RenderComponent>().shovel(bolt, RenderComponent::init(sprite));

    let source_position = ecs.get_position(&bolt);
    let target_position = ecs.read_storage::<AttackComponent>().grab(bolt).attack.target;

    let path_length = source_position.distance_to(target_position).unwrap() as u64;
    let animation_length = if frame < 4 { 4 * path_length } else { 2 * path_length };

    let mut animations = ecs.write_storage::<AnimationComponent>();
    let animation = AnimationComponent::movement(source_position.origin, target_position, frame, animation_length).with_effect(PostAnimationEffect::ApplyBolt);
    animations.shovel(bolt, animation);
}

pub fn begin_move_animation(ecs: &mut World, target: &Entity) {
    let movements = ecs.read_storage::<MovementComponent>();
    let new_position = movements.grab(*target).new_position;

    let frame = ecs.get_current_frame();
    let position = ecs.get_position(target);
    let mut animations = ecs.write_storage::<AnimationComponent>();
    let animation = AnimationComponent::movement(position.origin, new_position.origin, frame, 8).with_effect(PostAnimationEffect::Move);
    animations.shovel(*target, animation);
}
