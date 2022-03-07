use eframe::{egui, epi};
use image::{GenericImageView, DynamicImage,imageops::FilterType};

pub struct ImageOperations{
    pub image_data: Option<DynamicImage>,
    pub texture_id: egui::TextureId,
    redo_stack: Vec<DynamicImage>,
    undo_stack: Vec<DynamicImage>,
}

impl Default for ImageOperations{
    fn default() -> Self {
        Self{
            image_data: None,
            texture_id: egui::TextureId::User(0),
            redo_stack: Vec::new(),
            undo_stack: Vec::new(),
        }
    }
}

impl ImageOperations{

    pub fn load_image_from_path(&mut self, path: &std::path::Path, frame: &epi::Frame) {

        match image::io::Reader::open(path){
            Ok(img) => {
                self.image_data = Some(img.decode().unwrap());
                let dimensions = [self.image_data.as_ref().unwrap().dimensions().0 as usize, self.image_data.as_ref().unwrap().dimensions().1 as usize];
                let image_buffer = self.image_data.as_ref().unwrap().to_rgba8();
                let pixel = image_buffer.as_flat_samples();
                let epi_image = epi::Image::from_rgba_unmultiplied(dimensions, pixel.as_slice());
                self.texture_id = frame.alloc_texture((epi_image).clone());
                //panic!("couldn't open image: {}", img.display(), why)
            },
            Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        };

    }

    pub fn get_id_from_texture(&self, img: &DynamicImage,frame: &epi::Frame) -> egui::TextureId{

        let size = [img.dimensions().0 as usize, img.dimensions().1 as usize];
        let image_buffer = img.to_rgba8();
        let pixel = image_buffer.as_flat_samples();
        let epi_image = epi::Image::from_rgba_unmultiplied(size, pixel.as_slice());

        frame.alloc_texture((epi_image).clone())

    }

    pub fn resize(&mut self, width: usize, height: usize,filter_type: FilterType, frame: &epi::Frame){
        if let Some(img) = self.image_data.as_mut()
        {
            self.undo_stack.push(img.clone());
            if img.dimensions().0 as usize != width || img.dimensions().1 as usize != height {
                self.image_data = Some(img.resize_exact(width as u32, height as u32, filter_type));
                self.texture_id = self.get_id_from_texture(self.image_data.as_ref().unwrap(),frame);
            }
        }
    }

    pub fn rotate90(&mut self,frame: &epi::Frame){
        if let Some(img) = self.image_data.as_mut(){
            self.undo_stack.push(img.clone());
            self.image_data = Some(img.rotate90());
            self.texture_id = self.get_id_from_texture(self.image_data.as_ref().unwrap(),frame);
        }
    }

    pub fn rotate180(&mut self,frame: &epi::Frame){
        if let Some(img) = self.image_data.as_mut(){
            self.undo_stack.push(img.clone());
            self.image_data = Some(img.rotate180());
            self.texture_id = self.get_id_from_texture(self.image_data.as_ref().unwrap(),frame);
        }
    }

    pub fn rotate270(&mut self,frame: &epi::Frame){
        if let Some(img) = self.image_data.as_mut(){
            self.undo_stack.push(img.clone());
            self.image_data = Some(img.rotate270());
            self.texture_id = self.get_id_from_texture(self.image_data.as_ref().unwrap(),frame);
        }
    }

    pub fn flip_vertical(&mut self,frame: &epi::Frame){
        if let Some(img) = self.image_data.as_mut(){
            self.undo_stack.push(img.clone());
            self.image_data = Some(img.flipv());
            self.texture_id = self.get_id_from_texture(self.image_data.as_ref().unwrap(),frame);
        }
    }

    pub fn flip_horizontal(&mut self,frame: &epi::Frame){
        if let Some(img) = self.image_data.as_mut(){
            self.undo_stack.push(img.clone());
            self.image_data = Some(img.fliph());
            self.texture_id = self.get_id_from_texture(self.image_data.as_ref().unwrap(),frame);
        }
    }
	
    pub fn blur(&mut self, sigma: f32,frame: &epi::Frame){
        if let Some(img) = self.image_data.as_mut() {
            self.undo_stack.push(img.clone());
            self.image_data = Some(img.blur(sigma));
            self.texture_id = self.get_id_from_texture(self.image_data.as_ref().unwrap(),frame);
        }
    }

    pub fn brightness(&mut self, value: i32,frame: &epi::Frame){
        if let Some(img) = self.image_data.as_mut() {
            self.undo_stack.push(img.clone());
            self.image_data = Some(img.brighten(value));
            self.texture_id = self.get_id_from_texture(self.image_data.as_ref().unwrap(),frame);
        }
    }

    pub fn sharpness(&mut self, radius: f32, threshold: i32, frame: &epi::Frame){
        if let Some(img) = self.image_data.as_mut() {
            self.undo_stack.push(img.clone());
            self.image_data = Some(img.unsharpen(radius, threshold));
            self.texture_id = self.get_id_from_texture(self.image_data.as_ref().unwrap(),frame);
        }
    }

    pub fn contrast(&mut self,value: f32, frame: &epi::Frame){
        if let Some(img) = self.image_data.as_mut() {
            self.undo_stack.push(img.clone());
            self.image_data = Some(img.adjust_contrast(value));
            self.texture_id = self.get_id_from_texture(self.image_data.as_ref().unwrap(),frame);
        }
    }

    pub fn save(&mut self, path: &std::path::Path) {
        if let Some(img) = &self.image_data{
            img.save_with_format(path, image::ImageFormat::Png).unwrap(); 
        };
    }

    pub fn crop(&mut self, x: u32, y: u32, width: u32, height: u32, frame: &epi::Frame){
        if let Some(img) = self.image_data.as_mut() {
            self.undo_stack.push(img.clone());
            self.image_data = Some(img.crop_imm(x, y , width, height));
            self.texture_id = self.get_id_from_texture(self.image_data.as_ref().unwrap(),frame);
        }
    }

    pub fn undo(&mut self,frame: &epi::Frame){
        if self.undo_stack.len() > 0 {
            self.redo_stack.push(self.image_data.as_ref().unwrap().clone());
            self.image_data = self.undo_stack.pop();
            self.texture_id = self.get_id_from_texture(self.image_data.as_ref().unwrap(),frame);
        }
    }

    pub fn redo(&mut self,frame: &epi::Frame){
        if self.redo_stack.len() > 0{
            self.undo_stack.push(self.image_data.as_ref().unwrap().clone());
            self.image_data = self.redo_stack.pop();
            self.texture_id = self.get_id_from_texture(self.image_data.as_ref().unwrap(),frame);
        }
    }
}
