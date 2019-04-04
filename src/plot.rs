use im::Rgba;
use std::f64::consts::PI;
use crate::complex::*;

pub const PLOT_WIDTH  : u32 = 800;
pub const PLOT_HEIGHT : u32 = 800;

const DX_SAMPLE: u32 = 1000;
const DY_SAMPLE: u32 = 1000;

pub struct Plot {
    lb: Cpx, // describes position of bottom left coordinate point
    rt: Cpx, // describes position of top right coordinate point
    f: fn(Cpx) -> Option<Cpx>,
}

impl Plot {
    pub fn new() -> Plot {
        Plot {
            lb: Cpx::new(-10, -10),
            rt: Cpx::new(10, 10),
            f: |x| -> Option<Cpx> { 
                Some(x * x * x) 
            }
        }
    }

    fn get_max(&self) -> f64 {
        let mut max: f64 = 0.0;
        let diag = self.rt - self.lb;
        let mut val: f64 = 0.0;
        let dx = diag.r/(DX_SAMPLE as f64);
        let dy = diag.c/(DY_SAMPLE as f64);
        for x in 0..DX_SAMPLE {
            for y in 0..DY_SAMPLE {
                val = (self.f)(self.lb + Cpx::new(dx * x as f64, dy * y as f64)).unwrap_or(Cpx::new(0,0)).abs();
                if val > max { max = val; }
            }
        }
        max
    }
    
    fn draw_fn(&self, max: f64, canvas: &mut im::ImageBuffer<Rgba<u8>, Vec<u8>>) {
        let diag = self.rt - self.lb;
        let (cw, ch) = canvas.dimensions();
        let dy = diag.r/(cw as f64);
        let dx = diag.c/(ch as f64);
        for x in 0..cw {
            for y in 0..ch {
                canvas.put_pixel(x, y, 
                                 px_from_cpx((self.f)(self.lb + Cpx::new(dx * x as f64, dy * y as f64)), 
                                             max));
            }
        }
    }

    pub fn update(&self, canvas: &mut im::ImageBuffer<Rgba<u8>, Vec<u8>>) {
        println!("updating canvas");
        let max: f64 = self.get_max();
        println!("Maximum: {}", max);
        self.draw_fn(max, canvas);
    }
}

fn px_from_cpx(num: Option<Cpx>, max: f64) -> Rgba<u8> {
    match num {
        Some(num) => hsv_to_rgb(num.phi(), 1.0,(num.abs() / max)),
        None      => im::Rgba([255, 255, 255, 255]), 
    }
}

fn hsv_to_rgb(h: f64, s: f64, v: f64) -> Rgba<u8> {
    use palette::FromColor;
    let chsv: palette::Hsv<f64> = palette::Hsv { hue: palette::RgbHue::from_radians(h), saturation: s, value: 1.0 - v };
    let color: palette::Rgb<f64> = palette::Rgb::from_hsv(chsv);
    let r: u8 = (color.red * 255.0) as u8;
    let g: u8 = (color.green * 255.0) as u8;
    let b: u8 = (color.blue * 255.0) as u8;
    im::Rgba([r,g,b,255])
    /*
    let c = v * s;
    let h = (h * 3.0) / PI;
    let x = c * (1.0 - (h % 2.0 - 1.0).abs());
    let m = v - c;

    let (red, green, blue) = if h >= 0.0 && h < 0.0 {
        (c, x, 0.0)
    } else if h >= 1.0 && h < 2.0 {
        (x, c, 0.0)
    } else if h >= 2.0 && h < 3.0 {
        (0.0, c, x)
    } else if h >= 3.0 && h < 4.0 {
        (0.0, x, c)
    } else if h >= 4.0 && h < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    
    im::Rgba([((red + m) * 255.0) as u8, ((green + m) * 255.0) as u8, ((blue + m) * 255.0) as u8, 255])
    */
}
