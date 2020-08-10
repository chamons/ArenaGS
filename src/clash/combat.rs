use specs::prelude::*;
use specs_derive::Component;

use super::*;
use crate::atlas::{EasyECS, EasyMutWorld, Point, SizedPoint};
use crate::clash::{EventCoordinator, FieldComponent, Logger};

#[derive(Clone, Copy)]
pub enum FieldKind {
    Fire,
}

#[derive(Clone, Copy)]
pub enum BoltKind {
    Bullet,
    Fire,
}

#[derive(Clone, Copy)]
pub enum WeaponKind {
    Sword,
}

#[derive(Clone, Copy)]
pub enum AttackKind {
    Ranged(BoltKind),
    Melee(WeaponKind),
    Field(FieldKind),
    Explode(u32),
}

#[derive(Clone, Copy)]
pub struct AttackInfo {
    pub strength: u32,
    pub target: Point,
    pub kind: AttackKind,
}

impl AttackInfo {
    pub fn ranged_kind(&self) -> BoltKind {
        match self.kind {
            AttackKind::Ranged(kind) => kind,
            _ => panic!("Wrong type in ranged_kind"),
        }
    }

    pub fn melee_kind(&self) -> WeaponKind {
        match self.kind {
            AttackKind::Melee(kind) => kind,
            _ => panic!("Wrong type in melee_kind"),
        }
    }

    pub fn field_kind(&self) -> FieldKind {
        match self.kind {
            AttackKind::Field(kind) => kind,
            _ => panic!("Wrong type in field_kind"),
        }
    }
}

#[derive(Component)]
pub struct AttackComponent {
    pub attack: AttackInfo,
}

impl AttackComponent {
    pub fn init(target: Point, strength: u32, kind: AttackKind) -> AttackComponent {
        AttackComponent {
            attack: AttackInfo { target, strength, kind },
        }
    }
}

pub fn begin_bolt(ecs: &mut World, source: &Entity, target_position: Point, strength: u32, kind: BoltKind) {
    ecs.shovel(*source, AttackComponent::init(target_position, strength, AttackKind::Ranged(kind)));
    ecs.raise_event(EventKind::Bolt(BoltState::BeginCast), Some(*source));
}

pub fn bolt_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Bolt(state) => match state {
            BoltState::CompleteCast => start_bolt(ecs, target.unwrap()),
            BoltState::CompleteFlying => apply_bolt(ecs, target.unwrap()),
            _ => {}
        },
        _ => {}
    }
}

pub fn start_bolt(ecs: &mut World, source: Entity) {
    let source_position = ecs.get_position(&source);
    let attack = ecs.read_storage::<AttackComponent>().grab(source).attack;

    let bolt = ecs
        .create_entity()
        .with(PositionComponent::init(source_position))
        .with(AttackComponent { attack })
        .build();

    ecs.write_storage::<AttackComponent>().remove(source);
    ecs.raise_event(EventKind::Bolt(BoltState::BeginFlying), Some(bolt));
}

pub fn apply_bolt(ecs: &mut World, bolt: Entity) {
    let attack = {
        let attacks = ecs.read_storage::<AttackComponent>();
        attacks.grab(bolt).attack
    };
    ecs.log(format!("Enemy was struck ({}) at ({},{})!", attack.strength, attack.target.x, attack.target.y).as_str());
    ecs.delete_entity(bolt).unwrap();
}

pub fn begin_melee(ecs: &mut World, source: &Entity, target: Point, strength: u32, kind: WeaponKind) {
    ecs.shovel(*source, AttackComponent::init(target, strength, AttackKind::Melee(kind)));
    ecs.raise_event(EventKind::Melee(MeleeState::Begin), Some(*source));
}

pub fn melee_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if matches!(kind, EventKind::Melee(state) if state.is_complete()) {
        apply_melee(ecs, target.unwrap());
    }
}

pub fn apply_melee(ecs: &mut World, character: Entity) {
    let attack = {
        let attacks = ecs.read_storage::<AttackComponent>();
        attacks.grab(character).attack
    };
    ecs.log(format!("Enemy was struck ({}) in melee at ({},{})!", attack.strength, attack.target.x, attack.target.y).as_str());

    ecs.write_storage::<AttackComponent>().remove(character);
}

pub fn begin_field(ecs: &mut World, source: &Entity, target: Point, strength: u32, kind: FieldKind) {
    ecs.shovel(*source, AttackComponent::init(target, strength, AttackKind::Field(kind)));
    ecs.raise_event(EventKind::Field(FieldState::BeginCast), Some(*source));
}

pub fn field_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    match kind {
        EventKind::Field(state) => match state {
            FieldState::CompleteCast => start_field(ecs, target.unwrap()),
            FieldState::CompleteFlying => apply_field(ecs, target.unwrap()),
            _ => {}
        },
        _ => {}
    }
}

pub fn start_field(ecs: &mut World, source: Entity) {
    let source_position = ecs.get_position(&source);
    let attack = ecs.read_storage::<AttackComponent>().grab(source).attack;

    let field_projectile = ecs
        .create_entity()
        .with(PositionComponent::init(source_position))
        .with(AttackComponent { attack })
        .build();

    ecs.write_storage::<AttackComponent>().remove(source);
    ecs.raise_event(EventKind::Field(FieldState::BeginFlying), Some(field_projectile));
}

pub fn apply_field(ecs: &mut World, projectile: Entity) {
    let attack = {
        let attacks = ecs.read_storage::<AttackComponent>();
        attacks.grab(projectile).attack
    };
    let (r, g, b) = match attack.field_kind() {
        FieldKind::Fire => (255, 0, 0),
    };

    ecs.create_entity()
        .with(PositionComponent::init(SizedPoint::init(attack.target.x, attack.target.y)))
        .with(AttackComponent::init(attack.target, attack.strength, AttackKind::Explode(0)))
        .with(BehaviorComponent::init(BehaviorKind::Explode))
        .with(FieldComponent::init(r, g, b))
        .with(TimeComponent::init(-BASE_ACTION_COST))
        .build();
    ecs.delete_entity(projectile).unwrap();
}

pub fn begin_explode(ecs: &mut World, source: &Entity) {
    ecs.raise_event(EventKind::Explode(ExplodeState::Begin), Some(*source));
}

pub fn explode_event(ecs: &mut World, kind: EventKind, target: Option<Entity>) {
    if matches!(kind, EventKind::Explode(state) if state.is_complete()) {
        apply_explode(ecs, target.unwrap());
    }
}

pub fn apply_explode(ecs: &mut World, source: Entity) {
    let (strength, range) = {
        let attack_info = ecs.read_storage::<AttackComponent>().grab(source).attack;
        match attack_info.kind {
            AttackKind::Explode(range) => (attack_info.strength, range),
            _ => panic!("Explode with wrong AttackKind"),
        }
    };

    for in_blast in ecs.get_position(&source).origin.get_burst(range) {
        if let Some(target) = find_character_at_location(ecs, in_blast) {
            if target != source {
                ecs.log(format!("Struct by blast ({}) at {}", strength, in_blast).as_str());
            }
        }
    }

    ecs.delete_entity(source).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn melee_logs_on_hit() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(3, 2, 10).build();
        let player = find_player(&ecs);

        assert_eq!(0, ecs.read_resource::<LogComponent>().count());

        begin_melee(&mut ecs, &player, Point::init(3, 2), 1, WeaponKind::Sword);
        wait_for_animations(&mut ecs);

        assert_eq!(1, ecs.read_resource::<LogComponent>().count());
    }

    #[test]
    fn ranged_logs_on_hit() {
        let mut ecs = create_test_state().with_player(2, 2, 100).with_character(4, 2, 10).build();
        let player = find_player(&ecs);

        assert_eq!(0, ecs.read_resource::<LogComponent>().count());

        begin_bolt(&mut ecs, &player, Point::init(4, 2), 1, BoltKind::Fire);
        wait_for_animations(&mut ecs);

        assert_eq!(1, ecs.read_resource::<LogComponent>().count());
    }

    #[test]
    fn exbplode_logs_on_hit() {
        let mut ecs = create_test_state().with_character(2, 2, 0).with_character(2, 3, 0).with_map().build();
        let exploder = find_at(&mut ecs, 2, 3);
        ecs.shovel(exploder, AttackComponent::init(Point::init(2, 3), 2, AttackKind::Explode(1)));
        begin_explode(&mut ecs, &exploder);
        wait_for_animations(&mut ecs);

        assert_eq!(1, ecs.read_resource::<LogComponent>().count());
    }

    fn get_field_at(ecs: &World, target: &Point) -> Option<Entity> {
        let entities = ecs.read_resource::<specs::world::EntitiesRes>();
        let positions = ecs.read_storage::<PositionComponent>();
        let fields = ecs.read_storage::<FieldComponent>();

        for (entity, position, _) in (&entities, &positions, &fields).join() {
            if *target == position.position.origin {
                return Some(entity);
            }
        }
        None
    }

    #[test]
    fn field_is_placed() {
        let mut ecs = create_test_state().with_character(2, 2, 0).with_character(2, 3, 0).with_map().build();
        let player = find_at(&mut ecs, 2, 2);

        begin_field(&mut ecs, &player, Point::init(2, 3), 1, FieldKind::Fire);
        wait_for_animations(&mut ecs);

        assert_eq!(true, get_field_at(&ecs, &Point::init(2, 3)).is_some());
    }
}
