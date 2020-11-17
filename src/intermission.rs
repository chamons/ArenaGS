mod card_view;
mod character_scene;
mod equipment_view;
mod merchant_view;
mod next_battle_view;
mod profession_tree;
mod reward_scene;

pub use character_scene::*;
pub use reward_scene::*;

// https://github.com/rust-lang/rfcs/issues/2407#issuecomment-385291238
#[macro_export]
macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}
