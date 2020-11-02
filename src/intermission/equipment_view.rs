use std::cell::RefCell;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::rc::Rc;

use itertools::Itertools;
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use specs::prelude::*;

use super::skilltree_view::{get_tree, get_tree_icons, SKILL_NODE_SIZE};
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{EquipmentItem, EquipmentKinds, ProgressionComponent, ProgressionState, SkillTree};
use crate::props::{render_text_layout, Button, HitTestResult, MousePositionComponent, View};

pub struct CardView {
    frame: SDLRect,
    text_renderer: Rc<TextRenderer>,
    ui: Rc<IconCache>,
    icons: Rc<IconCache>,
    pub equipment: EquipmentItem,
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
        equipment: EquipmentItem,
    ) -> BoxResult<CardView> {
        Ok(CardView {
            frame: SDLRect::new(position.x(), position.y(), CARD_WIDTH, CARD_HEIGHT),
            text_renderer: Rc::clone(&text_renderer),
            ui: Rc::clone(&ui),
            icons: Rc::clone(&icons),
            equipment,
            grabbed: None,
            z_order: 0,
        })
    }

    fn border_color(&self) -> Color {
        match self.equipment.kind {
            EquipmentKinds::Weapon => Color::RGB(127, 0, 0),
            EquipmentKinds::Armor => Color::RGB(0, 7, 150),
            EquipmentKinds::Accessory => Color::RGB(26, 86, 0),
            EquipmentKinds::Mastery => Color::RGB(111, 38, 193),
        }
    }
}

impl View for CardView {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.copy(self.ui.get("card_frame.png"), None, self.frame)?;

        if let Some(image) = &self.equipment.image {
            let image_rect = SDLRect::new(
                (self.frame.x() + (CARD_WIDTH as i32 / 2) - (SKILL_NODE_SIZE as i32 / 4)) as i32,
                self.frame.y() + 20,
                SKILL_NODE_SIZE / 2,
                SKILL_NODE_SIZE / 2,
            );

            canvas.set_draw_color(self.border_color());
            canvas.fill_rect(SDLRect::new(
                image_rect.x() - 3,
                image_rect.y() - 3,
                image_rect.width() + 6,
                image_rect.height() + 6,
            ))?;

            canvas.copy(self.icons.get(&image), None, image_rect)?;
        }

        let layout = self.text_renderer.layout_text(
            &self.equipment.name,
            FontSize::Small,
            LayoutRequest::init(
                self.frame.x() as u32 + 12,
                self.frame.y() as u32 + SKILL_NODE_SIZE / 2 + 20 + 6,
                CARD_WIDTH - 24,
                5,
            ),
        )?;

        render_text_layout(&layout, canvas, &self.text_renderer, None, FontColor::Brown, 0, false, |_, _| {})?;
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
            Some(HitTestResult::Skill(self.equipment.name.clone()))
        } else {
            None
        }
    }
}

pub struct EquipmentSlotView {
    frame: SDLRect,
    ui: Rc<IconCache>,
    pub kind: EquipmentKinds,
    pub equipment_offset: usize,
    pub highlighted: RefCell<bool>,
}

const EQUIPMENT_SLOT_OFFSET: u32 = 2;
impl EquipmentSlotView {
    pub fn init(position: SDLPoint, ui: &Rc<IconCache>, kind: EquipmentKinds, equipment_offset: usize) -> EquipmentSlotView {
        EquipmentSlotView {
            frame: SDLRect::new(
                position.x(),
                position.y(),
                CARD_WIDTH + EQUIPMENT_SLOT_OFFSET * 2,
                CARD_HEIGHT + EQUIPMENT_SLOT_OFFSET * 2,
            ),
            ui: Rc::clone(ui),
            kind,
            equipment_offset,
            highlighted: RefCell::new(false),
        }
    }
}

impl View for EquipmentSlotView {
    fn render(&self, _ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        let highlighted = *self.highlighted.borrow();
        let equipment_frame = self.ui.get(match self.kind {
            EquipmentKinds::Weapon => {
                if highlighted {
                    "equipment_weapon_slot_highlight.png"
                } else {
                    "equipment_weapon_slot.png"
                }
            }
            EquipmentKinds::Armor => {
                if highlighted {
                    "equipment_armor_slot_highlight.png"
                } else {
                    "equipment_armor_slot.png"
                }
            }
            EquipmentKinds::Accessory => {
                if highlighted {
                    "equipment_accessory_slot_highlight.png"
                } else {
                    "equipment_accessory_slot.png"
                }
            }
            EquipmentKinds::Mastery => {
                if highlighted {
                    "equipment_mastery_slot_highlight.png"
                } else {
                    "equipment_mastery_slot.png"
                }
            }
        });
        canvas.copy(equipment_frame, None, self.frame)?;

        *self.highlighted.borrow_mut() = false;

        Ok(())
    }
}

pub struct EquipmentView {
    should_sort: Rc<RefCell<bool>>,
    tree: SkillTree,
    ui: Rc<IconCache>,
    icons: Rc<IconCache>,
    text_renderer: Rc<TextRenderer>,
    cards: RefCell<Vec<CardView>>,
    slots: Vec<EquipmentSlotView>,
    sort: Button,
    max_z_order: u32,
    needs_z_reorder: RefCell<bool>,
}

impl EquipmentView {
    pub fn init(render_context: &RenderContext, text_renderer: &Rc<TextRenderer>, ecs: &World) -> BoxResult<EquipmentView> {
        let tree = SkillTree::init(&get_tree(ecs));
        let ui = Rc::new(IconCache::init(
            &render_context,
            IconLoader::init_ui(),
            &[
                "card_frame.png",
                "equipment_weapon_slot.png",
                "equipment_armor_slot.png",
                "equipment_accessory_slot.png",
                "equipment_mastery_slot.png",
                "equipment_weapon_slot_highlight.png",
                "equipment_armor_slot_highlight.png",
                "equipment_accessory_slot_highlight.png",
                "equipment_mastery_slot_highlight.png",
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
            slots: EquipmentView::create_slots(ecs, &ui),
            ui,
            max_z_order: 1,
        };
        Ok(view)
    }

    fn create_slots(ecs: &World, ui: &Rc<IconCache>) -> Vec<EquipmentSlotView> {
        let mut slots = vec![];
        let progression = &ecs.read_resource::<ProgressionComponent>().state;

        for kind in &[
            EquipmentKinds::Weapon,
            EquipmentKinds::Armor,
            EquipmentKinds::Accessory,
            EquipmentKinds::Mastery,
        ] {
            for i in 0..progression.equipment.count(*kind) {
                slots.push(EquipmentSlotView::init(EquipmentView::frame_for_slot(*kind, i as u32), &ui, *kind, i));
            }
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
                CardView::init(SDLPoint::new(0, 0), &self.text_renderer, &self.ui, &self.icons, self.tree.get(&s).clone())
                    .expect("Unable to load equipment card")
            })
            .collect();
    }

    pub fn arrange(&self, progression: &ProgressionState) {
        let cards = &mut self.cards.borrow_mut();
        let cards_in_equipment: HashSet<EquipmentItem> = HashSet::from_iter(progression.equipment.all().drain(0..).filter_map(|x| x));

        let compact = cards.iter().filter(|&c| !cards_in_equipment.contains(&c.equipment)).count() > 12;

        for (i, c) in cards
            .iter_mut()
            .filter(|c| !cards_in_equipment.contains(&c.equipment))
            .sorted_by(|a, b| a.equipment.name.cmp(&b.equipment.name))
            .enumerate()
        {
            if compact {
                c.frame = SDLRect::new(840 + (i / 12) as i32 * -120, 525 + (i % 12) as i32 * -40, CARD_WIDTH, CARD_HEIGHT);
            } else {
                c.frame = SDLRect::new(600 + (i % 3) as i32 * 125, 70 + (i / 3) as i32 * 125, CARD_WIDTH, CARD_HEIGHT);
            }
        }
        for c in cards.iter_mut().filter(|c| cards_in_equipment.contains(&c.equipment)) {
            self.arrange_card_into_slot(c, progression);
        }
    }

    pub fn arrange_card_into_slot(&self, card: &mut CardView, progression: &ProgressionState) {
        let (kind, equipment_index) = progression.equipment.find(&card.equipment.name).unwrap();
        let equipment_frame = EquipmentView::frame_for_slot(kind, equipment_index as u32).offset(EQUIPMENT_SLOT_OFFSET as i32, EQUIPMENT_SLOT_OFFSET as i32);
        card.frame = SDLRect::new(equipment_frame.x(), equipment_frame.y(), CARD_WIDTH, CARD_HEIGHT);
    }

    pub fn check_for_missing_cards(&self, ecs: &World) {
        let progression = &(*ecs.read_resource::<ProgressionComponent>()).state;
        if progression.skills.len() != self.cards.borrow().len() {
            self.create_cards(&progression);
            self.arrange(&progression);
        }
    }

    fn find_slot_at(&self, x: i32, y: i32) -> Option<&EquipmentSlotView> {
        self.slots.iter().find(|s| s.frame.contains_point(SDLPoint::new(x, y)))
    }
}

impl View for EquipmentView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        if *self.should_sort.borrow() {
            *self.should_sort.borrow_mut() = false;
            self.arrange(&(*ecs.read_resource::<ProgressionComponent>()).state);
        }
        self.check_for_missing_cards(ecs);
        if *self.needs_z_reorder.borrow() {
            *self.needs_z_reorder.borrow_mut() = false;
            self.cards.borrow_mut().sort_by(|a, b| a.z_order.cmp(&b.z_order));
        }

        let grabbed_card_kind = self.cards.borrow().iter().filter(|c| c.grabbed.is_some()).map(|c| c.equipment.kind).next();
        // Slots below cards
        for s in &self.slots {
            if grabbed_card_kind.is_some() && grabbed_card_kind.unwrap() == s.kind {
                // If we're dragging a card over a slot, set a one render highlighted flag
                let mouse = ecs.read_resource::<MousePositionComponent>().position;
                let current_over_slot = self.find_slot_at(mouse.x as i32, mouse.y as i32);
                if Some((s.kind, s.equipment_offset)) == current_over_slot.map(|c| (c.kind, c.equipment_offset)) {
                    *s.highlighted.borrow_mut() = true;
                }
            }
            s.render(ecs, canvas, frame)?;
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
        ecs.write_resource::<MousePositionComponent>().position = Point::init(x as u32, y as u32);

        let progression = &mut ecs.write_resource::<ProgressionComponent>().state;

        for c in self.cards.borrow_mut().iter_mut() {
            let was_grabbed = c.grabbed.is_some();
            c.handle_mouse_move(ecs, x, y, state);
            if was_grabbed && c.grabbed.is_none() {
                let was_in_slot = progression.equipment.has(&c.equipment.name);
                let current_over_slot = self.find_slot_at(x, y);

                if !was_in_slot {
                    // Case 1: Not in slot, now over slot - If empty parent else nothing
                    if let Some(current_slot) = current_over_slot {
                        if progression.equipment.get(current_slot.kind, current_slot.equipment_offset).is_none() && c.equipment.kind == current_slot.kind {
                            assert!(progression.equipment.add(current_slot.kind, c.equipment.clone(), current_slot.equipment_offset));
                            self.arrange_card_into_slot(c, &progression);
                        }
                    }
                // Case 0: Not in slot, not over slot - No change
                } else {
                    let (previous_kind, previous_index) = progression.equipment.find(&c.equipment.name).unwrap();

                    if let Some(current_slot) = current_over_slot {
                        if previous_kind == current_slot.kind && previous_index == current_slot.equipment_offset {
                            // Case 2: In slot, over own slot - Rearrange back
                            self.arrange_card_into_slot(c, &progression);
                        } else {
                            // Case 3: In slot, over different slot - If empty remove and parent else rearrange back
                            if progression.equipment.get(current_slot.kind, current_slot.equipment_offset).is_none() && c.equipment.kind == current_slot.kind {
                                assert!(progression.equipment.remove(previous_kind, previous_index));
                                assert!(progression.equipment.add(current_slot.kind, c.equipment.clone(), current_slot.equipment_offset));
                            }
                            self.arrange_card_into_slot(c, &progression);
                        }
                    } else {
                        // Case 4: In slot, over not over slot - Unparent and keep there
                        assert!(progression.equipment.remove(previous_kind, previous_index));
                    }
                }
            }
        }
        self.sort.handle_mouse_move(ecs, x, y, state);
    }

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        self.cards.borrow().iter().filter_map(|c| c.hit_test(ecs, x, y)).next()
    }
}
