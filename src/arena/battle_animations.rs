use specs::prelude::*;

use super::components::*;
use super::AnimationComponent;

use crate::after_image::CharacterAnimationState;
use crate::atlas::{EasyECS, EasyMutWorld};
use crate::clash::*;

pub fn bolt_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Bolt(state) => match state {
            BoltState::BeginCast => begin_ranged_cast_animation(ecs, target.unwrap()),
            BoltState::BeginFlying => begin_ranged_bolt_animation(ecs, target.unwrap()),
            _ => {}
        },
        _ => {}
    }
}

pub fn cast_animation(ecs: &mut World, target: Entity, animation: CharacterAnimationState, post_event: EventKind) {
    let frame = ecs.get_current_frame();

    const CAST_LENGTH: u64 = 18;
    let cast_animation =
        AnimationComponent::sprite_state(animation, CharacterAnimationState::Idle, frame, CAST_LENGTH).with_post_event(post_event, Some(target));
    ecs.shovel(target, cast_animation);
}

pub fn begin_ranged_cast_animation(ecs: &mut World, target: Entity) {
    let animation = {
        let attacks = ecs.read_storage::<AttackComponent>();
        match attacks.grab(target).attack.ranged_kind() {
            BoltKind::Fire => CharacterAnimationState::Magic,
            BoltKind::Bullet => CharacterAnimationState::Crouch,
        }
    };

    cast_animation(ecs, target, animation, EventKind::Bolt(BoltState::CompleteCast));
}

pub fn begin_ranged_bolt_animation(ecs: &mut World, bolt: Entity) {
    let frame = ecs.get_current_frame();
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

    let animation = AnimationComponent::movement(source_position.origin, target_position, frame, animation_length)
        .with_post_event(EventKind::Bolt(BoltState::CompleteFlying), Some(bolt));
    ecs.shovel(bolt, animation);
}

pub fn field_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Field(state) => match state {
            FieldState::BeginCast => begin_ranged_cast_field_animation(ecs, target.unwrap()),
            FieldState::BeginFlying => begin_ranged_field_animation(ecs, target.unwrap()),
            _ => {}
        },
        _ => {}
    }
}

pub fn begin_ranged_cast_field_animation(ecs: &mut World, target: Entity) {
    let animation = {
        let attacks = ecs.read_storage::<AttackComponent>();
        match attacks.grab(target).attack.field_kind() {
            FieldKind::Fire => CharacterAnimationState::Crouch,
        }
    };
    cast_animation(ecs, target, animation, EventKind::Field(FieldState::CompleteCast));
}

pub fn begin_ranged_field_animation(ecs: &mut World, bolt: Entity) {
    let frame = ecs.get_current_frame();
    let sprite = {
        let attacks = ecs.write_storage::<AttackComponent>();
        match attacks.grab(bolt).attack.field_kind() {
            FieldKind::Fire => SpriteKinds::Bomb,
        }
    };
    ecs.shovel(bolt, RenderComponent::init(sprite));

    let source_position = ecs.get_position(&bolt);
    let target_position = ecs.read_storage::<AttackComponent>().grab(bolt).attack.target;

    let path_length = source_position.distance_to(target_position).unwrap() as u64;
    let animation_length = if frame < 4 { 4 * path_length } else { 2 * path_length };

    let animation = AnimationComponent::movement(source_position.origin, target_position, frame, animation_length)
        .with_post_event(EventKind::Field(FieldState::CompleteFlying), Some(bolt));
    ecs.shovel(bolt, animation);
}

pub fn melee_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if matches!(kind, EventKind::Melee(state) if state.is_begin()) {
        begin_melee_animation(ecs, target.unwrap());
    }
}

pub fn begin_melee_animation(ecs: &mut World, target: Entity) {
    let frame = ecs.get_current_frame();
    let animation = {
        let attacks = ecs.read_storage::<AttackComponent>();
        match attacks.grab(target).attack.melee_kind() {
            WeaponKind::Sword => CharacterAnimationState::AttackTwo,
        }
    };

    const MELEE_ATTACK_LENGTH: u64 = 18;
    let attack_animation = AnimationComponent::sprite_state(animation, CharacterAnimationState::Idle, frame, MELEE_ATTACK_LENGTH)
        .with_post_event(EventKind::Melee(MeleeState::Complete), Some(target));
    ecs.shovel(target, attack_animation);
}

pub fn move_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if matches!(kind, EventKind::Move(state) if state.is_begin()) {
        animate_move(ecs, target.unwrap());
    }
}

fn animate_move(ecs: &mut World, target: Entity) {
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

pub fn explode_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if matches!(kind, EventKind::Explode(state) if state.is_begin()) {
        begin_explode_animation(ecs, target.unwrap());
    }
}

pub fn begin_explode_animation(ecs: &mut World, target: Entity) {
    let frame = ecs.get_current_frame();
    ecs.shovel(target, RenderComponent::init(SpriteKinds::Explosion));

    const EXPLOSION_LENGTH: u64 = 18;
    let attack_animation = AnimationComponent::empty(frame, EXPLOSION_LENGTH).with_post_event(EventKind::Explode(ExplodeState::Complete), Some(target));
    ecs.shovel(target, attack_animation);
}
