use std::ops::{Deref, DerefMut};

use glib::object::IsA;
use gtk4::prelude::{SnapshotExt, WidgetExt};

#[derive(Debug, Clone, Copy)]
pub struct Selection {
    start: (f32, f32),
    end: (f32, f32),
}

pub enum DragHandle {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Top, 
    Bottom,
    Left,
    Right,
}

impl Selection {
    pub const fn new(start: (f32, f32), end: (f32, f32)) -> Self {
        Selection { start, end }
    }

    pub const fn new_from_size(
        start: (f32, f32),
        width: f32,
        height: f32,
    ) -> Self {
        let end = (start.0 + width, start.1 + height);
        Selection { start, end }
    }

    pub const fn start(&self) -> (f32, f32) {
        self.start
    }
    pub const fn end(&self) -> (f32, f32) {
        self.end
    }
    pub const fn set_start(&mut self, start: (f32, f32)) {
        self.start = start;
    }
    pub const fn set_end(&mut self, end: (f32, f32)) {
        self.end = end;
    }

    /// Returns the top-left corner of the selection.
    pub const fn top_left(&self) -> (f32, f32) {
        let top = self.start.1.min(self.end.1);
        let left = self.start.0.min(self.end.0);
        (left, top)
    }
    /// Returns the top-right corner of the selection.
    pub const fn top_right(&self) -> (f32, f32) {
        let top = self.start.1.min(self.end.1);
        let right = self.start.0.max(self.end.0);
        (right, top)
    }
    /// Returns the bottom-left corner of the selection.
    pub const fn bottom_left(&self) -> (f32, f32) {
        let bottom = self.start.1.max(self.end.1);
        let left = self.start.0.min(self.end.0);
        (left, bottom)
    }    
    /// Returns the bottom-right corner of the selection.
    pub const fn bottom_right(&self) -> (f32, f32) {
        let bottom = self.start.1.max(self.end.1);
        let right = self.start.0.max(self.end.0);
        (right, bottom)
    }

    /// Returns the width of the selection.
    pub const fn width(&self) -> f32 {
        (self.end.0 - self.start.0).abs()
    }
    /// Returns the height of the selection.
    pub const fn height(&self) -> f32 {
        (self.end.1 - self.start.1).abs()
    }
    /// Returns the size of the selection as a tuple (width, height).
    pub const fn size(&self) -> (f32, f32) {
        (self.width(), self.height())
    }

    /// Returns the center of the selection (x, y).
    pub const fn center(&self) -> (f32, f32) {
        let center_x = (self.start.0 + self.end.0) / 2.0;
        let center_y = (self.start.1 + self.end.1) / 2.0;
        (center_x, center_y)
    }
    /// Alias for [`Self::center`] for the British.
    pub const fn centre(&self) -> (f32, f32) {
        self.center()
    }

    /// Returns the y coordinate of the top edge of the selection.
    pub const fn top(&self) -> f32 {
        self.start.1.min(self.end.1)
    }
    /// Returns the y coordinate of the bottom edge of the selection.
    pub const fn bottom(&self) -> f32 {
        self.start.1.max(self.end.1)
    }
    /// Returns the x coordinate of the left edge of the selection.
    pub const fn left(&self) -> f32 {
        self.start.0.min(self.end.0)
    }
    /// Returns the x coordinate of the right edge of the selection.
    pub const fn right(&self) -> f32 {
        self.start.0.max(self.end.0)
    }

    // const fn corner_swap(&mut self) {
    //     std::mem::swap(&mut self.start.0, &mut self.end.0);
    //     std::mem::swap(&mut self.start.1, &mut self.end.1);
    // }
    const fn mirror_x(&mut self) {
        std::mem::swap(&mut self.start.0, &mut self.end.0);
    }
    const fn mirror_y(&mut self) {
        std::mem::swap(&mut self.start.1, &mut self.end.1);
    }
    /// Mirrors the selection along the x-axis if `x` is true, and along the y-axis if `y` is true.
    const fn conditional_mirror(&mut self, x: bool, y: bool) {
        if x {
            self.mirror_x();
        }
        if y {
            self.mirror_y();
        }
    }

    /// Sets the start corner to be top-left and the end corner to be bottom-right.
    /// 
    /// S = start, E = end
    /// 
    /// ```txt
    /// S - *
    /// |   |
    /// * - E
    /// ```
    const fn normalize(&mut self) {
        let x_needed = self.start.0 > self.end.0;
        let y_needed = self.start.1 > self.end.1;
        self.conditional_mirror(x_needed, y_needed);
    }
    /// Sets the start corner to be top right and the end corner to be bottom left.
    /// 
    /// S = start, E = end
    /// 
    /// ```txt
    /// * - S
    /// |   |
    /// E - *
    /// ```
    const fn normalize_mirrored_x(&mut self) {
        let x_needed = self.start.0 < self.end.0;
        let y_needed = self.start.1 > self.end.1;
        self.conditional_mirror(x_needed, y_needed);
    }
    // /// Sets the start corner to be bottom left and the end corner to be top right.
    // /// 
    // /// S = start, E = end
    // /// 
    // /// ```txt
    // /// * - E
    // /// |   |
    // /// S - *
    // /// ```
    // const fn normalize_mirrored_y(&mut self) {
    //     let x_needed = self.start.0 > self.end.0;
    //     let y_needed = self.start.1 < self.end.1;
    //     self.conditional_mirror(x_needed, y_needed);
    // }
    // /// Sets the start corner to be bottom right and the end corner to be top left.
    // /// 
    // /// S = start, E = end
    // /// 
    // /// ```txt
    // /// E - *
    // /// |   |
    // /// * - S
    // /// ```
    // const fn denormalize(&mut self) {
    //     let x_needed = self.start.0 < self.end.0;
    //     let y_needed = self.start.1 < self.end.1;
    //     self.conditional_mirror(x_needed, y_needed);
    // }
    /// Translates the selection by the given coordinates, keeping the same size.
    pub const fn move_relative(&mut self, dx: f32, dy: f32) {
        self.start.0 += dx;
        self.start.1 += dy;
        self.end.0 += dx;
        self.end.1 += dy;
    }

    /// Checks if the selection contains the given point.
    pub const fn contains(&self, point: (f32, f32)) -> bool {
        let (x, y) = point;
        let (start_x, start_y) = self.start;
        let (end_x, end_y) = self.end;

        x >= start_x && x <= end_x && 
        y >= start_y && y <= end_y
    }

    /// Moves the selection's top-left corner to the given coordinates.
    const fn move_top_left(&mut self, x: f32, y: f32) {
        self.normalize();
        self.start.0 = x;
        self.start.1 = y;
    }
    /// Translates the selection's top-left corner by the given relative coordinates.
    const fn move_top_left_relative(&mut self, dx: f32, dy: f32) {
        self.normalize();
        self.start.0 += dx;
        self.start.1 += dy;
    }
    /// Moves the selection's top-right corner to the given coordinates.
    const fn move_top_right(&mut self, x: f32, y: f32) {
        self.normalize_mirrored_x();
        self.start.0 = x;
        self.start.1 = y;
    }
    /// Translates the selection's top-right corner by the given relative coordinates.
    const fn move_top_right_relative(&mut self, dx: f32, dy: f32) {
        self.normalize_mirrored_x();
        self.start.0 += dx;
        self.start.1 += dy;
    }
    /// Moves the selection's bottom-right corner to the given coordinates.
    const fn move_bottom_right(&mut self, x: f32, y: f32) {
        self.normalize();
        self.end.0 = x;
        self.end.1 = y;
    }
    /// Translates the selection's bottom-right corner by the given relative coordinates.
    const fn move_bottom_right_relative(&mut self, dx: f32, dy: f32) {
        self.normalize();
        self.end.0 += dx;
        self.end.1 += dy;
    }
    /// Moves the selection's bottom-left corner to the given coordinates.
    const fn move_bottom_left(&mut self, x: f32, y: f32) {
        self.normalize_mirrored_x();
        self.end.0 = x;
        self.end.1 = y;
    }
    /// Translates the selection's bottom-left corner by the given relative coordinates.
    const fn move_bottom_left_relative(&mut self, dx: f32, dy: f32) {
        self.normalize_mirrored_x();
        self.end.0 += dx;
        self.end.1 += dy;
    }
    /// Moves the selection's top edge to the given y-coordinate.
    const fn move_top(&mut self, y: f32) {
        self.normalize();
        self.start.1 = y;
    }
    /// Translates the selection's top edge by the given relative y-coordinate.
    const fn move_top_relative(&mut self, dy: f32) {
        self.normalize();
        self.start.1 += dy;
    }
    /// Moves the selection's bottom edge to the given y-coordinate.
    const fn move_bottom(&mut self, y: f32) {
        self.normalize();
        self.end.1 = y;
    }
    /// Translates the selection's bottom edge by the given relative y-coordinate.
    const fn move_bottom_relative(&mut self, dy: f32) {
        self.normalize();
        self.end.1 += dy;
    }
    /// Moves the selection's left edge to the given x-coordinate.
    const fn move_left(&mut self, x: f32) {
        self.normalize();
        self.start.0 = x;
    }
    /// Translates the selection's left edge by the given relative x-coordinate.
    const fn move_left_relative(&mut self, dx: f32) {
        self.normalize();
        self.start.0 += dx;
    }
    /// Moves the selection's right edge to the given x-coordinate.
    const fn move_right(&mut self, x: f32) {
        self.normalize();
        self.end.0 = x;
    }
    /// Translates the selection's right edge by the given relative x-coordinate.
    const fn move_right_relative(&mut self, dx: f32) {
        self.normalize();
        self.end.0 += dx;
    }

    /// Reshapes the selection based on the given drag handle and coordinates.
    pub const fn reshape(
        &mut self,
        handle: DragHandle,
        x: f32,
        y: f32,
    ) {
        match handle {
            DragHandle::TopLeft => self.move_top_left(x, y),
            DragHandle::TopRight => self.move_top_right(x, y),
            DragHandle::BottomLeft => self.move_bottom_left(x, y),
            DragHandle::BottomRight => self.move_bottom_right(x, y),
            DragHandle::Top => self.move_top(y),
            DragHandle::Bottom => self.move_bottom(y),
            DragHandle::Left => self.move_left(x),
            DragHandle::Right => self.move_right(x),
        }
    }

    /// Reshapes the selection based on the given drag handle and relative coordinates.
    pub const fn reshape_relative(
        &mut self,
        handle: DragHandle,
        dx: f32,
        dy: f32,
    ) {
        match handle {
            DragHandle::TopLeft => self.move_top_left_relative(dx, dy),
            DragHandle::TopRight => self.move_top_right_relative(dx, dy),
            DragHandle::BottomLeft => self.move_bottom_left_relative(dx, dy),
            DragHandle::BottomRight => self.move_bottom_right_relative(dx, dy),
            DragHandle::Top => self.move_top_relative(dy),
            DragHandle::Bottom => self.move_bottom_relative(dy),
            DragHandle::Left => self.move_left_relative(dx),
            DragHandle::Right => self.move_right_relative(dx),
        }
    }

    /// Converts the selection to a `graphene::Rect`.
    pub fn to_graphene_rect(&self) -> graphene::Rect {
        let (start_x, start_y) = self.top_left();
        graphene::Rect::new(
            start_x.round(),
            start_y.round(),
            self.width().round(),
            self.height().round(),
        )
    }

    /// Converts the selection to a `graphene::Rect`, positioned to be
    /// drawn as a 1px border around the selection.
    pub fn to_graphene_border_rect(&self) -> graphene::Rect {
        let (start_x, start_y) = self.top_left();
        graphene::Rect::new(
            start_x.round() - 0.5,
            start_y.round() - 0.5,
            self.width().round() + 1.0,
            self.height().round() + 1.0,
        )
    }

    /// Draws the selection to a `gtk4::Snapshot`.
    pub fn draw_to_snapshot(
        &self,
        snapshot: &gtk4::Snapshot,
        widget: impl IsA<gtk4::Widget>,
    ) {
        if self.width() <= 0.0 || self.height() <= 0.0 {
            NoSelection::new().draw_to_snapshot(snapshot, widget);
            return;
        }
        let width = widget.width() as f32;
        let height = widget.height() as f32;
        let bounds = graphene::Rect::new(0.0, 0.0, width, height);

        snapshot.push_mask(gsk4::MaskMode::InvertedAlpha); // every mask needs to be paired with two pops
        let selection_rect = self.to_graphene_rect();
        snapshot.append_color(&gdk4::RGBA::WHITE, &selection_rect);

        // apply the mask and make it active
        snapshot.pop();

        let (red, green, blue, alpha) = (0.0, 0.0, 0.0, 0.5);
        let transparent_black = gdk4::RGBA::new(red, green, blue, alpha);
        snapshot.append_color(&transparent_black, &bounds);

        // stop using the mask
        snapshot.pop();
        
        let border_selection_rect = self.to_graphene_border_rect();
        let path_builder = gsk4::PathBuilder::new();
        path_builder.add_rect(&border_selection_rect);
        let path = path_builder.to_path();
        let stroke = gsk4::Stroke::new(1.0);

        snapshot.append_stroke(&path, &stroke, &gdk4::RGBA::WHITE);
    }
}

impl PartialEq for Selection {
    /// Checks if the given 2 selections are equal in their positioning and size.
    /// (with a tolerance of `f32::EPSILON` for floating point comparisons)
    fn eq(&self, other: &Self) -> bool {
        let (left_top_left_x, left_top_left_y) = self.top_left();
        let (left_bottom_right_x, left_bottom_right_y) = self.bottom_right();
        let (right_top_left_x, right_top_left_y) = other.top_left();
        let (right_bottom_right_x, right_bottom_right_y) = other.bottom_right();
        (left_top_left_x - right_top_left_x).abs() < f32::EPSILON &&
        (left_top_left_y - right_top_left_y).abs() < f32::EPSILON &&
        (left_bottom_right_x - right_bottom_right_x).abs() < f32::EPSILON &&
        (left_bottom_right_y - right_bottom_right_y).abs() < f32::EPSILON
    }
}

impl Default for Selection {
    fn default() -> Self {
        Selection {
            start: (0.0, 0.0),
            end: (1.0, 1.0),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct NoSelection;

impl NoSelection {
    pub const fn new() -> Self {
        NoSelection
    }

    pub fn draw_to_snapshot(
        &self,
        snapshot: &gtk4::Snapshot,
        widget: impl IsA<gtk4::Widget>,
    ) {
        // Just draw the black overlay

        let width = widget.width() as f32;
        let height = widget.height() as f32;
        let bounds = graphene::Rect::new(0.0, 0.0, width, height);

        let (red, green, blue, alpha) = (0.0, 0.0, 0.0, 0.5);
        let transparent_black = gdk4::RGBA::new(red, green, blue, alpha);
        snapshot.append_color(&transparent_black, &bounds);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MaybeSelection {
    Selection(Selection),
    NoSelection(NoSelection),
}

impl MaybeSelection {
    #[inline]
    pub fn draw_to_snapshot(
        &self,
        snapshot: &gtk4::Snapshot,
        widget: impl IsA<gtk4::Widget>,
    ) {
        match self {
            MaybeSelection::Selection(selection) => {
                selection.draw_to_snapshot(snapshot, widget);
            }
            MaybeSelection::NoSelection(no_selection) => {
                no_selection.draw_to_snapshot(snapshot, widget);
            }
        }
    }
}

impl Default for MaybeSelection {
    fn default() -> Self {
        MaybeSelection::NoSelection(NoSelection::new())
    }
}

#[derive(Debug, Clone, PartialEq, Default, glib::Boxed)]
#[boxed_type(name = "MaybeSelectionBoxed")]
pub struct MaybeSelectionBoxed(pub MaybeSelection);

impl Deref for MaybeSelectionBoxed {
    type Target = MaybeSelection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MaybeSelectionBoxed {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<MaybeSelectionBoxed> for MaybeSelection {
    fn from(selection: MaybeSelectionBoxed) -> Self {
        selection.0
    }
}
impl From<MaybeSelection> for MaybeSelectionBoxed {
    fn from(selection: MaybeSelection) -> Self {
        MaybeSelectionBoxed(selection)
    }
}
impl AsRef<MaybeSelection> for MaybeSelectionBoxed {
    fn as_ref(&self) -> &MaybeSelection {
        &self.0
    }
}
impl AsMut<MaybeSelection> for MaybeSelectionBoxed {
    fn as_mut(&mut self) -> &mut MaybeSelection {
        &mut self.0
    }
}