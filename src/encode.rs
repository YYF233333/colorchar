use ansi_term::Color::RGB;

use image::{DynamicImage, GenericImageView, Pixel, Rgb};

#[derive(Debug, Clone)]
pub struct Frame {
    buf: Vec<Vec<Rgb<u8>>>,
    width: usize,
    height: usize,
}

impl From<DynamicImage> for Frame {
    fn from(img: DynamicImage) -> Self {
        let mut buf = vec![];
        for i in 0..img.height() {
            let mut temp = vec![];
            for j in 0..img.width() {
                temp.push(img.get_pixel(j, i).to_rgb());
            }
            buf.push(temp);
        }
        Self {
            buf,
            width: img.width() as usize,
            height: img.height() as usize,
        }
    }
}

impl Frame {
    pub fn print(&self, width: usize, height: usize) -> String {
        assert!(width >= self.width);
        assert!(height >= self.height);
        let blank_w = (width - self.width) / 2;
        let blank_h = (height - self.height) / 2;
        let mut buf = String::with_capacity(width * height / 2);
        buf.push_str("\n".repeat(blank_h).as_str());
        for line in self.buf.iter() {
            buf.push_str(" ".repeat(blank_w).as_str());
            for px in line {
                buf.push_str(RGB(px[0], px[1], px[2]).paint("O").to_string().as_str());
            }
            buf.push('\n');
        }
        //buf.push_str("\n".repeat(blank_h).as_str());
        buf
    }

    pub fn black(width: usize, height: usize) -> Self {
        Self {
            buf: vec![vec![Rgb::from([255, 255, 255]); width]; height],
            width,
            height,
        }
    }

    // return self value when different
    pub fn diff(&self, other: Self) -> Vec<(usize, usize, Rgb<u8>)> {
        assert!(self.width == other.width);
        assert!(self.height == other.height);

        let mut diff = vec![];
        for i in 0..self.buf.len() {
            for j in 0..self.buf[0].len() {
                if self.buf[i][j] != other.buf[i][j] {
                    diff.push((i, j, self.buf[i][j]));
                }
            }
        }
        diff
    }
}
