use std::cell::RefCell;
use std::rc::Rc;

use sdl2::mouse::{MouseButton, MouseState};
use sdl2::pixels::Color;
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use super::skilltree_view::{get_tree, get_tree_icons, SKILL_NODE_SIZE};
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{CharacterWeaponKind, ProgressionState, SkillNodeStatus, SkillTree, SkillTreeNode};
use crate::props::{HitTestResult, View};

pub struct CardView {
    frame: SDLRect,
    text_renderer: Rc<TextRenderer>,
    ui: Rc<IconCache>,
    icons: Rc<IconCache>,
    name: String,
    image: Option<String>,
    grabbed: Option<SDLPoint>,
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
        })
    }
}

impl View for CardView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, _frame: u64) -> BoxResult<()> {
        canvas.copy(self.ui.get("card_frame.png"), None, self.frame)?;

        if let Some(image) = &self.image {
            canvas.copy(
                self.icons.get(&image),
                None,
                SDLRect::new(
                    (self.frame.x() as u32 + (CARD_WIDTH / 2) - (SKILL_NODE_SIZE / 2)) as i32,
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

    fn handle_mouse_click(&mut self, ecs: &World, x: i32, y: i32, button: Option<MouseButton>) {
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

pub struct EquipmentView {
    position: SDLPoint,
    tree: SkillTree,
    ui: Rc<IconCache>,
    icons: Rc<IconCache>,
    text_renderer: Rc<TextRenderer>,
    cards: RefCell<Vec<Box<CardView>>>,
}

impl EquipmentView {
    pub fn init(
        position: SDLPoint,
        render_context: &RenderContext,
        text_renderer: &Rc<TextRenderer>,
        progression: &ProgressionState,
    ) -> BoxResult<EquipmentView> {
        let tree = SkillTree::init(&get_tree(&progression.weapon));
        let ui = Rc::new(IconCache::init(&render_context, IconLoader::init_ui(), &["card_frame.png"])?);
        let view = EquipmentView {
            position,
            icons: Rc::new(get_tree_icons(render_context, &tree)?),
            tree,
            ui,
            text_renderer: Rc::clone(text_renderer),
            cards: RefCell::new(vec![]),
        };
        Ok(view)
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

    pub fn arrange(&self) {
        for (i, c) in &mut self.cards.borrow_mut().iter_mut().enumerate() {
            c.frame = SDLRect::new(50 + i as i32 * 150, 50, CARD_WIDTH, CARD_HEIGHT);
        }
    }

    pub fn check_for_missing_cards(&self, ecs: &World) {
        let progression = ecs.read_resource::<ProgressionState>();
        if progression.skills.len() != self.cards.borrow().len() {
            self.create_cards(&progression);
            self.arrange();
        }
    }
}

impl View for EquipmentView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        self.check_for_missing_cards(ecs);

        for c in self.cards.borrow().iter() {
            c.render(ecs, canvas, frame)?;
        }

        Ok(())
    }

    fn handle_mouse_click(&mut self, ecs: &World, x: i32, y: i32, button: Option<MouseButton>) {
        for c in self.cards.borrow_mut().iter_mut() {
            c.handle_mouse_click(ecs, x, y, button);
        }
    }

    fn handle_mouse_move(&mut self, ecs: &World, x: i32, y: i32, state: MouseState) {
        for c in self.cards.borrow_mut().iter_mut() {
            c.handle_mouse_move(ecs, x, y, state);
        }
    }

    fn hit_test(&self, ecs: &World, x: i32, y: i32) -> Option<HitTestResult> {
        self.cards.borrow().iter().filter_map(|c| c.hit_test(ecs, x, y)).next()
    }
}
