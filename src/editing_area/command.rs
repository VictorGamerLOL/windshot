use std::{f32::consts::SQRT_2, ops::{Deref, DerefMut}};

use gtk4::{prelude::{SnapshotExt, WidgetExt}, Widget};

use glib::object::IsA;

#[derive(Debug, Clone, Default, PartialEq, glib::Boxed)]
#[boxed_type(name = "CommandsBoxed")]
pub struct CommandsBoxed(Vec<Command>);

impl Deref for CommandsBoxed {
    type Target = Vec<Command>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for CommandsBoxed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<CommandsBoxed> for Vec<Command> {
    fn from(commands: CommandsBoxed) -> Self {
        commands.0
    }
}

impl From<Vec<Command>> for CommandsBoxed {
    fn from(commands: Vec<Command>) -> Self {
        CommandsBoxed(commands)
    }
}

impl AsRef<[Command]> for CommandsBoxed {
    fn as_ref(&self) -> &[Command] {
        &self.0
    }
}

impl AsMut<[Command]> for CommandsBoxed {
    fn as_mut(&mut self) -> &mut [Command] {
        &mut self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    pub command_type: CommandType,
    pub start: (f32, f32),
    pub color: u32,
    pub width: f32,
    pub fill_color: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum CommandType {
    Rectangle { end: (f32, f32) },
    Circle { end: (f32, f32) },
    Line { end: (f32, f32) },
    Arrow { end: (f32, f32) },
    Text { font: String, text: String },
    Freehand { points: Vec<(f32, f32)> },
}

impl Command {
    pub fn draw_to_snapshot(&self, snapshot: &gtk4::Snapshot, widget: impl IsA<Widget>) {
        let path_builder = gsk4::PathBuilder::new();

        snapshot.save();
        match &self.command_type {
            CommandType::Rectangle { end } => self.path_rectangle(&path_builder, *end),
            CommandType::Line { end } => self.path_line(&path_builder, *end),
            CommandType::Arrow { end } => self.path_arrow(&path_builder, *end),
            CommandType::Freehand { points } => self.path_freehand(&path_builder, points),
            CommandType::Text { font , text} => 
                self.path_text(&path_builder, snapshot, widget, font, text),
            CommandType::Circle { end} => self.path_circle(&path_builder, *end),
        }

        let path = path_builder.to_path();

        if let Some(fill_color) = self.fill_color {
            // color is RGBA, get each color as f32
            let red: f32 = (((fill_color >> 24) & 0xFF) as f32) / 255.0;
            let green: f32 = (((fill_color >> 16) & 0xFF) as f32) / 255.0;
            let blue: f32 = (((fill_color >> 8) & 0xFF) as f32) / 255.0;
            let alpha: f32 = ((fill_color & 0xFF) as f32) / 255.0;

            let color = gdk4::RGBA::new(red, green, blue, alpha);
            snapshot.append_fill(&path, gsk4::FillRule::Winding, &color);
        }
        let stroke = gsk4::Stroke::new(self.width);
        // color is RGBA, get each color as f32
        let red: f32 = (((self.color >> 24) & 0xFF) as f32) / 255.0;
        let green: f32 = (((self.color >> 16) & 0xFF) as f32) / 255.0;
        let blue: f32 = (((self.color >> 8) & 0xFF) as f32) / 255.0;
        let alpha: f32 = ((self.color & 0xFF) as f32) / 255.0;

        let color = gdk4::RGBA::new(red, green, blue, alpha);
        snapshot.append_stroke(&path, &stroke, &color);

        snapshot.restore();
    }

    pub(crate) fn path_rectangle(&self, path_builder: &gsk4::PathBuilder, end: (f32, f32)) {
        let (end_x, end_y) = end;
        let width = end_x - self.start.0;
        let height = end_y - self.start.1;
        // graphene::Rect::new takes f32, so cast if needed
        let rect = graphene::Rect::new(self.start.0, self.start.1, width, height);
        path_builder.add_rect(&rect);
    }

    pub(crate) fn path_line(&self, path_builder: &gsk4::PathBuilder, end: (f32, f32)) {
        let (end_x, end_y) = end;
        path_builder.move_to(self.start.0, self.start.1);
        path_builder.line_to(end_x, end_y);
    }

    pub(crate) fn path_arrow(&self, path_builder: &gsk4::PathBuilder, end: (f32, f32)) {
        pub(crate) const HEAD_LENGTH: f32 = 15.0;
        pub(crate) const HEAD_ANGLE: f32 = 90.0;
        let (start_x, start_y) = self.start;
        let (end_x, end_y) = end;

        let direction = (end_x - start_x, end_y - start_y);

        let direction_magnitude = (direction.0.powi(2) + direction.1.powi(2)).sqrt();

        let normalized_direction = (
            direction.0 / direction_magnitude,
            direction.1 / direction_magnitude,
        );

        let head_base = (
            end_x - normalized_direction.0 * HEAD_LENGTH,
            end_y - normalized_direction.1 * HEAD_LENGTH,
        );

        let perpendicular_direction = (
            -normalized_direction.1,
            normalized_direction.0,
        );

        let half_width = HEAD_LENGTH * f32::tan((HEAD_ANGLE / 2.0).to_radians());

        let head_left = (
            head_base.0 + half_width * perpendicular_direction.0,
            head_base.1 + half_width * perpendicular_direction.1
        );
        let head_right = (
            head_base.0 - half_width * perpendicular_direction.0,
            head_base.1 - half_width * perpendicular_direction.1
        );

        path_builder.move_to(start_x, start_y);
        path_builder.line_to(end_x, end_y);

        path_builder.move_to(head_left.0, head_left.1);
        path_builder.line_to(end_x, end_y);
        path_builder.line_to(head_right.0, head_right.1);
        // path_builder.close();
        path_builder.move_to(end_x, end_y);
    }

    pub(crate) fn path_freehand(&self, path_builder: &gsk4::PathBuilder, points: &[(f32, f32)]) {
        path_builder.move_to(self.start.0, self.start.1);
        for (x, y) in points {
            path_builder.line_to(*x, *y);
        }
    }

    pub(crate) fn path_text(&self, path_builder: &gsk4::PathBuilder, snapshot: &gtk4::Snapshot, widget: impl IsA<Widget>, font: &str, text: &str) {
        let pango_context = widget.pango_context();
        let font_description = pango::FontDescription::from_string(font);
        let lang = pango::Language::default();
        pango_context.load_fontset(&font_description, &lang);

        let layout = pango::Layout::new(&pango_context);
        layout.set_text(text);

        path_builder.add_layout(&layout);
        let translate_point = graphene::Point::new(self.start.0, self.start.1);
        snapshot.translate(&translate_point);
    }

    pub(crate) fn path_circle(&self, path_builder: &gsk4::PathBuilder, end: (f32, f32)) {
        let (end_x, end_y) = end;

        let start_x = self.start.0;
        let start_y = self.start.1;

        let left = start_x.min(end_x);
        let right = start_x.max(end_x);
        let top = start_y.min(end_y);
        let bottom = start_y.max(end_y);

        let center_x = (left + right) / 2.0;
        let center_y = (top + bottom) / 2.0;
        let radius_x: f32 = (right - left) / 2.0;
        let radius_y: f32 = (bottom - top) / 2.0;


        // Magic number gemini told me
        pub(crate) const KAPPA: f32 = (4.0 / 3.0) * (SQRT_2 - 1.0);

        let control_dx = radius_x * KAPPA;
        let control_dy = radius_y * KAPPA;
    
        path_builder.move_to(right, center_y);

        // I need to use cubic beziers instead of conics
        // because when this will eventually be converted
        // to a cairo path, conics are not supported
        // by cairo and will approximate them with cubic
        // beziers. So it is better the user sees
        // the approximation in the editing area,
        // rather than seeing perfect conics and
        // then seeing them approximated in the final
        // image.
        path_builder.cubic_to(
            right, center_y - control_dy,
            center_x + control_dx, top,
            center_x, top,
        );
        path_builder.cubic_to(
            center_x - control_dx, top,
            left, center_y - control_dy,
            left, center_y,
        );
        path_builder.cubic_to(
            left, center_y + control_dy,
            center_x - control_dx, bottom,
            center_x, bottom,
        );
        path_builder.cubic_to(
            center_x + control_dx, bottom,
            right, center_y + control_dy,
            right, center_y,
        );

        path_builder.close();
    }
}
