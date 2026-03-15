mod chase;
mod game;
mod input;
mod models;
mod ui;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use crossterm::{
    cursor, execute,
    style::Color,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use kira::{
    AudioManager, AudioManagerSettings, DefaultBackend, Tween,
    sound::static_sound::{StaticSoundData, StaticSoundHandle},
};
use std::io::{Stdout, stdout};
use std::thread;
use std::time::Duration;

use chase::{ChaseGame, ChaseMoveResult};
use game::{Game, MoveResult};
use input::{Input, PostRoundInput};
use ui::Board;

#[derive(Parser)]
#[command(name = "cyber-path", about = "A cyberpunk terminal game")]
struct Cli {
    /// Game mode to play
    #[arg(short, long, value_enum, default_value_t = Mode::Path)]
    mode: Mode,
}

#[derive(Clone, ValueEnum)]
enum Mode {
    /// Memorize and reproduce a path
    Path,
    /// Flee from a chasing bot
    Chase,
}

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
    let cli = Cli::parse();

    let mut stdout = stdout();
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;

    terminal::enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    let _guard = TerminalGuard;

    let board = Board::from_terminal()?;

    match cli.mode {
        Mode::Path => run_path_mode(&mut stdout, &mut manager, &board),
        Mode::Chase => run_chase_mode(&mut stdout, &mut manager, &board),
    }
}

fn run_path_mode(
    stdout: &mut Stdout,
    manager: &mut AudioManager<DefaultBackend>,
    board: &Board,
) -> Result<()> {
    let mut game = Game::new(board.cols, board.rows);

    let winning_sound = StaticSoundData::from_file("assets/winning_sound.mp3")?;
    let losing_sound = StaticSoundData::from_file("assets/losing_sound.mp3")?;
    let moving_sound = StaticSoundData::from_file("assets/movement.mp3")?;
    let bot_sound = StaticSoundData::from_file("assets/bot_path.mp3")?;
    let mut move_handle: Option<StaticSoundHandle> = None;

    'game: loop {
        game.generate_path()?;

        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        board.draw_title(stdout, "CYBER PATH")?;
        board.draw_grid(stdout)?;
        board.draw_round_info(stdout, game.round, game.move_count())?;
        board.draw_status(stdout, "Watch the path...", Color::Yellow)?;

        board.animate_path(stdout, &game.path, game.preview_step_ms(), || {
            manager.play(bot_sound.clone())?;
            Ok(())
        })?;
        thread::sleep(Duration::from_millis(game.preview_hold_ms()));

        board.clear_path(stdout, &game.path)?;
        board.draw_status(
            stdout,
            "Reproduce the path! (WASD / Arrow keys)",
            Color::Yellow,
        )?;
        board.fill_cell(stdout, game.start_position(), Color::Cyan)?;

        loop {
            match input::read_input()? {
                Input::Quit => break 'game,
                Input::Move(dir) => match game.check_move(dir) {
                    MoveResult::Correct(pos) => {
                        board.fill_cell(stdout, pos, Color::Green)?;
                        if let Some(ref mut h) = move_handle {
                            h.stop(Tween::default());
                        }
                        move_handle = Some(manager.play(moving_sound.clone())?);
                    }
                    MoveResult::RoundComplete => {
                        if let Some(&last) = game.path.last() {
                            board.fill_cell(stdout, last, Color::Green)?;
                        }
                        manager.play(winning_sound.clone())?;
                        board.draw_status(
                            stdout,
                            "Correct! Press any key for the next round...",
                            Color::Green,
                        )?;
                        input::wait_for_any_key()?;
                        game.advance_round();
                        continue 'game;
                    }
                    MoveResult::Wrong => {
                        manager.play(losing_sound.clone())?;
                        show_failure(stdout, board, &game, dir)?;
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

fn run_chase_mode(
    stdout: &mut Stdout,
    manager: &mut AudioManager<DefaultBackend>,
    board: &Board,
) -> Result<()> {
    let winning_sound = StaticSoundData::from_file("assets/winning_sound.mp3")?;
    let losing_sound = StaticSoundData::from_file("assets/losing_sound.mp3")?;
    let moving_sound = StaticSoundData::from_file("assets/movement.mp3")?;
    let mut move_handle: Option<StaticSoundHandle> = None;

    let mut game = ChaseGame::new(board.cols, board.rows);

    'game: loop {
        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
        board.draw_title(stdout, "CYBER CHASE")?;
        board.draw_grid(stdout)?;
        board.fill_cell(stdout, game.player_pos, Color::Cyan)?;
        board.fill_cell(stdout, game.bot_pos, Color::Magenta)?;

        let status = format!(
            "Flee from the bot! Survive {} moves | WASD / Arrow keys",
            game.moves_to_survive()
        );
        board.draw_status(stdout, &status, Color::Yellow)?;
        board.draw_chase_info(
            stdout,
            game.round,
            game.moves_survived,
            game.moves_to_survive(),
            game.bot_step_size(),
        )?;

        loop {
            match input::read_input()? {
                Input::Quit => break 'game,
                Input::Move(dir) => match game.move_player(dir) {
                    ChaseMoveResult::InvalidMove => continue,
                    ChaseMoveResult::Moved {
                        old_player,
                        old_bot,
                    } => {
                        board.clear_cell(stdout, old_player)?;
                        board.clear_cell(stdout, old_bot)?;
                        board.fill_cell(stdout, game.player_pos, Color::Cyan)?;
                        board.fill_cell(stdout, game.bot_pos, Color::Magenta)?;
                        if let Some(ref mut h) = move_handle {
                            h.stop(Tween::default());
                        }
                        move_handle = Some(manager.play(moving_sound.clone())?);
                        board.draw_chase_info(
                            stdout,
                            game.round,
                            game.moves_survived,
                            game.moves_to_survive(),
                            game.bot_step_size(),
                        )?;
                    }
                    ChaseMoveResult::Caught {
                        old_player,
                        old_bot,
                    } => {
                        board.clear_cell(stdout, old_player)?;
                        board.clear_cell(stdout, old_bot)?;
                        board.fill_cell(stdout, game.player_pos, Color::Red)?;
                        if game.bot_pos != game.player_pos {
                            board.fill_cell(stdout, game.bot_pos, Color::Magenta)?;
                        }
                        manager.play(losing_sound.clone())?;
                        let msg = format!(
                            "Caught! Round {} over. (R)estart or (Q)uit",
                            game.round
                        );
                        board.draw_status(stdout, &msg, Color::Red)?;
                        match input::read_post_round()? {
                            PostRoundInput::PlayAgain => {
                                game.restart();
                                continue 'game;
                            }
                            PostRoundInput::Quit => break 'game,
                        }
                    }
                    ChaseMoveResult::Survived {
                        old_player,
                        old_bot,
                    } => {
                        board.clear_cell(stdout, old_player)?;
                        board.clear_cell(stdout, old_bot)?;
                        board.fill_cell(stdout, game.player_pos, Color::Green)?;
                        manager.play(winning_sound.clone())?;
                        board.draw_status(
                            stdout,
                            "You survived! Press any key for the next round...",
                            Color::Green,
                        )?;
                        input::wait_for_any_key()?;
                        game.advance_round();
                        continue 'game;
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
