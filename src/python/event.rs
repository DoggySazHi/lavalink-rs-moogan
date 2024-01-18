use crate::prelude::LavalinkClient;
use crate::model::events::*;

use pyo3::prelude::*;

pyo3::import_exception!(builtins, NameError);

#[pyclass]
#[derive(Debug, Clone)]
pub struct EventHandler {
    pub inner: PyObject,
    pub current_loop: PyObject,
}

#[pymethods]
impl EventHandler {
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Periodic event that returns the statistics of the server.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `Stats`
    ///
    /// Returns: `Future<None>`
    fn stats(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when a player updates.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `PlayerUpdate`
    ///
    /// Returns: `Future<None>`
    fn player_update(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when a track starts playing.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `TrackStart`
    ///
    /// Returns: `Future<None>`
    fn track_start(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when a track finishes playing.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `TrackEnd`
    ///
    /// Returns: `Future<None>`
    fn track_end(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when a track raises an exception on the Lavalink server.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `TrackException`
    ///
    /// Returns: `Future<None>`
    fn track_exception(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when a track gets stuck while playing.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `TrackStuck`
    ///
    /// Returns: `Future<None>`
    fn track_stuck(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when the websocket connection to the voice channel closes.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `WebSocketClosed`
    ///
    /// Returns: `Future<None>`
    fn websocket_closed(&self) {}
    #[pyo3(text_signature = "($self, client, event, /)")]
    /// Event that triggers when the connection is ready.
    ///
    /// Positional Arguments:
    /// - `client` : `Lavalink`
    /// - `event` : `Ready`
    ///
    /// Returns: `Future<None>`
    fn ready(&self) {}
}

impl EventHandler {
    pub(crate) async fn event_stats(&self, client: LavalinkClient, session_id: String, event: Stats) {
        call_event(self, client, session_id, event, "stats");
    }
    pub(crate) async fn event_player_update(&self, client: LavalinkClient, session_id: String, event: PlayerUpdate) {
        call_event(self, client, session_id, event, "player_update");
    }
    pub(crate) async fn event_track_start(&self, client: LavalinkClient, session_id: String, event: TrackStart) {
        call_event(self, client, session_id, event, "track_start");
    }
    pub(crate) async fn event_track_end(&self, client: LavalinkClient, session_id: String, event: TrackEnd) {
        call_event(self, client, session_id, event, "track_end");
    }
    pub(crate) async fn event_track_exception(&self, client: LavalinkClient, session_id: String, event: TrackException) {
        call_event(self, client, session_id, event, "track_exception");
    }
    pub(crate) async fn event_track_stuck(&self, client: LavalinkClient, session_id: String, event: TrackStuck) {
        call_event(self, client, session_id, event, "track_stuck");
    }
    pub(crate) async fn event_websocket_closed(&self, client: LavalinkClient, session_id: String, event: WebSocketClosed) {
        call_event(self, client, session_id, event, "websocket_closed");
    }
    pub(crate) async fn event_ready(&self, client: LavalinkClient, session_id: String, event: Ready) {
        call_event(self, client, session_id, event, "ready");
    }
}

fn call_event<T: Send + Sync + pyo3::IntoPy<PyObject> + 'static>(
    handler: &EventHandler,
    client: LavalinkClient,
    session_id: String,
    event: T,
    name: &'static str,
) {
    let slf1 = handler.clone();
    let slf2 = handler.clone();

    Python::with_gil(|py| {
        let current_loop = slf1.current_loop.as_ref(py);

        pyo3_asyncio::tokio::future_into_py_with_locals(
            py,
            pyo3_asyncio::TaskLocals::new(current_loop),
            async move {
                let future = Python::with_gil(|py| {
                    let py_event_handler = slf2.inner.as_ref(py);
                    let coro_result = py_event_handler.call_method(
                        name,
                        (client, session_id, event),
                        None,
                    );

                    if let Ok(coro) = coro_result {
                        pyo3_asyncio::tokio::into_future(coro)
                    } else {
                        Err(NameError::new_err("Undefined event"))
                    }
                });

                if let Ok(f) = future {
                    if let Err(e) = f.await {
                        Python::with_gil(|py| {
                            e.print_and_set_sys_last_vars(py);
                        });
                    }
                }

                Ok(Python::with_gil(|py| py.None()))
            },
        )
        .unwrap();
    });
}