use std::cell::RefCell;
use std::rc::Rc;

use sdl2::mouse::{MouseButton, MouseState};
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{content, find_player, new_game, CharacterWeaponKind, ProgressionComponent};
use crate::enclose;
use crate::props::{Button, ButtonDelegate, InfoBarView, SkillBarView, View};

pub struct NextBattleView {
    continue_button: Button,
    preview_world: RefCell<World>,
    regenerate_world: Rc<RefCell<bool>>,
    skillbar: SkillBarView,
    infobar: InfoBarView,
    weapon_buttons: RefCell<Vec<Button>>,
    weapon_images: IconCache,
    weapon_frame: Rc<Texture>,
}

impl NextBattleView {
    pub fn init(render_context: &RenderContext, text_renderer: &Rc<TextRenderer>, ecs: &World, next_fight: &Rc<RefCell<bool>>) -> BoxResult<NextBattleView> {
        let continue_button = Button::text(
            SDLPoint::new(800, 650),
            "Next Fight",
            render_context,
            text_renderer,
            ButtonDelegate::init().handler(Box::new(enclose! { (next_fight) move |_| *next_fight.borrow_mut() = true })),
        )?;
        let preview_world = NextBattleView::generate_preview_world(ecs);
        let skillbar = SkillBarView::init(render_context, &preview_world, SDLPoint::new(80, 250), Rc::clone(&text_renderer), true)?;
        let infobar = InfoBarView::init(SDLPoint::new(100, 100), render_context, Rc::clone(&text_renderer), true)?;

        let weapon_images = {
            let progression = preview_world.read_resource::<ProgressionComponent>();
            match progression.state.weapon {
                CharacterWeaponKind::Gunslinger => content::gunslinger::get_all_trait_images(),
            }
        };
        let weapon_images = IconCache::init(render_context, IconLoader::init_icons(), &weapon_images)?;
        let weapon_frame = Rc::new(IconLoader::init_ui().get(render_context, "skill_tree_frame.png")?);
        let weapon_buttons = NextBattleView::get_weapon_specific_buttons(&preview_world, &weapon_images, &weapon_frame);

        Ok(NextBattleView {
            continue_button,
            preview_world: RefCell::new(preview_world),
            regenerate_world: Rc::new(RefCell::new(false)),
            skillbar,
            infobar,
            weapon_buttons: RefCell::new(weapon_buttons),
            weapon_images,
            weapon_frame,
        })
    }

    fn generate_preview_world(ecs: &World) -> World {
        new_game::create_equipment_preview_battle(ecs)
    }

    fn get_weapon_specific_buttons(preview_world: &World, weapon_images: &IconCache, weapon_frame: &Rc<Texture>) -> Vec<Button> {
        let progression = preview_world.read_resource::<ProgressionComponent>();
        match progression.state.weapon {
            CharacterWeaponKind::Gunslinger => {
                let ammos = content::gunslinger::get_equipped_ammos(preview_world, find_player(preview_world));
                ammos
                    .iter()
                    .enumerate()
                    .map(|(i, &a)| {
                        Button::image(
                            SDLRect::new(320 + 75 * i as i32, 140, 48, 48),
                            weapon_images.get_reference(content::gunslinger::get_image_for_kind(a)),
                            weapon_frame,
                            ButtonDelegate::init()
                                .enabled(Box::new(move |ecs| {
                                    if content::gunslinger::get_current_weapon_trait(ecs, find_player(ecs)) == a {
                                        crate::props::ButtonEnabledState::Ghosted
                                    } else {
                                        crate::props::ButtonEnabledState::Shown
                                    }
                                }))
                                .handler(Box::new(move |ecs| {
                                    content::gunslinger::set_ammo_to(ecs, find_player(ecs), a);
                                })),
                        )
                        .expect("Unable to load weapon buttons")
                    })
                    .collect()
            }
        }
    }
}

impl View for NextBattleView {
    fn render(&self, outside_world: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        // Do not use passed in world as we're using our own "preview" simulation for this tab
        // Unless we are regenerating the preview
        let mut regenerate_world = self.regenerate_world.borrow_mut();
        if *regenerate_world {
            *regenerate_world = false;
            *self.preview_world.borrow_mut() = NextBattleView::generate_preview_world(outside_world);
            *self.weapon_buttons.borrow_mut() =
                NextBattleView::get_weapon_specific_buttons(&self.preview_world.borrow(), &self.weapon_images, &self.weapon_frame);
        }
        let preview_world = self.preview_world.borrow();

        self.continue_button.render(&preview_world, canvas, frame)?;

        self.skillbar.render(&preview_world, canvas, frame)?;
        self.infobar.render(&preview_world, canvas, frame)?;

        for b in self.weapon_buttons.borrow().iter() {
            b.render(&preview_world, canvas, frame)?;
        }

        Ok(())
    }

    fn handle_mouse_click(&mut self, _: &mut World, x: i32, y: i32, button: Option<MouseButton>) {
        let mut preview_world = self.preview_world.borrow_mut();
        self.continue_button.handle_mouse_click(&mut preview_world, x, y, button);

        for b in self.weapon_buttons.borrow_mut().iter_mut() {
            b.handle_mouse_click(&mut preview_world, x, y, button);
        }
    }

    fn handle_mouse_move(&mut self, _: &World, x: i32, y: i32, state: MouseState) {
        let preview_world = self.preview_world.borrow();
        self.continue_button.handle_mouse_move(&preview_world, x, y, state);
    }

    fn on_tab_swap(&mut self) {
        *self.regenerate_world.borrow_mut() = true;
    }
}
