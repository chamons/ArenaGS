use specs::prelude::*;

use super::components::*;
use crate::clash::*;

use crate::after_image::CharacterAnimationState;

pub fn begin_ranged_cast_animation(ecs: &mut World, target: &Entity, invoker: Entity) {
    let frame = ecs.get_current_frame();
    let animation = {
        let attacks = ecs.write_storage::<AttackComponent>();
        match attacks.get(*target).unwrap().attack.ranged_kind() {
            BoltKind::Fire => CharacterAnimationState::Magic,
        }
    };

    let cast_animation = AnimationComponent::sprite_state(animation, CharacterAnimationState::Idle, frame, 18).with_effect(PostAnimationEffect::StartBolt);
    ecs.write_storage::<AnimationComponent>().insert(invoker, cast_animation).unwrap();
}

pub fn begin_melee_animation(ecs: &mut World, target: &Entity, invoker: Entity) {
    let frame = ecs.get_current_frame();
    let animation = {
        let attacks = ecs.read_storage::<AttackComponent>();
        match attacks.get(*target).unwrap().attack.melee_kind() {
            WeaponKind::Sword => CharacterAnimationState::AttackTwo,
        }
    };

    let mut animations = ecs.write_storage::<AnimationComponent>();
    let attack_animation = AnimationComponent::sprite_state(animation, CharacterAnimationState::Idle, frame, 18).with_effect(PostAnimationEffect::ApplyMelee);
    animations.insert(invoker, attack_animation).unwrap();
}

pub fn begin_ranged_bolt_animation(ecs: &mut World, target: &Entity) {
    let frame = ecs.get_current_frame();
    let bolt = start_bolt(ecs, &target);
    let sprite = {
        let attacks = ecs.write_storage::<AttackComponent>();
        match attacks.get(bolt).unwrap().attack.ranged_kind() {
            BoltKind::Fire => SpriteKinds::FireBolt,
        }
    };
    ecs.write_storage::<RenderComponent>().insert(bolt, RenderComponent::init(sprite)).unwrap();

    let source_position = ecs.get_position(&bolt);
    let target_position = ecs.read_storage::<AttackComponent>().get(bolt).unwrap().attack.target;

    let path_length = source_position.distance_to(target_position).unwrap() as u64;
    let animation_length = if frame < 4 { 4 * path_length } else { 2 * path_length };

    let mut animations = ecs.write_storage::<AnimationComponent>();
    let animation = AnimationComponent::movement(source_position.origin, target_position, frame, animation_length).with_effect(PostAnimationEffect::ApplyBolt);
    animations.insert(bolt, animation).unwrap();
}
