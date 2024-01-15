use eframe::emath::Vec2;
use eframe::epaint::ColorImage;
use egui::{Image, Widget};
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
        let path = self.assetto_corsa_path.clone().unwrap() + "\\content\\tracks";
        let inner = std::fs::read_dir(path.clone()).unwrap();
        let mut result: Vec<Vec<String>> = Vec::new();
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
        return result;
    }

    // TODO: If for some reason a track doesnt have a preview it might have a 'outline.png' file instead
    fn generate_track_textures(&mut self, ui: &mut egui::Ui) {
        let path = self.assetto_corsa_path.clone().unwrap() + "\\content\\tracks";
        for track in &self.track_list {
            let mut tracks = Vec::new();
            if track.len() == 1 {
                let image_path = path.clone() + "\\" + &*track.get(0).unwrap() + "\\ui\\preview.png";
                println!("{}", image_path);
                let file = std::fs::read(image_path);
                match file {
                    Ok(ref thing) => {
                        let image = image::load_from_memory(&*file.unwrap()).unwrap();
                        let size = [image.width() as usize, image.height() as usize];
                        let image_buffer = image.to_rgba8();
                        let pixels = image_buffer.into_vec();
                        let image = ColorImage::from_rgba_unmultiplied(size, &pixels);
                        let texture = ui.ctx().load_texture("track", image, Default::default());
                        tracks.push(texture);
                    }
                    _ => {
                        let image_path = path.clone() + "\\" + track.get(0).unwrap() + "\\ui\\outline.png";
                        let file = std::fs::read(image_path);
                        match file {
                            Ok(ref thing) => {
                                let image = image::load_from_memory(&*file.unwrap()).unwrap();
                                let size = [image.width() as usize, image.height() as usize];
                                let image_buffer = image.to_rgba8();
                                let pixels = image_buffer.into_vec();
                                let image = ColorImage::from_rgba_unmultiplied(size, &pixels);
                                let texture = ui.ctx().load_texture("track", image, Default::default());
                                tracks.push(texture);
                            }
                            _ => {
                                println!("Could not find image for: {}", track.get(0).unwrap());
                            }
                        }
                    }
                }
            } else {
                let mut i = 1;
                while i < track.len() { // track.len() { // TODO: Change it so that not all loaded at once for less lag
                    let image_path = path.clone() + "\\" + track.get(0).unwrap() + "\\ui\\" + track.get(i).unwrap() + "\\preview.png";
                    let file = std::fs::read(image_path);
                    match file {
                        Ok(ref thing) => {
                            let image = image::load_from_memory(&*file.unwrap()).unwrap();
                            let size = [image.width() as usize, image.height() as usize];
                            let image_buffer = image.to_rgba8();
                            let pixels = image_buffer.into_vec();
                            let image = ColorImage::from_rgba_unmultiplied(size, &pixels);
                            let texture = ui.ctx().load_texture("track", image, Default::default());
                            tracks.push(texture);
                        }
                        _ => {
                            println!("Could not fine image for: {}", track.get(0).unwrap());
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
        for arr in &self.track_textures {
            let mut j = 0;
            ui.horizontal(|ui| {
                for tex in arr {
                    let image = Image::from_texture(&*tex).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });

                    // let button = egui::ImageButton::new(image);
                    // let sense = egui::Sense::click();
                    // let button = button.sense(sense);
                    if egui::Button::image(image).ui(ui).clicked() {
                        println!("{}, {}", i, j);
                    }
                    j += 1;
                }
            });
            i += 1;
        }
    }
}