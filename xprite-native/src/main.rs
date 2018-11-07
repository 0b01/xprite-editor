extern crate fern;
#[macro_use]
extern crate log;

extern crate xprite;
extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;

use xprite::prelude::*;
use crate::render::imgui_renderer::ImguiRenderer;

mod render;
mod ui;
mod state;

const BGCOLOR: [f32; 4] = [0.,0.,0.,0.];

fn main() {
    init_logger();
    info!("Starting Xprite");
    let xpr = Xprite::new(600, 600);
    let mut state = state::State::new(xpr);
    render::run("Xprite", BGCOLOR, |ui| {
        let mut renderer = ImguiRenderer::new();
        renderer.init(ui);
        xpr.init(Box::new(renderer));

        ui::draw(&mut state, ui)
    });
}

fn init_logger() {
    fern::Dispatch::new()
        // Perform allocation-free log formatting
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
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