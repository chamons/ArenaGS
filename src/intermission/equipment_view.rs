use std::cell::RefCell;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use sdl2::mouse::{MouseButton, MouseState};
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::skilltree_view::{get_tree, get_tree_icons, SKILL_NODE_SIZE};
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{EquipmentKinds, ProgressionState, SkillTree};
use crate::props::{Button, HitTestResult, View};

pub struct CardView {
    frame: SDLRect,
    text_renderer: Rc<TextRenderer>,
    ui: Rc<IconCache>,
    icons: Rc<IconCache>,
    name: String,
    image: Option<String>,
    grabbed: Option<SDLPoint>,
    pub z_order: u32,
}

const CARD_WIDTH: u32 = 110;
const CARD_HEIGHT: u32 = 110;

impl CardView {
    pub fn init(
        position: SDLPoint,
        text_renderer: &Rc<TextRenderer>,
        ui: &Rc<IconCache>,
        icons: &Rc<IconCache>,
        name: &str,
        image: &Option<String>,
    ) -> BoxResult<CardView> {
        Ok(CardView {
            frame: SDLRect::new(position.x(), position.y(), CARD_WIDTH, CARD_HEIGHT),
            text_renderer: Rc::clone(&text_renderer),
            ui: Rc::clone(&ui),
            icons: Rc::clone(&icons),
            name: name.to_string(),
            grabbed: None,
            image: image.clone(),
            z_order: 0,
        })
    }
}

impl View for CardView {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.copy(self.ui.get("card_frame.png"), None, self.frame)?;

        if let Some(image) = &self.image {
            canvas.copy(
                self.icons.get(&image),
                None,
                SDLRect::new(
                    (self.frame.x() + (CARD_WIDTH as i32 / 2) - (SKILL_NODE_SIZE as i32 / 2)) as i32,
                    self.frame.y() + 20,
                    SKILL_NODE_SIZE,
                    SKILL_NODE_SIZE,
                ),
            )?;
        }

        self.text_renderer.render_text_centered(
            &self.name,
            self.frame.x(),
            self.frame.y() + 75,
            CARD_WIDTH,
            canvas,
            FontSize::Bold,
            FontColor::Brown,
        )?;

        Ok(())
    }

    fn handle_mouse_click(&mut self, _ecs: &World, x: i32, y: i32, button: Option<MouseButton>) {
        if let Some(button) = button {
            if button == MouseButton::Left {
                if self.frame.contains_point(SDLPoint::new(x, y)) {
                    self.grabbed = Some(SDLPoint::new(x - self.frame.x(), y - self.frame.y()));
                    return;
                }
            }
        }
    }

    fn handle_mouse_move(&mut self, _ecs: &World, x: i32, y: i32, state: MouseState) {
        if let Some(origin) = self.grabbed {
            if state.left() {
                self.frame = SDLRect::new(x - origin.x(), y - origin.y(), CARD_WIDTH, CARD_HEIGHT);
            } else {
                self.grabbed = None;
            }
        }
    }

    fn hit_test(&self, _ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        if self.frame.contains_point(SDLPoint::new(x, y)) {
            Some(HitTestResult::Skill(self.name.clone()))
        } else {
            None
        }
    }
}

pub struct EquipmentSlotView {
    frame: SDLRect,
    ui: Rc<IconCache>,
    kind: EquipmentKinds,
}

const EQUIPMENT_SLOT_OFFSET: u32 = 2;
impl EquipmentSlotView {
    pub fn init(position: SDLPoint, ui: &Rc<IconCache>, kind: EquipmentKinds) -> EquipmentSlotView {
        EquipmentSlotView {
            frame: SDLRect::new(
                position.x(),
                position.y(),
                CARD_WIDTH + EQUIPMENT_SLOT_OFFSET * 2,
                CARD_HEIGHT + EQUIPMENT_SLOT_OFFSET * 2,
            ),
            ui: Rc::clone(ui),
            kind,
        }
    }
}

impl View for EquipmentSlotView {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        let equipment_frame = self.ui.get(match self.kind {
            EquipmentKinds::Weapon => "equipment_weapon_slot.png",
            EquipmentKinds::Armor => "equipment_armor_slot.png",
            EquipmentKinds::Accessory => "equipment_accessory_slot.png",
            EquipmentKinds::Mastery => "equipment_mastery_slot.png",
        });
        canvas.copy(equipment_frame, None, self.frame)?;

        Ok(())
    }
}

pub struct EquipmentView {
    should_sort: Rc<RefCell<bool>>,
    tree: SkillTree,
    ui: Rc<IconCache>,
    icons: Rc<IconCache>,
    text_renderer: Rc<TextRenderer>,
    cards: RefCell<Vec<Box<CardView>>>,
    slots: Vec<Box<EquipmentSlotView>>,
    sort: Button,
    max_z_order: u32,
    needs_z_reorder: RefCell<bool>,
}

impl EquipmentView {
    pub fn init(render_context: &RenderContext, text_renderer: &Rc<TextRenderer>, progression: &ProgressionState) -> BoxResult<EquipmentView> {
        let tree = SkillTree::init(&get_tree(&progression.weapon));
        let ui = Rc::new(IconCache::init(
            &render_context,
            IconLoader::init_ui(),
            &[
                "card_frame.png",
                "equipment_weapon_slot.png",
                "equipment_armor_slot.png",
                "equipment_accessory_slot.png",
                "equipment_mastery_slot.png",
            ],
        )?);
        let should_sort = Rc::new(RefCell::new(false));

        let view = EquipmentView {
            icons: Rc::new(get_tree_icons(render_context, &tree)?),
            tree,
            text_renderer: Rc::clone(text_renderer),
            cards: RefCell::new(vec![]),
            should_sort: Rc::clone(&should_sort),
            needs_z_reorder: RefCell::new(false),
            sort: Button::text(
                SDLPoint::new(650, 650),
                "Sort",
                render_context,
                text_renderer,
                true,
                true,
                None,
                Some(Box::new(move || *should_sort.borrow_mut() = true)),
            )?,
            slots: EquipmentView::create_slots(progression, &ui),
            ui,
            max_z_order: 1,
        };
        Ok(view)
    }

    fn create_slots(progression: &ProgressionState, ui: &Rc<IconCache>) -> Vec<Box<EquipmentSlotView>> {
        let mut slots = vec![];

        for i in 0..progression.equipment.weapon_count {
            slots.push(Box::from(EquipmentSlotView::init(
                EquipmentView::frame_for_slot(EquipmentKinds::Weapon, i),
                &ui,
                EquipmentKinds::Weapon,
            )));
        }

        for i in 0..progression.equipment.armor_count {
            slots.push(Box::from(EquipmentSlotView::init(
                EquipmentView::frame_for_slot(EquipmentKinds::Armor, i),
                &ui,
                EquipmentKinds::Armor,
            )));
        }

        for i in 0..progression.equipment.accessory_count {
            slots.push(Box::from(EquipmentSlotView::init(
                EquipmentView::frame_for_slot(EquipmentKinds::Accessory, i),
                &ui,
                EquipmentKinds::Accessory,
            )));
        }

        for i in 0..progression.equipment.mastery_count {
            slots.push(Box::from(EquipmentSlotView::init(
                EquipmentView::frame_for_slot(EquipmentKinds::Mastery, i),
                &ui,
                EquipmentKinds::Mastery,
            )));
        }

        slots
    }

    fn frame_for_slot(kind: EquipmentKinds, i: u32) -> SDLPoint {
        match kind {
            EquipmentKinds::Weapon => SDLPoint::new(70 + (i as i32 % 7) * 125, 210),
            EquipmentKinds::Armor => SDLPoint::new(70 + (i as i32 % 7) * 125, 330),
            EquipmentKinds::Accessory => SDLPoint::new(70 + (i as i32 % 7) * 125, 450),
            EquipmentKinds::Mastery => SDLPoint::new(70 + (i as i32 % 7) * 125, 570),
        }
    }

    fn create_cards(&self, progression: &ProgressionState) {
        *self.cards.borrow_mut() = progression
            .skills
            .iter()
            .map(|s| {
                Box::from(
                    CardView::init(SDLPoint::new(0, 0), &self.text_renderer, &self.ui, &self.icons, s, self.tree.get_image(&s))
                        .expect("Unable to load equipment card"),
                )
            })
            .collect();
    }

    pub fn arrange(&self, progression: &ProgressionState) {
        let cards = &mut self.cards.borrow_mut();
        let cards_in_equipment: HashSet<String> = HashSet::from_iter(progression.equipment.all());

        let compact = cards.iter().filter(|&c| !cards_in_equipment.contains(&c.name)).count() > 12;

        for (i, c) in cards.iter_mut().enumerate() {
            if cards_in_equipment.contains(&c.name) {
                let (kind, equipment_index) = progression.equipment.find(&c.name).unwrap();
                let equipment_frame =
                    EquipmentView::frame_for_slot(kind, equipment_index as u32).offset(EQUIPMENT_SLOT_OFFSET as i32, EQUIPMENT_SLOT_OFFSET as i32);
                c.frame = SDLRect::new(equipment_frame.x(), equipment_frame.y(), CARD_WIDTH, CARD_HEIGHT)
            } else {
                if compact {
                    c.frame = SDLRect::new(840 + (i / 12) as i32 * -120, 525 + (i % 12) as i32 * -40, CARD_WIDTH, CARD_HEIGHT);
                } else {
                    c.frame = SDLRect::new(600 + (i % 3) as i32 * 125, 70 + (i / 3) as i32 * 125, CARD_WIDTH, CARD_HEIGHT);
                }
            }
        }
    }

    pub fn check_for_missing_cards(&self, ecs: &World) {
        let progression = ecs.read_resource::<ProgressionState>();
        if progression.skills.len() != self.cards.borrow().len() {
            self.create_cards(&progression);
            self.arrange(&progression);
        }
    }
}

impl View for EquipmentView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        if *self.should_sort.borrow() {
            *self.should_sort.borrow_mut() = false;
            self.arrange(&ecs.read_resource::<ProgressionState>());
        }
        self.check_for_missing_cards(ecs);
        if *self.needs_z_reorder.borrow() {
            *self.needs_z_reorder.borrow_mut() = false;
            self.cards.borrow_mut().sort_by(|a, b| a.z_order.cmp(&b.z_order));
        }

        // Slots below cards
        for c in &self.slots {
            c.render(ecs, canvas, frame)?;
        }

        for c in self.cards.borrow().iter() {
            c.render(ecs, canvas, frame)?;
        }

        self.sort.render(ecs, canvas, frame)?;

        Ok(())
    }

    fn handle_mouse_click(&mut self, ecs: &World, x: i32, y: i32, button: Option<MouseButton>) {
        for c in self.cards.borrow_mut().iter_mut().rev() {
            c.handle_mouse_click(ecs, x, y, button);
            if c.grabbed.is_some() {
                c.z_order = self.max_z_order;
                self.max_z_order += 1;
                *self.needs_z_reorder.borrow_mut() = true;
                return;
            }
        }
        self.sort.handle_mouse_click(ecs, x, y, button);
    }

    fn handle_mouse_move(&mut self, ecs: &World, x: i32, y: i32, state: MouseState) {
        for c in self.cards.borrow_mut().iter_mut() {
            c.handle_mouse_move(ecs, x, y, state);
        }
        self.sort.handle_mouse_move(ecs, x, y, state);
    }

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        self.cards.borrow().iter().filter_map(|c| c.hit_test(ecs, x, y)).next()
    }
}
