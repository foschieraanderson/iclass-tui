mod app;
mod api;
mod config;
mod database;
mod models;
mod services;
mod tui;
mod ui;

use std::io;

use app::{
    actions::Action,
    reducer::reducer,
    routes::Route,
    state::AppState,
};

use config::Config;

use crossterm::{
    event::{
        KeyCode,
        KeyModifiers,
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    color_eyre::install().unwrap();

    tracing_subscriber::fmt::init();

    // ---- database setup ----------------------------------------

    let config = Config::load();

    let pool = database::sqlite::connect(
        &config.database_url,
    )
    .await?;

    database::migrations::migrate(&pool).await?;

    // ---- restore session from cache ----------------------------

    let mut state = AppState::default();

    if let Some(session) =
        database::session_repository::load_session(&pool).await?
    {
        state.session = Some(session);
        state.route   = Route::Dashboard;
    }

    // ---- terminal setup ----------------------------------------

    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(
        stdout,
        EnterAlternateScreen,
    )?;

    let backend =
        CrosstermBackend::new(stdout);

    let mut terminal =
        Terminal::new(backend)?;

    // ---- main event loop ---------------------------------------

    loop {

        terminal.draw(|frame| {

            match state.route {

                Route::Login => {

                    ui::screens::login::render(
                        frame,
                        frame.area(),
                        &state,
                    );
                }

                _ => {

                    let chunks =
                        ui::layout::layout(frame);

                    ui::components::sidebar::render(
                        frame,
                        chunks[0],
                        state.sidebar_index,
                    );

                    match state.route {

                        Route::Users => {
                            ui::screens::users::render(
                                frame,
                                chunks[1],
                                &state,
                            );
                        }

                        Route::Classes => {
                            ui::screens::classes::render(
                                frame,
                                chunks[1],
                                &state,
                            );
                        }

                        Route::Tasks => {
                            ui::screens::tasks::render(
                                frame,
                                chunks[1],
                                &state,
                            );
                        }

                        _ => {
                            ui::screens::dashboard::render(
                                frame,
                                chunks[1],
                                &state,
                            );
                        }
                    }

                    ui::components::footer::render(
                        frame,
                        chunks[2],
                    );
                }
            }
        })?;

        if crossterm::event::poll(
            std::time::Duration::from_millis(16),
        )? {

            if let crossterm::event::Event::Key(key) =
                crossterm::event::read()?
            {

                match state.route {

                    // ---- login screen input --------------------

                    Route::Login => {

                        match (key.modifiers, key.code) {

                            (KeyModifiers::CONTROL, KeyCode::Char('c')) => {
                                break;
                            }

                            (_, KeyCode::Tab) => {

                                reducer(
                                    &mut state,
                                    Action::InputToggleField,
                                    &pool,
                                    &config,
                                )
                                .await?;
                            }

                            (_, KeyCode::Backspace) => {

                                reducer(
                                    &mut state,
                                    Action::InputBackspace,
                                    &pool,
                                    &config,
                                )
                                .await?;
                            }

                            (_, KeyCode::Enter) => {

                                let email =
                                    state.login_form.email.clone();

                                let password =
                                    state.login_form.password.clone();

                                reducer(
                                    &mut state,
                                    Action::Login { email, password },
                                    &pool,
                                    &config,
                                )
                                .await?;
                            }

                            (_, KeyCode::Char(c)) => {

                                reducer(
                                    &mut state,
                                    Action::InputChar(c),
                                    &pool,
                                    &config,
                                )
                                .await?;
                            }

                            _ => {}
                        }
                    }

                    // ---- dashboard / other screens input -------

                    _ => {

                        match key.code {

                            KeyCode::Char('q') => {
                                break;
                            }

                            KeyCode::Char('l') => {

                                reducer(
                                    &mut state,
                                    Action::Logout,
                                    &pool,
                                    &config,
                                )
                                .await?;
                            }

                            KeyCode::Down => {

                                reducer(
                                    &mut state,
                                    Action::NavigateDown,
                                    &pool,
                                    &config,
                                )
                                .await?;
                            }

                            KeyCode::Up => {

                                reducer(
                                    &mut state,
                                    Action::NavigateUp,
                                    &pool,
                                    &config,
                                )
                                .await?;
                            }

                            _ => {}
                        }
                    }
                }
            }
        }
    }

    // ---- cleanup -----------------------------------------------

    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
    )?;

    Ok(())
}
