use penrose::x::XConnExt;
use penrose::{
    core::{
        bindings::{MotionNotifyEvent, MouseEvent, MouseEventHandler, MouseEventKind},
        State,
    },
    pure::geometry::Rect,
    x::XConn,
    Result, Xid,
};

type DragAction = fn(&mut Rect, i32, i32);

#[derive(Debug)]
struct DragData {
    id: Xid,
    r: Rect,
    x: i32,
    y: i32,
    action: DragAction,
}

impl DragData {
    fn init<X: XConn>(
        action: DragAction,
        evt: &MouseEvent,
        state: &mut State<X>,
        x: &X,
    ) -> Result<Self> {
        let bwidth = state.config.border_width as i32 * 2;
        let id = evt.data.id;
        let mut r = x.client_geometry(id)?;
        let p = evt.data.rpt;

        r.resize(bwidth, bwidth);
        state.client_set.float(id, r)?;
        Ok(Self {
            id,
            r,
            x: p.x as i32,
            y: p.y as i32,
            action,
        })
    }

    fn on_move<X: XConn>(
        &mut self,
        evt: &MotionNotifyEvent,
        state: &mut State<X>,
        x: &X,
    ) -> Result<()> {
        let bwidth = state.config.border_width as i32 * 2;
        let p = evt.data.rpt;
        let mut r = self.r;
        let dx = p.x as i32 - self.x as i32;
        let dy = p.y as i32 - self.y as i32;

        (self.action)(&mut r, dx, dy);
        x.position_client(self.id, r)?;
        // r.resize(bwidth, bwidth);
        state.client_set.float(evt.data.id, r)?;

        Ok(())
    }
}

#[derive(Debug, Default)]
struct DragHandler {
    data: Option<DragData>,
}

impl<X: XConn> MouseEventHandler<X> for DragHandler {
    fn on_mouse_event(&mut self, evt: &MouseEvent, state: &mut State<X>, x: &X) -> Result<()> {
        match evt.kind {
            MouseEventKind::Press => {
                self.data = Some(DragData::init(
                    |r, dx, dy| r.reposition(dx, dy),
                    evt,
                    state,
                    x,
                )?)
            }
            MouseEventKind::Release => self.data = None,
        };

        Ok(())
    }

    fn on_motion(&mut self, evt: &MotionNotifyEvent, state: &mut State<X>, x: &X) -> Result<()> {
        if let Some(ref mut data) = self.data {
            data.on_move(evt, state, x)?;
        }
        Ok(())
    }
}

pub fn drag_handler<X: XConn>() -> Box<dyn MouseEventHandler<X>> {
    Box::new(DragHandler::default())
}

#[derive(Debug, Default)]
struct ResizeHandler {
    data: Option<DragData>,
}

impl<X: XConn> MouseEventHandler<X> for ResizeHandler {
    fn on_mouse_event(&mut self, evt: &MouseEvent, state: &mut State<X>, x: &X) -> Result<()> {
        match evt.kind {
            MouseEventKind::Press => {
                let r = x.client_geometry(evt.data.id)?;
                let wd = r.w / 2 + r.x;
                let hd = r.h / 2 + r.y;
                let p = evt.data.wpt;
                let action: DragAction = if p.x < wd {
                    if p.y < hd {
                        |r, dx, dy| {
                            r.reposition(dx, dy);
                            r.resize(-dx, -dy);
                        }
                    } else {
                        |r, dx, dy| {
                            r.reposition(dx, 0);
                            r.resize(-dx, dy);
                        }
                    }
                } else {
                    if p.y < hd {
                        |r, dx, dy| {
                            r.reposition(0, dy);
                            r.resize(dx, -dy);
                        }
                    } else {
                        |r, dx, dy| {
                            r.resize(dx, dy);
                        }
                    }
                };

                self.data = Some(DragData::init(action, evt, state, x)?)
            }

            MouseEventKind::Release => self.data = None,
        };

        Ok(())
    }

    fn on_motion(&mut self, evt: &MotionNotifyEvent, state: &mut State<X>, x: &X) -> Result<()> {
        if let Some(ref mut data) = self.data {
            data.on_move(evt, state, x)?;
        }
        Ok(())
    }
}

pub fn resize_handler<X: XConn>() -> Box<dyn MouseEventHandler<X>> {
    Box::new(ResizeHandler::default())
}
