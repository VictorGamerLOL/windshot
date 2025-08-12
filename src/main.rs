mod editing_area;

use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};
use editing_area::Command;
use editing_area::CommandType;

fn main() {
    let app = Application::new(Some("com.example.TestApp"), Default::default());

    app.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_default_size(800, 600);

        let editing_area = editing_area::EditingArea::new();

        let commands = vec![
            Command {
                command_type: CommandType::Rectangle { end: (200.0, 200.0) },
                start: (50.0, 50.0),
                color: 0xFF0000FF, // Red
                width: 20.0,
                fill_color: Some(0xFFFF00FF), // Yellow
            },
            Command {
                command_type: CommandType::Circle { end: (300.0, 300.0) },
                start: (150.0, 150.0),
                color: 0x00FF00FF, // Green
                width: 2.0,
                fill_color: None,
            },
            Command {
                command_type: CommandType::Line { end: (400.0, 400.0) },
                start: (100.0, 100.0),
                color: 0x0000FFFF, // Blue
                width: 2.0,
                fill_color: None,
            },
            Command {
                command_type: CommandType::Arrow { end: (500.0, 500.0) },
                start: (200.0, 200.0),
                color: 0xFFFF00FF, // Yellow
                width: 5.0,
                fill_color: None,
            },
            Command {
                command_type: CommandType::Text {
                    font: "Sans 16".to_string(),
                    text: "Hello, World!".to_string(),
                },
                start: (50.0, 300.0),
                color: 0xFFFFFFFF, // White
                width: 1.0,
                fill_color: None,
            },
            Command {
                command_type: CommandType::Freehand {
                    points: vec![(200.0, 200.0), (250.0, 250.0), (300.0, 200.0)],
                },
                start: (150.0, 150.0),
                color: 0xFF00FFFF, // Cyan
                width: 2.0,
                fill_color: None,
            },
        ];

        editing_area.set_undo_stack(editing_area::CommandsBoxed::from(commands));

        window.set_child(Some(&editing_area));
        window.set_visible(true);
    });

    app.run();
}
