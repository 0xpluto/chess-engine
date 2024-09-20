use chess_engine::gui::{BoardState, BOARD_SIZE};
use ggez::{event::run, GameResult};



fn main() -> GameResult {
    let (ctx, event_loop) = ggez::ContextBuilder::new("chess", "Your Name")
        .window_setup(ggez::conf::WindowSetup::default().title("Chess"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(BOARD_SIZE, BOARD_SIZE))
        .build()?;

    let state = BoardState::new();
    run(ctx, event_loop, state)
}
