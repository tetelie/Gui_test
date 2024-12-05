use eframe::egui;
use gilrs::{Gilrs, Button, Event, EventType};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "GUI with Gamepad",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(GamepadApp::new()))),
    )
}

struct GamepadApp {
    gilrs: Gilrs,
    selected_item: usize,
    items: Vec<String>,
}

impl GamepadApp {
    fn new() -> Self {
        let gilrs = Gilrs::new().unwrap();
        Self {
            gilrs,
            selected_item: 0,
            items: vec!["Option 1".to_string(), "Option 2".to_string(), "Option 3".to_string()],
        }
    }

    fn handle_gamepad_input(&mut self) {
        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            match event {
                EventType::ButtonPressed(Button::DPadUp, ..) => {
                    if self.selected_item > 0 {
                        self.selected_item -= 1;
                    }
                }
                EventType::ButtonPressed(Button::DPadDown, ..) => {
                    if self.selected_item < self.items.len() - 1 {
                        self.selected_item += 1;
                    }
                }
                EventType::ButtonPressed(Button::South, ..) => {
                    println!("Selected: {}", self.items[self.selected_item]);
                }
                _ => {}
            }
        }
    }
}

impl eframe::App for GamepadApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_gamepad_input();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Use the gamepad to navigate and select:");

            for (i, item) in self.items.iter().enumerate() {
                if i == self.selected_item {
                    ui.label(format!("> {}", item));
                } else {
                    ui.label(item);
                }
            }
        });
    }
}
