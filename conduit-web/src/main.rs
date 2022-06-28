mod app;
mod components;
mod hooks;
mod pages;
mod router;

pub mod layout;

fn main() -> anyhow::Result<()> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}
