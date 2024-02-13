use ratatui::widgets::{canvas::Canvas, canvas::Rectangle};
use ratatui::{prelude::*, widgets::*};

use crate::game::{GameState, SnakeGameViewModel};

pub fn snake_screen(frame: &mut Frame, model: &SnakeGameViewModel) {
    let size = frame.size();
    let snake_cells = model.snake.iter().cloned().collect::<Vec<_>>();
    let head_cell = model.head;
    let dot_cell = model.dot;

    let popup_size = Rect {
        x: size.width / 6,
        y: size.height / 12,
        width: size.width / 3 * 2,
        height: size.height / 3 * 2,
    };
    let shortcuts_size = Rect {
        x: 0,
        y: size.height / 9 * 8,
        width: size.width,
        height: size.height / 9,
    };
    let score_string = format!("Score: {}", model.score);
    let pause_message = Span::raw("Press <SpaceBar> to Pause / Unpause.");
    let move_instructions = Span::raw("To move, use the arrow keys. Alternatively, use 'k'/'j' for up/down, and 'h'/'l' for left and right.");
    let speed_up = Span::raw(
        "To temporarily speed up the snake, enter Ctrl+<Direction> to move twice as fast.",
    );

    let text = vec![
        Line::from(pause_message),
        Line::from(move_instructions),
        Line::from(speed_up),
    ];

    let game_instructions = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    // Create a canvas for the game grid
    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Snake Game")
                .title_bottom(score_string.as_str()),
        )
        .paint(|ctx| {
            // Draw the snake body
            for &cell in &snake_cells {
                ctx.draw(&Rectangle {
                    x: cell.0 as f64,
                    y: cell.1 as f64,
                    width: 1.0,
                    height: 1.0,
                    color: Color::Green,
                });
            }
            // Draw the snake head
            ctx.draw(&Rectangle {
                x: head_cell.0 as f64,
                y: head_cell.1 as f64,
                width: 1.0,
                height: 1.0,
                color: Color::Green,
            });
            // Paint the dot
            ctx.draw(&Rectangle {
                x: dot_cell.0 as f64,
                y: dot_cell.1 as f64,
                width: 1.0,
                height: 1.0,
                color: Color::White,
            });
        })
        .x_bounds([0.0, 50.0])
        .y_bounds([0.0, 50.0]);

    // Check the game state and render accordingly
    match model.state {
        GameState::Playing => {
            frame.render_widget(canvas, popup_size);
            frame.render_widget(game_instructions, shortcuts_size);
        }
        GameState::Won | GameState::Lost => game_over_screen(frame, model, size),
        GameState::Paused => pause_screen(frame, model, size),
    }
}

fn pause_screen(frame: &mut Frame, model: &SnakeGameViewModel, size: Rect) {
    let pause_message = Span::raw("Game Paused. Press <SpaceBar> to unpause.");
    let score_message = Span::raw(format!("Current Score: {}", model.score));

    let text = vec![Line::from(pause_message), Line::from(score_message)];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Paused"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    // Calculate the center position for the paragraph
    // Assuming `size` is the size of your terminal or the area where you want to render the widget
    let popup_size = Rect {
        x: size.width / 4,
        y: size.height / 4,
        width: size.width / 2,
        height: size.height / 2,
    };

    frame.render_widget(paragraph, popup_size);
}

fn game_over_screen(frame: &mut Frame, model: &SnakeGameViewModel, size: Rect) {
    let message = match model.state {
        GameState::Won => "Congratulations! You won!",
        GameState::Lost => "Game Over! Try again!",
        _ => unreachable!(),
    };
    let game_over_message = Span::raw(message);
    let score_message = Span::raw(format!("Final Score: {}", model.score));
    let info_message = Span::raw("Press 'n' to play a new game, or 'q' to quit");

    let text = vec![
        Line::from(game_over_message),
        Line::from(score_message),
        Line::from(info_message),
    ];

    let paragraph = Paragraph::new(text)
        .block(Block::default().borders(Borders::ALL).title("Game Over"))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    // Calculate the center position for the paragraph
    // Assuming `size` is the size of your terminal or the area where you want to render the widget
    let popup_size = Rect {
        x: size.width / 4,
        y: size.height / 4,
        width: size.width / 2,
        height: size.height / 2,
    };

    frame.render_widget(paragraph, popup_size);
}
