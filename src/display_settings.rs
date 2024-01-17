use egui::Ui;
use crate::ServerManager;

macro_rules! row_display_str {
    ($u:ident, $n:literal, $i:expr) => {
        $u.label($n);
        $u.text_edit_singleline($i);
        $u.end_row();
    };
}

/*
macro_rules! row_display_num {
    ($u:ident, $n:literal, $i:expr) => {
        $u.label($n);
        let mut t = $i.to_string();
        $u.text_edit_singleline(t);
        let p = t.parse();
        match p {
            None => { }

            Some(string) => {
                $i = p.unwrap()
            }
        }
        $u.end_row();
    };
}
*/

impl ServerManager {
    pub(crate) fn display(&mut self, ui: &mut Ui) {
        ui.label("BRUH!");
        self.expand ^= ui.button("Expand/Collapse Master Server Config").clicked();
        // egui::Grid::new("master_config").show(ui, |ui| {
        if self.expand {
            let expand_all_clicked = ui.button("Expand All").clicked();
            if expand_all_clicked {
                self.expand_server = true;
                self.expand_ftp = true;
                self.expand_practice = true;
                self.expand_dynamic_track = true;
                self.expand_weather = true;
                self.expand_data = true;
            }
            self.expand_all ^= expand_all_clicked;
            self.expand_server ^= ui.button("Expand/Collapse Server Config").clicked();
            ui.end_row();
            if self.expand_server { self.display_server(ui) }
            self.expand_ftp ^= ui.button("Expand/Collapse FTP Config").clicked();
            if self.expand_ftp { self.display_ftp(ui); }
            self.expand_practice ^= ui.button("Expand/Collapse Practice Config").clicked();
            if self.expand_practice { self.display_practice(ui); }
            self.expand_dynamic_track ^= ui.button("Expand/Collapse Dynamic Track Config").clicked();
            if self.expand_dynamic_track { self.display_dynamic_track(ui); }
            self.expand_weather ^= ui.button("Expand/Collapse Weather Config").clicked();
            if self.expand_weather { self.display_weather(ui); }
            self.expand_data ^= ui.button("Expand/Collapse Data Config").clicked();
            if self.expand_data { self.display_data(ui); }
        }
    }

    fn display_server(&mut self, ui: &mut Ui) {
        ui.set_max_width(120.0);
        egui::Grid::new("server_config").striped(true).show(ui, |ui| {
            ui.label("Name\t\t\t\t\t\t\t\t\t\t\t\t");
            ui.label("Value\t\t\t\t\t\t\t\t\t\t");
            ui.end_row();

            row_display_str!(ui, "Server Name", &mut self.config.server.name);
            row_display_str!(ui, "Cars (DON'T EDIT)", &mut self.config.server.cars);
            row_display_str!(ui, "Track Configuration", &mut self.config.server.config_track);
            row_display_str!(ui, "Track Name", &mut self.config.server.track);
            row_display_str!(ui, "Sun Angle", &mut self.config.server.sun_angle);
            row_display_str!(ui, "Password", &mut self.config.server.password);
            row_display_str!(ui, "Admin Password", &mut self.config.server.admin_password);
            row_display_str!(ui, "UDP Port", &mut self.config.server.udp_port);
            row_display_str!(ui, "TCP Port", &mut self.config.server.tcp_port);
            row_display_str!(ui, "HTTP Port", &mut self.config.server.http_port);
            row_display_str!(ui, "Max Ballast (kg)", &mut self.config.server.max_ballast_kg);
            row_display_str!(ui, "Qualify Max Wait Percent", &mut self.config.server.qualify_max_wait_perc);
            row_display_str!(ui, "Race Pit Window Start", &mut self.config.server.race_pit_window_start);
            row_display_str!(ui, "Race Pit Window End", &mut self.config.server.race_pit_window_end);
            row_display_str!(ui, "Reversed Grid Race Positions", &mut self.config.server.reversed_grid_race_positions);
            row_display_str!(ui, "Locked Entry List", &mut self.config.server.locked_entry_list);
            row_display_str!(ui, "Pickup Mode", &mut self.config.server.pickup_mode_enabled);
            row_display_str!(ui, "Loop Mode", &mut self.config.server.loop_mode);
            row_display_str!(ui, "Sleep Time", &mut self.config.server.sleep_time);
            row_display_str!(ui, "Client Send Interval hz", &mut self.config.server.client_send_interval_hz);
            row_display_str!(ui, "Send Buffer Size", &mut self.config.server.send_buffer_size);
            row_display_str!(ui, "Receive Buffer Size", &mut self.config.server.recv_buffer_size);
            row_display_str!(ui, "Race Over Time", &mut self.config.server.race_over_time);
            row_display_str!(ui, "Kick Quorum", &mut self.config.server.kick_quorum);
            row_display_str!(ui, "Voting Quorum", &mut self.config.server.voting_quorum);
            row_display_str!(ui, "Vote Duration", &mut self.config.server.vote_duration);
            row_display_str!(ui, "Blacklist Mode", &mut self.config.server.blacklist_mode);
            row_display_str!(ui, "Fuel Rate", &mut self.config.server.fuel_rate);
            row_display_str!(ui, "Damage Multiplier", &mut self.config.server.damage_multiplier);
            row_display_str!(ui, "Tire Wear Rate", &mut self.config.server.tyre_wear_rate);
            row_display_str!(ui, "Allowed Tires Out", &mut self.config.server.allowed_tyres_out);
            row_display_str!(ui, "ABS Allowed", &mut self.config.server.abs_allowed);
            row_display_str!(ui, "Traction Control Allowed", &mut self.config.server.tc_allowed);
            row_display_str!(ui, "Start Rule", &mut self.config.server.start_rule);
            row_display_str!(ui, "Race Gas Penalty Disable", &mut self.config.server.race_gas_penalty_disabled);
            row_display_str!(ui, "Time of Day Multiplier", &mut self.config.server.time_of_day_mult);
            row_display_str!(ui, "Result Screen Time", &mut self.config.server.result_screen_time);
            row_display_str!(ui, "Max Contacts Per km", &mut self.config.server.max_contacts_per_km);
            row_display_str!(ui, "Stability Allowed", &mut self.config.server.stability_allowed);
            row_display_str!(ui, "Autoclutch Allowed", &mut self.config.server.autoclutch_allowed);
            row_display_str!(ui, "Tire Blankets Allowed", &mut self.config.server.tyre_blankets_allowed);
            row_display_str!(ui, "Force Virtual Mirrors", &mut self.config.server.force_virtual_mirror);
            row_display_str!(ui, "Register to Lobby", &mut self.config.server.register_to_lobby);
            row_display_str!(ui, "Max Clients", &mut self.config.server.max_clients);
            row_display_str!(ui, "Number of Threads", &mut self.config.server.num_threads);
            row_display_str!(ui, "UDP Plugin Local Port", &mut self.config.server.udp_plugin_local_port);
            row_display_str!(ui, "UDP Plugin Address", &mut self.config.server.udp_plugin_address);
            row_display_str!(ui, "Auth Plugin Address", &mut self.config.server.auth_plugin_address);
            row_display_str!(ui, "Legal Tires", &mut self.config.server.legal_tyres);
            row_display_str!(ui, "Welcome Message", &mut self.config.server.welcome_message);
        });
    }

    fn display_ftp(&mut self, ui: &mut Ui) {
        egui::Grid::new("ftp_config").striped(true).show(ui, |ui| {
            ui.label("Name\t\t\t\t\t\t\t\t\t\t\t\t");
            ui.label("Value\t\t\t\t\t\t\t\t\t\t");
            ui.end_row();
            row_display_str!(ui, "Host", &mut self.config.ftp.host);
            row_display_str!(ui, "User", &mut self.config.ftp.login);
            row_display_str!(ui, "Pass", &mut self.config.ftp.password);
            row_display_str!(ui, "Path", &mut self.config.ftp.folder);
            row_display_str!(ui, "Port", &mut self.config.ftp.linux);
        });
    }

    fn display_practice(&mut self, ui: &mut Ui) {
        egui::Grid::new("practice_config").striped(true).show(ui, |ui| {
            ui.label("Name\t\t\t\t\t\t\t\t\t\t\t\t");
            ui.label("Value\t\t\t\t\t\t\t\t\t\t");
            ui.end_row();
            row_display_str!(ui, "Name", &mut self.config.practice.name);
            row_display_str!(ui, "Time", &mut self.config.practice.time);
            row_display_str!(ui, "Is Open", &mut self.config.practice.is_open);
        });
    }

    fn display_dynamic_track(&mut self, ui: &mut Ui) {
        egui::Grid::new("dynamic_track_config").striped(true).show(ui, |ui| {
            ui.label("Name\t\t\t\t\t\t\t\t\t\t\t\t");
            ui.label("Value\t\t\t\t\t\t\t\t\t\t");
            ui.end_row();
            row_display_str!(ui, "Session Start", &mut self.config.dynamic_track.session_start);
            row_display_str!(ui, "Randomness", &mut self.config.dynamic_track.randomness);
            row_display_str!(ui, "Session Transfer", &mut self.config.dynamic_track.session_transfer);
            row_display_str!(ui, "Lap Gain", &mut self.config.dynamic_track.lap_gain);
        });
    }

    fn display_weather(&mut self, ui: &mut Ui) {
        egui::Grid::new("weather_config").striped(true).show(ui, |ui| {
            ui.label("Name\t\t\t\t\t\t\t\t\t\t\t\t");
            ui.label("Value\t\t\t\t\t\t\t\t\t\t");
            ui.end_row();
            if self.config.weather.is_empty() {
                return;
            }
            row_display_str!(ui, "GRAPHICS", &mut self.config.weather[0].graphics);
            row_display_str!(ui, "BASE_TEMPERATURE_AMBIENT", &mut self.config.weather[0].base_temperature_ambient);
            row_display_str!(ui, "BASE_TEMPERATURE_ROAD", &mut self.config.weather[0].base_temperature_road);
            row_display_str!(ui, "VARIATION_AMBIENT", &mut self.config.weather[0].variation_ambient);
            row_display_str!(ui, "VARIATION_ROAD", &mut self.config.weather[0].variation_road);
            row_display_str!(ui, "WIND_BASE_SPEED_MIN", &mut self.config.weather[0].wind_base_speed_min);
            row_display_str!(ui, "WIND_BASE_SPEED_MAX", &mut self.config.weather[0].wind_base_speed_max);
            row_display_str!(ui, "WIND_BASE_DIRECTION", &mut self.config.weather[0].wind_base_direction);
            row_display_str!(ui, "WIND_VARIATION_DIRECTION", &mut self.config.weather[0].wind_variation_direction);
        });
    }

    fn display_data(&mut self, ui: &mut Ui) {
        egui::Grid::new("data_config").striped(true).show(ui, |ui| {
            ui.label("Name\t\t\t\t\t\t\t\t\t\t\t\t");
            ui.label("Value\t\t\t\t\t\t\t\t\t\t");
            ui.end_row();
            row_display_str!(ui, "Description", &mut self.config.data.description);
            row_display_str!(ui, "Exserverexe", &mut self.config.data.exserverexe);
            row_display_str!(ui, "Exserverbat", &mut self.config.data.exserverbat);
            row_display_str!(ui, "Exserverhidewin", &mut self.config.data.exserverhidewin);
            row_display_str!(ui, "Weblink", &mut self.config.data.weblink);
            row_display_str!(ui, "Welcome Path", &mut self.config.data.welcome_path);
        });
    }
}

