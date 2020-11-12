use std::cell::RefCell;
use std::rc::Rc;

use sdl2::mouse::MouseButton;
use sdl2::rect::Point as SDLPoint;
use specs::prelude::*;

use super::card_view::CardView;
use super::reward_scene::{draw_selection_frame, icons_for_items};
use crate::after_image::prelude::*;
use crate::atlas::prelude::*;
use crate::clash::{gambler, sales, EquipmentKinds, ProgressionComponent};
use crate::enclose;
use crate::props::{Button, ButtonDelegate, ButtonEnabledState, View};

pub struct MerchantView {
    text_renderer: Rc<TextRenderer>,
    ui: Rc<IconCache>,
    cards: Rc<RefCell<Vec<Option<CardView>>>>,
    equipment_expand_buttons: Rc<RefCell<Vec<Option<Button>>>>,
    accept_button: Button,
    selection: Rc<RefCell<Option<u32>>>,
}

impl MerchantView {
    pub fn init(render_context: &RenderContext, text_renderer: &Rc<TextRenderer>, ecs: &World) -> BoxResult<MerchantView> {
        let mut items = gambler::get_merchant_items(ecs);
        let icons = icons_for_items(render_context, &items)?;

        let ui = Rc::new(IconCache::init(
            &render_context,
            IconLoader::init_ui(),
            &["card_frame_large.png", "card_frame_large_selection.png", "button_frame_full_selection.png"],
        )?);

        let cards: Rc<RefCell<Vec<Option<CardView>>>> = {
            Rc::new(RefCell::new(
                items
                    .drain(..)
                    .enumerate()
                    .map(|(i, s)| {
                        Some(
                            CardView::init(
                                SDLPoint::new(120 + ((i % 4) * 200) as i32, 90 + ((i / 4) * 275) as i32),
                                text_renderer,
                                &ui,
                                &icons,
                                s,
                                true,
                                false,
                            )
                            .expect("Unable to load merchant card"),
                        )
                    })
                    .collect(),
            ))
        };

        // The following code is correct, but more complex than I'd like
        // A downside of my buttons respond in callback with rust's ownership
        // is anything they want to touch outside of the ECS has to be Rc<RefCell>
        // and cloned via enclose! macro. It works, but is not pretty...
        let selection = Rc::new(RefCell::new(None));

        let mut equipment_expand_buttons = vec![];
        let progression = &mut ecs.read_resource::<ProgressionComponent>();
        for (i, kind) in vec![
            EquipmentKinds::Weapon,
            EquipmentKinds::Armor,
            EquipmentKinds::Accessory,
            EquipmentKinds::Mastery,
        ]
        .iter()
        .filter(|k| !progression.state.equipment_expansions.contains(&format!("{:#?} Store Expansion", k)))
        .enumerate()
        {
            equipment_expand_buttons.push(Some(
                Button::text(
                    SDLPoint::new(50 + 155 * i as i32, 655),
                    &format!("+1 {:#?} Slot", kind),
                    render_context,
                    text_renderer,
                    ButtonDelegate::init()
                        .enabled(Box::new(|ecs| {
                            let progression = ecs.read_resource::<ProgressionComponent>();
                            if sales::can_purchase_expansion(&progression) {
                                ButtonEnabledState::Shown
                            } else {
                                ButtonEnabledState::Ghosted
                            }
                        }))
                        .handler(Box::new(enclose! { (selection) move |_| {
                            *selection.borrow_mut() = Some(8 + i as u32);
                        }})),
                )?
                .with_size(FontSize::Small),
            ))
        }
        let equipment_expand_buttons = Rc::new(RefCell::new(equipment_expand_buttons));

        let accept_button = Button::text(
            SDLPoint::new(800, 650),
            "Purchase",
            &render_context,
            text_renderer,
            ButtonDelegate::init()
                .enabled(Box::new(enclose! { (cards, selection) move |ecs| {
                    let cards = cards.borrow();

                    if let Some(selection_index) = *selection.borrow() {
                        let progression = &mut ecs.write_resource::<ProgressionComponent>();
                        if selection_index < 8 {
                            let selection_equip = &cards[selection_index as usize].as_ref().unwrap().equipment;
                            if sales::can_purchase_selection (&progression, &selection_equip) {
                                return ButtonEnabledState::Shown;
                            }
                        }
                        else {
                            if sales::can_purchase_expansion(progression) {
                                return ButtonEnabledState::Shown;
                            }
                        }
                    }
                    ButtonEnabledState::Ghosted
                }}))
                .handler(Box::new(enclose! { (cards, selection, equipment_expand_buttons) move |ecs| {
                    let mut selection = selection.borrow_mut();
                    if let Some(selection_index) = *selection {
                        let mut progression = &mut ecs.write_resource::<ProgressionComponent>();
                        if selection_index < 8 {
                            let mut cards = cards.borrow_mut();
                            let selection_equip = &cards[selection_index as usize].as_ref().unwrap().equipment;
                            if sales::can_purchase_selection (&progression, &selection_equip) {
                                sales::purchase_selection(&mut progression, &selection_equip);
                                cards[selection_index as usize] = None;
                            }
                        }
                        else {
                            let mut equipment_expand_buttons = equipment_expand_buttons.borrow_mut();

                            if sales::can_purchase_expansion(progression) {
                                let kind_index = selection_index as usize - 8;
                                let kind = vec![
                                    EquipmentKinds::Weapon,
                                    EquipmentKinds::Armor,
                                    EquipmentKinds::Accessory,
                                    EquipmentKinds::Mastery,
                                ][kind_index];
                                sales::purchase_expansion(progression, kind);
                                equipment_expand_buttons[kind_index] = None;
                            }
                        }
                        *selection = None;
                    }
                }})),
        )?;

        Ok(MerchantView {
            text_renderer: Rc::clone(text_renderer),
            ui,
            cards: Rc::clone(&cards),
            accept_button,
            selection: Rc::clone(&selection),
            equipment_expand_buttons,
        })
    }
}

impl View for MerchantView {
    fn render(&self, ecs: &World, canvas: &mut RenderCanvas, frame: u64) -> BoxResult<()> {
        let cards = self.cards.borrow();
        let equipment_expand_buttons = self.equipment_expand_buttons.borrow();

        if let Some(selection) = *self.selection.borrow() {
            if selection < 8 {
                if let Some(card) = &cards[selection as usize] {
                    draw_selection_frame(canvas, &self.ui, card.frame, "card_frame_large_selection.png")?;
                }
            } else {
                if let Some(button) = &equipment_expand_buttons[selection as usize - 8] {
                    draw_selection_frame(canvas, &self.ui, button.frame, "button_frame_full_selection.png")?;
                }
            };
        }

        for c in cards.iter().flatten() {
            c.render(ecs, canvas, frame)?;

            self.text_renderer.render_text_centered(
                &format!("{} Influence", sales::selection_cost(&c.equipment)),
                c.frame.x(),
                c.frame.y() - 20,
                c.frame.width(),
                canvas,
                FontSize::Bold,
                FontColor::Brown,
            )?;
        }

        for b in equipment_expand_buttons.iter().flatten() {
            b.render(ecs, canvas, frame)?;

            self.text_renderer.render_text_centered(
                "100 Influence",
                b.frame.x(),
                b.frame.y() - 20,
                b.frame.width(),
                canvas,
                FontSize::Bold,
                FontColor::Brown,
            )?;
        }

        self.accept_button.render(&ecs, canvas, frame)?;

        let progression = &(*ecs.read_resource::<ProgressionComponent>()).state;
        self.text_renderer.render_text(
            &format!("Current Influence: {}", progression.influence),
            780,
            615,
            canvas,
            FontSize::Bold,
            FontColor::Brown,
        )?;

        Ok(())
    }

    fn handle_mouse_click(&mut self, ecs: &mut World, x: i32, y: i32, button: Option<MouseButton>) {
        for (i, c) in self.cards.borrow_mut().iter_mut().enumerate() {
            if let Some(c) = c {
                c.handle_mouse_click(ecs, x, y, button);
                if c.grabbed.is_some() {
                    c.grabbed = None;
                    *self.selection.borrow_mut() = Some(i as u32);
                }
            }
        }

        for b in self.equipment_expand_buttons.borrow_mut().iter_mut().flatten() {
            b.handle_mouse_click(ecs, x, y, button);
        }

        self.accept_button.handle_mouse_click(ecs, x, y, button);
    }
}
