use geng::prelude::*;
use geng_egui::*;

struct State {
    egui: EguiGeng,
    name: String,
    age: u32,
}

impl State {
    /// Here we will do ui stuff
    fn ui(&mut self) {
        egui::Window::new("Egui Window").show(self.egui.get_context(), |ui| {
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}

impl geng::State for State {
    fn update(&mut self, _delta_time: f64) {
        // Begin frame
        self.egui.begin_frame();

        // UI logic here
        self.ui();

        // End frame
        self.egui.end_frame();
    }

    fn handle_event(&mut self, event: geng::Event) {
        self.egui.handle_event(event);
    }

    fn draw(&mut self, framebuffer: &mut ugli::Framebuffer) {
        ugli::clear(framebuffer, Some(Rgba::GRAY), None, None);

        // Render GUI
        self.egui.draw(framebuffer);
    }
}

fn main() {
    logger::init();
    geng::setup_panic_handler();

    let mut options = geng::ContextOptions::default();
    options.window.title = "Simple UI Example".to_string();

    Geng::run_with(&options, move |geng| async move {
        let state = State {
            egui: EguiGeng::new(&geng),
            name: "<Name>".to_owned(),
            age: 0,
        };
        geng.run_state(state).await;
    });
}
