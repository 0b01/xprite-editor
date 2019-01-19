#![allow(clippy::float_cmp)]
#![allow(non_snake_case)]

extern crate fern;
#[macro_use]
extern crate log;

#[cfg(feature = "cairo-renderer")]
extern crate cairo;
extern crate clap;
#[macro_use] extern crate glium;
extern crate imgui;
extern crate imgui_glium_renderer;
extern crate imgui_winit_support;
extern crate xprite;

mod consts;
mod prelude;
mod render;
mod state;
mod ui;

use self::prelude::*;
use crate::render::imgui::ImguiRenderer;
use clap::{App, Arg, SubCommand};
use std::sync::{Arc, Mutex};

#[allow(unused)]
fn main() -> Result<(), String> {
    let mut t = App::new("xprite")
        .version("1.0")
        .author("Ricky Han <xprite@rickyhan.com>")
        .about("pixel art editor");
    if cfg!(feature = "python-scripting") {
        t = t.arg(
            Arg::with_name("INPUT")
                .short("-p")
                .long("python")
                .value_name("PY_FILE")
                .help("Run python script"),
        );
    }
    if cfg!(feature = "dyon-scripting") {
        t = t.subcommand(
            SubCommand::with_name("dyon")
                .about("run dyon script")
                .version("1.0")
                .arg(
                    Arg::with_name("INPUT")
                        .help("INPUT.dyon script")
                        .required(true)
                        .index(1),
                ),
        );
    }
    let matches = t.get_matches();

    if let Some(matches) = matches.subcommand_matches("dyon") {
        #[cfg(feature = "dyon-scripting")]
        {
            let inp_file = matches.value_of("INPUT").unwrap();
            run_dyon_script(inp_file)?;
        }
    } else if let Some(inp_file) = matches.value_of("INPUT") {
        #[cfg(feature = "python-scripting")]
        {
            run_python_script(inp_file)?;
        }
    } else {
        run_ui();
    }

    Ok(())
}

#[cfg(feature = "python-scripting")]
fn run_python_script(fname: &str) -> Result<(), String> {
    println!("Running Python script {}", fname);
    let xpr = xprite::scripting::python::python(fname)?;
    println!("Finished {}", fname);
    let mut state = State::new(xpr);
    state.save_png("1.png");
    Ok(())
}

#[cfg(feature = "dyon-scripting")]
fn run_dyon_script(fname: &str) -> Result<(), String> {
    let xpr = Xprite::new(DEFAULT_WIDTH, DEFAULT_HEIGHT);
    let mut state = State::new(xpr);
    state.xpr.execute_dyon_script(fname)?;
    state.save_png("1.png");
    Ok(())
}

fn run_ui() {
    trace!("Starting Xprite");
    let art_w = DEFAULT_WIDTH;
    let art_h = DEFAULT_HEIGHT;
    let xpr = Xprite::new(art_w, art_h);
    init_full_logger(Arc::clone(&xpr.log));
    let mut state = State::new(xpr);

    render::run("Xprite".to_owned(), BGCOLOR, |ui, gl_ctx, textures| {
        let mut rdr = ImguiRenderer::new(&ui, gl_ctx, textures, art_w, art_h);
        // let mut rdr = ImguiCairoRenderer::new(&ui, gl_ctx, textures, &state);
        ui::draw(&mut rdr, &mut state, ui)
    });
}

fn init_full_logger(console_logger: Arc<Mutex<String>>) {
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
        .chain(fern::Output::call(move |record| {
            console_logger
                .lock()
                .unwrap()
                .push_str(&format!("{}\n", record.args()));
        }))
        // .chain(fern::log_file("output.log")?)
        // Apply globally
        .apply()
        .unwrap();
}
