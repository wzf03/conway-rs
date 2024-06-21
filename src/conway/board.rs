use sdl2::{pixels::Color, rect::Rect};

use crate::engine::view::View;

use super::game_state::ConwayGameState;

const BOARD_BACKGROUND_COLOR: Color = Color::RGB(0, 0, 0);
const BOARD_CELL_COLOR: Color = Color::RGB(255, 255, 255);
const BOARD_HOVERING_ALIVE_CELL_COLOR: Color = Color::RGB(255, 0, 0);
const BOARD_HOVERING_DEAD_CELL_COLOR: Color = Color::RGB(0, 255, 0);

pub struct BoardView {
    view_bound: Rect,
    game_state: ConwayGameState,
    hovering_cell: Option<(usize, usize)>,
}

impl BoardView {
    pub fn new(board_width: usize, board_height: usize, view_bound: Rect) -> BoardView {
        BoardView {
            view_bound,
            game_state: ConwayGameState::new(board_width, board_height, true),
            hovering_cell: None,
        }
    }

    pub fn step(&mut self) {
        self.game_state.step();
    }

    fn get_cell_index(&self, x: i32, y: i32) -> (usize, usize) {
        let cell_width = self.view_bound.width() as f32 / self.game_state.get_width() as f32;
        let cell_height = self.view_bound.height() as f32 / self.game_state.get_height() as f32;
        let x = ((x - self.view_bound.x()) as f32 / cell_width) as usize;
        let y = ((y - self.view_bound.y()) as f32 / cell_height) as usize;
        (x, y)
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
            self.view_bound.width(),
            self.view_bound.height(),
        )?;

        canvas.with_texture_canvas(&mut texture, |texture_target| {
            texture_target.set_draw_color(BOARD_BACKGROUND_COLOR);
            texture_target.clear();
            let cell_width = self.view_bound.width() as f32 / self.game_state.get_width() as f32;
            let cell_height = self.view_bound.height() as f32 / self.game_state.get_height() as f32;
            for x in 0..self.game_state.get_width() {
                for y in 0..self.game_state.get_height() {
                    if self.game_state.get_cell(x, y) {
                        texture_target.set_draw_color(BOARD_CELL_COLOR);
                        let _ = texture_target.fill_rect(Rect::new(
                            (x as f32 * cell_width) as i32,
                            (y as f32 * cell_height) as i32,
                            cell_width as u32,
                            cell_height as u32,
                        ));
                    }
                }
            }

            if let Some((x, y)) = self.hovering_cell {
                if self.game_state.get_cell(x, y) {
                    texture_target.set_draw_color(BOARD_HOVERING_ALIVE_CELL_COLOR);
                } else {
                    texture_target.set_draw_color(BOARD_HOVERING_DEAD_CELL_COLOR);
                }
                let _ = texture_target.fill_rect(Rect::new(
                    (x as f32 * cell_width) as i32,
                    (y as f32 * cell_height) as i32,
                    cell_width as u32,
                    cell_height as u32,
                ));
            }
        })?;
        canvas.copy(&texture, None, self.view_bound)?;
        Ok(())
    }

    fn get_bound(&self) -> Rect {
        self.view_bound
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
