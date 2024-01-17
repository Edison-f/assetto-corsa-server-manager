#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod convert;
mod serialization;
mod deserialization;
mod defaults;
mod display_settings;
mod display_tracks;
mod display_available_cars;
mod display_car_list;

use std::io::Write;
// hide console window on Windows in release

use eframe::egui;
use egui::TextureHandle;
use regex::Regex;
use serde::{Deserialize};

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1280.0, 720.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Server Manager",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::<ServerManager>::default()
        }),
    )
}

#[derive(PartialEq, Debug)]
struct ServerConfig {
    name: String,
    cars: String,
    config_track: String,
    track: String,
    sun_angle: String,
    password: String,
    admin_password: String,
    udp_port: String,
    tcp_port: String,
    http_port: String,
    max_ballast_kg: String,
    qualify_max_wait_perc: String,
    race_pit_window_start: String,
    race_pit_window_end: String,
    reversed_grid_race_positions: String,
    locked_entry_list: String,
    pickup_mode_enabled: String,
    loop_mode: String,
    sleep_time: String,
    client_send_interval_hz: String,
    send_buffer_size: String,
    recv_buffer_size: String,
    race_over_time: String,
    kick_quorum: String,
    voting_quorum: String,
    vote_duration: String,
    blacklist_mode: String,
    fuel_rate: String,
    damage_multiplier: String,
    tyre_wear_rate: String,
    allowed_tyres_out: String,
    abs_allowed: String,
    tc_allowed: String,
    start_rule: String,
    race_gas_penalty_disabled: String,
    time_of_day_mult: String,
    result_screen_time: String,
    max_contacts_per_km: String,
    stability_allowed: String,
    autoclutch_allowed: String,
    tyre_blankets_allowed: String,
    force_virtual_mirror: String,
    register_to_lobby: String,
    max_clients: String,
    num_threads: String,
    udp_plugin_local_port: String,
    udp_plugin_address: String,
    auth_plugin_address: String,
    legal_tyres: String,
    welcome_message: String,
}

#[derive(PartialEq, Debug)]
struct FTPConfig {
    host: String,
    login: String,
    password: String,
    folder: String,
    linux: String,
}

#[derive(PartialEq, Debug, Deserialize)]
struct PracticeConfig {
    name: String,
    time: String,
    is_open: String,
}

#[derive(PartialEq, Debug, Deserialize)]
struct DynamicTrackConfig {
    session_start: String,
    randomness: String,
    session_transfer: String,
    lap_gain: String,
}

#[derive(PartialEq, Debug, Deserialize)]
struct DataConfig {
    description: String,
    exserverexe: String,
    exserverbat: String,
    exserverhidewin: String,
    weblink: String,
    welcome_path: String,
}

#[derive(Default, PartialEq, Debug)]
struct MasterConfig {
    server: ServerConfig,
    ftp: FTPConfig,
    practice: PracticeConfig,
    dynamic_track: DynamicTrackConfig,
    weather: Vec<WeatherConfig>,
    data: DataConfig,
}

#[derive(PartialEq, Debug)]
struct WeatherConfig {
    graphics: String,
    base_temperature_ambient: String,
    base_temperature_road: String,
    variation_ambient: String,
    variation_road: String,
    wind_base_speed_min: String,
    wind_base_speed_max: String,
    wind_base_direction: String,
    wind_variation_direction: String,
}

#[derive(PartialEq, Debug)]
struct EntryList {
    entries: Vec<Car>,
}

#[derive(PartialEq, Debug)]
struct Car {
    model: String,
    skin: String,
    spectator_mode: bool,
    driver_name: Option<String>,
    team: Option<String>,
    guid: u32,
    ballast: u64,
    restrictor: u8,
}

#[derive(PartialEq)]
struct ServerManager {
    assetto_corsa_path: Option<String>,
    is_path_selected: bool,
    config: MasterConfig,
    entry_list: Option<EntryList>,
    expand_all: bool,
    expand_server: bool,
    expand_ftp: bool,
    expand_practice: bool,
    expand_dynamic_track: bool,
    expand_weather: bool,
    expand_data: bool,
    expand: bool,
    discovered_tracks: bool,
    display_track_images: bool,
    track_list: Vec<Vec<String>>,
    track_textures: Vec<Vec<TextureHandle>>,
    discovered_cars: bool,
    display_car_images: bool,
    available_car_list: Vec<String>,
    car_list: Vec<String>,
    car_textures: Vec<Vec<TextureHandle>>,
    available_car_filter: String,
    car_list_filter: String,
    car_list_changed: bool,
    car_indices: Vec<usize>,
}

impl ServerManager {
    fn parse(&mut self) {
        let config_path = self.assetto_corsa_path.clone().unwrap() + "\\server\\cfg\\server_cfg.ini";
        let config_contents = String::from_utf8(std::fs::read(config_path).unwrap()).unwrap();
        //let entry_path = self.server_path.clone().unwrap() + "\\cfg\\entry_list.ini";
        //let entry_contents = String::from_utf8(std::fs::read(entry_path).unwrap()).unwrap();
        let regex = Regex::new(r"(?m)([\r\n])+").unwrap();
        let split_config = regex.split(config_contents.as_str());
        self.convert(split_config);
        println!("{}", serde_ini::to_string(&self.config).unwrap());
    }
}

impl eframe::App for ServerManager {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("AC Alt Server Manager");
            ui.columns(4, |ui| {
                // Config Editing
                egui::ScrollArea::vertical().show(&mut ui[0], |ui| {
                    if ui.button("Select 'assetto_corsa' Folder…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.assetto_corsa_path = Some(path.display().to_string());
                            self.is_path_selected = true;
                        }
                    }
                    if let Some(picked_path) = &self.assetto_corsa_path {
                        ui.label("Selected Folder Path:");
                        ui.monospace(picked_path);
                        ui.checkbox(&mut self.display_track_images, "Display Track Images");
                        ui.checkbox(&mut self.display_car_images, "Display Car Images");
                        if self.is_path_selected && ui.button("Load Config").clicked() {
                            self.parse();
                            self.update_car_list_from_config();
                            // Update car list from config
                            self.is_path_selected = false;
                        }
                        let save_to_file = ui.button("Save to file");
                        if save_to_file.clicked() {
                            let _ = std::fs::remove_file(format!("{}\\server\\cfg\\temp_server_cfg.ini", self.assetto_corsa_path.clone().unwrap()));
                            let mut file = std::fs::File::create(format!("{}\\server\\cfg\\temp_server_cfg.ini", self.assetto_corsa_path.clone().unwrap())).unwrap();
                            file.write_all(serde_ini::to_string(&self.config).unwrap().as_bytes()).expect("Serialization Error");
                        }
                    }
                    let print_config = ui.button("Print config");
                    if print_config.clicked() {
                        println!("{}", serde_ini::to_string(&self.config).unwrap());
                    }

                    ServerManager::display(self, ui);
                });
                // Track Selection
                egui::ScrollArea::both().id_source("col2").show(&mut ui[1], |ui| {
                    if self.display_track_images {
                        self.display_tracks(ui);
                    }
                });
                if self.display_car_images {
                    self.display_available_cars(&mut ui[2]);
                    self.display_car_list(&mut ui[3]);
                }
            });
        });
    }
}
