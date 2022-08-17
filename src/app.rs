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

    #[derive(Debug, Default, serde::Deserialize, serde::Serialize)]
    pub struct Daylies {
        daylies: HashMap<String, bool>,
    }

    impl Daylies {
        pub fn new() -> Self {
            Self {
                daylies: [
                    (String::from("Commit"), true),
                    (String::from("Anki"), true),
                    (String::from("Schwidisch"), true),
                    (String::from("Musik"), true),
                    (String::from("Tanzen"), true),
                    (String::from("Lesen"), true),
                    (String::from("Workout"), true),
                ]
                .into(),
            }
        }

        pub fn show(&mut self, ui: &mut egui::Ui) {
            for (text, checked) in &mut self.daylies {
                ui.checkbox(checked, text);
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
