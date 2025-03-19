use clap::{Arg, ArgAction, ArgMatches, Command, Parser, Subcommand};

use libp2p::{gossipsub, mdns, noise, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux};

use once_cell::sync::OnceCell;
use std::{error::Error, time::Duration};
use tokio::{io, io::AsyncBufReadExt};
use tracing::debug;
use tracing_subscriber::EnvFilter;

mod p2p;
mod ui;
use p2p::evt_loop;
mod msg;
use msg::*;

use bevy::{prelude::*, window::WindowResized};

use bevy_egui::{egui, EguiContexts, EguiPlugin};
use gnostr_chat::RataguiBackend;
use ratatui::{layout::Rect as RatatuiRect, prelude::Direction};
use ratatui::{
    prelude::{Stylize, Terminal},
    widgets::{Block, Borders, Paragraph, Wrap},
};

//use egui::Rect as EguiRect;

use bevy_egui::egui::Rect as EguiRect;

fn ratatui_rect_from_bevy_egui_rect(egui_rect: EguiRect) -> RatatuiRect {
    RatatuiRect::new(
        egui_rect.min.x as u16,
        egui_rect.min.y as u16,
        (egui_rect.max.x - egui_rect.min.x) as u16,
        (egui_rect.max.y - egui_rect.min.y) as u16,
    )
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<BevyTerminal<RataguiBackend>>()
        //Initialize the ratatui terminal
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .add_systems(Update, handle_window_events)
        .run();
}

// Render to the terminal and to egui , both are immediate mode
fn ui_example_system(
    mut contexts: EguiContexts,
    windows: Query<&Window>,
    mut termres: ResMut<BevyTerminal<RataguiBackend>>,
) {
    for window in windows.iter() {
        let max_width = window.resolution.width();
        debug!("Max width: {}", max_width);
        let max_height = window.resolution.height();
        debug!("Max height: {}", max_height);
        //}

        let primary_window = windows.single();
        let width_height = egui::vec2(primary_window.width() - 10., primary_window.height() - 10.);
        termres
            .terminal
            .draw(|frame| {
                let primary_window = windows.single();

                let pos = egui::pos2(20.0, 20.0);
                let width_height =
                    egui::vec2(primary_window.width() - 10., primary_window.height() - 10.);
                let window_area = RatatuiRect::new(
                    pos.x as u16,
                    pos.y as u16,
                    max_height as u16, //width_height.x as u16, //- pos.x as u16,
                    max_width as u16,  //width_height.y as u16, //- pos.y as u16,
                );
                let paragraph_area = RatatuiRect::new(
                    pos.x as u16,
                    pos.y as u16,
                    max_height as u16 - 20.0 as u16, //width_height.x as u16, //- pos.x as u16,
                    max_width as u16 - 20.0 as u16,  //width_height.y as u16, //- pos.y as u16,
                );
                let area = frame.area();
                let textik = format!("Hello bevy!\nThe adjusted window area is {}", window_area);
                frame.render_widget(
                    Paragraph::new(textik)
                        .block(Block::new().title("LOL").borders(Borders::ALL))
                        .white()
                        .on_blue()
                        .wrap(Wrap { trim: false }),
                    area,
                );
            })
            .expect("epic fail");

        egui::Window::new("Hello Bevy-Egui")
            //.min_size(egui::vec2(200.0, 150.0)) // Minimum width and height
            //.max_size(egui::vec2(600.0, 400.0)) // Maximum width and height
            .show(contexts.ctx_mut(), |ui| {
                //ui.set_opacity(0.5);
                ui.set_max_width(max_width - 40.);
                ui.set_max_height(max_height - 40.);
                //ui.label("This label will be constrained to a maximum width of 300 pixels.");
                //ui.text_edit_multiline(&mut String::new()); // Multiline text edit
                //ui.button("Click me!");
                let huh = termres.terminal.backend_mut();
                ui.add(huh);
            });
    }
}
// Create resource to hold the ratatui terminal
#[derive(Resource)]
struct BevyTerminal<RataguiBackend: ratatui::backend::Backend> {
    terminal: Terminal<RataguiBackend>,
}

// Implement default on the resource to initialize it
impl Default for BevyTerminal<RataguiBackend> {
    fn default() -> Self {
        let backend = RataguiBackend::new(0, 0);
        let terminal = Terminal::new(backend).unwrap();
        BevyTerminal { terminal }
    }
}

fn handle_window_events(mut resize_events: EventReader<WindowResized>, windows: Query<&Window>) {
    for event in resize_events.read() {
        let window = windows.get(event.window).unwrap(); // Get the window that was resized.
        debug!(
            "Window resized: id: {:?}, width: {}, height: {}",
            event.window,
            window.width(),
            window.height()
        );
        // You can now do whatever you want with this information, such as:
        // * Adjusting camera projection.
        // * Re-laying out UI elements.
        // * Updating rendering parameters.
    }
}
