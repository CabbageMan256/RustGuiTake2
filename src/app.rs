/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    tab: String,
    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            tab: String::from("Start"),
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    //let mut tab: i8 = 0;
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }
                ui.menu_button("Calculator Mode", |ui| {
                    if ui.button("Arithmetic").clicked() {
                        self.tab = String::from("Arithmetic");
                    }
                });
                ui.menu_button("Info", |ui| {
                    if ui.button("History").clicked() {
                        self.tab = String::from("History");
                    }
                    if ui.button("About the Author").clicked() {
                       self.tab = String::from("Author");
                    }

                });

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            if (self.tab == "History") {
                ui.heading("History");
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(egui::Label::new("blah\
                    blah blah blahbl ahblah blahbl ahblahbl ahblahbla hblahbla hblahblahbl ahblahblahb lahbl ahblahbl ahblah blahblahbl ahblahblah
                    blah
                    blah
                    blah
                    blah
                    blah
                    blah
                    blah
                    blah
                    blah
                    blah
                    blah
                    blah
                    blah
                    blah
                    ").wrap());
                });
            } else if (self.tab == "Author") {
                ui.heading("About the Author");
            } else if (self.tab == "Arithmetic") {
                ui.heading("Arithmetic");
            } else {
                ui.heading("eframe template");

                ui.horizontal(|ui| {
                    ui.label("Write something: ");
                    ui.text_edit_singleline(&mut self.label);
                });

                ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
                if ui.button("Increment").clicked() {
                    self.value += 1.0;
                }

                ui.separator();

                ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/main/",
                "Source code."
            ));
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add(egui::Label::new("Powered by alksdj flaksdjf laskdjf laskd fjlasd kflaskdj flaskjdf laskjd flaskj dflajs dflj sdl ajsdflj asdf").wrap());
        //ui.label("Powered by alksdj flaksdjf laskdjf laskd fjlasd kflaskdj flaskjdf laskjd flaskj dflajs dflj sdl ajsdflj asdf");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
