// use std::fmt::Formatter;
// use std::marker::PhantomData;
// use egui::Key::P;
// use serde::{Deserialize, Deserializer, Serialize};
// use serde::de::Visitor;
// use crate::{PracticeConfig, ServerConfig};
//
// struct ServerConfigVisitor {
//     marker: PhantomData<ServerConfig>
// }
//
// impl ServerConfigVisitor {
//     fn new() -> Self {
//         ServerConfigVisitor {
//             marker: PhantomData
//         }
//     }
// }
//
// impl <'de> Visitor<'de> for ServerConfigVisitor {
//     type Value = ServerConfig;
//
//     fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
//         formatter.write_str("Server config portion")
//     }
// }
//
// impl<'de> Deserialize<'de> for ServerConfig {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
//         todo!()
//     }
// }
//
// struct PracticeConfigVisitor {
//     marker: PhantomData<PracticeConfig>
// }
//
// impl PracticeConfigVisitor {
//     fn new() -> Self {
//         PracticeConfigVisitor {
//             marker: PhantomData
//         }
//     }
// }
//
// impl<'de> Deserialize<'de> for PracticeConfig {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
//         todo!()
//     }
// }