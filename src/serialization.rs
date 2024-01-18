use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

use crate::{DataConfig, DynamicTrackConfig, EntryList, FTPConfig, MasterConfig, PracticeConfig, ServerConfig, WeatherConfig};

/*
macro_rules! option_serialize {
    ($a:ident, $l:expr, $s:literal, $n:literal) => {
        match $l {
            None => {
                $a.serialize_field($s, "").expect($n)
            }
            Some(_string) => {
                $a.serialize_field($s, $l).expect($n);
            }
        }
    };
}
*/


impl Serialize for MasterConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("MasterConfig", 3)?;
        state.serialize_field("SERVER", &self.server).expect("Serialize MC went wrong");
        state.serialize_field("FTP", &self.ftp).expect("Serialize MC went wrong");
        state.serialize_field("PRACTICE", &self.practice).expect("Serialize MC went wrong");
        state.serialize_field("DYNAMIC_TRACK", &self.dynamic_track).expect("Serialize MC went wrong");
        for i in 0..self.weather.len() {
            state.serialize_field(concat!("WEATHER_{}", stringify!(i)), &self.weather.get(i)).expect("Serialize MC went wrong");
        }
        state.serialize_field("DATA", &self.data).expect("Serialize MC went wrong");
        state.end()
    }
}

impl EntryList {
    // Have to do this because of CAR_{i} needs to be static but I don't know how to make a string static and have changing variables without extreme hardcoding
    pub(crate) fn serialize(&self) -> String {
        let mut result = String::from("");
        for i in 0..self.list.len() {
            result += format!("[CAR_{}]\n", i).as_str();
            result += format!("MODEL={}\n", self.list.get(i).unwrap().model).as_str();
            result += format!("SKIN={}\n", self.list.get(i).unwrap().skin).as_str();
            result += format!("SPECTATOR_MODE={}\n", self.list.get(i).unwrap().spectator_mode).as_str();
            result += format!("DRIVERNAME={}\n", self.list.get(i).unwrap().driver_name).as_str();
            result += format!("TEAM={}\n", self.list.get(i).unwrap().team).as_str();
            result += format!("GUID={}\n", self.list.get(i).unwrap().guid).as_str();
            result += format!("BALLAST={}\n", self.list.get(i).unwrap().ballast).as_str();
            result += format!("RESTRICTOR={}\n\n", self.list.get(i).unwrap().restrictor).as_str();
        }
        result
    }
}

impl Serialize for ServerConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("ServerConfig", 49)?;
        state.serialize_field("NAME", &self.name).expect("ServerConfig serialize went wrong");
        state.serialize_field("CARS", &self.cars).expect("ServerConfig serialize went wrong");
        state.serialize_field("CONFIG_TRACK", &self.config_track).expect("ServerConfig serialize went wrong");
        state.serialize_field("TRACK", &self.track).expect("ServerConfig serialize went wrong");
        state.serialize_field("SUN_ANGLE", &self.sun_angle).expect("ServerConfig serialize went wrong");
        state.serialize_field("PASSWORD", &self.password).expect("ServerConfig serialize went wrong");
        state.serialize_field("ADMIN_PASSWORD", &self.admin_password).expect("ServerConfig serialize went wrong");
        state.serialize_field("UDP_PORT", &self.udp_port).expect("ServerConfig serialize went wrong");
        state.serialize_field("TCP_PORT", &self.tcp_port).expect("ServerConfig serialize went wrong");
        state.serialize_field("HTTP_PORT", &self.http_port).expect("ServerConfig serialize went wrong");
        state.serialize_field("MAX_BALLAST_KG", &self.max_ballast_kg).expect("ServerConfig serialize went wrong");
        state.serialize_field("QUALIFY_MAX_WAIT_PERC", &self.qualify_max_wait_perc).expect("ServerConfig serialize went wrong");
        state.serialize_field("RACE_PIT_WINDOW_START", &self.race_pit_window_start).expect("ServerConfig serialize went wrong");
        state.serialize_field("RACE_PIT_WINDOW_END", &self.race_pit_window_end).expect("ServerConfig serialize went wrong");
        state.serialize_field("REVERSED_GRID_RACE_POSITIONS", &self.reversed_grid_race_positions).expect("ServerConfig serialize went wrong");
        state.serialize_field("LOCKED_ENTRY_LIST", &self.locked_entry_list).expect("ServerConfig serialize went wrong");
        state.serialize_field("PICKUP_MODE_ENABLED", &self.pickup_mode_enabled).expect("ServerConfig serialize went wrong");
        state.serialize_field("LOOP_MODE", &self.loop_mode).expect("ServerConfig serialize went wrong");
        state.serialize_field("SLEEP_TIME", &self.sleep_time).expect("ServerConfig serialize went wrong");
        state.serialize_field("CLIENT_SEND_INTERVAL_HZ", &self.client_send_interval_hz).expect("ServerConfig serialize went wrong");
        state.serialize_field("SEND_BUFFER_SIZE", &self.send_buffer_size).expect("ServerConfig serialize went wrong");
        state.serialize_field("RECV_BUFFER_SIZE", &self.recv_buffer_size).expect("ServerConfig serialize went wrong");
        state.serialize_field("RACE_OVER_TIME", &self.race_over_time).expect("ServerConfig serialize went wrong");
        state.serialize_field("KICK_QUORUM", &self.kick_quorum).expect("ServerConfig serialize went wrong");
        state.serialize_field("VOTING_QUORUM", &self.voting_quorum).expect("ServerConfig serialize went wrong");
        state.serialize_field("VOTE_DURATION", &self.vote_duration).expect("ServerConfig serialize went wrong");
        state.serialize_field("BLACKLIST_MODE", &self.blacklist_mode).expect("ServerConfig serialize went wrong");
        state.serialize_field("FUEL_RATE", &self.fuel_rate).expect("ServerConfig serialize went wrong");
        state.serialize_field("DAMAGE_MULTIPLIER", &self.damage_multiplier).expect("ServerConfig serialize went wrong");
        state.serialize_field("TYRE_WEAR_RATE", &self.tyre_wear_rate).expect("ServerConfig serialize went wrong");
        state.serialize_field("ALLOWED_TYRES_OUT", &self.allowed_tyres_out).expect("ServerConfig serialize went wrong");
        state.serialize_field("ABS_ALLOWED", &self.abs_allowed).expect("ServerConfig serialize went wrong");
        state.serialize_field("TC_ALLOWED", &self.tc_allowed).expect("ServerConfig serialize went wrong");
        state.serialize_field("START_RULE", &self.start_rule).expect("ServerConfig serialize went wrong");
        state.serialize_field("RACE_GAS_PENALTY_DISABLED", &self.race_gas_penalty_disabled).expect("ServerConfig serialize went wrong");
        state.serialize_field("TIME_OF_DAY_MULT", &self.time_of_day_mult).expect("ServerConfig serialize went wrong");
        state.serialize_field("RESULT_SCREEN_TIME", &self.result_screen_time).expect("ServerConfig serialize went wrong");
        state.serialize_field("MAX_CONTACTS_PER_KM", &self.max_contacts_per_km).expect("ServerConfig serialize went wrong");
        state.serialize_field("STABILITY_ALLOWED", &self.stability_allowed).expect("ServerConfig serialize went wrong");
        state.serialize_field("AUTOCLUTCH_ALLOWED", &self.autoclutch_allowed).expect("ServerConfig serialize went wrong");
        state.serialize_field("TYRE_BLANKETS_ALLOWED", &self.tyre_blankets_allowed).expect("ServerConfig serialize went wrong");
        state.serialize_field("FORCE_VIRTUAL_MIRROR", &self.force_virtual_mirror).expect("ServerConfig serialize went wrong");
        state.serialize_field("REGISTER_TO_LOBBY", &self.register_to_lobby).expect("ServerConfig serialize went wrong");
        state.serialize_field("MAX_CLIENTS", &self.max_clients).expect("ServerConfig serialize went wrong");
        state.serialize_field("NUM_THREADS", &self.num_threads).expect("ServerConfig serialize went wrong");
        state.serialize_field("UDP_PLUGIN_LOCAL_PORT", &self.udp_plugin_local_port).expect("ServerConfig serialize went wrong");
        state.serialize_field("UDP_PLUGIN_ADDRESS", &self.udp_plugin_address).expect("ServerConfig serialize went wrong");
        state.serialize_field("AUTH_PLUGIN_ADDRESS", &self.auth_plugin_address).expect("ServerConfig serialize went wrong");
        state.serialize_field("LEGAL_TYRES", &self.legal_tyres).expect("ServerConfig serialize went wrong");
        state.serialize_field("WELCOME_MESSAGE", &self.welcome_message).expect("ServerConfig serialize went wrong");
        state.end()
    }
}

impl Serialize for FTPConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("FTPConfig", 5)?;
        state.serialize_field("HOST", &self.host).expect("FTPConfig serialize went wrong");
        state.serialize_field("LOGIN", &self.login).expect("FTPConfig serialize went wrong");
        state.serialize_field("PASSWORD", &self.password).expect("FTPConfig serialize went wrong");
        state.serialize_field("FOLDER", &self.folder).expect("FTPConfig serialize went wrong");
        state.serialize_field("LINUX", &self.linux).expect("FTPConfig serialize went wrong");
        state.end()
    }
}

impl Serialize for PracticeConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("PracticeConfig", 3)?;
        state.serialize_field("NAME", &self.name).expect("PracticeConfig serialize went wrong");
        state.serialize_field("TIME", &self.time).expect("PracticeConfig serialize went wrong");
        state.serialize_field("IS_OPEN", &self.is_open).expect("PracticeConfig serialize went wrong");
        state.end()
    }
}

impl Serialize for DynamicTrackConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("DynamicTrackConfig", 4)?;
        state.serialize_field("SESSION_START", &self.session_start).expect("DynamicTrackConfig serialize went wrong");
        state.serialize_field("RANDOMNESS", &self.randomness).expect("DynamicTrackConfig serialize went wrong");
        state.serialize_field("SESSION_TRANSFER", &self.session_transfer).expect("DynamicTrackConfig serialize went wrong");
        state.serialize_field("LAP_GAIN", &self.lap_gain).expect("DynamicTrackConfig serialize went wrong");
        state.end()
    }
}

impl Serialize for WeatherConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("WeatherConfig", 9)?;
        state.serialize_field("GRAPHICS", &self.graphics).expect("WeatherConfig serialize went wrong");
        state.serialize_field("BASE_TEMPERATURE_AMBIENT", &self.base_temperature_ambient).expect("WeatherConfig serialize went wrong");
        state.serialize_field("BASE_TEMPERATURE_ROAD", &self.base_temperature_road).expect("WeatherConfig serialize went wrong");
        state.serialize_field("VARIATION_AMBIENT", &self.variation_ambient).expect("WeatherConfig serialize went wrong");
        state.serialize_field("VARIATION_ROAD", &self.variation_road).expect("WeatherConfig serialize went wrong");
        state.serialize_field("WIND_BASE_SPEED_MIN", &self.wind_base_speed_min).expect("WeatherConfig serialize went wrong");
        state.serialize_field("WIND_BASE_SPEED_MAX", &self.wind_base_speed_max).expect("WeatherConfig serialize went wrong");
        state.serialize_field("WIND_BASE_DIRECTION", &self.wind_base_direction).expect("WeatherConfig serialize went wrong");
        state.serialize_field("WIND_VARIATION_DIRECTION", &self.wind_variation_direction).expect("WeatherConfig serialize went wrong");
        state.end()
    }
}

impl Serialize for DataConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let mut state = serializer.serialize_struct("DataConfig", 6)?;
        state.serialize_field("DESCRIPTION", &self.description).expect("DataConfig serialize went wrong");
        state.serialize_field("EXSERVEREXE", &self.exserverexe).expect("DataConfig serialize went wrong");
        state.serialize_field("EXSERVERBAT", &self.exserverbat).expect("DataConfig serialize went wrong");
        state.serialize_field("EXSERVERHIDEWIN", &self.exserverhidewin).expect("DataConfig serialize went wrong");
        state.serialize_field("WEBLINK", &self.weblink).expect("DataConfig serialize went wrong");
        state.serialize_field("WELCOME_PATH", &self.welcome_path).expect("DataConfig serialize went wrong");
        state.end()
    }
}
