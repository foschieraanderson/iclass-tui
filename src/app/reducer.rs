use sqlx::SqlitePool;

use crate::{
    app::{
        actions::Action,
        routes::Route,
        state::{
            AppState,
            LoginField,
            LoginForm,
        },
    },
    config::Config,
    database::session_repository,
    services::auth_service,
};

const SIDEBAR_MAX: usize = 3;

fn route_for_sidebar(index: usize) -> Route {
    match index {
        1 => Route::Users,
        2 => Route::Classes,
        3 => Route::Tasks,
        _ => Route::Dashboard,
    }
}

pub async fn reducer(
    state: &mut AppState,
    action: Action,
    pool: &SqlitePool,
    config: &Config,
) -> anyhow::Result<()> {

    match action {

        // -- navigation ----------------------------------------

        Action::NavigateDown => {

            if state.sidebar_index < SIDEBAR_MAX {
                state.sidebar_index += 1;
                state.route = route_for_sidebar(state.sidebar_index);
            }
        }

        Action::NavigateUp => {

            if state.sidebar_index > 0 {
                state.sidebar_index -= 1;
                state.route = route_for_sidebar(state.sidebar_index);
            }
        }

        // -- routing -------------------------------------------

        Action::ChangeRoute(route) => {
            state.route = route;
        }

        // -- login form input ----------------------------------

        Action::InputChar(c) => {

            match state.login_form.active_field {

                LoginField::Email => {
                    state.login_form.email.push(c);
                }

                LoginField::Password => {
                    state.login_form.password.push(c);
                }
            }
        }

        Action::InputBackspace => {

            match state.login_form.active_field {

                LoginField::Email => {
                    state.login_form.email.pop();
                }

                LoginField::Password => {
                    state.login_form.password.pop();
                }
            }
        }

        Action::InputToggleField => {

            state.login_form.active_field = match state.login_form.active_field {
                LoginField::Email    => LoginField::Password,
                LoginField::Password => LoginField::Email,
            };
        }

        // -- authentication ------------------------------------

        Action::Login { email, password } => {

            state.loading = true;
            state.error   = None;

            match auth_service::authenticate(
                &config.api_url,
                pool,
                email,
                password,
            )
            .await
            {
                Ok(session) => {
                    state.session = Some(session);
                    state.route   = Route::Dashboard;
                    state.loading = false;
                }

                Err(e) => {
                    state.error   = Some(e.to_string());
                    state.loading = false;
                }
            }
        }

        Action::LoginSuccess(session) => {
            state.session = Some(session);
            state.route   = Route::Dashboard;
        }

        // -- logout --------------------------------------------

        Action::Logout => {

            session_repository::delete_session(pool).await?;

            state.session    = None;
            state.route      = Route::Login;
            state.error      = None;
            state.login_form = LoginForm::default();
        }

        // -- error management ----------------------------------

        Action::SetError(msg) => {
            state.error = msg;
        }

        // -- not yet implemented -------------------------------

        _ => {}
    }

    Ok(())
}
