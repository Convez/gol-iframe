use crate::universe;
use universe::Universe;
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct UniverseApp {
    // Example stuff:
    universe: Universe,

    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f32,
}

impl Default for UniverseApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            universe: Universe::new(),
            value: 2.7,
        }
    }
}

impl UniverseApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for UniverseApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let subpainter = ui.painter_at(egui::Rect::EVERYTHING.translate(egui::vec2(50.0, 0.0)));
            self.universe.draw_grid(&subpainter);
            self.universe.draw_cells(&subpainter);
            });
            self.universe.tick();
            ctx.request_repaint();
    }
}
