#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(target_arch = "wasm32")] {
        use wasm_bindgen::prelude::*;

        #[wasm_bindgen(start)]
        pub fn main() -> Result<(), JsValue> {
            cheese_game::run_app(Some("#game-canvas".to_string()));

            Ok(())
        }
    }
}
