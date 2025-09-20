use std::collections::HashMap;

use niri_ipc::{Action, Output, Reply, Request, socket::Socket};
pub use state::{LayoutEvent, Snapshot, Window};
pub use window_stream::WindowStream;

use crate::error::Error;

mod reply;
mod state;
mod window_stream;

/// The top level client for Niri.
#[derive(Debug, Clone, Copy)]
pub struct Niri {}

impl Niri {
    pub fn new() -> Self {
        // Since niri_ipc is essentially stateless, we don't maintain anything much here.
        Self {}
    }

    /// Requests that the given window ID should be activated.
    #[tracing::instrument(level = "TRACE", err)]
    pub fn activate_window(&self, id: u64) -> Result<(), Error> {
        let reply = request(Request::Action(Action::FocusWindow { id }))?;
        reply::typed!(Handled, reply)
    }

    #[tracing::instrument(level = "TRACE", err)]
    pub fn close_window(&self, id: u64) -> Result<(), Error> {
        let reply = request(Request::Action(Action::CloseWindow { id: Some(id) }))?;
        reply::typed!(Handled, reply)
    }

    /// Returns the current outputs.
    pub fn outputs(&self) -> Result<HashMap<String, Output>, Error> {
        let reply = request(Request::Outputs)?;
        reply::typed!(Outputs, reply)
    }

    /// Returns a stream of window snapshots.
    pub fn window_stream(&self) -> WindowStream {
        WindowStream::new()
    }

    pub fn focus_tiling(&self) -> Result<HashMap<String, Output>, Error> {
        let reply = request(Request::Action(Action::FocusTiling {}))?;
        reply::typed!(Outputs, reply)
    }
}

// Helper to marshal request errors into our own type system.
//
// This can't be used for event streams, since the stream callback is thrown away in this function.
#[tracing::instrument(level = "TRACE", err)]
fn request(request: Request) -> Result<Reply, Error> {
    socket()?.send(request).map_err(Error::NiriIpc)
}

// Helper to connect to the Niri socket.
#[tracing::instrument(level = "TRACE", err)]
fn socket() -> Result<Socket, Error> {
    Socket::connect().map_err(Error::NiriIpc)
}
