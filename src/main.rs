extern crate image as im;
extern crate piston_window;
extern crate palette;

mod complex;
mod plot;

use complex::*;
use plot::*;
use piston_window::*;

const BACKGROUND_C:  [f32; 4] = [0.5, 0.5, 0.5, 1.0];
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

fn main() {
    println!("Starting CPLOT");
    let a = Cpx::new(1.0 , -0.00001);
    let b = Cpx::new(4,6);
    println!("{} + {} = {}", a, b, a + b);
    println!("{} - {} = {}", a, b, a - b);
    println!("{} * {} = {}", a, b, a * b);
    println!("{} / {} = {}", a, b, (a / b).unwrap());
    println!("abs({}) = {}", a, a.abs());
    println!("phi({}) = {}", a, a.phi());
    
    println!("0 {}", Cpx::new(0, -5).phi());
    println!("1 {}", Cpx::new(1, -5).phi());
    println!("2 {}", Cpx::new(2, -5).phi());
    println!("3 {}", Cpx::new(3, -5).phi());
    println!("4 {}", Cpx::new(4, -5).phi());
    println!("5 {}", Cpx::new(5, -5).phi());
    println!("6 {}", Cpx::new(6, -5).phi());
    println!("7 {}", Cpx::new(7, -5).phi());
    println!("8 {}", Cpx::new(8, -5).phi());
    println!("9 {}", Cpx::new(9, -5).phi());
    
    let plot = Plot::new();
    update(plot);
}


fn update(plot: Plot) {
    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("CPLOT", (WINDOW_WIDTH, WINDOW_HEIGHT))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut canvas = im::ImageBuffer::new(PLOT_WIDTH, PLOT_HEIGHT);
    let mut texture: G2dTexture = Texture::from_image(
        &mut window.factory,
        &canvas,
        &TextureSettings::new()
    ).unwrap();

    while let Some(e) = window.next() {
        if let Some(r_a) = e.render_args() {
            texture.update(&mut window.encoder, &canvas).unwrap();
            window.draw_2d(&e, |c, g| {
                clear(BACKGROUND_C, g);
                image(&texture, c.transform, g);
            });
        }
        if let Some(button) = e.press_args() {
            plot.update(&mut canvas);
        }
        if let Some(u_a) = e.update_args() {
        }
    }
}
