use egui::{CentralPanel, Label, Sense};

use self::todos::Daylies;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BetterNotes {
    daylies: Daylies,
}

impl Default for BetterNotes {
    fn default() -> Self {
        Self {
            daylies: Daylies::new(),
        }
    }
}

impl BetterNotes {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        if let Some(storage) = cc.storage {
            //return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

mod todos {
    use std::collections::HashMap;

    use chrono::{DateTime, Datelike, Local};
    use egui::{Color32, Frame, Sense};

    #[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct Daylies {
        daylies: HashMap<String, bool>,
        new_item_text: String,
        new_item_field: bool,
        curr_day: DateTime<Local>,
    }

    impl Daylies {
        pub fn new() -> Self {
            Self {
                daylies: [
                    (String::from("Commit"), false),
                    (String::from("Anki"), false),
                    (String::from("Schwidisch"), false),
                    (String::from("Musik"), false),
                    (String::from("Tanzen"), false),
                    (String::from("Lesen"), false),
                    (String::from("Workout"), false),
                ]
                .into(),
                new_item_text: "".to_string(),
                curr_day: Local::now(),
                new_item_field: false,
            }
        }

        pub fn show(&mut self, ui: &mut egui::Ui) {
            // reset dailies when a new day has begun
            // this should not be here
            if self.curr_day.num_days_from_ce() != Local::now().num_days_from_ce() {
                self.daylies.values_mut().for_each(|value| *value = false);
                self.curr_day = Local::now();
            }

            // The frame is used to recognise outside clicks of the todo-item¿
            let resp = Frame::default().show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.vertical(|ui| {
                    for (text, checked) in &mut self.daylies {
                        ui.checkbox(checked, text);
                    }
                });
            });

            // logic to toggle the text-input for a new item
            let mut newly_active = false;
            if resp.response.interact(Sense::click()).clicked() && !self.new_item_field {
                self.new_item_field = true;
                newly_active = true;
            }

            if self.new_item_field {
                let resp = ui.text_edit_singleline(&mut self.new_item_text);

                // request focus on initial creation
                if newly_active {
                    resp.request_focus();
                }

                // delete field as soon as focus is lost
                if resp.lost_focus() {

                    // if the reason for focus-lost was an pressing enter, then
                    // add/remove the new/old item
                    if ui.input().key_pressed(egui::Key::Enter) {
                        if self.daylies.contains_key(&self.new_item_text) {
                            self.daylies.remove(&self.new_item_text);
                        } else {
                            self.daylies.insert(self.new_item_text.clone(), false);
                        }
                    }
                    // reset form data
                    self.new_item_field = false;
                    self.new_item_text = "".into();
                }
            }
        }
    }
}

impl eframe::App for BetterNotes {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Daily Todos").show(ctx, |ui| {
            self.daylies.show(ui);
        });
    }
}
