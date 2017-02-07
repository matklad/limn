extern crate limn;
extern crate glutin;
extern crate cassowary;

mod util;

use cassowary::strength::*;

use limn::widget::{EventHandler, EventArgs};
use limn::widget::builder::WidgetBuilder;
use limn::widgets::button::ToggleButtonBuilder;
use limn::widgets::primitives::{self, RectStyle};
use limn::widgets::drag::DragEvent;
use limn::widget::style::Value;
use limn::event::{EventAddress, EventId};
use limn::event::id::*;
use limn::util::Dimensions;

struct DragHandler {
    start_pos: f64,
}
impl DragHandler {
    pub fn new() -> Self {
        DragHandler { start_pos: 0.0 }
    }
}
impl EventHandler for DragHandler {
    fn event_id(&self) -> EventId {
        WIDGET_DRAG
    }
    fn handle_event(&mut self, args: EventArgs) {
        let EventArgs { data, solver, layout, .. } = args;
        let &(ref drag_event, pos) = data.downcast_ref::<(DragEvent, (i32, i32))>().unwrap();
        let drag_pos = pos.0 as f64;
        match *drag_event {
            DragEvent::DragStart => {
                self.start_pos = drag_pos - solver.get_value(layout.left);
                if !solver.has_edit_variable(&layout.left) {
                    solver.add_edit_variable(layout.left, STRONG).unwrap();
                }
            },
            _ => {
                solver.suggest_value(layout.left, drag_pos - self.start_pos).unwrap();
            }
        }
    }
}

fn main() {
    let (window, ui, event_queue) = util::init_default("Limn slider demo");
    let font_id = util::load_default_font();

    let mut root_widget = WidgetBuilder::new();
    root_widget.layout.dimensions(Dimensions {
        width: 300.0,
        height: 300.0,
    });

    let style = RectStyle { background: Value::Single([0.1, 0.1, 0.1, 1.0]) };
    let mut slider_container = WidgetBuilder::new().set_drawable(primitives::rect_drawable(style));
    slider_container.layout.dimensions(Dimensions {
        width: 200.0,
        height: 30.0,
    });
    slider_container.layout.align_top(&root_widget, Some(10.0));
    slider_container.layout.center_horizontal(&root_widget);

    let style = RectStyle { background: Value::Single([0.4, 0.4, 0.4, 1.0]) };
    let mut slider = WidgetBuilder::new()
        .set_drawable(primitives::rect_drawable(style))
        .draggable()
        .add_handler(Box::new(DragHandler::new()));
    slider.layout.dimensions(Dimensions {
        width: 30.0,
        height: 30.0,
    });
    slider.layout.align_top(&root_widget, Some(10.0));

    slider_container.add_child(Box::new(slider));
    root_widget.add_child(Box::new(slider_container));

    util::set_root_and_loop(window, ui, root_widget, event_queue, vec!{});
}