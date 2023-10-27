use eframe::egui::{self};
use serde_json::Value;

use std::fs;
use std::path::{Path, PathBuf};

use crate::proguard::{self, ProguardClass};

use reqwest::header::USER_AGENT;

const P: &str = " | ";

pub fn gui_main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 500.0)),
        ..Default::default()
    };
    eframe::run_native(
        "MCP Mapping Viewer",
        options,
        Box::new(|_| Box::<MyApp>::default()),
    )
}

struct MyApp {
    version_entry: Vec<PathBuf>,
    selected_version: String,
    version_entry_expanded: bool,
    classes: Vec<ProguardClass>,
    focused_class: Option<ProguardClass>,
    class_search: String,
    method_search: String,
    field_search: String,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut a = Self {
            version_entry: Vec::new(),
            selected_version: String::from("No version selected"),
            version_entry_expanded: false,
            classes: Vec::new(),
            focused_class: None,
            class_search: String::from("Class search"),
            field_search: String::from("Field search"),
            method_search: String::from("Method search"),
        };
        a.update_version_entry();
        a
    }
}

impl MyApp {
    fn update_version_entry(&mut self) {
        self.version_entry.clear();
        let file_path = Path::new(&std::env::var("APPDATA").unwrap())
            .join(".minecraft")
            .join("versions");
        let a = fs::read_dir(file_path).unwrap();
        a.into_iter().for_each(|s| {
            if s.is_ok() {
                let unwrap = s.unwrap();
                let unwrap_name = unwrap.file_name();
                let aho = unwrap
                    .path()
                    .join(unwrap_name.into_string().unwrap() + ".json");
                if aho.exists() {
                    self.version_entry.push(aho);
                }
            }
        });

        let client = reqwest::blocking::Client::builder()
            .user_agent("MMV/1.0.0")
            .build().unwrap();
        let res = client.get("https://maven.minecraftforge.net/de/oceanlabs/mcp/versions.json")
            .send().unwrap().text().unwrap();
        println!("{}", res);
    }

    fn load_mapping(&mut self) {
        let json_len = self.selected_version.len();
        let file_path = Path::new(&std::env::var("APPDATA").unwrap())
            .join(".minecraft")
            .join("versions")
            .join(
                &self
                    .selected_version
                    .get(0..(json_len - ".json".len()))
                    .unwrap(),
            )
            .join(&self.selected_version);
        if file_path.exists() {
            let read_str = fs::read_to_string(file_path).unwrap();
            let json_data: Value = serde_json::from_str(&read_str).unwrap();
            let client_mappings_url = json_data["downloads"].as_object().unwrap()
                ["client_mappings"]
                .as_object()
                .unwrap()["url"]
                .as_str()
                .unwrap();
            let response =
                reqwest::blocking::get(client_mappings_url).expect("Failed to send request");
            let body = response.text().expect("Failed to get response body");

            self.focused_class = None;
            self.classes.clear();
            let sp = body.split("\n");
            let mut buffer = Vec::new();
            for s in sp.into_iter().filter(|t| t.contains(" -> ")) {
                if !s.contains("    ") && !buffer.is_empty() {
                    self.classes
                        .push(proguard::ProguardClass::deserialize(&buffer));
                    buffer.clear();
                }
                buffer.push(String::from(s));
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut version_changed_flag = false;

            ui.horizontal_top(|ui| {
                // Version combo box
                ui.vertical(|ui| {
                    egui::ScrollArea::vertical()
                        .show(ui, |ui| {
                            if ui.button(self.selected_version.as_str()).clicked() {
                                self.version_entry_expanded = !self.version_entry_expanded;
                            }
                            if self.version_entry_expanded {
                                self.version_entry.iter_mut().for_each(|f| {
                                    if ui
                                        .button(f.file_name().unwrap().to_string_lossy())
                                        .clicked()
                                    {
                                        self.version_entry_expanded = false;
                                        self.selected_version =
                                            f.file_name().unwrap().to_string_lossy().to_string();
                                        version_changed_flag = true;
                                    }
                                })
                            }
                        });
                });

                if ui.button("Fetch Minecraft Versions").clicked() {
                    self.update_version_entry();
                }
            });

            if version_changed_flag {
                self.load_mapping();
            }

            ui.text_edit_singleline(&mut self.class_search);

            egui::ScrollArea::vertical()
                .max_height(_frame.info().window_info.size.y / 2f32)
                .max_width(_frame.info().window_info.size.x)
                .id_source("Classess")
                .show(ui, |ui| {
                    if !self.class_search.is_empty() {
                        self.classes
                            .iter_mut()
                            .filter(|class| class.deobfed.contains(&self.class_search))
                            .for_each(|class| {
                                let mut display_str = String::new();
                                display_str.push_str(&class.deobfed.as_str());
                                display_str.push_str(P);
                                display_str.push_str(&class.obfed.as_str());

                                if ui.button(display_str).clicked() {
                                    self.focused_class = Some(class.copy());
                                }
                            });
                    }
            });
            match &self.focused_class {
                Some(focused_class) => {
                        ui.vertical(|ui| {
                            ui.text_edit_singleline(&mut self.method_search);
                            egui::ScrollArea::vertical()
                                .max_width(_frame.info().window_info.size.x)
                                .max_height(_frame.info().window_info.size.y / 5f32)
                                .id_source("Methods")
                                .show(ui, |ui| {
                                    if !self.method_search.is_empty() {
                                        focused_class
                                            .methods
                                            .iter()
                                            .filter(|method| method.deobfed.contains(self.method_search.as_str()))
                                            .for_each(|method| {
                                                let mut display_str = String::new();
                                                display_str.push_str(method.deobfed.as_str());
                                                display_str.push_str(P);
                                                display_str.push_str(method.obfed.as_str());
                
                                                ui.button(display_str);
                                            });
                                    }
                                });
                        });
                        ui.vertical(|ui| {
                            ui.text_edit_singleline(&mut self.field_search);

                            egui::ScrollArea::vertical()
                                .max_width(_frame.info().window_info.size.x)
                                .max_height(_frame.info().window_info.size.y / 5f32)
                                .id_source("Fields")
                                .show(ui, |ui| {
                                    if !self.field_search.is_empty() {
                                        focused_class
                                            .fields
                                            .iter()
                                            .filter(|field| field.deobfed.contains(self.field_search.as_str()))
                                            .for_each(|field| {
                                                let mut display_str = String::new();
                                                display_str.push_str(field.deobfed.as_str());
                                                display_str.push_str(P);
                                                display_str.push_str(&field.obfed.as_str());
                
                                                ui.button(display_str);
                                            });
                                    }
                                });
                        });
                }
                None => {}
            }
                
        });
    }
}
