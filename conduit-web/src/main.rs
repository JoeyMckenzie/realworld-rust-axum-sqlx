mod app;
mod components;
mod pages;
mod router;

fn main() -> anyhow::Result<()> {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}
