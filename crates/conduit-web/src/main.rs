mod app;
mod components;
mod contexts;
mod hooks;
mod layout;
mod pages;
mod router;
mod services;
mod utilities;

fn main() -> anyhow::Result<()> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}
