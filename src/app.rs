#[warn(unused_imports)]
 
use eframe::{egui, epi};
use crate::app::egui::{RichText, TextStyle};
use std::path::{PathBuf, Path};
use image::{GenericImageView,imageops::FilterType};

use crate::image_operations::ImageOperations;
use crate::settings::Settings;


#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] 

#[derive(Default)]
pub struct ImageEditor {
    settings: Settings,
    image_operations: ImageOperations,
    picked_path: Option<PathBuf>,
    flip_v: egui::TextureId,
    flip_h: egui::TextureId,
}


impl epi::App for ImageEditor {
    fn name(&self) -> &str {
        "Image Editor"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        let mut fonts = egui::FontDefinitions::default();
 
        fonts.family_and_size.insert(
            egui::TextStyle::Button,
            (egui::FontFamily::Proportional, 15.0)
        );

        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../docs/Hack-Regular.ttf")),
        );
 
        fonts
            .fonts_for_family
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "my_font".to_owned());
 
        // Put my font as last fallback for monospace:
        fonts
            .fonts_for_family
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("my_font".to_owned());

        _ctx.set_fonts(fonts);

        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
        self.flip_v = load_image(Path::new("./docs/flip_vertical.png"),frame);
        self.flip_h = load_image(Path::new("./docs/flip-h.png"),frame);
    }
    

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }
 
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {

        
        //let Self{image_operations: image, settings ,text, picked_path} = self;

        for event in ctx.input().events.iter(){
            match event{
                egui::Event::Key{key, pressed, modifiers} => { 
                    if *key == egui::Key::Z && *pressed && modifiers.ctrl{
                        self.image_operations.undo(frame);
                    }
                    if *key == egui::Key::Y && *pressed && modifiers.ctrl{
                        self.image_operations.redo(frame);
                    }

                } 
                _ => {}
            }
        }
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            
            egui::menu::bar(ui, |ui| {
                
                ui.menu_button(RichText::new("File"), |ui| {
                    
                    if ui.button(RichText::new("Open")).clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_file(){

                            self.image_operations.load_image_from_path(path.as_path(), frame);
                            self.picked_path = Some(path);
    
                        }else {
                            frame.quit()
                        };
                    }

                    if ui.button(RichText::new("Save")).clicked() {
                        if let Some(path) = rfd::FileDialog::new().save_file(){
                            
                            self.image_operations.save(path.as_path());

                        };

                    }

                    if ui.button(RichText::new("Exit")).clicked() {
                        frame.quit()
                    }
 
                });
                
                if ui.button(RichText::new("Undo").text_style(TextStyle::Body)).clicked() {
                    self.image_operations.undo(frame);
                }
                
                if ui.button("Redo").clicked() {
                    self.image_operations.redo(frame);
                }
 
            });
        });
 
        egui::SidePanel::right("side_panel").resizable(true).show(ctx, |ui| {
            ui.label("Selected Photo: ");

            if let Some(path) = self.picked_path.as_mut(){
                ui.monospace(path.display().to_string());
            };

            ui.separator();

            ui.heading("Resizing");
            ui.vertical(|ui| {
                ui.add(egui::Slider::new(&mut self.settings.dimensions[0], 0..=1000).prefix("w: "));
                ui.add(egui::Slider::new(&mut self.settings.dimensions[1], 0..=1000).prefix("h: "));

                ui.horizontal(|ui| {
                    egui::ComboBox::from_label("Filter Type")
                    .selected_text(format!("{:?}", self.settings.filter_type))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.settings.filter_type, FilterType::Nearest, "near");
                        ui.selectable_value(&mut self.settings.filter_type, FilterType::Triangle, "tri");
                        ui.selectable_value(&mut self.settings.filter_type, FilterType::CatmullRom, "cmr");
                        ui.selectable_value(&mut self.settings.filter_type, FilterType::Gaussian, "gauss");
                        ui.selectable_value(&mut self.settings.filter_type, FilterType::Lanczos3, "lcz2");
                    });

                    if ui.button("Resize").clicked() {

                        self.image_operations.resize(self.settings.dimensions[0], 
                            self.settings.dimensions[1],self.settings.filter_type, frame);

                    }
                });
            });

            ui.separator();

            ui.heading("Cropping");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut self.settings.x).prefix("x: ").clamp_range(std::ops::RangeInclusive::new(0, self.settings.dimensions[0] - 1)));
                ui.add(egui::DragValue::new(&mut self.settings.y).prefix("y: ").clamp_range(std::ops::RangeInclusive::new(0, self.settings.dimensions[1] - 1)));
            
                ui.add(egui::DragValue::new(&mut self.settings.width).prefix("w: ").clamp_range(std::ops::RangeInclusive::new(1, self.settings.dimensions[0])));
                ui.add(egui::DragValue::new(&mut self.settings.height).prefix("h: ").clamp_range(std::ops::RangeInclusive::new(1, self.settings.dimensions[1])));
            
                
                if ui.button("Crop").clicked() {
                    self.image_operations.crop(self.settings.x, 
                        self.settings.y,self.settings.width, self.settings.height, frame);
                }
            });

            ui.separator();
 
            ui.heading("Rotation");
            ui.horizontal(|ui| {

                if ui.button("Rotate 90").clicked() {
                    self.image_operations.rotate90(frame);
                }

                if ui.button("Rotate 180").clicked() {
                    self.image_operations.rotate180(frame);
                }

                if ui.button("Rotate 270").clicked() {
                    self.image_operations.rotate270(frame);
                }

            });


            ui.separator();
            
            
            ui.horizontal(|ui| {
                ui.heading("Flipping");
                ui.add_space(25.0);
                if ui.add(egui::ImageButton::new(self.flip_v, egui::vec2(18.0, 18.0))).clicked() {
                    self.image_operations.flip_vertical(frame);
                }
                ui.add_space(8.0);
                if ui.add(egui::ImageButton::new(self.flip_h, egui::vec2(18.0, 18.0))).clicked() {
                    self.image_operations.flip_horizontal(frame);
                }
                
            });
            
            ui.separator();
            

            ui.heading("Blurring");
            ui.vertical(|ui| {
            ui.horizontal(|ui| {

                ui.add(egui::Slider::new(&mut self.settings.gaussian_sigma, 0.0..=10.0));
                if ui.button("Apply").clicked()
                {
                    self.image_operations.blur(self.settings.gaussian_sigma,frame);
                }
            });

            ui.separator();

            ui.heading("Brighten");
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut self.settings.brightness, -30..=30));
                if ui.button("Apply").clicked()
                {
                    self.image_operations.brightness(self.settings.brightness, frame);
                }
            });

            ui.separator();

            ui.heading("Sharpness");
            ui.horizontal(|ui| {

                ui.add(egui::DragValue::new(&mut self.settings.radius));
                ui.label("radius");
           
                ui.add(egui::DragValue::new(&mut self.settings.threshold));
                ui.label("threshold");

                if ui.button("Apply").clicked(){

                    self.image_operations.sharpness(self.settings.radius,self.settings.threshold, frame);
    
                }
            });
            
            
            ui.separator();

            ui.heading("Contrast");
            ui.horizontal(|ui| {

                ui.add(egui::Slider::new(&mut self.settings.contrast, -30.0..=30.0));
                if ui.button("Apply").clicked()
                {
                    self.image_operations.contrast(self.settings.contrast, frame);

                }
            });
            ui.end_row();
            
        });
        ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.1;
                ui.label("created by npandelieva");
                
                //ui.hyperlink_to("npandelieva", "https://github.com/emilk/egui/tree/master/eframe");
            });
        });
 
 
        });
 
        egui::CentralPanel::default().show(ctx, |ui| {        
 
            
            if let Some(img) = self.image_operations.image_data.as_mut(){
                ui.image(self.image_operations.texture_id,[img.dimensions().0 as f32, img.dimensions().1 as f32]);
                
            }
 
        });
 
 
 
        if false{
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("");
                ui.label("You would normally chose either panels OR windows.");
 
            });
        }
 
 
    }
}

fn load_image(path: &std::path::Path, frame: &epi::Frame) -> egui::TextureId{

    match image::io::Reader::open(path){
        Ok(result) => {
            let image= result.decode().unwrap();
            let dimensions = [image.dimensions().0 as usize, image.dimensions().1 as usize];
            let image_buffer = image.to_rgba8();
            let pixel = image_buffer.as_flat_samples();
            let epi_image = epi::Image::from_rgba_unmultiplied(dimensions, pixel.as_slice());
            return frame.alloc_texture((epi_image).clone());
        },
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
    };

}
