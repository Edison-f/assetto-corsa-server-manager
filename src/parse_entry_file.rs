use regex::{Regex, Split};

use crate::{Car, ServerManager};
use crate::parse_server_config::as_vec;

impl ServerManager {
    pub(crate) fn parse_entry_list(&mut self, split: Split) {
        let regex = Regex::new(r"=").unwrap();
        let mut curr: Car = Car::default();
        let mut first = true;
        let mut i = 0;
        for s in split {
            let arr = as_vec(regex.splitn(s, 2));
            if arr.len() == 1 {
                if !first && i % 8 == 0 {
                    i = 0;
                    self.entry_list.list.push(curr.clone());
                    curr.reset();
                } else {
                    first = false;
                }
                curr = Default::default();
            } else if arr.len() == 2 {
                let entry = arr.first().unwrap().as_str();
                let val = arr.get(1).unwrap().to_string();
                match entry {
                    "MODEL" => { curr.model = val; }
                    "SKIN" => { curr.skin = val; }
                    "SPECTATOR_MODE" => { curr.spectator_mode = val; }
                    "DRIVER_NAME" => { curr.driver_name = val; }
                    "TEAM" => { curr.team = val; }
                    "GUID" => { curr.guid = val; }
                    "BALLAST" => { curr.ballast = val; }
                    "RESTRICTOR" => { curr.restrictor = val; }
                    _ => { println!("Unknown attribute in entry list") }
                }
                i += 1;
            }
        }
    }
}