use eframe::egui;
use gilrs::{Axis, Button, Event, EventType, Gilrs};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "Gamepad Tester",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(GamepadTesterApp::new()))),
    )
}

struct GamepadTesterApp {
    gilrs: Gilrs,
    last_event: String,
    pressed_buttons: Vec<String>,
    axis_values: Vec<String>,
}

impl GamepadTesterApp {
    fn new() -> Self {
        let gilrs = Gilrs::new().unwrap();
        Self {
            gilrs,
            last_event: "No event yet".to_string(),
            pressed_buttons: vec![],
            axis_values: vec![],
        }
    }

    fn handle_gamepad_input(&mut self) {
        self.pressed_buttons.clear();
        self.axis_values.clear();

        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, ..) => {
                    self.last_event = format!("Button Pressed: {:?}", button);
                    self.pressed_buttons.push(format!("{:?}", button));
                }
                EventType::ButtonReleased(button, ..) => {
                    self.last_event = format!("Button Released: {:?}", button);
                }
                EventType::AxisChanged(axis, value, ..) => {
                    self.last_event = format!("Axis Changed: {:?}, Value: {:.2}", axis, value);
                    self.axis_values.push(format!("{:?}: {:.2}", axis, value));
                }
                _ => {}
            }
        }
    }
}

impl eframe::App for GamepadTesterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle gamepad input
        self.handle_gamepad_input();

        // Force a repaint every frame
        ctx.request_repaint();

        // Build the UI
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Gamepad Tester");
            ui.label(format!("Last Event: {}", self.last_event));

            ui.separator();
            ui.label("Pressed Buttons:");
            for button in &self.pressed_buttons {
                ui.label(format!("- {}", button));
            }

            ui.separator();
            ui.label("Axis Values:");
            for axis in &self.axis_values {
                ui.label(format!("- {}", axis));
            }
        });
    }
}
