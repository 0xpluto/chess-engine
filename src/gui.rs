use ggez::{event::{EventHandler, MouseButton}, glam::Vec2, graphics::{self, Canvas, Color, DrawParam, Rect}, input::keyboard::{KeyCode, KeyInput}, Context, GameResult};

use crate::{assets::AssetManager, board::Board, coords::{Coords, MoveCoords}, pieces::PieceType};

pub struct BoardState {
    selected_square: Option<Coords>,
    pub board: Board,
    assets: AssetManager,
}

const GRID_CELL_SIZE: f32 = 100.0;
pub const BOARD_SIZE: f32 = GRID_CELL_SIZE * 8.0;

impl BoardState {
    pub fn new() -> Self {
        let assets = AssetManager::new(std::path::PathBuf::from("assets/chess"));
        BoardState { selected_square: None, board: Board::new(), assets }
    }

    fn get_square_at(&self, x: f32, y: f32) -> Option<Coords> {
        let col = (x / GRID_CELL_SIZE) as u8;
        let row = (y / GRID_CELL_SIZE) as u8;
        if col < 8 && row < 8 {
            Some((col, row).into())
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
        for row in 0..8 {
            for col in 0..8 {
                let piece = self.board.get_piece((col, row));
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

                let coords: Coords = (col, row).into();
                let row = coords.row();
                let col = coords.col();
                let scale = Vec2::new(1.25, 1.25);
                let notation_offset = 15.0;
                if row == 8 {
                    canvas.draw(
                        &graphics::Text::new(format!("{}", col)),
                        DrawParam::new()
                            .dest(Vec2::new(rect.x + GRID_CELL_SIZE - notation_offset, rect.y + GRID_CELL_SIZE - notation_offset))
                            .scale(scale)
                            .color(Color::BLACK),
                    );
                }
                if col == 'a' {
                    canvas.draw(
                        &graphics::Text::new(format!("{}", 9 - row)),
                        DrawParam::new()
                            .dest(Vec2::new(rect.x + notation_offset, rect.y + notation_offset))
                            .scale(scale)
                            .color(Color::BLACK),
                    );
                }

                // Draw chess piece
                if piece.piece_type == PieceType::Empty {
                    continue;
                }
                let image = self.assets.image(&piece, ctx);
                canvas.draw(
                    &image,
                    DrawParam::new()
                        .dest(Vec2::new(rect.x + GRID_CELL_SIZE / 2.0, rect.y + GRID_CELL_SIZE / 2.0))
                        .scale(Vec2::new(3.7f32, 3.7f32))
                        .offset(Vec2::new(0.5, 0.5)),
                );
            }
        }

        // Highlight selected square
        if let Some(coords) = self.selected_square {
            let col = coords.x as f32;
            let row = coords.y as f32;
            let highlight_rect = Rect::new(
                col * GRID_CELL_SIZE,
                row * GRID_CELL_SIZE,
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

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        input: KeyInput,
        _repeated: bool,
    ) -> GameResult {
        match input.keycode {
            Some(KeyCode::Q) => ctx.request_quit(),
            Some(KeyCode::R) => self.board = Board::new(),
            _ => (),
        }
        if input.keycode == Some(KeyCode::Escape) {
            ctx.request_quit();
        }

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> GameResult {
        let selected = self.get_square_at(x, y);
        if selected.is_none() {
            println!("Clicked outside the board");
        }
        println!("Clicked: {}, x: {}, y: {}", selected.unwrap(), selected.unwrap().x, selected.unwrap().y);
        match (button, self.selected_square, selected) {
            (MouseButton::Left, Some(from), Some(to)) => {
                // Clicked on the same square
                if from == to {
                    self.selected_square = None;
                    return Ok(());
                }
                // First click was on a piece and second click on different
                if self.board.movable_piece(from) {
                    let promotion = if self.board.is_promote(from, to) {
                        Some(PieceType::Queen)
                    } else {
                        None
                    };
                    
                    let moved = self.board.move_piece(from, to, promotion);
                    println!("Valid move? {:?}", moved);
                    self.selected_square = None;
                } else {
                    self.selected_square = Some(to);
                }

            }
            _ => self.selected_square = selected,
        }
        // if button == MouseButton::Left {
        //     let selected = self.get_square_at(x, y);
        //     if let Some(from) = self.selected_square {
        //         let present = !self.board.is_empty(from);
        //         if let Some(to) = selected {
        //             let moved = self.board.move_piece(from, to, Some(PieceType::Queen));
        //             println!("{:?}", moved);
        //             self.selected_square = None;
        //         }
        //     }
        //     self.selected_square = self.get_square_at(x, y);
        // }
        Ok(())
    }
}
