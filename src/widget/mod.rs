pub mod layout;
pub mod primitives;
pub mod text;
pub mod image;
pub mod button;
pub mod scroll;

use backend::gfx::G2d;
use graphics::Context;

use event::Event;
use input::EventId;
use super::util::*;

use super::ui::Resources;
use self::layout::WidgetLayout;

use cassowary::Solver;

use std::any::Any;

pub trait EventHandler {
    fn event_id(&self) -> EventId;
    fn handle_event(&mut self, Event, &mut Any, &mut WidgetLayout, &WidgetLayout, &mut Solver) -> Option<Event>;
}

pub struct Widget {
    pub draw_fn: Option<fn(&Any, Rectangle, Rectangle, &mut Resources, Context, &mut G2d)>,
    pub drawable: Option<Box<Any>>,
    pub mouse_over_fn: fn(Point, Rectangle) -> bool,
    pub layout: WidgetLayout,
    pub event_handlers: Vec<Box<EventHandler>>,
}

use input::{Input, Motion};
impl Widget {
    pub fn new() -> Self {
        Widget {
            draw_fn: None,
            drawable: None,
            mouse_over_fn: point_inside_rect,
            layout: WidgetLayout::new(),
            event_handlers: Vec::new(),
        }
    }
    pub fn set_drawable(&mut self, draw_fn: fn(&Any, Rectangle, Rectangle, &mut Resources, Context, &mut G2d), drawable: Box<Any>) {
        self.draw_fn = Some(draw_fn);
        self.drawable = Some(drawable);
    }
    pub fn print(&self, solver: &mut Solver) {
        println!("{:?}", self.layout.bounds(solver));
    }
    pub fn draw(&self, parent: &Widget, resources: &mut Resources, solver: &mut Solver, c: Context, g: &mut G2d) {
        if let (Some(draw_fn), Some(ref drawable)) = (self.draw_fn, self.drawable.as_ref()) {
            let parent_bounds = parent.layout.bounds(solver);
            let bounds = self.layout.bounds(solver);
            draw_fn(drawable.as_ref(), parent_bounds, bounds, resources, c, g);
        }
    }
    pub fn is_mouse_over(&self, solver: &mut Solver, mouse: Point) -> bool {
        let bounds = self.layout.bounds(solver);
        (self.mouse_over_fn)(mouse, bounds)
    }
    pub fn trigger_event(&mut self, id: EventId, event: Event, parent_layout: &WidgetLayout, solver: &mut Solver) -> Option<Event> {
        let event_handler = self.event_handlers.iter_mut().find(|event_handler| event_handler.event_id() == id).unwrap();

        let any = &mut "Any";
        let drawable = {
            if let Some(ref mut drawable) = self.drawable {
                drawable.as_mut()
            } else {
                any
            }
        };
        event_handler.handle_event(event, drawable, &mut self.layout, parent_layout, solver)
    }
}