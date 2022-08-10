#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


// PreDefs
use eframe::egui;
use egui::Ui;


// Vars
const APP_NAME: &str = "shiro.lua";


fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Box::new(Shiro::default())),
    );
}


// Drawing UI
struct Shiro {
    selectedtab: i32
}

impl Default for Shiro {
    fn default() -> Self {
        Self {
            selectedtab: 1,
        }
    }
}

// Implementing Funcs
impl Shiro {

    // Aimbot
    fn draw_aimbot_panel(&mut self, ui: &mut Ui) {

        ui.heading("Aimbot");

        //

        ui.add(egui::Separator::spacing(egui::Separator::horizontal(egui::Separator::default()), 10.0));
    }

    // ESP
    fn draw_esp_panel(&mut self, ui: &mut Ui) {

        ui.heading("ESP");
        
        //

        ui.add(egui::Separator::spacing(egui::Separator::horizontal(egui::Separator::default()), 10.0));
    }

    // Misc
    fn draw_misc_panel(&mut self, ui: &mut Ui) {

        ui.heading("Misc");

        // Health Edit
        ui.horizontal(|ui| {
            ui.label("Your current health: ");
            ui.text_edit_singleline(&mut self.health.to_string())
        });

        ui.horizontal(|ui| {

            if ui.button("Set").clicked() {
                // do the stuff 
            }
            
            ui.add(egui::Slider::new(&mut self.healthset, 0..=10000).text("Health"));   
        });

        ui.add(egui::Separator::spacing(egui::Separator::horizontal(egui::Separator::default()), 10.0));
    }
}


impl eframe::App for Shiro {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        
        egui::CentralPanel::default().show(ctx, |ui| {

            // Tab Array
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selectedtab,1,"Aimbot");
                ui.selectable_value(&mut self.selectedtab,2,"ESP");
                ui.selectable_value(&mut self.selectedtab,3,"Misc");
            });
            ui.add(egui::Separator::spacing(egui::Separator::horizontal(egui::Separator::default()), 10.0));

            // Panels
            match self.selectedtab {
                1 => {
                    self.draw_aimbot_panel(ui);
                },
                2 => {
                    self.draw_esp_panel(ui);
                },
                3 => {
                    self.draw_misc_panel(ui);
                },


                // Invalid Tab Handling
                _ => panic!("Invalid Tab Selected: {:?}", self.selectedtab),
            }

        });
    }
}
