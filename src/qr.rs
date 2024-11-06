
use qrcode::{EcLevel, QrCode};
use image::{Luma, Rgba, RgbaImage, ImageBuffer};


pub enum QrStyle {
    Default,
    Rounded,
    Half
}

pub struct QrCat {
	version: i16,
	data: String,
	resolution: u32,
	style: QrStyle,
	color: Rgba<u8>,
	bgcolor: Rgba<u8>,
}

impl QrCat {

	pub fn new() -> Self {

		return QrCat { 
			version: 2 as i16,
			data: String::new(),
			resolution: 20,
			style: QrStyle::Default,
			color: Rgba([255, 255, 255, 255]),
			bgcolor: Rgba([0, 0, 0, 255]),
		 }
	}

	pub fn version(mut self, version: i16) -> Self {
		self.version = version;
		self
	}

	pub fn data(mut self, data: &str) -> Self {
		self.data = data.to_string();
		self
	}

	pub fn resolution(mut self, resolution: u32) -> Self {
		self.resolution = resolution;
		self
	}

	pub fn style(mut self, style: QrStyle) -> Self {
		self.style = style;
		self
	}

	pub fn color(mut self, color: [u8; 4]) -> Self {
		self.color = Rgba(color);
		self
	}

	pub fn bgcolor(mut self, bgcolor: [u8; 4]) -> Self {
		self.bgcolor = Rgba(bgcolor);
		self
	}

	pub fn to_png(&mut self, output_path: &str) {
		let qr = QrCode::with_version(self.data.clone(), qrcode::Version::Normal(self.version), EcLevel::H).unwrap();
		let img = qr.render::<Luma<u8>>().build();
		let mut rgba_image = RgbaImage::from_pixel(img.width()*self.resolution, img.height()*self.resolution, self.bgcolor);

		match self.style {
			QrStyle::Default => self.set_default_style(&mut rgba_image, &img),
			QrStyle::Rounded => self.set_rounded_style(&mut rgba_image, &img),
			QrStyle::Half => self.set_half_style(&mut rgba_image, &img),
		}

		rgba_image.save(output_path).unwrap();
	}

	pub fn build(&mut self) -> Result<RgbaImage, Box<dyn std::error::Error>> {
		let qr = QrCode::with_version(self.data.clone(), qrcode::Version::Normal(self.version), EcLevel::H).unwrap();
		let img = qr.render::<Luma<u8>>().build();
		let mut rgba_image = RgbaImage::from_pixel(img.width()*self.resolution, img.height()*self.resolution, self.bgcolor);

		match self.style {
			QrStyle::Default => self.set_default_style(&mut rgba_image, &img),
			QrStyle::Rounded => self.set_rounded_style(&mut rgba_image, &img),
			QrStyle::Half => self.set_half_style(&mut rgba_image, &img),
		}

		Ok(rgba_image)
	}

	// fn check_version(mut self, s: &str) -> Self {
	// 	match s.as_bytes().len() {
	// 		0..=12 => self.version = 1,
	// 		13..=14 => self.version = 2,
	// 		15..=24 => self.version = 3,
	// 		_ => self.version = 10,
	// 	}
	// 	self
	// }

	fn set_default_style(&self, rgba_image: &mut RgbaImage, img: &ImageBuffer<Luma<u8>, Vec<u8>>) {
					
		for x in (0..img.width()).step_by(8) {
	        for y in (0..img.height()).step_by(8) {
	            let pixel = img.get_pixel(x, y).0[0];
	            if pixel == 0 {
	                let cx = x * self.resolution + self.resolution / 2;
	                let cy = y * self.resolution + self.resolution / 2;
	                
	                draw_px(rgba_image, cx, cy, self.resolution * 4, self.color);
	            }
        	}
    	}

	    fn draw_px(image: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
		    for x in 0..2 * radius {
		        for y in 0..2 * radius {
		            let px = cx + x - radius;
		            let py = cy + y - radius;
		            if px < image.width() && py < image.height() {
		                image.put_pixel(px, py, color);
		            }
		        }
		    }
		}
	}

	fn set_rounded_style(&self, rgba_image: &mut RgbaImage, img: &ImageBuffer<Luma<u8>, Vec<u8>>) {
					
		for x in (0..img.width()).step_by(8) {
	        for y in (0..img.height()).step_by(8) {
	            let pixel = img.get_pixel(x, y).0[0];
	            if pixel == 0 {
	                let cx = x * self.resolution + self.resolution / 2;
	                let cy = y * self.resolution + self.resolution / 2;

	                let is_simple = img.get_pixel(x, y-1).0[0] != 0 && img.get_pixel(x, y+8).0[0] != 0;
	                let is_bot_angle = img.get_pixel(x, y+8).0[0] != 0;
	                let is_top_angle = img.get_pixel(x, y-1).0[0] != 0;
	                
	                if is_simple {
	                    draw_circle(rgba_image, cx, cy, self.resolution * 4, self.color);
	                } else if is_top_angle {
	                    draw_top(rgba_image, cx, cy, self.resolution * 4, self.color);
	                } else if is_bot_angle {
	                    draw_bot(rgba_image, cx, cy, self.resolution * 4, self.color);
	                } else {
	                    draw_px(rgba_image, cx, cy, self.resolution * 4, self.color);
	                }
	            }
        	}
    	}

	    fn draw_px(image: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
		    for x in 0..2 * (radius - 16) {
		        for y in 0..2 * radius {
		            let px = cx + x - (radius - 16);
		            let py = cy + y - radius;
		            if px < image.width() && py < image.height() {
		                image.put_pixel(px, py, color);
		            }
		        }
		    }
		}

		fn draw_circle(image: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
		    let rad = radius - 16;
		    for x in 0..2 * rad {
		        for y in 0..2 * rad {
		            let dx = x as i32 - rad as i32;
		            let dy = y as i32 - rad as i32;
		            if dx * dx + dy * dy <= (rad * rad) as i32 {
		                let px = cx + x - rad;
		                let py = cy + y - rad;
		                if px < image.width() && py < image.height() {
		                    image.put_pixel(px, py, color);
		                }
		            }
		        }
		    }
		}

		fn draw_top(image: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
			draw_circle(image, cx, cy, radius, color);
			for x in 0..2 * (radius - 16) {
				for y in radius..radius * 2 {
					let px = cx + x - (radius - 16);
					let py = cy + y - radius;
					if px < image.width() && py < image.height() {
						image.put_pixel(px, py, color);
					}
				}
			}
		}

		fn draw_bot(image: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
		    draw_circle(image, cx, cy, radius, color);
		    for x in 0..2 * (radius - 16) {
				for y in 0..radius {
					let px = cx + x - (radius - 16);
					let py = cy + y - radius;
					if px < image.width() && py < image.height() {
						image.put_pixel(px, py, color);
					}
				}
			}
		}
	}

	fn set_half_style(&self, rgba_image: &mut RgbaImage, img: &ImageBuffer<Luma<u8>, Vec<u8>>) {
					
		for x in (0..img.width()).step_by(8) {
	        for y in (0..img.height()).step_by(8) {
	            let pixel = img.get_pixel(x, y).0[0];
	            if pixel == 0 {
	                let cx = x * self.resolution + self.resolution / 2;
	                let cy = y * self.resolution + self.resolution / 2;

	                let is_simple = img.get_pixel(x, y-1).0[0] != 0 && img.get_pixel(x, y+8).0[0] != 0;
	                let is_bot_angle = img.get_pixel(x, y+8).0[0] != 0;
	                let is_top_angle = img.get_pixel(x, y-1).0[0] != 0;
	                if x < y {
	                	draw_px_default(rgba_image, cx, cy, self.resolution * 4, Rgba([255, 0, 0, 255]));
	                } else {
                		if is_simple {
		                    draw_circle(rgba_image, cx, cy, self.resolution * 4, self.color);
		                } else if is_top_angle {
		                    draw_top(rgba_image, cx, cy, self.resolution * 4, self.color);
		                } else if is_bot_angle {
		                    draw_bot(rgba_image, cx, cy, self.resolution * 4, self.color);
		                } else {
		                    draw_px(rgba_image, cx, cy, self.resolution * 4, self.color);
		                }
	                }
	            }
        	}
    	}

	    fn draw_px_default(image: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
		    for x in 0..2 * radius {
		        for y in 0..2 * radius {
		            let px = cx + x - radius;
		            let py = cy + y - radius;
		            if px < image.width() && py < image.height() {
		                image.put_pixel(px, py, color);
		            }
		        }
		    }
		}

	    fn draw_px(image: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
		    for x in 0..2 * (radius - 16) {
		        for y in 0..2 * radius {
		            let px = cx + x - (radius - 16);
		            let py = cy + y - radius;
		            if px < image.width() && py < image.height() {
		                image.put_pixel(px, py, color);
		            }
		        }
		    }
		}

		fn draw_circle(image: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
		    let rad = radius - 16;
		    for x in 0..2 * rad {
		        for y in 0..2 * rad {
		            let dx = x as i32 - rad as i32;
		            let dy = y as i32 - rad as i32;
		            if dx * dx + dy * dy <= (rad * rad) as i32 {
		                let px = cx + x - rad;
		                let py = cy + y - rad;
		                if px < image.width() && py < image.height() {
		                    image.put_pixel(px, py, color);
		                }
		            }
		        }
		    }
		}

		fn draw_top(image: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
			draw_circle(image, cx, cy, radius, color);
			for x in 0..2 * (radius - 16) {
				for y in radius..radius * 2 {
					let px = cx + x - (radius - 16);
					let py = cy + y - radius;
					if px < image.width() && py < image.height() {
						image.put_pixel(px, py, color);
					}
				}
			}
		}

		fn draw_bot(image: &mut RgbaImage, cx: u32, cy: u32, radius: u32, color: Rgba<u8>) {
		    draw_circle(image, cx, cy, radius, color);
		    for x in 0..2 * (radius - 16) {
				for y in 0..radius {
					let px = cx + x - (radius - 16);
					let py = cy + y - radius;
					if px < image.width() && py < image.height() {
						image.put_pixel(px, py, color);
					}
				}
			}
		}
	}
}