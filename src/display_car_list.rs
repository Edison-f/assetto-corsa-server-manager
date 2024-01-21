use std::string::String;

use eframe::emath::Vec2;
use egui::{Image, Widget};

use crate::ServerManager;

impl ServerManager {
    pub(crate) fn display_car_list(&mut self, ui: &mut egui::Ui) {
        if self.car_list_changed {
            self.update_config_car_list();
            self.car_list_changed = false;
        }
        egui::ScrollArea::both().id_source("car_list_scroll").show(ui, |ui| {
            // let indices = self.car_indices.clone();
            for (car_name, arr) in &self.car_list {
                ui.horizontal(|ui| {
                    let first_skin_name = &arr.get(0).unwrap().0;
                    let image = Image::from_texture(self.car_textures.get(car_name).unwrap().get(first_skin_name).unwrap()).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
                    if egui::Button::image(image).ui(ui).clicked() {
                        println!("those!")
                    }
                    ui.label(car_name.clone());
                    for (skin_name, count) in arr {
                        for _ in 0..*count {
                            let image = Image::from_texture(self.car_textures.get(car_name).unwrap().get(skin_name).unwrap()).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
                            if egui::Button::image(image).ui(ui).clicked() {
                                println!("{}, {}", car_name, skin_name)
                            }
                        }
                    }
                });
            }
            // for index in indices {
            //     let arr = self.car_textures.get(index);
            //     if let Some(arr) = arr {
            //         let tex = arr.first();
            //         if let Some(tex) = tex {
            //             let image = Image::from_texture(tex).fit_to_exact_size(Vec2 { x: 120.0, y: 120.0 });
            //             ui.horizontal(|ui| {
            //                 if egui::Button::image(image).ui(ui).clicked() {
            //                     println!("{}", index);
            //                     self.remove_car(&index);
            //                 }
            //                 ui.label(self.available_car_list.get(index).unwrap());
            //             });
            //         }
            //     }
            // }
        });
    }

    fn update_config_car_list(&mut self) {
        let mut builder = String::from("");
        let mut i = 0;
        for map in &self.car_list {
            builder += &map.1.iter().nth(0).unwrap().0;
            if i != self.car_list.len() - 1 {
                builder += ";";
            }

            i += 1;
        }
        self.config.server.cars = builder;
    }

    fn remove_car(&mut self, name: String, skin: String) {
        // let car_name = self.available_car_list.get(*index);
        // if let Some(car_name) = car_name {
        for i in 0..self.car_list.len() {
            let entry = &self.car_list.clone();
            let entry = entry.get(i).unwrap();
            let car_name = &entry.0;
            let mut arr = &entry.1;
            if &name != car_name {
                continue;
            }
            for j in 0..arr.len() {
                let mut pair = &arr.get(j).unwrap();
                if pair.0 == skin { // If name match
                    self.car_list.get_mut(i).unwrap().1.get_mut(j).unwrap().1 -= 1; // Subtract count
                    if pair.1 == 0 { // If no more
                        self.car_list.get_mut(i).unwrap().1.remove(j); // Remove skin entry
                        if arr.len() == 0 { // If no skin entries
                            self.car_list.remove(i); // Remove
                        }
                    }
                    return;
                }
            }
        }
        // }
        // for i in 0..self.car_indices.len() {
        //     if self.car_indices.get(i).unwrap() == index {
        //         self.car_indices.remove(i);
        //         let car_name = self.available_car_list.get(*index);
        //         match car_name {
        //             Some(car_name) => {
        //                 let regex = Regex::new(car_name).unwrap();
        //                 for j in 0..self.car_list.len() {
        //                     if let Some(res) = regex.find(self.car_list.get(j).unwrap()) {
        //                         if res.start() == 0 && res.end() == self.car_list.get(j[0]).unwrap().len() {
        //                             self.car_list.remove(j[0]);
        //                             break;
        //                         }
        //                     }
        //                 }
        //             }
        //
        //             None => {println!("remove car: index out of bounds")}
        //         }
        //         break;
        //     }
        // }
        self.update_config_car_list();
    }

    //Create helper methods for this
    /*
     * If entry for car name already exists
     *      If skin exists
     *          Increment
     *      Else
     *          Create and init at 1
     * Else
     *      Create entry for car name and skin, init at 1
     */
    pub(crate) fn add_car(&mut self, i: usize, j: usize) {
        self.car_indices.push(i);
        let car_name = String::from(self.available_car_list.get(i).unwrap());
        let binding = self.available_skins_list.clone();
        let mut skin_name = binding.get(&car_name).unwrap().get(j).unwrap();
        let find = self.name_exists(car_name.clone());
        if find.0 {
            let mut curr = &mut self.car_list.clone();
                let mut curr = &mut curr.get_mut(find.1).unwrap().1;
            let find = self.skin_exists(String::from(skin_name), &curr);
            if find.0 {
                curr.get_mut(find.1).unwrap().1 += 1;
            } else {
                curr.push((String::from(skin_name), 1))
            }
        } else {
            let arr = vec![(String::from(skin_name), 1)];
            self.car_list.push((String::from(car_name), arr))
        }
        // self.car_list.insert(car_name.parse().unwrap(), self.car_list.get(car_name).unwrap_or());
        // self.update_config_car_list();
    }

    fn name_exists(&mut self, car: String) -> (bool, usize) {
        let mut i = 0;
        for (s, _) in &self.car_list {
            if s == &car {
                return (true, i);
            }
            i += 1;
        }
        (false, 0)
    }

    // Could probably consolidate both with generics
    fn skin_exists(&mut self, skin: String, arr: &Vec<(String, u8)>) -> (bool, usize) {
        let mut i = 0;
        for (s, _) in arr {
            if s == &skin {
                return (true, i);
            }
            i += 1;
        }
        (false, 0)
    }

    // pub(crate) fn update_from_config(&mut self) {
    //     let regex = Regex::new(";").unwrap();
    //     let split = regex.split(&self.config.server.cars);
    //     self.car_indices = vec![];
    //     for name in split {
    //         let finder = Regex::new(name).unwrap();
    //         'inner: for (i, car) in self.available_car_list.iter().enumerate() {
    //             let found = finder.find(car);
    //             if found.is_some() {
    //                 self.car_indices.push(i);
    //                 self.car_list.insert(String::from(car), 0);
    //                 break 'inner;
    //             }
    //         }
    //     }
    // }
    //
    // pub(crate) fn update_from_entry_list(&mut self) {
    //     for car in &self.entry_list.list {
    //         let mut index = 0;
    //         for c in self.available_skins_list.get(&car.model).unwrap() {
    //             if c == &car.skin {
    //                 break;
    //             }
    //             index += 1;
    //         }
    //         self.car_list.insert(car.model.clone(), index);
    //     }
    // }
}