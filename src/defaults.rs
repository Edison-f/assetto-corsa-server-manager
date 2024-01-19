use crate::{DataConfig, DynamicTrackConfig, FTPConfig, MasterConfig, ServerManager, PracticeConfig, ServerConfig, Car, EntryList};

impl Default for ServerManager {
    fn default() -> Self {
        Self {
            assetto_corsa_path: None,
            is_path_selected: false,
            config: MasterConfig::default(),
            entry_list: EntryList::default(),
            expand_all: false,
            expand_server: false,
            expand_ftp: false,
            expand_practice: false,
            expand_dynamic_track: false,
            expand_weather: false,
            expand_data: false,
            expand: false,
            discovered_tracks: false,
            display_track_images: false,
            track_list: vec![],
            track_textures: vec![],
            discovered_cars: false,
            display_car_images: false,
            available_car_list: vec![],
            available_skins_list: Default::default(),
            car_list: vec![],
            car_textures: vec![],
            available_car_filter: "".to_string(),
            car_list_filter: "".to_string(),
            car_list_changed: false,
            car_indices: vec![],
            car_skins: Default::default(),
            car_count: Default::default(),
            expand_available_skins: Default::default(),
        }
    }
}

impl Default for Car {
    fn default() -> Self {
        Self {
            model: "".to_string(),
            skin: "".to_string(),
            spectator_mode: "".to_string(),
            driver_name: "".to_string(),
            team: "".to_string(),
            guid: "".to_string(),
            ballast: "".to_string(),
            restrictor: "".to_string(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            cars: "".to_owned(),
            config_track: "".to_owned(),
            track: "".to_owned(),
            sun_angle: 0.to_string(),
            password: "".to_owned(),
            admin_password: "".to_owned(),
            udp_port: 9600.to_string(),
            tcp_port: 9600.to_string(),
            http_port: 8081.to_string(),
            max_ballast_kg: 150.to_string(),
            qualify_max_wait_perc: 120.to_string(),
            race_pit_window_start: 0.to_string(),
            race_pit_window_end: 0.to_string(),
            reversed_grid_race_positions: 0.to_string(),
            locked_entry_list: 0.to_string(),
            pickup_mode_enabled: 1.to_string(),
            loop_mode: 1.to_string(),
            sleep_time: 1.to_string(),
            client_send_interval_hz: 18.to_string(),
            send_buffer_size: 0.to_string(),
            recv_buffer_size: 0.to_string(),
            race_over_time: 60.to_string(),
            kick_quorum: 70.to_string(),
            voting_quorum: 70.to_string(),
            vote_duration: 15.to_string(),
            blacklist_mode: 0.to_string(),
            fuel_rate: 0.0.to_string(),
            damage_multiplier: 0.0.to_string(),
            tyre_wear_rate: 0.0.to_string(),
            allowed_tyres_out: 4.to_string(),
            abs_allowed: 1.to_string(),
            tc_allowed: 1.to_string(),
            start_rule: 1.to_string(),
            race_gas_penalty_disabled: 1.to_string(),
            time_of_day_mult: 1.to_string(),
            result_screen_time: 60.to_string(),
            max_contacts_per_km: 99.to_string(),
            stability_allowed: 0.to_string(),
            autoclutch_allowed: 0.to_string(),
            tyre_blankets_allowed: 1.to_string(),
            force_virtual_mirror: 0.to_string(),
            register_to_lobby: 1.to_string(),
            max_clients: 8.to_string(),
            num_threads: 2.to_string(),
            udp_plugin_local_port: 0.to_string(),
            udp_plugin_address: 16.to_string(),
            auth_plugin_address: 16.to_string(),
            legal_tyres: "".to_owned(),
            welcome_message: "".to_owned(),
        }
    }
}

impl Default for FTPConfig {
    fn default() -> Self {
        Self {
            host: "".to_owned(),
            login: "".to_owned(),
            password: "".to_owned(),
            folder: "".to_owned(),
            linux: 0.to_string(),
        }
    }
}

impl Default for PracticeConfig {
    fn default() -> Self {
        Self {
            name: "".to_owned(),
            time: 9999.to_string(),
            is_open: 1.to_string(),
        }
    }
}

impl Default for DynamicTrackConfig {
    fn default() -> Self {
        Self {
            session_start: 8.to_string(),
            randomness: 8.to_string(),
            session_transfer: 8.to_string(),
            lap_gain: 16.to_string(),
        }
    }
}

impl Default for DataConfig {
    fn default() -> Self {
        Self {
            description: "".to_owned(),
            exserverexe: "".to_owned(),
            exserverbat: "".to_owned(),
            exserverhidewin: 0.to_string(),
            weblink: "".to_owned(),
            welcome_path: "".to_owned(),
        }
    }
}
