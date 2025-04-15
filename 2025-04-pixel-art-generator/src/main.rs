//! A simple pixel art editor using macroquad.

// Cargo.toml (in case you want to copy this single file)
// 
// [package]
// name = "pixel-art-editor"
// version = "0.0.0"
// edition = "2021"
//
// [dependencies]
// macroquad = "0.4"
// image = "0.25"

use macroquad::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const DEFAULT_GRID_SIZE: usize = 16; // 16×16 pixel grid by default
const BACKGROUND_COLOR: Color = Color::new(0.2, 0.2, 0.2, 0.1); // light grey
const CELL_SIZE: f32 = 20.0; // Size of each pixel in the editor
const PALETTE_CELL_SIZE: f32 = 30.0; // Size of palette color squares
const PALETTE_X: f32 = 20.0; // X position of palette
const PALETTE_Y: f32 = 40.0; // Y position of palette
const PALETTE_COLS: usize = 8; // Palette columns

fn generate_default_palette() -> Vec<Color> {
    vec![
        // Transparent
        Color::new(0.0, 0.0, 0.0, 0.0),
        // Basic colors
        WHITE, BLACK, RED, GREEN, BLUE, YELLOW, PURPLE, ORANGE,
        // Shades of grey
        Color::new(0.2, 0.2, 0.2, 1.0),
        Color::new(0.4, 0.4, 0.4, 1.0),
        Color::new(0.6, 0.6, 0.6, 1.0),
        Color::new(0.8, 0.8, 0.8, 1.0),
        // Additional colors
        Color::new(0.5, 0.0, 0.0, 1.0), // Dark red
        Color::new(0.0, 0.5, 0.0, 1.0), // Dark green
        Color::new(0.0, 0.0, 0.5, 1.0), // Dark blue
        Color::new(1.0, 0.5, 0.5, 1.0), // Light red
        Color::new(0.5, 1.0, 0.5, 1.0), // Light green
        Color::new(0.5, 0.5, 1.0, 1.0), // Light blue
        Color::new(1.0, 1.0, 0.5, 1.0), // Light yellow
        Color::new(1.0, 0.5, 1.0, 1.0), // Light purple
        Color::new(0.5, 1.0, 1.0, 1.0), // Light cyan
        Color::new(0.5, 0.25, 0.0, 1.0), // Brown
        Color::new(0.0, 0.5, 0.5, 1.0), // Teal
        Color::new(0.5, 0.0, 0.5, 1.0), // Dark purple
        Color::new(1.0, 0.75, 0.8, 1.0), // Pink
        Color::new(0.75, 0.0, 0.25, 1.0), // Maroon
        Color::new(0.1, 0.5, 0.1, 1.0), // Forest green
        Color::new(0.0, 0.75, 1.0, 1.0), // Sky blue
        Color::new(1.0, 0.65, 0.0, 1.0), // Gold
        Color::new(0.85, 0.85, 0.1, 1.0), // Bright yellow
        Color::new(0.6, 0.4, 0.2, 1.0), // Tan
        Color::new(0.3, 0.2, 0.1, 1.0), // Dark brown
    ]
}

struct PixelArtEditor {
    grid_size: usize,
    pixels: Vec<Color>,
    palette: Vec<Color>,
    selected_color: usize,
    background_color: Color,
    canvas_x: f32,
    canvas_y: f32,
    is_dragging: bool,
    prev_width: f32,
    prev_height: f32,
}

impl Default for PixelArtEditor {
    fn default() -> Self {
        let grid_size = DEFAULT_GRID_SIZE;
        let pixels = vec![Color::new(0.0, 0.0, 0.0, 0.0); grid_size * grid_size];

        let screen_width = 800.0; // Default window width
        let screen_height = 600.0; // Default window height

        let canvas_size = grid_size as f32 * CELL_SIZE;
        let canvas_x = (screen_width - canvas_size) / 2.0;
        let canvas_y = (screen_height - canvas_size) / 2.0;

        PixelArtEditor {
            grid_size,
            pixels,
            palette: generate_default_palette(),
            selected_color: 0,
            background_color: BACKGROUND_COLOR,
            canvas_x,
            canvas_y,
            is_dragging: false,
            prev_width: screen_width,
            prev_height: screen_height,
        }
    }
}



impl PixelArtEditor {
    fn new(grid_size: usize) -> Self {
        PixelArtEditor {
            grid_size,
            .. Self::default()
        }
    }

    fn update_layout(&mut self) {
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        // Calculate scale factor for responsive sizing
        let min_dimension = screen_width.min(screen_height);
        let scale_factor = min_dimension / 600.0; // Base on 600px reference size
        
        // Ensure the cell size is proportional to the window size
        let scaled_cell_size = CELL_SIZE * scale_factor;
        
        let canvas_size = self.grid_size as f32 * scaled_cell_size;
        self.canvas_x = (screen_width - canvas_size) / 2.0;
        self.canvas_y = (screen_height - canvas_size) / 2.0;
        
        // Update previous dimensions
        self.prev_width = screen_width;
        self.prev_height = screen_height;
    }

    fn resize_grid(&mut self, new_size: usize) {
        let mut new_pixels = vec![Color::new(0.0, 0.0, 0.0, 0.0); new_size * new_size];

        // Copy existing pixels to new grid where possible
        for y in 0..self.grid_size.min(new_size) {
            for x in 0..self.grid_size.min(new_size) {
                let old_index = y * self.grid_size + x;
                let new_index = y * new_size + x;
                new_pixels[new_index] = self.pixels[old_index];
            }
        }

        self.pixels = new_pixels;
        self.grid_size = new_size;
        
        // Update layout to maintain centered position
        self.update_layout();
    }

    fn draw_palette(&self) {
        // Calculate scale factor for responsive sizing
        let min_dimension = screen_width().min(screen_height());
        let scale_factor = min_dimension / 600.0; // Base on 600px reference size
        
        let scaled_palette_cell_size = PALETTE_CELL_SIZE * scale_factor;
        let scaled_palette_x = PALETTE_X * scale_factor;
        let scaled_palette_y = PALETTE_Y * scale_factor;
        let scaled_text_size = 20.0 * scale_factor;
        let scaled_border_thickness = 2.0 * scale_factor;
        
        for (i, color) in self.palette.iter().enumerate() {
            let row = i / PALETTE_COLS;
            let col = i % PALETTE_COLS;
            let x = scaled_palette_x + col as f32 * scaled_palette_cell_size;
            let y = scaled_palette_y + row as f32 * scaled_palette_cell_size;
            
            // Draw color cell
            draw_rectangle(x, y, scaled_palette_cell_size - 2.0 * scale_factor, scaled_palette_cell_size - 2.0 * scale_factor, *color);
            
            // Draw border
            let border_color = if i == self.selected_color {
                GOLD
            } else {
                Color::new(0.5, 0.5, 0.5, 1.0)
            };
            draw_rectangle_lines(x, y, scaled_palette_cell_size - 2.0 * scale_factor, scaled_palette_cell_size - 2.0 * scale_factor, 
                                scaled_border_thickness, border_color);
            
            // Draw "T" for transparent color
            if i == 0 {
                draw_text("T", x + scaled_palette_cell_size / 2.0 - 5.0 * scale_factor, 
                         y + scaled_palette_cell_size / 2.0 + 5.0 * scale_factor, scaled_text_size, BLACK);
            }
        }

        // Draw background color selector
        let spacing = 20.0 * scale_factor;
        let bg_y = spacing + scaled_palette_y + (self.palette.len() / PALETTE_COLS + 1) as f32 * scaled_palette_cell_size;
        draw_text("Background:", scaled_palette_x, bg_y, scaled_text_size, WHITE);
        draw_rectangle(scaled_palette_x, bg_y + 10.0 * scale_factor, 
                      scaled_palette_cell_size - 2.0 * scale_factor, 
                      scaled_palette_cell_size - 2.0 * scale_factor, 
                      self.background_color);
        draw_rectangle_lines(scaled_palette_x, bg_y + 10.0 * scale_factor, 
                           scaled_palette_cell_size - 2.0 * scale_factor,
                           scaled_palette_cell_size - 2.0 * scale_factor, 
                           scaled_border_thickness, WHITE);
    }

    fn draw_grid(&self) {
        // Calculate scale factor for responsive sizing
        let min_dimension = screen_width().min(screen_height());
        let scale_factor = min_dimension / 600.0; // Base on 600px reference size
        let scaled_cell_size = CELL_SIZE * scale_factor;
        
        let grid_pixel_size = self.grid_size as f32 * scaled_cell_size;

        // Draw background
        draw_rectangle(
            self.canvas_x,
            self.canvas_y,
            grid_pixel_size,
            grid_pixel_size,
            self.background_color
        );

        // Draw pixels
        for y in 0..self.grid_size {
            for x in 0..self.grid_size {
                let index = y * self.grid_size + x;
                let color = self.pixels[index];

                // Only draw non-transparent pixels
                if color.a > 0.0 {
                    draw_rectangle(
                        self.canvas_x + x as f32 * scaled_cell_size,
                        self.canvas_y + y as f32 * scaled_cell_size,
                        scaled_cell_size,
                        scaled_cell_size,
                        color,
                    );
                }
            }
        }

        // Draw grid lines
        for i in 0..=self.grid_size {
            let line_pos = i as f32 * scaled_cell_size;
            let line_thickness = scale_factor.max(0.5); // Ensure grid lines scale but remain visible
            
            // Vertical lines
            draw_line(
                self.canvas_x + line_pos,
                self.canvas_y,
                self.canvas_x + line_pos,
                self.canvas_y + grid_pixel_size,
                line_thickness,
                Color::new(0.5, 0.5, 0.5, 0.5),
            );
            // Horizontal lines
            draw_line(
                self.canvas_x,
                self.canvas_y + line_pos,
                self.canvas_x + grid_pixel_size,
                self.canvas_y + line_pos,
                line_thickness,
                Color::new(0.5, 0.5, 0.5, 0.5),
            );
        }
    }

    fn draw_ui(&self) {
        // Calculate scale factor for responsive sizing
        let min_dimension = screen_width().min(screen_height());
        let scale_factor = min_dimension / 600.0; // Base on 600px reference size
        
        let scaled_cell_size = CELL_SIZE * scale_factor;
        let scaled_palette_x = PALETTE_X * scale_factor;
        let scaled_text_size = 20.0 * scale_factor;
        let scaled_line_spacing = 30.0 * scale_factor;
        
        let grid_instructions_y = self.canvas_y + (self.grid_size as f32 * scaled_cell_size) + 20.0 * scale_factor;
        draw_text("Press +/- to change grid size", scaled_palette_x, grid_instructions_y, scaled_text_size, WHITE);
        draw_text(&format!("Current grid: {}x{}", self.grid_size, self.grid_size), 
                 scaled_palette_x, grid_instructions_y + scaled_line_spacing, scaled_text_size, WHITE);

        let export_y = grid_instructions_y + 2.0 * scaled_line_spacing;
        draw_text("Press 'S' to save as PNG", scaled_palette_x, export_y, scaled_text_size, WHITE);
    }

    fn handle_input(&mut self) {
        let mouse_pos = mouse_position();
        
        // Calculate scale factor for responsive sizing
        let min_dimension = screen_width().min(screen_height());
        let scale_factor = min_dimension / 600.0; // Base on 600px reference size
        
        let scaled_palette_cell_size = PALETTE_CELL_SIZE * scale_factor;
        let scaled_palette_x = PALETTE_X * scale_factor;
        let scaled_palette_y = PALETTE_Y * scale_factor;
        let scaled_cell_size = CELL_SIZE * scale_factor;

        // Handle mouse button down
        if is_mouse_button_pressed(MouseButton::Left) {
            // Check if click is in the palette
            for (i, _) in self.palette.iter().enumerate() {
                let row = i / PALETTE_COLS;
                let col = i % PALETTE_COLS;
                let x = scaled_palette_x + col as f32 * scaled_palette_cell_size;
                let y = scaled_palette_y + row as f32 * scaled_palette_cell_size;
                
                if mouse_pos.0 >= x && mouse_pos.0 < x + scaled_palette_cell_size
                   && mouse_pos.1 >= y && mouse_pos.1 < y + scaled_palette_cell_size {
                    self.selected_color = i;
                    return;
                }
            }

            // Check if click is on background color selector
            let bg_y = scaled_palette_y + (self.palette.len() / PALETTE_COLS + 1) as f32 * scaled_palette_cell_size + 10.0 * scale_factor;
            if mouse_pos.0 >= scaled_palette_x && mouse_pos.0 < scaled_palette_x + scaled_palette_cell_size
               && mouse_pos.1 >= bg_y && mouse_pos.1 < bg_y + scaled_palette_cell_size {
                // Cycle through some background colors on click
                if self.background_color == Color::new(0.2, 0.2, 0.2, 1.0) {
                    self.background_color = WHITE;
                } else if self.background_color == WHITE {
                    self.background_color = BLACK;
                } else {
                    self.background_color = Color::new(0.2, 0.2, 0.2, 1.0);
                }
                return;
            }

            // Check if click is in the grid
            if mouse_pos.0 >= self.canvas_x && mouse_pos.0 < self.canvas_x + self.grid_size as f32 * scaled_cell_size
                && mouse_pos.1 >= self.canvas_y && mouse_pos.1 < self.canvas_y + self.grid_size as f32 * scaled_cell_size {
                self.is_dragging = true;
                self.draw_at_mouse_position(mouse_pos);
            }
        }

        // Handle mouse button up
        if is_mouse_button_released(MouseButton::Left) {
            self.is_dragging = false;
        }

        // Handle dragging
        if self.is_dragging && is_mouse_button_down(MouseButton::Left) {
            self.draw_at_mouse_position(mouse_pos);
        }
    }

    fn draw_at_mouse_position(&mut self, mouse_pos: (f32, f32)) {
        // Calculate scale factor for responsive sizing
        let min_dimension = screen_width().min(screen_height());
        let scale_factor = min_dimension / 600.0; // Base on 600px reference size
        let scaled_cell_size = CELL_SIZE * scale_factor;
        
        // Check if position is in the grid
        if mouse_pos.0 >= self.canvas_x && mouse_pos.0 < self.canvas_x + self.grid_size as f32 * scaled_cell_size
            && mouse_pos.1 >= self.canvas_y && mouse_pos.1 < self.canvas_y + self.grid_size as f32 * scaled_cell_size {
            let grid_x = ((mouse_pos.0 - self.canvas_x) / scaled_cell_size) as usize;
            let grid_y = ((mouse_pos.1 - self.canvas_y) / scaled_cell_size) as usize;
            let index = grid_y * self.grid_size + grid_x;

            // Set pixel color
            if index < self.pixels.len() {
                self.pixels[index] = self.palette[self.selected_color];
            }
        }
    }

    fn handle_keyboard_input(&mut self) {
        // Handle grid size changes
        if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
            if self.grid_size < 64 {
                self.resize_grid(self.grid_size + 1);
            }
        }

        if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
            if self.grid_size > 1 {
                self.resize_grid(self.grid_size - 1);
            }
        }

        // Handle save
        if is_key_pressed(KeyCode::S) {
            self.export_png("pixel_art.png");
        }
    }

    fn export_png(&self, filename: &str) {
        // Create a new image
        let width = self.grid_size as u32;
        let height = self.grid_size as u32;
        let mut image = image::RgbaImage::new(width, height);

        // Fill with pixel data
        for y in 0..self.grid_size {
            for x in 0..self.grid_size {
                let index = y * self.grid_size + x;
                let color = self.pixels[index];

                let r = (color.r * 255.0) as u8;
                let g = (color.g * 255.0) as u8;
                let b = (color.b * 255.0) as u8;
                let a = (color.a * 255.0) as u8;

                // If pixel is transparent, use background color
                let (r, g, b, a) = if color.a == 0.0 {
                    let bg = self.background_color;
                    (
                        (bg.r * 255.0) as u8,
                        (bg.g * 255.0) as u8,
                        (bg.b * 255.0) as u8,
                        255, // Background is always opaque in export
                    )
                } else {
                    (r, g, b, a)
                };

                image.put_pixel(x as u32, y as u32, image::Rgba([r, g, b, a]));
            }
        }

        // Save the image
        let path = Path::new(filename);
        let file = File::create(path).unwrap();
        let ref mut buffer = BufWriter::new(file);
        image::write_buffer_with_format(buffer, image.as_raw(), width, height,
            image::ColorType::Rgba8, image::ImageFormat::Png).unwrap();

        println!("Saved to {}", filename);
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "timClicks.dev - Pixel Art Editor".to_owned(),
        window_width: 1200,
        window_height: 800,
        window_resizable: true,
        fullscreen: false,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut editor = PixelArtEditor::new(DEFAULT_GRID_SIZE);
    
    // Initial layout update
    editor.update_layout();

    loop {
        clear_background(LIGHTGRAY);
        
        // Check if window size has changed
        let current_width = screen_width();
        let current_height = screen_height();
        
        if editor.prev_width != current_width || editor.prev_height != current_height {
            editor.update_layout();
            editor.prev_width = current_width;
            editor.prev_height = current_height;
        }

        editor.handle_input();
        editor.handle_keyboard_input();
        editor.draw_grid();
        editor.draw_palette();
        editor.draw_ui();

        next_frame().await
    }
}
