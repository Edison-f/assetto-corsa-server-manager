use eframe::emath::Vec2;
use eframe::epaint::ColorImage;
use egui::{Image, Ui, Widget};
use crate::ServerManager;

/**
 * $p: path
 * $n: name
 * $s: is single variant track
 */
// macro_rules! display_image {
//     ($p:ident, $n:ident, $s:ident) => {
//         if $s {
//             let track_path = $p.clone() + $n
//         }
//     };
// }

impl ServerManager {
    pub(crate) fn display_tracks(&mut self, ui: &mut egui::Ui) {
        if !self.discovered_tracks {
            self.track_list = self.discover_tracks();
            for track in &self.track_list {
                println!("{:?}", track);
            }
            self.generate_track_textures(ui);
            self.discovered_tracks = true;
        }
        self.display_track_images(ui);
        // TODO: Add texture to Vec so dont have to lag out, use names??? multithreading????
    }

    // TODO: Still not perfect, relies on map.png not being present in root folder (see sunrise_circuit). Change to check for map inside subfolders??
    fn discover_tracks(&mut self) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let path = self.assetto_corsa_path.clone().unwrap() + "\\content\\tracks";
        let inner = std::fs::read_dir(path.clone()).unwrap();
        'track_discovery: for outer_track_folder in inner {
            let mut track_arr: Vec<String> = Vec::new();
            let drm = outer_track_folder.unwrap().file_name();
            let curr = drm.to_str().unwrap();
            track_arr.push(curr.parse().unwrap());
            let inner_dir = std::fs::read_dir(format!("{}\\{}", path.clone(), curr)).unwrap();
            for track_file in inner_dir {
                if track_file.unwrap().file_name().to_str().unwrap() == "map.png" {
                    result.push(track_arr);
                    continue 'track_discovery;
                }
            }
            let inner_dir = std::fs::read_dir(format!("{}\\{}", path.clone(), curr)).unwrap();
            'variant_discovery: for track_file in inner_dir {
                let folder_name = track_file.unwrap().file_name();
                if std::fs::metadata(format!("{}\\{}\\{}", path.clone(), curr, folder_name.clone().to_str().unwrap())).unwrap().is_file() {
                    continue 'variant_discovery;
                }
                let in_folder = std::fs::read_dir(format!("{}\\{}\\{}", path.clone(), curr, folder_name.clone().to_str().unwrap())).unwrap();

                for sub_track_file in in_folder {
                    if sub_track_file.unwrap().file_name().to_str().unwrap() == "map.png" {
                        track_arr.push(folder_name.to_str().unwrap().parse().unwrap());
                        break;
                    }
                }
            }
            result.push(track_arr);
        }
        result
    }

    // TODO: If for some reason a track doesnt have a preview it might have a 'outline.png' file instead
    fn generate_track_textures(&mut self, ui: &mut Ui) {
        let path = self.assetto_corsa_path.clone().unwrap() + "\\content\\tracks";
        for track in &self.track_list {
            let mut tracks = Vec::new();
            if track.len() == 1 {
                let image_path = path.clone() + "\\" + track.first().unwrap() + "\\ui\\preview.png";
                println!("{}", image_path);
                let file = std::fs::read(image_path);
                match file {
                    Ok(ref thing) => {
                        let image = image::load_from_memory(thing).unwrap();
                        let size = [image.width() as usize, image.height() as usize];
                        let image_buffer = image.to_rgba8();
                        let pixels = image_buffer.into_vec();
                        let image = ColorImage::from_rgba_unmultiplied(size, &pixels);
                        let texture = ui.ctx().load_texture("track", image, Default::default());
                        tracks.push(texture);
                    }
                    _ => {
                        let image_path = path.clone() + "\\" + track.first().unwrap() + "\\ui\\outline.png";
                        let file = std::fs::read(image_path);
                        match file {
                            Ok(ref thing) => {
                                let image = image::load_from_memory(thing);
                                match image {
                                    Ok(image) => {
                                        let size = [image.width() as usize, image.height() as usize];
                                        let image_buffer = image.to_rgba8();
                                        let pixels = image_buffer.into_vec();
                                        let image = ColorImage::from_rgba_unmultiplied(size, &pixels);
                                        let texture = ui.ctx().load_texture("track", image, Default::default());
                                        tracks.push(texture);
                                    }
                                    _ => {}
                                }
                            }
                            _ => {
                                println!("Could not find image for: {}", track.first().unwrap());
                            }
                        }
                    }
                }
            } else {
                let mut i = 1;
                while i < track.len() { // track.len() { // TODO: Maybe change it so that not all loaded at once for less lag (only problem in debug)
                    let image_path = path.clone() + "\\" + track.first().unwrap() + "\\ui\\" + track.get(i).unwrap() + "\\preview.png";
                    let file = std::fs::read(image_path.clone());
                    println!("{}", image_path);
                    match file {
                        Ok(ref valid_file) => {
                            let image = image::load_from_memory(valid_file);
                            match image {
                                Ok(image) => {
                                    let size = [image.width() as usize, image.height() as usize];
                                    let image_buffer = image.to_rgba8();
                                    let pixels = image_buffer.into_vec();
                                    let image = ColorImage::from_rgba_unmultiplied(size, &pixels);
                                    let texture = ui.ctx().load_texture("track", image, Default::default());
                                    tracks.push(texture);
                                }
                                _ => {}
                            }
                        }
                        _ => {
                            println!("Could not fine image for: {}", track.first().unwrap());
                        }
                    }

                    i += 1;
                }
            }
            self.track_textures.push(tracks);
        }
    }

    fn display_track_images(&mut self, ui: &mut egui::Ui) {
        let mut i = 0;
        let textures = self.track_list.clone();
        for arr in textures {
            let mut j = 0;
            ui.horizontal(|ui| {
                if arr.len() == 1 {
                    let tex = self.track_textures.get(i).unwrap().first();
                    match tex {
                        Some(tex) => {
                            let image = Image::from(tex).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
                            if egui::Button::image(image).ui(ui).clicked() {
                                println!("{}, {}", i, j);
                                self.change_track(i, j);
                            }
                        }
                        _ => {}
                    };
                } else {
                    'inner: for _track in &arr {
                        if j == 0 && arr.len() > 1 {
                            j += 1;
                            continue 'inner;
                        }
                        let tex = self.track_textures.get(i).unwrap().get(j - 1);
                        match tex {
                            Some(tex) => {
                                let image = Image::from_texture(tex).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
                                if egui::Button::image(image).ui(ui).clicked() {
                                    println!("{}, {}", i, j);
                                    self.change_track(i, j);
                                }
                            }
                            None => {}
                        }

                        j += 1;
                    }
                }
            });
            i += 1;
        }
    }


    fn change_track(&mut self, x: usize, y: usize) {
        let selected_track = self.track_list.get(x);
        match selected_track {
            Some(arr) => {
                let track_config = arr.get(y);
                self.config.server.track = self.track_list.get(x).unwrap().first().unwrap().to_string();
                if y == 0 {
                    self.config.server.config_track = String::from("");
                    return;
                }
                match track_config {
                    Some(track_config) => {
                        self.config.server.config_track = track_config.to_string();
                    }
                    None => {
                        self.config.server.config_track = String::from("");
                    }
                }
            }
            None => {}
        }
    }
}