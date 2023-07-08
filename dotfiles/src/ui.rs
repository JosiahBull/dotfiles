use std::collections::HashMap;

use ratatui::{
    backend::Backend,
    style::Color,
    widgets::{
        canvas::{Canvas, Circle, Line},
        Block, Borders,
    },
    Frame,
};

use crate::app::App;

// there are several screens to render, which we will switch between
// - Explanation of what the app does, list of actions
//   - Install
//   - Update // TODO
//   - Remove // TODO
//   - Quit
// - Install screen
//   - List of dependencies, select dependencies to disable/what's already installed
//   - Install button
//   - Back button

/// Find the intersection point between a circle and a line, picks the intersection that will reduce
/// the length of the line the most.
fn find_intersection(
    circle_pos: (f64, f64),
    circle_radius: f64,
    line_start: (f64, f64),
    line_end: (f64, f64),
) -> Option<(f64, f64)> {
    let dx = line_end.0 - line_start.0;
    let dy = line_end.1 - line_start.1;

    let a = dx * dx + dy * dy;
    let b = 2.0 * (dx * (line_start.0 - circle_pos.0) + dy * (line_start.1 - circle_pos.1));
    let c = (line_start.0 - circle_pos.0) * (line_start.0 - circle_pos.0)
        + (line_start.1 - circle_pos.1) * (line_start.1 - circle_pos.1)
        - circle_radius * circle_radius;

    let discriminant = b * b - 4.0 * a * c;

    if discriminant <= 0.0 {
        return None;
    }

    let t1 = (-b + discriminant.sqrt()) / (2.0 * a);
    Some((line_start.0 + t1 * dx, line_start.1 + t1 * dy))
}

#[cfg(test)]
mod test_find_intersection {
    #[test]
    fn test_find_intersection() {
        let circle_pos = (0.0, 0.0);
        let circle_radius = 1.0;
        let line_start = (0.0, 0.0);
        let line_end = (0.0, 3.0);

        let intersection =
            super::find_intersection(circle_pos, circle_radius, line_start, line_end).unwrap();
        assert_eq!(intersection, (0.0, 1.0));
    }

    // should panic if no intersection is found
    #[test]
    #[should_panic]
    fn test_find_intersection_no_intersection_outside() {
        let circle_pos = (0.0, 0.0);
        let circle_radius = 1.0;
        let line_start = (2.0, 2.0);
        let line_end = (2.0, 2.5);

        super::find_intersection(circle_pos, circle_radius, line_start, line_end);
    }
}

const CIRCLE_DISTANCE: f64 = 100.0;
const CIRCLE_RADIUS: f64 = 20.0;
const COL_SEPARATION: f64 = 100.0;

/// Renders the user interface widgets.
pub fn render<B: Backend>(app: &mut App, frame: &mut Frame<'_, B>) {
    let (view_x, view_y) = app.view_loc;

    let canvas = Canvas::default()
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Dependency Graph"),
        )
        .x_bounds([0.0, 1000.0])
        .y_bounds([0.0, 500.0])
        .paint(|ctx| {
            // The goal is to calculate all the positions of the nodes in the graph, we do this by first constructing a dependency graph
            // then we can use that graph to draw onto a canvas.

            let mut dependency_map = HashMap::new();

            // For each dependency in each column, draw a circle
            for (col_index, column) in app.dependency_view.iter().enumerate() {
                for (row_index, dependency) in column.iter().enumerate() {
                    let x = (col_index as f64 * COL_SEPARATION) + CIRCLE_DISTANCE;
                    let y = (row_index as f64 * CIRCLE_DISTANCE) + CIRCLE_DISTANCE;

                    ctx.draw(&Circle {
                        x: x + view_x,
                        y: y + view_y,
                        radius: CIRCLE_RADIUS,
                        color: Color::White,
                    });

                    ctx.print(x - 5.0 + view_x, y - 5.0 + view_y, dependency.name().to_owned());

                    dependency_map.insert(dependency.name().to_owned(), (x, y));
                }
            }

            // If there is a dependency between two nodes, draw a line between them
            // FIXME: If two dependencies are in a line, the line will be drawn over the intermediate circle
            for (_, column) in app.dependency_view.iter().enumerate() {
                for (_, dependency) in column.iter().enumerate() {
                    // get children of this dependency
                    let children = dependency.children();

                    // for each child, draw a line between this dependency and the child
                    for child in children {
                        let start_point = *dependency_map.get(dependency.name()).unwrap();
                        let end_point = *dependency_map.get(child.name()).unwrap();

                        let start_point =
                            find_intersection(start_point, CIRCLE_RADIUS, start_point, end_point)
                                .unwrap();

                        let end_point =
                            find_intersection(end_point, CIRCLE_RADIUS, end_point, start_point)
                                .unwrap();

                        ctx.draw(&Line {
                            x1: start_point.0 + view_x,
                            y1: start_point.1 + view_y,
                            x2: end_point.0 + view_x,
                            y2: end_point.1 + view_y,
                            color: Color::White,
                        })
                    }
                }
            }
        });

    frame.render_widget(canvas, frame.size());
}
