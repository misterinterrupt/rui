use crate::*;

#[derive(Clone, Debug)]
pub enum EventKind {
    PressButton(String),
    TouchBegin { id: usize },
    TouchMove { id: usize },
    TouchEnd { id: usize },
}

#[derive(Clone, Debug)]
pub struct Event {
    pub kind: EventKind,
    pub position: LocalPoint,
}

/// Trait for the unit of UI composition.
pub trait View {
    fn print(&self, id: ViewID, cx: &mut Context);
    fn process(&self, event: &Event, id: ViewID, cx: &mut Context, vger: &mut VGER);
    fn draw(&self, id: ViewID, cx: &mut Context, vger: &mut VGER);
    fn layout(&self, id: ViewID, sz: LocalSize, cx: &mut Context, vger: &mut VGER) -> LocalSize;
    fn hittest(
        &self,
        id: ViewID,
        pt: LocalPoint,
        cx: &mut Context,
        vger: &mut VGER,
    ) -> Option<ViewID>;
}

pub struct EmptyView {}

impl View for EmptyView {
    fn print(&self, _id: ViewID, _cx: &mut Context) {
        println!("EmptyView");
    }
    fn process(&self, _event: &Event, _id: ViewID, _cx: &mut Context, _vger: &mut VGER) {}
    fn draw(&self, _id: ViewID, _cx: &mut Context, _vger: &mut VGER) {}
    fn layout(
        &self,
        _id: ViewID,
        _sz: LocalSize,
        _cx: &mut Context,
        _vger: &mut VGER,
    ) -> LocalSize {
        [0.0, 0.0].into()
    }
    fn hittest(
        &self,
        _id: ViewID,
        _pt: LocalPoint,
        _cx: &mut Context,
        _vger: &mut VGER,
    ) -> Option<ViewID> {
        None
    }
}
