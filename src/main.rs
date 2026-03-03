mod game;
mod input;
mod models;
mod ui;

use anyhow::Result;
use crossterm::{
    cursor, execute,
    style::Color,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{Stdout, stdout};
use std::thread;
use std::time::Duration;

use game::{Game, MoveResult};
use input::{Input, PostRoundInput};
use ui::Board;

/// Restores terminal state on drop, even during panics.
struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let _ = terminal::disable_raw_mode();
        let _ = execute!(stdout(), LeaveAlternateScreen, cursor::Show);
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut stdout = stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    let _guard = TerminalGuard;

    let board = Board::from_terminal()?;
    let mut game = Game::new(board.cols, board.rows);

    'game: loop {
        game.generate_path()?;

        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        board.draw_title(&mut stdout)?;
        board.draw_grid(&mut stdout)?;
        board.draw_round_info(&mut stdout, game.round, game.move_count())?;
        board.draw_status(&mut stdout, "Watch the path...", Color::Yellow)?;

        board.animate_path(&mut stdout, &game.path, game.preview_step_ms())?;
        thread::sleep(Duration::from_millis(game.preview_hold_ms()));

        board.clear_path(&mut stdout, &game.path)?;
        board.draw_status(
            &mut stdout,
            "Reproduce the path! (WASD / Arrow keys)",
            Color::Yellow,
        )?;
        board.fill_cell(&mut stdout, game.start_position(), Color::Cyan)?;

        loop {
            match input::read_input()? {
                Input::Quit => break 'game,
                Input::Move(dir) => match game.check_move(dir) {
                    MoveResult::Correct(pos) => {
                        board.fill_cell(&mut stdout, pos, Color::Green)?;
                    }
                    MoveResult::RoundComplete => {
                        if let Some(&last) = game.path.last() {
                            board.fill_cell(&mut stdout, last, Color::Green)?;
                        }
                        board.draw_status(
                            &mut stdout,
                            "Correct! Press any key for the next round...",
                            Color::Green,
                        )?;
                        input::wait_for_any_key()?;
                        game.advance_round();
                        continue 'game;
                    }
                    MoveResult::Wrong => {
                        show_failure(&mut stdout, &board, &game, dir)?;
                        match input::read_post_round()? {
                            PostRoundInput::PlayAgain => {
                                game = Game::new(board.cols, board.rows);
                                continue 'game;
                            }
                            PostRoundInput::Quit => break 'game,
                        }
                    }
                },
            }
        }
    }

    Ok(())
}

fn show_failure(
    stdout: &mut Stdout,
    board: &Board,
    game: &Game,
    dir: models::Direction,
) -> Result<()> {
    let current = game.path[game.player_index];
    if let Some(wrong) = current.neighbor(dir)
        && wrong.x < game.cols
        && wrong.y < game.rows
    {
        board.fill_cell(stdout, wrong, Color::Red)?;
    }
    for &pos in game.remaining_path() {
        board.fill_cell(stdout, pos, Color::DarkYellow)?;
    }
    let msg = format!("Wrong! Round {} over. (R)estart or (Q)uit", game.round);
    board.draw_status(stdout, &msg, Color::Red)?;
    Ok(())
}
