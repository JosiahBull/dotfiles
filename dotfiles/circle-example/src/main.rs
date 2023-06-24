use std::{
    error::Error,
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{
        canvas::{Canvas, Circle, Label},
        Block, Borders,
    },
    Terminal,
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;
    run(&mut terminal)?;
    restore_terminal(&mut terminal)?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen)?;
    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen,)?;
    Ok(terminal.show_cursor()?)
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut circle_x = 0.0;
    let mut circle_y = 0.0;

    loop {
        terminal.draw(|frame| {
            // let greeting = Paragraph::new("Hello World!");
            // frame.render_widget(greeting, frame.size());

            let canvas = Canvas::default()
                .block(Block::default().title("Canvas").borders(Borders::ALL))
                .x_bounds([-180.0, 180.0])
                .y_bounds([-90.0, 90.0])
                .paint(|ctx| {
                    /// Find the intersection points between a circle and a line
                    /// panics if there are no intersections
                    fn find_intersection(
                        circle_pos: (f64, f64),
                        circle_radius: f64,
                        line_start: (f64, f64),
                        line_end: (f64, f64),
                    ) -> (f64, f64) {
                        let dx = line_end.0 - line_start.0;
                        let dy = line_end.1 - line_start.1;

                        let a = dx * dx + dy * dy;
                        let b = 2.0
                            * (dx * (line_start.0 - circle_pos.0)
                                + dy * (line_start.1 - circle_pos.1));
                        let c = (line_start.0 - circle_pos.0) * (line_start.0 - circle_pos.0)
                            + (line_start.1 - circle_pos.1) * (line_start.1 - circle_pos.1)
                            - circle_radius * circle_radius;

                        let discriminant = b * b - 4.0 * a * c;

                        if discriminant < 0.0 {
                            panic!("No intersection points found.");
                        }

                        // if the ending point, return the starting point
                        if discriminant == 0.0 {
                            return line_start;
                        }

                        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
                        (line_start.0 + t1 * dx, line_start.1 + t1 * dy)
                    }
                    // line starts at the center of the first circle, and ends at the center of the second circle
                    let a = (circle_x, circle_y);
                    let b = (100.0, 50.0);

                    // trim the line so that it starts and ends at the edges of the circles
                    let a = find_intersection(a, 10.0, a, b);
                    let b = find_intersection(b, 10.0, b, a);

                    // draw a line between the two circles
                    ctx.draw(&ratatui::widgets::canvas::Line {
                        x1: a.0,
                        y1: a.1,
                        x2: b.0,
                        y2: b.1,
                        color: Color::Green,
                    });

                    // draw a circle with a label in the center
                    ctx.print(
                        circle_x - 2.0,
                        circle_y,
                        Line {
                            spans: vec![Span {
                                content: std::borrow::Cow::Borrowed("Docker"),
                                style: Style::default(),
                            }],
                            alignment: Some(ratatui::layout::Alignment::Left),
                        },
                    );
                    ctx.draw(&Circle {
                        x: circle_x,
                        y: circle_y,
                        radius: 10.0,
                        color: Color::Blue,
                    });

                    // draw a second circle, 100 pixels to the right of the first one
                    ctx.print(
                        100.0 - 2.0,
                        50.0,
                        Line {
                            spans: vec![Span {
                                content: std::borrow::Cow::Borrowed("Dep A"),
                                style: Style::default(),
                            }],
                            alignment: Some(ratatui::layout::Alignment::Left),
                        },
                    );
                    ctx.draw(&Circle {
                        x: 100.0,
                        y: 50.0,
                        radius: 10.0,
                        color: Color::Red,
                    });
                });

            frame.render_widget(canvas, frame.size());
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    break;
                }

                match key.code {
                    KeyCode::Left => circle_x -= 10.0,
                    KeyCode::Right => circle_x += 10.0,
                    KeyCode::Up => circle_y += 10.0,
                    KeyCode::Down => circle_y -= 10.0,
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
