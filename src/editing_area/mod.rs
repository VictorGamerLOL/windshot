mod command;
mod selection;

pub use command::{Command, CommandType, CommandsBoxed};

mod imp {
    use std::cell::RefCell;
    use std::sync::atomic::AtomicBool;

    use glib::Properties;
    use gtk4::glib;
    use gtk4::subclass::prelude::*;
    use gtk4::prelude::*;

    use crate::editing_area::selection::{MaybeSelection, Selection};

    use super::command::CommandsBoxed;
    use super::selection::MaybeSelectionBoxed;


    #[derive(Properties, Default, Debug)]
    #[properties(wrapper_type = super::EditingArea)]
    pub struct EditingArea {
        #[property(get, set)]
        pub undo_stack: RefCell<CommandsBoxed>,
        // #[property(get, set)]
        // pub redo_stack: RefCell<CommandsBoxed>,
        #[property(get, set)]
        pub selection: RefCell<MaybeSelectionBoxed>,
        #[property(get, set)]
        pub active_drag: AtomicBool,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for EditingArea {
        const NAME: &'static str = "WEditingArea";
        type Type = super::EditingArea;
        type ParentType = gtk4::Widget;
    }

    #[glib::derived_properties]
    impl ObjectImpl for EditingArea {
        fn constructed(&self) {
            self.parent_constructed();

            let editing_area = self.obj().clone();
            let editing_area_weak = editing_area.downgrade();

            let drag_gesture = gtk4::GestureDrag::new();
            
            let mut editing_area_weak_clone = editing_area_weak.clone();
            drag_gesture.connect_drag_begin(move |_, x, y| {
                let Some(editing_area) = editing_area_weak_clone.upgrade() else {
                    return;
                };
                println!("Drag started at ({}, {})", x, y);
                let x = x as f32;
                let y = y as f32;
                editing_area.set_active_drag(true);

                let selection = Selection::new((x, y), (x, y));
                let selection_boxed: MaybeSelectionBoxed = MaybeSelection::Selection(selection).into();
                editing_area.set_selection(selection_boxed);
                editing_area.queue_draw();
            });
            
            editing_area_weak_clone = editing_area_weak.clone();
            drag_gesture.connect_drag_update(move |_, x, y| {
                let Some(editing_area) = editing_area_weak_clone.upgrade() else {
                    return;
                };
                println!("Drag updated at ({}, {})", x, y);
                let x = x as f32;
                let y = y as f32;
                let maybe_selection = editing_area.selection();

                let sel = match maybe_selection.0 {
                    MaybeSelection::Selection(mut sel) => {
                        let start = sel.start();
                        sel.set_end((start.0 + x, start.1 + y));
                        sel
                    }
                    _ => {
                        Selection::new((x, y), (x, y))
                    }
                };
                editing_area.set_selection(MaybeSelectionBoxed::from(MaybeSelection::Selection(sel)));
                editing_area.queue_draw();

                dbg!(editing_area.selection());
                editing_area.queue_draw();
            });

            editing_area_weak_clone = editing_area_weak.clone();
            drag_gesture.connect_drag_end(move |_, x, y| {
                let Some(editing_area) = editing_area_weak_clone.upgrade() else {
                    return;
                };
                println!("Drag ended at ({}, {})", x, y);
                editing_area.set_active_drag(false);
                editing_area.queue_draw();
            });

            drop(editing_area_weak);
            editing_area.add_controller(drag_gesture);
        }
    }

    impl WidgetImpl for EditingArea {
        fn snapshot(&self, snapshot: &gtk4::Snapshot) {
            let undo_stack = self.undo_stack.borrow();
            for command in undo_stack.iter() {
                command.draw_to_snapshot(snapshot, self.obj().clone());
            }
            

            self.selection.borrow().draw_to_snapshot(snapshot, self.obj().clone());
        }
    }
}

glib::wrapper! {
    pub struct EditingArea(ObjectSubclass<imp::EditingArea>)
        @extends gtk4::Widget,
        @implements gtk4::Buildable, gtk4::Actionable, gtk4::ConstraintTarget, gtk4::Accessible;
}

impl EditingArea {
    pub fn new() -> Self {
        glib::Object::new::<Self>()
    }
}
