use eframe::emath::Vec2;
use eframe::epaint::ColorImage;
use egui::Image;
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
        // ui.menu_image_button();
        let path = self.assetto_corsa_path.clone().unwrap() + "\\content\\tracks";
        // let track_path = path.clone() + "kanazawa\\ui\\layout_circuit\\preview.png";
        // let image = image::load_from_memory(&*std::fs::read(track_path).unwrap()).unwrap();
        // let size = [image.width() as usize, image.height() as usize];
        // let image_buffer = image.to_rgba8();
        // let pixels = image_buffer.into_vec();
        // let image = ColorImage::from_rgba_unmultiplied(size, &pixels);
        // let texture = ui.ctx().load_texture("track", image, Default::default());
        // let image = Image::from_texture(&texture).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
        // let resize = egui::containers::Resize::min_width(Default::default(), 0.0).min_height(0.0);
        // resize.show(ui, |ui| {
        // ui.add(image);
        // });
        if !self.discovered_tracks {
            self.track_list = self.discover_tracks();
            for track in &self.track_list {
                println!("{:?}", track);
            }
            for track in &self.track_list {
                let mut tracks = Vec::new();
                if track.len() == 1 {
                    let image_path = path.clone() + "\\" + track.get(0).unwrap() + "\\map.png";
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
                            // let image = Image::from_texture(&texture).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
                            // ui.add(image);
                        }
                        _ => {}
                    }
                } else {
                    let mut i = 1;
                    while i < 2 { // track.len() { // TODO: Change it so that not all loaded at once for less lag
                        let image_path = path.clone() + "\\" + track.get(0).unwrap() + "\\" + track.get(i).unwrap() + "\\map.png";
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
                                // let image = Image::from_texture(&texture).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
                                // ui.add(image);
                            }
                            _ => {}
                        }

                        i += 1;
                    }
                }
                self.track_textures.push(tracks);
            }
            self.discovered_tracks = true;
        }
        for arr in &self.track_textures {
            for tex in arr {
                let image = Image::from_texture(&*tex).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
                ui.add(image);
            }
        }
        // TODO: Add texture to Vec so dont have to lag out, use names???
    }

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
}