use specs::prelude::*;

use super::components::*;
use super::{add_animation, Animation};

use crate::after_image::CharacterAnimationState;
use crate::atlas::{EasyECS, EasyMutWorld, Point};
use crate::clash::*;

pub fn battle_animation_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Bolt(state) => match state {
            BoltState::BeginCastAnimation => begin_ranged_cast_animation(ecs, target.unwrap()),
            BoltState::BeginFlyingAnimation => begin_ranged_bolt_animation(ecs, target.unwrap()),
            _ => {}
        },
        EventKind::Orb(state) => match state {
            OrbState::BeginCastAnimation => begin_orb_cast_animation(ecs, target.unwrap()),
            OrbState::Created => create_orb_sprite(ecs, target.unwrap()),
            _ => {}
        },
        _ => {}
    }
}

pub fn cast_animation(ecs: &mut World, target: Entity, animation: CharacterAnimationState, post_event: EventKind) {
    let frame = ecs.get_current_frame();

    const CAST_LENGTH: u64 = 18;
    let cast_animation = Animation::sprite_state(animation, CharacterAnimationState::Idle, frame, CAST_LENGTH).with_post_event(post_event, Some(target));
    add_animation(ecs, target, cast_animation);
}

pub fn projectile_animation(ecs: &mut World, projectile: Entity, target_position: Point, sprite: SpriteKinds, post_event: EventKind) {
    let frame = ecs.get_current_frame();
    ecs.shovel(projectile, RenderComponent::init(RenderInfo::init(sprite)));

    let source_position = ecs.get_position(&projectile);

    let path_length = source_position.distance_to(target_position).unwrap() as u64;
    let animation_length = if frame < 4 { 6 * path_length } else { 3 * path_length };

    let animation = Animation::movement(source_position.origin, target_position, frame, animation_length).with_post_event(post_event, Some(projectile));
    add_animation(ecs, projectile, animation);
}

pub fn begin_ranged_cast_animation(ecs: &mut World, target: Entity) {
    let animation = {
        let attacks = ecs.read_storage::<AttackComponent>();
        match attacks.grab(target).attack.ranged_kind() {
            BoltKind::Fire => CharacterAnimationState::Magic,
            BoltKind::Water => CharacterAnimationState::Magic,
            BoltKind::Lightning => CharacterAnimationState::Magic,
            BoltKind::Bullet => CharacterAnimationState::Crouch,
            BoltKind::FireBullet => CharacterAnimationState::Crouch,
            BoltKind::AirBullet => CharacterAnimationState::Crouch,
            BoltKind::Smoke => CharacterAnimationState::Magic,
        }
    };

    cast_animation(ecs, target, animation, EventKind::Bolt(BoltState::CompleteCastAnimation));
}

pub fn begin_orb_cast_animation(ecs: &mut World, target: Entity) {
    let animation = {
        let attacks = ecs.read_storage::<AttackComponent>();
        match attacks.grab(target).attack.orb_kind() {
            OrbKind::Feather => CharacterAnimationState::Magic,
        }
    };

    cast_animation(ecs, target, animation, EventKind::Orb(OrbState::CompleteCastAnimation));
}

pub fn create_orb_sprite(ecs: &mut World, orb: Entity) {
    let sprite = {
        let attacks = ecs.write_storage::<AttackComponent>();
        match attacks.grab(orb).attack.orb_kind() {
            OrbKind::Feather => SpriteKinds::AirBullet,
        }
    };
    ecs.shovel(orb, RenderComponent::init(RenderInfo::init(sprite)));
}

pub fn begin_ranged_bolt_animation(ecs: &mut World, bolt: Entity) {
    let (target, sprite) = {
        let attacks = ecs.write_storage::<AttackComponent>();
        let attack = attacks.grab(bolt).attack;
        let sprite = match attack.ranged_kind() {
            BoltKind::Fire => SpriteKinds::FireBolt,
            BoltKind::Water => SpriteKinds::WaterBolt,
            BoltKind::Lightning => SpriteKinds::LightningOrb,
            BoltKind::Bullet => SpriteKinds::Bullet,
            BoltKind::FireBullet => SpriteKinds::FireBullet,
            BoltKind::AirBullet => SpriteKinds::AirBullet,
            BoltKind::Smoke => SpriteKinds::Smoke,
        };
        (attack.target, sprite)
    };
    projectile_animation(ecs, bolt, target, sprite, EventKind::Bolt(BoltState::CompleteFlyingAnimation));
}

pub fn field_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Field(state) => match state {
            FieldState::BeginCastAnimation => begin_ranged_cast_field_animation(ecs, target.unwrap()),
            FieldState::BeginFlyingAnimation => begin_ranged_field_animation(ecs, target.unwrap()),
            _ => {}
        },
        _ => {}
    }
}

pub fn begin_ranged_cast_field_animation(ecs: &mut World, target: Entity) {
    let animation = {
        let field_casts = ecs.read_storage::<FieldCastComponent>();
        match field_casts.grab(target).kind {
            FieldKind::Fire => CharacterAnimationState::Crouch,
            FieldKind::Hail => CharacterAnimationState::Crouch,
        }
    };
    cast_animation(ecs, target, animation, EventKind::Field(FieldState::CompleteCastAnimation));
}

pub fn begin_ranged_field_animation(ecs: &mut World, bolt: Entity) {
    let (target, sprite) = {
        let field_casts = ecs.read_storage::<FieldCastComponent>();
        let cast = field_casts.grab(bolt);
        let sprite = match cast.kind {
            FieldKind::Fire => SpriteKinds::FireBolt,
            FieldKind::Hail => SpriteKinds::NoBolt,
        };
        (cast.target.origin, sprite)
    };
    projectile_animation(ecs, bolt, target, sprite, EventKind::Field(FieldState::CompleteFlyingAnimation));
}

pub fn melee_cone_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if matches!(kind, EventKind::Melee(state) if state.is_begin_animation()) {
        begin_weapon_animation(ecs, target.unwrap(), EventKind::Melee(MeleeState::CompleteAnimation));
    } else if matches!(kind, EventKind::Cone(state) if state.is_begin_swing_animation()) {
        begin_weapon_animation(ecs, target.unwrap(), EventKind::Cone(ConeState::CompleteSwingAnimation));
    } else if matches!(kind, EventKind::Cone(state) if state.is_begin_hit_animation()) {
        begin_cone_hit_animation(ecs, target.unwrap());
    }
}

fn begin_weapon_animation(ecs: &mut World, target: Entity, post_event: EventKind) {
    let frame = ecs.get_current_frame();

    const MELEE_ATTACK_LENGTH: u64 = 18;
    let attack_animation = Animation::sprite_state(CharacterAnimationState::AttackTwo, CharacterAnimationState::Idle, frame, MELEE_ATTACK_LENGTH)
        .with_post_event(post_event, Some(target));
    add_animation(ecs, target, attack_animation);
}

fn begin_cone_hit_animation(ecs: &mut World, entity: Entity) {
    let frame = ecs.get_current_frame();
    let sprite = {
        match ecs.read_storage::<AttackComponent>().grab(entity).attack.cone_kind() {
            ConeKind::Fire => SpriteKinds::FireBolt,
            ConeKind::Water => SpriteKinds::WaterBolt,
        }
    };
    let target = ecs.get_position(&entity).single_position();
    ecs.shovel(entity, RenderComponent::init(RenderInfo::init(sprite)));

    let animation = Animation::movement(target, target, frame, 18).with_post_event(EventKind::Cone(ConeState::CompleteHitAnimation), Some(entity));
    add_animation(ecs, entity, animation);
}

pub fn move_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Move(state, action) => {
            if state.is_begin_animation() {
                animate_move(ecs, target.unwrap(), action);
            }
        }
        _ => {}
    }
}

fn animate_move(ecs: &mut World, target: Entity, action: PostMoveAction) {
    let new_position = {
        let movements = ecs.read_storage::<MovementComponent>();
        movements.grab(target).new_position
    };

    const MOVE_LENGTH: u64 = 8;
    let frame = ecs.get_current_frame();
    let position = ecs.get_position(&target);

    let animation = Animation::movement(position.origin, new_position.origin, frame, MOVE_LENGTH)
        .with_post_event(EventKind::Move(MoveState::CompleteAnimation, action), Some(target));
    add_animation(ecs, target, animation);
}

pub fn explode_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if matches!(kind, EventKind::Explode(state) if state.is_begin_animation()) {
        begin_explode_animation(ecs, target.unwrap());
    }
}

pub fn begin_explode_animation(ecs: &mut World, target: Entity) {
    let frame = ecs.get_current_frame();
    let attack_info = ecs.read_storage::<AttackComponent>().grab(target).attack;
    let sprite = match attack_info.explode_kind() {
        ExplosionKind::Bomb => SpriteKinds::Explosion,
        ExplosionKind::Cloud => SpriteKinds::Cloud,
        ExplosionKind::Lightning => SpriteKinds::LightningStrike,
        ExplosionKind::Fire => SpriteKinds::FireColumn,
        ExplosionKind::Water => SpriteKinds::WaterColumn,
    };
    ecs.shovel(target, RenderComponent::init(RenderInfo::init(sprite)));
    ecs.write_storage::<FieldComponent>().remove(target);

    const EXPLOSION_LENGTH: u64 = 22;
    let attack_animation = Animation::empty(frame, EXPLOSION_LENGTH).with_post_event(EventKind::Explode(ExplodeState::CompleteAnimation), Some(target));
    add_animation(ecs, target, attack_animation);
}
