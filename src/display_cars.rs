use crate::ServerManager;

impl ServerManager {
    pub(crate) fn display_cars(&mut self, ui: &mut egui::Ui) {
        if !self.discovered_cars {
            self.car_list = self.discover_cars();
            self.generate_car_textures(ui);
            self.discovered_cars = true;
        }
    }

    fn discover_cars(&self) -> Vec<String> {
        let result: Vec<String> = Vec::new();
        return result;
    }

    fn generate_car_textures(&self, ui: &mut egui::Ui) {

    }

    fn generate_skin_textures(&self, ui: &mut egui::Ui, index: usize) {

    }

    fn display_car_images(&self, ui: &mut egui::Ui) {

    }
}