use eframe::egui;

const MIN_WINDOW_SIZE: [f32; 2] = [350.0, 150.0];

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_min_inner_size(MIN_WINDOW_SIZE).with_inner_size(MIN_WINDOW_SIZE),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Audio Player Application",
        options,
        Box::new(|_cc| {
            Ok(Box::<Application>::default())
        }),
    )
}

#[derive(PartialEq, Eq)]
enum State {
    Playing,
    Paused,
}

struct Application {
    state: State,
    current_file_name: String,
    position: u64,
    length: u64,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            state: State::Paused,
            current_file_name: "No Name".to_owned(),
            position: 0,
            length: 600,
        }
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Currently playing '{}'!", self.current_file_name));
            
            if self.state == State::Playing {
                self.position += 1;
                ctx.request_repaint(); //Makes the frame repaint so that the progress bar is updated correctly
            }

            ui.add(egui::ProgressBar::new(self.position as f32 / self.length as f32));
            
            ui.columns(2, |cols| {
                cols[0].vertical_centered_justified(|ui| {
                    let button_text = match self.state {
                        State::Playing => "Pause",
                        State::Paused => "Play",
                    };
    
                    if ui.button(button_text).clicked() {
                        self.state = match self.state {
                            State::Playing => State::Paused,
                            State::Paused => State::Playing,
                        };
                    }
                });

                cols[1].vertical_centered_justified(|ui| {
                    if ui.button("Restart").clicked() {
                        self.position = 0;
                    }
                });
            });
        });
    }
}
