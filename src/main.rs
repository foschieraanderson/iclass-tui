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
    focus::Focus,
    reducer::reducer,
    routes::Route,
    state::{
        AppState,
        UserFormField,
        UserModal,
    },
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

                    let role = state.session
                        .as_ref()
                        .map(|s| s.role.as_str())
                        .unwrap_or("student");

                    ui::components::sidebar::render(
                        frame,
                        chunks[0],
                        state.sidebar_index,
                        role,
                        state.focus == Focus::Sidebar,
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
                        &state,
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

                    // ---- users screen input --------------------

                    Route::Users => {

                        let modal = state.user_modal.clone();

                        match modal {

                            UserModal::None => {

                                match key.code {

                                    KeyCode::Left => {
                                        reducer(&mut state, Action::SetFocus(Focus::Sidebar), &pool, &config).await?;
                                    }

                                    KeyCode::Right => {
                                        reducer(&mut state, Action::SetFocus(Focus::Content), &pool, &config).await?;
                                    }

                                    KeyCode::Down => {
                                        if state.focus == Focus::Sidebar {
                                            reducer(&mut state, Action::NavigateDown, &pool, &config).await?;
                                            if state.route == Route::Users {
                                                reducer(&mut state, Action::LoadUsers, &pool, &config).await?;
                                            }
                                        } else if let app::resources::Resource::Success(ref list) = state.users {
                                            let max = list.len().saturating_sub(1);
                                            if state.selected_user_index < max {
                                                let idx = state.selected_user_index + 1;
                                                reducer(&mut state, Action::SelectUser(idx), &pool, &config).await?;
                                            }
                                        }
                                    }

                                    KeyCode::Up => {
                                        if state.focus == Focus::Sidebar {
                                            reducer(&mut state, Action::NavigateUp, &pool, &config).await?;
                                            if state.route == Route::Users {
                                                reducer(&mut state, Action::LoadUsers, &pool, &config).await?;
                                            }
                                        } else if state.selected_user_index > 0 {
                                            let idx = state.selected_user_index - 1;
                                            reducer(&mut state, Action::SelectUser(idx), &pool, &config).await?;
                                        }
                                    }

                                    KeyCode::Char('a') => {
                                        reducer(&mut state, Action::OpenAddUserModal, &pool, &config).await?;
                                    }

                                    KeyCode::Char('e') => {
                                        reducer(&mut state, Action::OpenEditUserModal, &pool, &config).await?;
                                    }

                                    KeyCode::Char('d') => {
                                        reducer(&mut state, Action::OpenConfirmDeleteModal, &pool, &config).await?;
                                    }

                                    KeyCode::Char('l') => {
                                        reducer(&mut state, Action::Logout, &pool, &config).await?;
                                    }

                                    KeyCode::Char('q') => {
                                        break;
                                    }

                                    _ => {}
                                }
                            }

                            UserModal::Add | UserModal::Edit => {

                                match key.code {

                                    KeyCode::Esc => {
                                        reducer(&mut state, Action::CloseUserModal, &pool, &config).await?;
                                    }

                                    KeyCode::Tab => {
                                        reducer(&mut state, Action::UserFormNextField, &pool, &config).await?;
                                    }

                                    KeyCode::Enter => {
                                        reducer(&mut state, Action::SubmitUserForm, &pool, &config).await?;
                                    }

                                    KeyCode::Backspace => {
                                        reducer(&mut state, Action::UserFormBackspace, &pool, &config).await?;
                                    }

                                    KeyCode::Char(' ') => {
                                        if state.user_form.active_field == UserFormField::Role {
                                            reducer(&mut state, Action::UserFormCycleRole, &pool, &config).await?;
                                        } else {
                                            reducer(&mut state, Action::UserFormChar(' '), &pool, &config).await?;
                                        }
                                    }

                                    KeyCode::Char(c) => {
                                        reducer(&mut state, Action::UserFormChar(c), &pool, &config).await?;
                                    }

                                    _ => {}
                                }
                            }

                            UserModal::ConfirmDelete => {

                                match key.code {

                                    KeyCode::Enter | KeyCode::Char('y') => {
                                        reducer(&mut state, Action::ConfirmDeleteUser, &pool, &config).await?;
                                    }

                                    KeyCode::Esc | KeyCode::Char('n') => {
                                        reducer(&mut state, Action::CloseUserModal, &pool, &config).await?;
                                    }

                                    _ => {}
                                }
                            }
                        }
                    }

                    // ---- dashboard / other screens input -------

                    _ => {

                        match key.code {

                            KeyCode::Left => {
                                reducer(&mut state, Action::SetFocus(Focus::Sidebar), &pool, &config).await?;
                            }

                            KeyCode::Right => {
                                reducer(&mut state, Action::SetFocus(Focus::Content), &pool, &config).await?;
                            }

                            KeyCode::Char('q') => {
                                break;
                            }

                            KeyCode::Char('l') => {
                                reducer(&mut state, Action::Logout, &pool, &config).await?;
                            }

                            KeyCode::Down => {
                                reducer(&mut state, Action::NavigateDown, &pool, &config).await?;
                                if state.route == Route::Users {
                                    reducer(&mut state, Action::LoadUsers, &pool, &config).await?;
                                }
                            }

                            KeyCode::Up => {
                                reducer(&mut state, Action::NavigateUp, &pool, &config).await?;
                                if state.route == Route::Users {
                                    reducer(&mut state, Action::LoadUsers, &pool, &config).await?;
                                }
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
