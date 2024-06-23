extern crate rfd;
use std::{
    cmp::min,
    fs::File,
    io::{BufRead, Write},
};

use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};

use crate::engine::view::View;

use super::game_state::ConwayGameState;

const BOARD_BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);
const BOARD_CELL_COLOR: Color = Color::RGB(255, 255, 255);
const BOARD_HOVERING_ALIVE_CELL_COLOR: Color = Color::RGB(255, 0, 0);
const BOARD_HOVERING_DEAD_CELL_COLOR: Color = Color::RGB(0, 255, 0);
const BOARD_NEWLY_ALIVE_CELL_COLOR: Color = Color::RGB(255, 255, 255);
const BOARD_NEWLY_DEAD_CELL_COLOR: Color = Color::RGB(50, 50, 50);

const MIN_BOARD_SIZE: usize = 8;
const MAX_BOARD_SIZE: usize = 64;

pub struct BoardView {
    view_bound: Rect,
    render_bound: Rect,
    cell_length: usize,
    game_state: ConwayGameState,
    hovering_cell: Option<(usize, usize)>,
}

impl BoardView {
    pub fn new(
        board_width: usize,
        board_height: usize,
        periodic: bool,
        view_bound: Rect,
    ) -> BoardView {
        // Calculate the cell width and height
        let cell_length = min(
            view_bound.width() as usize / board_width,
            view_bound.height() as usize / board_height,
        );

        // Calculate the actual view bound
        let render_bound = Rect::from_center(
            view_bound.center(),
            (cell_length * board_width) as u32,
            (cell_length * board_height) as u32,
        );

        BoardView {
            view_bound,
            render_bound,
            cell_length,
            game_state: ConwayGameState::new(board_width, board_height, periodic),
            hovering_cell: None,
        }
    }

    pub fn step(&mut self) {
        self.game_state.step();
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        self.game_state.resize(width, height);

        self.cell_length = min(
            self.view_bound.width() as usize / width,
            self.view_bound.height() as usize / height,
        );

        self.render_bound = Rect::from_center(
            self.view_bound.center(),
            (self.cell_length * width) as u32,
            (self.cell_length * height) as u32,
        );
    }

    pub fn set_periodic(&mut self, periodic: bool) {
        self.game_state.set_is_periodic(periodic);
    }

    pub fn export_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = rfd::FileDialog::new()
            .set_directory("./")
            .add_filter("Conway's Game Save", &["conway"])
            .save_file()
            .ok_or("No file selected")?;

        let mut file = File::create(file)?;

        let width = self.game_state.get_width();
        let height = self.game_state.get_height();
        let periodic = self.game_state.get_is_periodic();

        file.write_all(format!("{} {} {}\n", width, height, periodic).as_bytes())?;

        file.write_all(
            self.game_state
                .get_cells()
                .iter()
                .map(|cell| if *cell { '1' } else { '0' })
                .collect::<String>()
                .as_bytes(),
        )?;

        Ok(())
    }

    pub fn import_from_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file = rfd::FileDialog::new()
            .set_directory("./")
            .add_filter("Conway's Game Save", &["conway"])
            .pick_file()
            .ok_or("No file selected")?;

        let file = File::open(file)?;

        let mut lines = std::io::BufReader::new(file).lines();

        let (width, height, periodic): (usize, usize, bool) = {
            let line = lines.next().ok_or("Invalid file format")??;
            let mut parts = line.split_whitespace();
            let width = parts.next().ok_or("Invalid file format")?.parse()?;
            let height = parts.next().ok_or("Invalid file format")?.parse()?;
            let periodic = parts.next().ok_or("Invalid file format")?.parse()?;
            (width, height, periodic)
        };

        // Check if the width and height are within the valid range
        if width < MIN_BOARD_SIZE
            || width > MAX_BOARD_SIZE
            || height < MIN_BOARD_SIZE
            || height > MAX_BOARD_SIZE
        {
            return Err("Invalid board size".into());
        }

        let cells: Vec<bool> = {
            let line = lines.next().ok_or("Invalid file format")??;
            line.chars()
                .map(|c| match c {
                    '0' => false,
                    '1' => true,
                    _ => false,
                })
                .collect()
        };

        // Check if the number of cells match the width and height
        if cells.len() != width * height {
            return Err("Invalid number of cells".into());
        }

        self.resize(width, height);
        self.set_periodic(periodic);
        self.game_state.set_cells(cells);

        Ok(())
    }

    pub fn clear(&mut self) {
        self.game_state.clear();
    }

    fn get_cell_index(&self, x: i32, y: i32) -> (usize, usize) {
        let cell_width = self.view_bound.width() as f32 / self.game_state.get_width() as f32;
        let cell_height = self.view_bound.height() as f32 / self.game_state.get_height() as f32;
        let x = ((x - self.view_bound.x()) as f32 / cell_width) as usize;
        let y = ((y - self.view_bound.y()) as f32 / cell_height) as usize;
        (x, y)
    }

    fn map_cells_to_rect(&self, cells: Vec<(usize, usize)>, offset: bool) -> Vec<Rect> {
        cells
            .iter()
            .map(|(x, y)| {
                if offset {
                    Rect::new(
                        (x * self.cell_length) as i32 + self.render_bound.x(),
                        (y * self.cell_length) as i32 + self.render_bound.y(),
                        self.cell_length as u32,
                        self.cell_length as u32,
                    )
                } else {
                    Rect::new(
                        (x * self.cell_length) as i32,
                        (y * self.cell_length) as i32,
                        self.cell_length as u32,
                        self.cell_length as u32,
                    )
                }
            })
            .collect()
    }

    fn render_cells(
        &self,
        cells: Vec<(usize, usize)>,
        color: Color,
        offset: bool,
        texture_target: &mut Canvas<Window>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let rects = self.map_cells_to_rect(cells, offset);
        texture_target.set_draw_color(color);
        texture_target.fill_rects(&rects)?;
        Ok(())
    }
}

impl View for BoardView {
    fn render(
        &mut self,
        canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
        texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
        _font_manager: &mut crate::engine::resource_manager::FontManager,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut texture = texture_creator.create_texture_target(
            None,
            self.render_bound.width(),
            self.render_bound.height(),
        )?;

        canvas.with_texture_canvas(&mut texture, |texture_target| {
            texture_target.set_draw_color(BOARD_BACKGROUND_COLOR);
            texture_target.clear();

            let _ = self.render_cells(
                self.game_state.get_retained_cells(),
                BOARD_CELL_COLOR,
                false,
                texture_target,
            );
            let _ = self.render_cells(
                self.game_state.get_newly_born_cells(),
                BOARD_NEWLY_ALIVE_CELL_COLOR,
                false,
                texture_target,
            );
            let _ = self.render_cells(
                self.game_state.get_newly_dead_cells(),
                BOARD_NEWLY_DEAD_CELL_COLOR,
                false,
                texture_target,
            );

            if let Some((x, y)) = self.hovering_cell {
                let color = if self.game_state.get_cell(x, y) {
                    BOARD_HOVERING_ALIVE_CELL_COLOR
                } else {
                    BOARD_HOVERING_DEAD_CELL_COLOR
                };
                let _ = self.render_cells(vec![(x, y)], color, false, texture_target);
            }
        })?;
        canvas.copy(&texture, None, self.view_bound)?;
        unsafe {
            texture.destroy();
        }
        Ok(())
    }

    fn get_bound(&self) -> Rect {
        self.view_bound
    }

    fn on_tick(&mut self) {
        self.step();
    }

    fn on_mouse_motion(&mut self, x: i32, y: i32) {
        if self.view_bound.contains_point((x, y)) {
            self.hovering_cell = Some(self.get_cell_index(x, y));
        } else {
            self.hovering_cell = None;
        }
    }

    fn on_mouse_button_down(&mut self, button: sdl2::mouse::MouseButton, x: i32, y: i32) {
        if self.view_bound.contains_point((x, y)) {
            if let sdl2::mouse::MouseButton::Left = button {
                let (x, y) = self.get_cell_index(x, y);
                self.game_state
                    .set_cell(x, y, !self.game_state.get_cell(x, y));
            }
        }
    }
}
