#![recursion_limit = "512"]

mod agents;
mod app;
mod components;
mod services;
mod views;

use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    yew::start_app::<app::Model>();
}
