use image::{imageops::FilterType};

#[derive(PartialEq)]
pub struct Settings {
    pub gaussian_sigma: f32,
    pub brightness: i32,
    pub radius: f32,
    pub threshold: i32,
    pub contrast: f32,
    pub bright_value: i32,
    pub rotation: f32,
    pub dimensions: [usize; 2],
    pub filter_type: FilterType,
    pub x: u32, 
    pub y: u32, 
    pub width: u32, 
    pub height: u32,
}

impl Default for Settings{
    fn default() -> Self {
        Self {
            gaussian_sigma: 0.0,
            brightness: 0,
            radius: 0.0,
            threshold: 0,
            contrast: 0.0,
            bright_value: 0,
            rotation: 0.0,
            dimensions: [600,600],
            filter_type: FilterType::Triangle,
            x: 0, 
            y: 0, 
            width: 0, 
            height: 0,
        }
    }
}