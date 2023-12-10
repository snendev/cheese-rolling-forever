#[macro_use]
extern crate cfg_if;

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        fn main() {
            cheese_game::run_app(None);
        }
    }
}
