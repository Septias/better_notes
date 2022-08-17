use self::todos::Daylies;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct BetterNotes {}

impl Default for BetterNotes {
    fn default() -> Self {
        Self {}
    }
}

impl BetterNotes {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        /* if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        } */

        Default::default()
    }
}

mod todos {
    use std::collections::HashMap;

    #[derive(Debug, Default)]
    pub struct Daylies {
        daylies: HashMap<String, bool>,
    }

    impl Daylies {
        pub fn new() -> Self {
            Self {
                daylies: [(String::from("hi"), true)].into(),
            }
        }

        pub fn show(&mut self, ui: &mut egui::Ui) {
            for (text, checked) in &mut self.daylies {
                if ui.checkbox(checked, text).changed() {
                    *checked = false;
                    println!("hi, {}", checked);
                }
            }
        }
    }
}

impl eframe::App for BetterNotes {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        //eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Daily Todos").show(ctx, |ui| {
            ui.label("Dailies");
            Daylies::new().show(ui);
        });
    }
}
