use std::{
    io::{self, Stdout},
    sync::Arc,
    time::Duration,
};

use crate::app::{termination::Interrupted, App};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, EventStream},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use tokio::sync::{broadcast, RwLock};
use tokio_stream::StreamExt;

mod ui;

const TICK_RATE: Duration = Duration::from_millis(250);

pub(crate) async fn main_loop(
    mut interrupt_rx: broadcast::Receiver<Interrupted>,
    app: Arc<RwLock<App>>,
) -> anyhow::Result<Interrupted> {
    let mut terminal = setup_terminal()?;
    let mut ticker = tokio::time::interval(TICK_RATE);
    let mut crossterm_events = EventStream::new();

    let result = loop {
        tokio::select! {
            _ = ticker.tick() => (),
            Some(Ok(Event::Key(key))) = crossterm_events.next() => {
                let mut app = app.write().await;

                app.handle_key_event(key);
            }
            Ok(interrupted) = interrupt_rx.recv() => {
                break interrupted;
            }
        }

        let app = app.read().await;

        terminal.draw(|frame| ui::render_app_too_frame(frame, &app))?;
    };

    restore_terminal(&mut terminal)?;

    Ok(result)
}

fn setup_terminal() -> anyhow::Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();

    enable_raw_mode()?;

    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    Ok(Terminal::new(CrosstermBackend::new(stdout))?)
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> anyhow::Result<()> {
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    Ok(terminal.show_cursor()?)
}