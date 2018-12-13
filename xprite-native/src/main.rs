extern crate fern;
#[macro_use]
extern crate log;

extern crate xprite;
extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;

extern crate cairo;

use xprite::prelude::*;

// use crate::render::imgui_cairo_renderer::ImguiCairoRenderer;
use crate::render::imgui_renderer::ImguiRenderer;

mod hotkey;
mod consts;
mod render;
mod prelude;
mod ui;
mod state;



fn main() {
    init_logger();
    trace!("Starting Xprite");
    let xpr = Xprite::new(100., 100.);
    let mut state = state::State::new(xpr);

    render::run("Xprite", BGCOLOR, |ui, gl_ctx, textures| {
        let mut rdr = ImguiRenderer::new(&ui);
        // let mut rdr = ImguiCairoRenderer::new(&ui, gl_ctx, textures, &state);
        ui::draw(&mut rdr, &mut state, ui)
    });
}

fn init_logger() {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        // Add blanket level filter -
        .level(log::LevelFilter::Debug)
        // - and per-module overrides
        .level_for("hyper", log::LevelFilter::Info)
        // Output to stdout, files, and other Dispatch configurations
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        // Apply globally
        .apply().unwrap();
}