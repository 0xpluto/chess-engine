use ggez::{event::{EventHandler, MouseButton}, glam::Vec2, graphics::{self, Canvas, Color, DrawParam, Rect}, Context, GameResult};

use crate::{assets::AssetManager, board::Board, pieces::PieceType};

pub struct BoardState {
    selected_square: Option<(i8, i8)>,
    board: Board,
    assets: AssetManager,
}

const GRID_CELL_SIZE: f32 = 100.0;
pub const BOARD_SIZE: f32 = GRID_CELL_SIZE * 8.0;

impl BoardState {
    pub fn new() -> Self {
        let assets = AssetManager::new(std::path::PathBuf::from("assets/chess"));
        BoardState { selected_square: None, board: Board::new(), assets }
    }

    fn get_square_at(&self, x: f32, y: f32) -> Option<(i8, i8)> {
        let col = (x / GRID_CELL_SIZE) as i8;
        let row = (y / GRID_CELL_SIZE) as i8;
        if col >= 0 && col < 8 && row >= 0 && row < 8 {
            Some((row, col))
        } else {
            None
        }
    }
}

impl EventHandler for BoardState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        for (row, rows) in self.board.state.iter().enumerate() {
            for (col, piece) in rows.iter().enumerate() {
                let color: Color = if (row + col) % 2 == 0 {
                    graphics::Color::from_rgb(240, 217, 181)
                } else {
                    graphics::Color::from_rgb(181, 136, 99)
                };

                let rect = Rect::new(
                    col as f32 * GRID_CELL_SIZE,
                    row as f32 * GRID_CELL_SIZE,
                    GRID_CELL_SIZE,
                    GRID_CELL_SIZE,
                );

                canvas.draw(
                    &graphics::Quad,
                    DrawParam::new()
                        .dest(Vec2::new(rect.x, rect.y))
                        .scale(Vec2::new(rect.w, rect.h))
                        .color(color),
                );

                // Draw chess piece
                if piece.piece_type == PieceType::Empty {
                    continue;
                }
                let image = self.assets.image(piece, ctx);
                canvas.draw(
                    &image,
                    DrawParam::new()
                        .dest(Vec2::new(rect.x + GRID_CELL_SIZE / 2.0, rect.y + GRID_CELL_SIZE / 2.0))
                        .scale(Vec2::new(4f32, 4f32))
                        .offset(Vec2::new(0.5, 0.5)),
                );
            }
        }

        // Highlight selected square
        if let Some((row, col)) = self.selected_square {
            let highlight_rect = Rect::new(
                col as f32 * GRID_CELL_SIZE,
                row as f32 * GRID_CELL_SIZE,
                GRID_CELL_SIZE,
                GRID_CELL_SIZE,
            );

            canvas.draw(
                &graphics::Quad,
                DrawParam::new()
                    .dest(Vec2::new(highlight_rect.x, highlight_rect.y))
                    .scale(Vec2::new(highlight_rect.w, highlight_rect.h))
                    .color(graphics::Color::from_rgba(255, 0, 0, 100)),
            );
        }
        canvas.finish(ctx)
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        if button == MouseButton::Left {

            self.selected_square = self.get_square_at(x, y);
        }
        Ok(())
    }
}
