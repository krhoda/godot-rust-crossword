use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(ColorRect)]
pub struct PuzzleTile {}

#[methods]
impl PuzzleTile {
    fn new(_base: &ColorRect) -> Self {
        PuzzleTile {}
    }

    #[method]
    fn color_and_text(
        &self,
        #[base] base: &ColorRect,
        char_slot: Option<String>,
        has_char_slot: bool,
    ) {
        let name = base.name();
        if has_char_slot {
            if let Some(s) = char_slot {
                let label = unsafe { base.get_node_as::<Label>("Label").unwrap() };
                label.set_text(s);
            }
        } else {
            // base.add_color_override("color", Color::from_rgb(0.0, 0.0, 0.0));
            base.set_modulate(Color::from_rgb(0.0, 0.0, 0.0));
            // let label = unsafe { base.get_node_as::<Label>("Label").unwrap() };
            // label.set_text("!");
        }
    }
}
