#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


// PreDefs //
use eframe::egui;
use egui::{Ui, Color32};

// Vars //
const APP_NAME: &str = "shiro.lua";

// ESP Box Types
const BOX_TYPES: [&str; 4] = ["None", "2D Box", "3D Box", "Corners"];
const TRACER_TYPES: [&str; 4] = ["Top", "Middle", "Bottom Middle", "Bottom"];
const WALLCHECK_TYPES: [&str; 3] = ["Only Show Visible", "Highlight Visible", "Highlight Invisible"];


// Main //
fn main() {
    let mut options = eframe::NativeOptions::default();
    options.always_on_top = true;
    options.resizable = false;
    options.initial_window_size = Some(egui::vec2(550.0, 500.0));

    eframe::run_native(
        APP_NAME,
        options,
        Box::new(|_cc| Box::new(Shiro::default())),
    );
}


// Funcs //

// Draw Toggle
fn ui_toggle(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());
    if response.clicked() {
        *on = !*on;
        response.mark_changed();
    }
    response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

    if ui.is_rect_visible(rect) {
        let how_on = ui.ctx().animate_bool(response.id, *on);
        let visuals = ui.style().interact_selectable(&response, *on);
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    response
}

pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| ui_toggle(ui, on)
}


// Drawing UI //
struct Shiro {
    // Health
    health: i32,
    healthset: i32,

    // Tabs
    selectedtab: u8,

    // Aimbot
    aimbot_enabled: bool,
    aimbot_teamcheck: bool,
    aimbot_stickyaim: bool,
    aimbot_wallcheck: bool,
    aimbot_type: usize,
    aimbot_aimpart: usize,

    aimbot_range_enabled: bool,
    aimbot_range_value: u32,

    aimbot_fov_enabled: bool,
    aimbot_fov_value: u16,
    aimbot_fov_xoffset: f32,
    aimbot_fov_yoffset: f32,
    aimbot_fov_color: Color32,

    aimbot_smoothness: u8,
    aimbot_prediction: u8,

    aimbot_triggerbot: bool,
    aimbot_triggerbot_delay: u16,

    // ESP
    esp_enabled: bool,
    esp_color: [f32; 3],
    esp_type: usize,

    esp_names: bool,
    esp_distance: bool,
    
    // Teamcheck
    esp_teamcheck: bool,
    esp_team_name: bool,
    esp_team_color: bool,
    esp_hide_team: bool,

    // Tracers
    esp_tracers_enabled: bool,
    esp_tracers_type: usize, 
    esp_tracers_color: [f32; 3],
    esp_tracers_distance_based: bool,

    // Health
    esp_show_health: bool,
    esp_health_bar: bool,
    esp_health_text: bool,

    // Distance Limited
    esp_distance_limited: bool,
    esp_distance_limit: u32,

    // Wallcheck
    esp_wallcheck_enabled: bool,
    esp_wallcheck_type: usize,
    esp_wallcheck_color: [f32; 3],

    // Lua Code Editor
    code: String,
}

impl Default for Shiro {
    fn default() -> Self {
        Self {
            // Health
            health: 100,
            healthset: 100,

            // Tabs
            selectedtab: 1,

            // Aimbot
            aimbot_enabled: false,
            aimbot_teamcheck: false,
            aimbot_stickyaim: false,
            aimbot_wallcheck: false,
            aimbot_type: 0,
            aimbot_aimpart: 0,

            aimbot_range_enabled: false,
            aimbot_range_value: 1000,

            aimbot_fov_enabled: false,
            aimbot_fov_value: 100,
            aimbot_fov_xoffset: 0.0,
            aimbot_fov_yoffset: 0.0,
            aimbot_fov_color: Color32::from_rgba_premultiplied(200, 200, 200, 200),

            aimbot_smoothness: 0,
            aimbot_prediction: 0,

            aimbot_triggerbot: false,
            aimbot_triggerbot_delay: 0,

            // ESP
            esp_enabled: false,
            esp_color: [255.0, 255.0, 255.0],
            esp_type: 1,

            esp_names: false,
            esp_distance: false,

            // Teamcheck
            esp_teamcheck: false,
            esp_team_name: false,
            esp_team_color: false,
            esp_hide_team: false,

            // Tracers
            esp_tracers_enabled: false,
            esp_tracers_type: 0, 
            esp_tracers_color: [255.0, 255.0, 255.0],
            esp_tracers_distance_based: false,

            // Health
            esp_show_health: false,
            esp_health_bar: false,
            esp_health_text: false,

            // Distance Limited
            esp_distance_limited: false,
            esp_distance_limit: 1000,
            
            // Wallcheck
            esp_wallcheck_enabled: false,
            esp_wallcheck_type: 0,
            esp_wallcheck_color: [255.0, 255.0, 100.0],

            // Lua Code Editor
            code: "luh".into(),
        }
    }
}

// Implementing Funcs //
impl Shiro {

    // Aimbot Panel
    fn draw_aimbot_panel(&mut self, ui: &mut Ui) {

        // Enabled
        ui.horizontal(|ui| {
            ui.add(toggle(&mut self.aimbot_enabled));
            ui.label(egui::RichText::new("Aimbot Enabled").strong());
        });

        // Aimbot Settings
        if self.aimbot_enabled {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.aimbot_teamcheck, "Teamcheck");
                ui.checkbox(&mut self.aimbot_wallcheck, "Wallcheck");
                ui.checkbox(&mut self.aimbot_stickyaim, "Sticky Aim");
            });
            ui.add_space(5.0);
        }

        ui.add_enabled_ui(self.aimbot_enabled, |ui| { // If Aimbot On

            // Range
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.add(toggle(&mut self.aimbot_range_enabled));
                ui.label(egui::RichText::new("Range Limit").strong());

                ui.add_enabled_ui(self.aimbot_range_enabled, |ui| { // If Range On
                    ui.add(egui::Slider::new(&mut self.aimbot_range_value, 0..=5000).clamp_to_range(false));
                });
            });

            // FOV
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.add(toggle(&mut self.aimbot_fov_enabled));
                ui.label(egui::RichText::new("FOV").strong());

                ui.add_enabled_ui(self.aimbot_fov_enabled, |ui| { // If Range On
                    ui.add(egui::Slider::new(&mut self.aimbot_fov_value, 0..=1000).clamp_to_range(false));
                    
                    ui.label(egui::RichText::new("X Offset:"));
                    ui.add(egui::DragValue::new(&mut self.aimbot_fov_xoffset).speed(0.01));

                    ui.label(egui::RichText::new("Y Offset:"));
                    ui.add(egui::DragValue::new(&mut self.aimbot_fov_yoffset).speed(0.01));

                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        ui.color_edit_button_srgba(&mut self.aimbot_fov_color);
                    });
                });
            });
        
            // Triggerbot
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.add(toggle(&mut self.aimbot_triggerbot));
                ui.label(egui::RichText::new("Triggerbot").strong());

                ui.add_enabled_ui(self.aimbot_triggerbot, |ui| { // If Range On
                    ui.add(egui::Slider::new(&mut self.aimbot_triggerbot_delay, 0..=1000).clamp_to_range(false).text("Shoot Delay"));
                });
            });

            // Aimbot Extras
            ui.add(egui::Separator::spacing(egui::Separator::horizontal(egui::Separator::default()), 10.0));

            ui.add(egui::Slider::new(&mut self.aimbot_smoothness, 0..=100).text("Aimbot Smoothness"));

            ui.add_space(3.0);

            ui.add(egui::Slider::new(&mut self.aimbot_prediction, 0..=100).text("Prediction Strength"));

            ui.add_space(5.0);

            egui::ComboBox::from_label("Target Priority")
            .selected_text(format!("{:?}", ["Distance", "Cursor"][self.aimbot_type]))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.aimbot_type, 0, "Distance");
                ui.selectable_value(&mut self.aimbot_type, 1, "Cursor");
            });

            ui.add_space(3.0);

            egui::ComboBox::from_label("Aimpart")
            .selected_text(format!("{:?}", ["Head", "Torso"][self.aimbot_aimpart]))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.aimbot_aimpart, 0, "Head");
                ui.selectable_value(&mut self.aimbot_aimpart, 1, "Torso");
            });

        });

        ui.add(egui::Separator::spacing(egui::Separator::horizontal(egui::Separator::default()), 10.0));
    }

    // ESP Panel
    fn draw_esp_panel(&mut self, ui: &mut Ui) {
        
        // ESP Enabled
        ui.horizontal(|ui| {
            ui.add(toggle(&mut self.esp_enabled));
            ui.label(egui::RichText::new("ESP Enabled").strong());

            ui.add_enabled_ui(self.esp_enabled, |ui| { // If ESP On
                ui.with_layout(egui::Layout::right_to_left(), |ui| {

                    egui::ComboBox::from_id_source("esp_type")
                    .selected_text(format!("{:?}", (BOX_TYPES)[self.esp_type]))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.esp_type, 0, "None");
                        ui.selectable_value(&mut self.esp_type, 1, "2D Box");
                        ui.selectable_value(&mut self.esp_type, 2, "3D Box");
                        ui.selectable_value(&mut self.esp_type, 3, "Corners");
                    });

                    ui.add_enabled_ui(!self.esp_team_color, |ui| { // If "Team Based ESP Color" Off
                        ui.color_edit_button_rgb(&mut self.esp_color);
                    });
                });
            });
        });

        // If ESP Enabled
        ui.add_enabled_ui(self.esp_enabled, |ui| { // If ESP On

            // ESP Types
            if self.esp_enabled {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.esp_names, "Show Names");
                    ui.checkbox(&mut self.esp_distance, "Show Distance");
                });
                ui.add_space(5.0);
            }
            
            {
                // Teamcheck
                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.add(toggle(&mut self.esp_teamcheck));
                    ui.label(egui::RichText::new("Teamcheck").strong());

                    
                });

                // If enabled
                ui.add_enabled_ui(self.esp_teamcheck, |ui| { // If Teamcheck On


                    // ESP Types
                    if self.esp_teamcheck {
                        ui.horizontal(|ui| {
                        
                            ui.checkbox(&mut self.esp_hide_team, "Hide Team Members");
                            ui.checkbox(&mut self.esp_team_name, "Show Team Names");
                            ui.checkbox(&mut self.esp_team_color, "Team Based ESP Color");
                            
                        });
                        ui.add_space(5.0);
                    }
                });
            }

            {
                // Wallcheck
                ui.horizontal(|ui| {
                    ui.add(toggle(&mut self.esp_wallcheck_enabled));
                    ui.label(egui::RichText::new("Wallcheck").strong());

                    ui.add_enabled_ui(self.esp_wallcheck_enabled, |ui| { // If ESP On
                        ui.with_layout(egui::Layout::right_to_left(), |ui| {

                            egui::ComboBox::from_id_source("esp_type")
                            .selected_text(format!("{:?}", (WALLCHECK_TYPES)[self.esp_wallcheck_type]))
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.esp_wallcheck_type, 0, "Only Show Visible");
                                ui.selectable_value(&mut self.esp_wallcheck_type, 1, "Highlight Visible");
                                ui.selectable_value(&mut self.esp_wallcheck_type, 2, "Highlight Invisible");
                            });

                            ui.add_enabled_ui(self.esp_wallcheck_type == 1 || self.esp_wallcheck_type == 2, |ui| { // If "Team Based ESP Color" Off
                                ui.color_edit_button_rgb(&mut self.esp_wallcheck_color);
                            });
                        });
                    });
                });
            }
            
            // Tracers
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.add(toggle(&mut self.esp_tracers_enabled));
                ui.label(egui::RichText::new("Tracers").strong());

                // If enabled
                ui.add_enabled_ui(self.esp_tracers_enabled, |ui| { // If Tracers On
                    ui.add_space(10.0);
                    ui.checkbox(&mut self.esp_tracers_distance_based, "Distance Based Color");
                    
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {

                        egui::ComboBox::from_id_source("tracer_type")
                        .selected_text(format!("{:?}", (TRACER_TYPES)[self.esp_tracers_type]))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.esp_tracers_type, 0, "Top");
                            ui.selectable_value(&mut self.esp_tracers_type, 1, "Middle");
                            ui.selectable_value(&mut self.esp_tracers_type, 2, "Bottom Middle");
                            ui.selectable_value(&mut self.esp_tracers_type, 3, "Bottom");
                        });

                        ui.add_enabled_ui(!self.esp_tracers_distance_based, |ui| { // If "Distance Based Tracers" On
                            ui.color_edit_button_rgb(&mut self.esp_tracers_color);
                        });

                    });
                });
            });


            // Health
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.add(toggle(&mut self.esp_show_health));
                ui.label(egui::RichText::new("Show Player Health").strong());

                // If enabled
                ui.add_enabled_ui(self.esp_show_health, |ui| { // If Show Health On
                    ui.add_space(10.0);

                    ui.checkbox(&mut self.esp_health_bar, "Health Bar");
                    ui.checkbox(&mut self.esp_health_text, "Health Text");
                });
            });


            // Distance Limit
            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.add(toggle(&mut self.esp_distance_limited));
                ui.label(egui::RichText::new("Distance Limited").strong());

                // If enabled
                ui.add_enabled_ui(self.esp_distance_limited, |ui| { // If Show Health On
                    ui.add_space(10.0);

                    ui.add(egui::Slider::new(&mut self.esp_distance_limit, 0..=5000).clamp_to_range(false));
                });
            });



        });

        ui.add(egui::Separator::spacing(egui::Separator::horizontal(egui::Separator::default()), 10.0));
    }

    // Misc Panel
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

    // Lua Panel
    fn draw_lua_panel(&mut self, ui: &mut Ui) {

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut self.code)
                    .font(egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(10)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    //.layouter(&mut layouter),
            );
        });
    }
}

// Update Loop
impl eframe::App for Shiro {

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {

            // Tab Array
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.selectedtab, 1, "Aimbot");
                ui.selectable_value(&mut self.selectedtab, 2, "ESP");
                ui.selectable_value(&mut self.selectedtab, 3, "Misc");
                ui.selectable_value(&mut self.selectedtab, 4, "Lua");
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
                4 => {
                    self.draw_lua_panel(ui);
                },


                // Invalid Tab Handling
                _ => panic!("Invalid Tab Selected: {:?}", self.selectedtab),
            }

        });
    }
}
