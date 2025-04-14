use crate::rs_error::RSLogoError;
use std::path::Path;
use unsvg::{Color, Image, COLORS};

pub struct Turtle {
    x: i32,
    y: i32,
    heading: i32,
    pen_down: bool,
    color: Color,
    image: Image,
}

impl Turtle {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            x: (width / 2) as i32,
            y: (height / 2) as i32,
            heading: 0,
            pen_down: false,
            color: Color::white(),
            image: Image::new(width, height),
        }
    }

    pub fn pen_up(&mut self) {
        self.pen_down = false;
    }

    pub fn pen_down(&mut self) {
        self.pen_down = true;
    }

    pub fn forward(&mut self, numpixels: i32) -> Result<(), RSLogoError> {
        let forward_heading = self.heading;
        if numpixels < 0 {
            return self.back(-numpixels);
        }
        self.process_movement(numpixels, forward_heading)
    }

    pub fn back(&mut self, numpixels: i32) -> Result<(), RSLogoError> {
        let back_heading = self.heading + 180;
        if numpixels < 0 {
            return self.forward(-numpixels);
        }
        self.process_movement(numpixels, back_heading)
    }

    pub fn left(&mut self, numpixels: i32) -> Result<(), RSLogoError> {
        let left_heading = self.heading - 90;
        if numpixels < 0 {
            return self.right(-numpixels);
        }
        self.process_movement(numpixels, left_heading)
    }

    pub fn right(&mut self, numpixels: i32) -> Result<(), RSLogoError> {
        let right_heading = self.heading + 90;
        if numpixels < 0 {
            return self.left(numpixels);
        }
        self.process_movement(numpixels, right_heading)
    }

    pub fn set_pen_color(&mut self, colorcode: u32) -> Result<(), RSLogoError> {
        if colorcode >= (COLORS.len() as u32) {
            return Err(RSLogoError::InvalidArgument {
                command: "SETPENCOLOR".to_string(),
                argument: colorcode.to_string(),
                expected: "SETPENCOLOR <u32 type String>".to_string(),
            });
        }
        self.color = COLORS[colorcode as usize];
        Ok(())
    }

    pub fn turn(&mut self, degrees: i32) {
        self.heading += degrees;
    }

    pub fn set_heading(&mut self, degrees: i32) {
        self.heading = degrees;
    }

    pub fn set_x(&mut self, location: i32) {
        self.x = location;
    }

    pub fn set_y(&mut self, location: i32) {
        self.y = location;
    }

    pub fn save_image(&self, image_path: &Path) -> Result<(), RSLogoError> {
        match image_path.extension().and_then(|s| s.to_str()) {
            Some("svg") => {
                self.image
                    .save_svg(image_path)
                    .map_err(|e| RSLogoError::ImageSaveError(e.to_string()))?;
            }
            Some("png") => {
                self.image
                    .save_png(image_path)
                    .map_err(|e| RSLogoError::ImageSaveError(e.to_string()))?;
            }
            _ => {
                return Err(RSLogoError::ImageSaveError(
                    "File extension not supported".to_string(),
                ));
            }
        }
        Ok(())
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_heading(&self) -> i32 {
        self.heading
    }

    pub fn get_pen_color(&self) -> u32 {
        COLORS.iter().position(|&c| c == self.color).unwrap_or(8) as u32
    }

    fn process_movement(&mut self, numpixels: i32, direction: i32) -> Result<(), RSLogoError> {
        let new_position = if self.pen_down {
            self.image
                .draw_simple_line(self.x, self.y, direction, numpixels, self.color)
                .map_err(|e| RSLogoError::DrawError(e.to_string()))?
        } else {
            unsvg::get_end_coordinates(self.x, self.y, direction, numpixels)
        };

        self.set_x(new_position.0);
        self.set_y(new_position.1);
        Ok(())
    }
}
