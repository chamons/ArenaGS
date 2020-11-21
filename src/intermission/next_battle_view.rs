use std::cell::RefCell;
use std::rc::Rc;

use sdl2::mouse::{MouseButton, MouseState};
use sdl2::rect::Point as SDLPoint;
use sdl2::rect::Rect as SDLRect;
use sdl2::render::Texture;
use specs::prelude::*;

use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{find_player, new_game, CharacterWeaponKind, ProgressionComponent};
use crate::enclose;
use crate::props::{Button, ButtonDelegate, HitTestResult, InfoBarView, SkillBarView, View};

pub struct NextBattleView {
    continue_button: Button,
    regenerate_world: Rc<RefCell<bool>>,
    skillbar: SkillBarView,
    infobar: InfoBarView,
    weapon_buttons: RefCell<Vec<Button>>,
    weapon_images: IconCache,
    weapon_frame: Rc<Texture>,
}

// Down the rabbit hole we go
// This is a resource on the "outside" world that contains an entire "preview" world
// that gets created only for this view. We need to store it here, so the character_scene
pub struct PreviewWorld {
    pub preview_world: World,
}

impl PreviewWorld {
    pub fn init(preview_world: World) -> PreviewWorld {
        PreviewWorld { preview_world }
    }
}

impl NextBattleView {
    pub fn init(
        render_context: &RenderContext,
        text_renderer: &Rc<TextRenderer>,
        ecs: &mut World,
        next_fight: &Rc<RefCell<bool>>,
    ) -> BoxResult<NextBattleView> {
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
            crate::clash::content::weapon_pack::get_weapon_pack_for(progression.state.weapon).get_all_mode_images()
        };
        let weapon_images = IconCache::init(render_context, IconLoader::init_icons(), &weapon_images)?;
        let weapon_frame = Rc::new(IconLoader::init_ui().get(render_context, "skill_tree_frame.png")?);
        let weapon_buttons = NextBattleView::get_weapon_specific_buttons(&preview_world, &weapon_images, &weapon_frame);

        ecs.insert(PreviewWorld::init(preview_world));

        Ok(NextBattleView {
            continue_button,
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
                let weapon_pack = crate::clash::content::weapon_pack::get_weapon_pack_for(progression.state.weapon);
                let ammos = weapon_pack.get_equipped_mode(preview_world, find_player(preview_world));
                ammos
                    .iter()
                    .enumerate()
                    .map(|(i, ammo)| {
                        Button::image(
                            SDLRect::new(320 + 75 * i as i32, 140, 48, 48),
                            weapon_images.get_reference(weapon_pack.get_image_for_weapon_mode(ammo)),
                            weapon_frame,
                            ButtonDelegate::init()
                                .enabled(Box::new(enclose! { (ammo) move |ecs| {
                                    let current = crate::clash::content::weapon_pack::get_weapon_pack(ecs).get_current_weapon_mode(ecs, find_player(ecs));
                                    if current == ammo {
                                        crate::props::ButtonEnabledState::Ghosted
                                    } else {
                                        crate::props::ButtonEnabledState::Shown
                                    }
                                }}))
                                .handler(Box::new(enclose! { (ammo) move |ecs| {
                                    crate::clash::content::weapon_pack::get_weapon_pack(ecs).set_mode_to(ecs, find_player(ecs), &ammo);
                                }})),
                        )
                        .expect("Unable to load weapon buttons")
                    })
                    .collect()
            }
        }
    }

    fn inner_hit_test(&self, preview_world: &World, x: i32, y: i32) -> Option<HitTestResult> {
        let result = self.skillbar.hit_test(&preview_world, x, y);
        if result.is_some() {
            return result;
        }

        let weapon_button_index = self
            .weapon_buttons
            .borrow()
            .iter()
            .enumerate()
            .filter_map(|(i, b)| if b.hit_test(&preview_world, x, y).is_some() { Some(i) } else { None })
            .next();
        if let Some(weapon_button_index) = weapon_button_index {
            let progression = preview_world.read_resource::<ProgressionComponent>();
            match progression.state.weapon {
                CharacterWeaponKind::Gunslinger => {
                    let weapon_pack = crate::clash::content::weapon_pack::get_weapon_pack_for(progression.state.weapon);
                    let ammos = weapon_pack.get_equipped_mode(&preview_world, find_player(&preview_world));
                    return Some(HitTestResult::Skill(format!("{}", ammos[weapon_button_index])));
                }
            }
        }

        None
    }
}

impl View for NextBattleView {
    fn render(&self, outside_world: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        // Do not use passed in world as we're using our own "preview" simulation for this tab
        // Unless we are regenerating the preview
        let mut regenerate_world = self.regenerate_world.borrow_mut();
        if *regenerate_world {
            *regenerate_world = false;
            let mut preview_world_component = outside_world.write_resource::<PreviewWorld>();
            preview_world_component.preview_world = NextBattleView::generate_preview_world(outside_world);
            *self.weapon_buttons.borrow_mut() =
                NextBattleView::get_weapon_specific_buttons(&preview_world_component.preview_world, &self.weapon_images, &self.weapon_frame);
        }
        let preview_world = &outside_world.read_resource::<PreviewWorld>().preview_world;

        self.continue_button.render(preview_world, canvas, frame)?;

        self.skillbar.render(preview_world, canvas, frame)?;
        self.infobar.render(preview_world, canvas, frame)?;

        for b in self.weapon_buttons.borrow().iter() {
            b.render(preview_world, canvas, frame)?;
        }

        Ok(())
    }

    fn handle_mouse_click(&mut self, outside_world: &mut World, x: i32, y: i32, button: Option<MouseButton>) {
        let preview_world = &mut outside_world.write_resource::<PreviewWorld>().preview_world;
        self.continue_button.handle_mouse_click(preview_world, x, y, button);

        for b in self.weapon_buttons.borrow_mut().iter_mut() {
            b.handle_mouse_click(preview_world, x, y, button);
        }
    }

    fn handle_mouse_move(&mut self, outside_world: &World, x: i32, y: i32, state: MouseState) {
        let preview_world = &outside_world.read_resource::<PreviewWorld>().preview_world;
        self.continue_button.handle_mouse_move(&preview_world, x, y, state);
    }

    fn on_tab_swap(&mut self) {
        *self.regenerate_world.borrow_mut() = true;
    }

    fn hit_test(&self, a_world: &World, x: i32, y: i32) -> Option<HitTestResult> {
        // During hit_tests we could be passed EITHER the "real" outside world OR the PreviewWorld
        // depending on who's hit testing. We know that the PreviewWorld doesn't have a nested preview
        // so we if can't find one, use what we get passed. Something something red pill blue pill
        if a_world.has_value::<PreviewWorld>() {
            self.inner_hit_test(&a_world.read_resource::<PreviewWorld>().preview_world, x, y)
        } else {
            self.inner_hit_test(&a_world, x, y)
        }
    }
}
