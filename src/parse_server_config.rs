use regex::{Regex, Split, SplitN};
use crate::ServerManager;

pub(crate) fn as_vec(arr: SplitN) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for s in arr {
        result.push(s.to_string());
    }
    result
}

impl ServerManager {
    pub(crate) fn parse_server_config(&mut self, arr: Split) {
        let regex = Regex::new(r"=").unwrap();
        let mut modifier = 0;
        for s in arr {
            if !s.is_empty() {
                modifier += 1;
            }
            let split: Vec<String> = as_vec(regex.splitn(s, 2));
            if split.len() >= 2 {
                let curr = split.first().unwrap().as_str();
                let val = split.get(1).unwrap();
                if val.is_empty() {
                    continue;
                }
                if (2..=51).contains(&modifier) {
                    match curr {
                        "NAME" => { self.config.server.name = val.parse().unwrap(); }
                        "CARS" => {
                            self.config.server.cars = val.parse().unwrap();
                        }
                        "CONFIG_TRACK" => { self.config.server.config_track = val.parse().unwrap(); }
                        "TRACK" => { self.config.server.track = val.parse().unwrap(); }
                        "SUN_ANGLE" => { self.config.server.sun_angle = val.parse().unwrap(); }
                        "PASSWORD" => { self.config.server.password = val.parse().unwrap(); }
                        "ADMIN_PASSWORD" => { self.config.server.admin_password = val.parse().unwrap(); }
                        "UDP_PORT" => { self.config.server.udp_port = val.parse().unwrap(); }
                        "TCP_PORT" => { self.config.server.tcp_port = val.parse().unwrap(); }
                        "HTTP_PORT" => { self.config.server.http_port = val.parse().unwrap(); }
                        "MAX_BALLAST_KG" => { self.config.server.max_ballast_kg = val.parse().unwrap(); }
                        "QUALIFY_MAX_WAIT_PERC" => { self.config.server.qualify_max_wait_perc = val.parse().unwrap(); }
                        "RACE_PIT_WINDOW_START" => { self.config.server.race_pit_window_start = val.parse().unwrap(); }
                        "RACE_PIT_WINDOW_END" => { self.config.server.race_pit_window_end = val.parse().unwrap(); }
                        "REVERSED_GRID_RACE_POSITIONS" => { self.config.server.reversed_grid_race_positions = val.parse().unwrap(); }
                        "LOCKED_ENTRY_LIST" => { self.config.server.locked_entry_list = val.parse().unwrap(); }
                        "PICKUP_MODE_ENABLED" => { self.config.server.pickup_mode_enabled = val.parse().unwrap(); }
                        "LOOP_MODE" => { self.config.server.loop_mode = val.parse().unwrap(); }
                        "SLEEP_TIME" => { self.config.server.sleep_time = val.parse().unwrap(); }
                        "CLIENT_SEND_INTERVAL_HZ" => { self.config.server.client_send_interval_hz = val.parse().unwrap(); }
                        "SEND_BUFFER_SIZE" => { self.config.server.send_buffer_size = val.parse().unwrap(); }
                        "RECV_BUFFER_SIZE" => { self.config.server.recv_buffer_size = val.parse().unwrap(); }
                        "RACE_OVER_TIME" => { self.config.server.race_over_time = val.parse().unwrap(); }
                        "KICK_QUORUM" => { self.config.server.kick_quorum = val.parse().unwrap(); }
                        "VOTING_QUORUM" => { self.config.server.voting_quorum = val.parse().unwrap(); }
                        "VOTE_DURATION" => { self.config.server.vote_duration = val.parse().unwrap(); }
                        "BLACKLIST_MODE" => { self.config.server.blacklist_mode = val.parse().unwrap(); }
                        "FUEL_RATE" => { self.config.server.fuel_rate = val.parse().unwrap(); }
                        "DAMAGE_MULTIPLIER" => { self.config.server.damage_multiplier = val.parse().unwrap(); }
                        "TYRE_WEAR_RATE" => { self.config.server.tyre_wear_rate = val.parse().unwrap(); }
                        "ALLOWED_TYRES_OUT" => { self.config.server.allowed_tyres_out = val.parse().unwrap(); }
                        "ABS_ALLOWED" => { self.config.server.abs_allowed = val.parse().unwrap(); }
                        "TC_ALLOWED" => { self.config.server.tc_allowed = val.parse().unwrap(); }
                        "START_RULE" => { self.config.server.start_rule = val.parse().unwrap(); }
                        "RACE_GAS_PENALTY_DISABLED" => { self.config.server.race_gas_penalty_disabled = val.parse().unwrap(); }
                        "TIME_OF_DAY_MULT" => { self.config.server.time_of_day_mult = val.parse().unwrap(); }
                        "RESULT_SCREEN_TIME" => { self.config.server.result_screen_time = val.parse().unwrap(); }
                        "MAX_CONTACTS_PER_KM" => { self.config.server.max_contacts_per_km = val.parse().unwrap(); }
                        "STABILITY_ALLOWED" => { self.config.server.stability_allowed = val.parse().unwrap(); }
                        "AUTOCLUTCH_ALLOWED" => { self.config.server.autoclutch_allowed = val.parse().unwrap(); }
                        "TYRE_BLANKETS_ALLOWED" => { self.config.server.tyre_blankets_allowed = val.parse().unwrap(); }
                        "FORCE_VIRTUAL_MIRROR" => { self.config.server.force_virtual_mirror = val.parse().unwrap(); }
                        "REGISTER_TO_LOBBY" => { self.config.server.register_to_lobby = val.parse().unwrap(); }
                        "MAX_CLIENTS" => { self.config.server.max_clients = val.parse().unwrap(); }
                        "NUM_THREADS" => { self.config.server.num_threads = val.parse().unwrap(); }
                        "UDP_PLUGIN_LOCAL_PORT" => { self.config.server.udp_plugin_local_port = val.parse().unwrap(); }
                        "UDP_PLUGIN_ADDRESS" => { self.config.server.udp_plugin_address = val.parse().unwrap(); }
                        "AUTH_PLUGIN_ADDRESS" => { self.config.server.auth_plugin_address = val.parse().unwrap(); }
                        "LEGAL_TYRES" => { self.config.server.legal_tyres = val.parse().unwrap(); }
                        "WELCOME_MESSAGE" => { self.config.server.welcome_message = val.parse().unwrap(); }
                        _ => {println!("Entry {} does not exist", curr);}
                    }
                } else if (53..=57).contains(&modifier) {
                    match curr {
                        "HOST" => { self.config.ftp.host = val.parse().unwrap(); }
                        "LOGIN" => { self.config.ftp.login = val.parse().unwrap(); }
                        "PASSWORD" => { self.config.ftp.password = val.parse().unwrap(); }
                        "FOLDER" => { self.config.ftp.folder = val.parse().unwrap(); }
                        "LINUX" => { self.config.ftp.linux = val.parse().unwrap(); }
                        _ => {println!("Entry {} does not exist", curr);}
                    }
                } else if (59..=61).contains(&modifier) {
                    match curr {
                        "NAME" => { self.config.practice.name = val.parse().unwrap(); }
                        "TIME" => { self.config.practice.time = val.parse().unwrap(); }
                        "IS_OPEN" => { self.config.practice.is_open = val.parse().unwrap(); }
                        _ => {println!("Entry {} does not exist", curr);}
                    }
                } else if (63..=66).contains(&modifier) {
                    match curr {
                        "SESSION_START" => { self.config.dynamic_track.session_start = val.parse().unwrap(); }
                        "RANDOMNESS" => { self.config.dynamic_track.randomness = val.parse().unwrap(); }
                        "SESSION_TRANSFER" => { self.config.dynamic_track.session_transfer = val.parse().unwrap(); }
                        "LAP_GAIN" => { self.config.dynamic_track.lap_gain = val.parse().unwrap(); }
                        _ => {println!("Entry {} does not exist", curr);}
                    }
                } else if modifier >= 78 {
                    match curr {
                        "DESCRIPTION" => { self.config.data.description = val.parse().unwrap(); }
                        "EXSERVEREXE" => { self.config.data.exserverexe = val.parse().unwrap(); }
                        "EXSERVERBAT" => { self.config.data.exserverbat = val.parse().unwrap(); }
                        "EXSERVERHIDEWIN" => { self.config.data.exserverhidewin = val.parse().unwrap(); }
                        "WEBLINK" => { self.config.data.weblink = val.parse().unwrap(); }
                        "WELCOME_PATH" => { self.config.data.welcome_path = val.parse().unwrap(); }
                        _ => {println!("Entry {} does not exist", curr);}
                    }
                }
            }
        }
    }
}

