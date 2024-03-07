//! Thanks to [altunenes] and [GiveMe30Dollars] for many useful references.
//!
//! [altunenes]: https://github.com/altunenes/rusty_art/
//! [GiveMe30Dollars]: https://github.com/GiveMe30Dollars/Terraces/blob/main/Perlin_Module.pde

use std::path::PathBuf;

use nannou::image::{open, ImageBuffer, Rgba};
use nannou::noise::{NoiseFn, Seedable};
use nannou::noise::utils::{NoiseMap, NoiseMapBuilder};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

type Image = ImageBuffer<Rgba<u8>, Vec<u8>>;

fn main() {
    nannou::app(model)
        // .update(update)
        .run();
}


struct Settings {
    noise_map_width: usize,
    noise_map_height: usize,
    seed: u32,
}

struct Model {
    // img: Image,
    field: NoiseMap,
    texture: wgpu::Texture,
    settings: Settings,
    // egui: Egui,
    // noise: Box<dyn nannou::noise::NoiseFn>,
}

fn model(app: &App) -> Model {
    let main_window_id = app
        .new_window()
        .size(800, 600)
        .view(view)
        .build()
        .unwrap();

    // let win_id = app
    //     .new_window()
    //     .size(800, 600)
    //     .view(view)
    //     .raw_event(raw_window_event)
    //     .build()
    //     .unwrap();

    let window = app.window(main_window_id).unwrap();

    // let egui: Egui = Egui::from_window(&window);
    let settings = Settings {
        noise_map_width: 800,
        noise_map_height: 600,
        seed: 1,
    };

    let noise = nannou::noise::SuperSimplex::new().set_seed(settings.seed);

    let mut field = NoiseMap::new(settings.noise_map_width, settings.noise_map_height);

    for x in 0..settings.noise_map_width {
        for y in 0..settings.noise_map_height {
            field.set_value(x, y, noise.get([x as f64, y as f64]))
        }
    }

    let win = window.rect();
    let texture = wgpu::TextureBuilder::new()
        .size([win.w() as u32, win.h() as u32])
        .format(wgpu::TextureFormat::Rgba8Unorm)
        .usage(wgpu::TextureUsages::COPY_DST | wgpu::TextureUsages::TEXTURE_BINDING)
        .build(window.device());

    Model {
        field,
        settings,
        // egui,
        texture,
        // noise: Box::new(noise),
    }
}

// fn update(app: &App, model: &mut Model, _update: Update) {
//     // let egui = &mut model.egui;
//     let settings = &mut model.settings;
    // egui.set_elapsed_time(_update.since_start);
    // let ctx = egui.begin_frame();

    // egui::Window::new("Settings").show(&ctx, |ui| {
    //     ui.add(egui::Slider::new(&mut settings.noise_map_width, 0..=1000).text("noise map width"));
    //     ui.add(egui::Slider::new(&mut settings.noise_map_height, 0..=1000).text("noise map height"));
    // });

    // model.field = NoiseMap::new(settings.noise_map_width, settings.noise_map_height);

    // let new_dims = (
    //     model.settings.pixelation.max(1.0).round() as u32,
    //     model.settings.pixelation.max(1.0).round() as u32,
    // );
    // let img_ = pixelate(&model.img, new_dims);
    // let dynamic_img = nannou::image::DynamicImage::ImageRgba8(img_);
    // let texture = wgpu::Texture::from_image(app, &dynamic_img);
    // model.texture = Some(texture);
    // model.settings.pixelation += model.settings.speed * model.settings.direction;
    // model.settings.pixelation = model.settings.pixelation.min(50.0).max(1.0);
    // if model.settings.pixelation <= 1.0 || model.settings.pixelation >= 50.0 {
    //     model.settings.direction = -model.settings.direction;
    // }
// }
fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(BLACK);
    let win = app.window_rect();

    let image = ImageBuffer::from_fn(win.w() as u32, win.h() as u32, |x, y| {
        let val = model.field.get_value(x as usize, y as usize);
        let val = map_range(val, -1.0, 1.0, 0.0, 255.0);
        let val = val as u8;

        nannou::image::Rgba([val, val, val, 255])
    });

    // let image = image::ImageBuffer::from_fn(win.w() as u32, win.h() as u32, |x, y| {
    //     let noise_x = map_range(x, 0, win.w() as u32, 0.0, noise_x_range) as f64;
    //     let noise_y = map_range(y, 0, win.h() as u32, 0.0, noise_y_range) as f64;
    //     let mut noise_value = 0.0;

    //     if model.noise_mode == 1 {
    //         noise_value = map_range(
    //             noise.get([noise_x, noise_y]),
    //             1.0,
    //             -1.0,
    //             0.0,
    //             std::u8::MAX as f64,
    //         );
    //     } else if model.noise_mode == 2 {
    //         let n = map_range(
    //             noise.get([noise_x, noise_y]),
    //             -1.0,
    //             1.0,
    //             0.0,
    //             std::u8::MAX as f64 / 10.0,
    //         );
    //         noise_value = (n - n.floor()) * std::u8::MAX as f64;
    //     }
    //     let n = noise_value as u8;
    //     nannou::image::Rgba([n, n, n, std::u8::MAX])
    // });

    let flat_samples = image.as_flat_samples();
    model.texture.upload_data(
        app.main_window().device(),
        &mut *frame.command_encoder(),
        &flat_samples.as_slice(),
    );


    let draw = app.draw();
    draw.texture(&model.texture);
    draw.to_frame(app, &frame).unwrap();
    // model.egui.draw_to_frame(&frame).unwrap();
    // if app.keys.down.contains(&Key::Space) {
    //     let file_path = app
    //         .project_path()
    //         .expect("failed to locate project directory")
    //         .join("frames")
    //         .join(format!("{:04}.png", app.elapsed_frames()));
    //     app.main_window().capture_frame(file_path);
    // }
}

// fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
//     model.egui.handle_raw_event(event);
// }
