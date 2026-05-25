use sqlx::SqlitePool;

use crate::{
    api::{
        client::ApiClient,
        users as api_users,
    },
    app::{
        actions::Action,
        resources::Resource,
        routes::Route,
        state::{
            AppState,
            LoginField,
            LoginForm,
            UserForm,
            UserFormField,
            UserModal,
        },
    },
    config::Config,
    database::session_repository,
    models::user::{
        CreateUserRequest,
        UpdateUserRequest,
    },
    services::auth_service,
};

const SIDEBAR_MAX_ADMIN: usize = 3;
const SIDEBAR_MAX_DEFAULT: usize = 2;

fn sidebar_max(role: &str) -> usize {
    if role == "admin" { SIDEBAR_MAX_ADMIN } else { SIDEBAR_MAX_DEFAULT }
}

fn route_for_sidebar(index: usize, role: &str) -> Route {
    if role == "admin" {
        match index {
            1 => Route::Users,
            2 => Route::Classes,
            3 => Route::Tasks,
            _ => Route::Dashboard,
        }
    } else {
        match index {
            1 => Route::Classes,
            2 => Route::Tasks,
            _ => Route::Dashboard,
        }
    }
}

fn api_client(config: &Config, state: &AppState) -> Option<ApiClient> {
    state.session.as_ref().map(|s| {
        ApiClient::new(&config.api_url, Some(s.access_token.clone()))
    })
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

            let max = state.session
                .as_ref()
                .map(|s| sidebar_max(&s.role))
                .unwrap_or(SIDEBAR_MAX_DEFAULT);

            if state.sidebar_index < max {
                state.sidebar_index += 1;
                let role = state.session.as_ref().map(|s| s.role.as_str()).unwrap_or("student");
                state.route = route_for_sidebar(state.sidebar_index, role);
            }
        }

        Action::NavigateUp => {

            if state.sidebar_index > 0 {
                state.sidebar_index -= 1;
                let role = state.session.as_ref().map(|s| s.role.as_str()).unwrap_or("student");
                state.route = route_for_sidebar(state.sidebar_index, role);
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

        // -- users: load ---------------------------------------

        Action::LoadUsers => {

            let Some(session) = &state.session else {
                return Ok(());
            };

            if session.role != "admin" {
                return Ok(());
            }

            let api = ApiClient::new(
                &config.api_url,
                Some(session.access_token.clone()),
            );

            state.users = Resource::Loading;

            match api_users::list_users(&api).await {

                Ok(list) => {
                    state.selected_user_index = 0;
                    state.users = Resource::Success(list);
                }

                Err(e) => {
                    state.users = Resource::Error(e.to_string());
                }
            }
        }

        // -- users: list navigation ----------------------------

        Action::SelectUser(index) => {
            state.selected_user_index = index;
        }

        // -- users: modals -------------------------------------

        Action::OpenAddUserModal => {
            state.user_form  = UserForm::default();
            state.user_modal = UserModal::Add;
        }

        Action::OpenEditUserModal => {

            if let Resource::Success(ref list) = state.users {

                if let Some(user) = list.get(state.selected_user_index) {

                    state.user_form = UserForm {
                        name:         user.name.clone(),
                        email:        user.email.clone(),
                        password:     String::new(),
                        role:         user.role.clone(),
                        active_field: UserFormField::Name,
                    };

                    state.user_modal = UserModal::Edit;
                }
            }
        }

        Action::OpenConfirmDeleteModal => {

            if let Resource::Success(ref list) = state.users {
                if list.get(state.selected_user_index).is_some() {
                    state.user_modal = UserModal::ConfirmDelete;
                }
            }
        }

        Action::CloseUserModal => {
            state.user_modal = UserModal::None;
            state.user_form  = UserForm::default();
        }

        // -- users: form input ---------------------------------

        Action::UserFormChar(c) => {

            match state.user_form.active_field {
                UserFormField::Name     => state.user_form.name.push(c),
                UserFormField::Email    => state.user_form.email.push(c),
                UserFormField::Password => state.user_form.password.push(c),
                UserFormField::Role     => {}
            }
        }

        Action::UserFormBackspace => {

            match state.user_form.active_field {
                UserFormField::Name     => { state.user_form.name.pop(); }
                UserFormField::Email    => { state.user_form.email.pop(); }
                UserFormField::Password => { state.user_form.password.pop(); }
                UserFormField::Role     => {}
            }
        }

        Action::UserFormNextField => {

            state.user_form.active_field = match state.user_form.active_field {
                UserFormField::Name     => UserFormField::Email,
                UserFormField::Email    => UserFormField::Password,
                UserFormField::Password => UserFormField::Role,
                UserFormField::Role     => UserFormField::Name,
            };
        }

        Action::UserFormCycleRole => {

            state.user_form.role = match state.user_form.role.as_str() {
                "student" => "teacher".to_string(),
                "teacher" => "admin".to_string(),
                _         => "student".to_string(),
            };
        }

        // -- users: submit / delete ----------------------------

        Action::SubmitUserForm => {

            let Some(ref api) = api_client(config, state) else {
                return Ok(());
            };

            let modal = state.user_modal.clone();

            match modal {

                UserModal::Add => {

                    let req = CreateUserRequest {
                        name:     state.user_form.name.clone(),
                        email:    state.user_form.email.clone(),
                        password: state.user_form.password.clone(),
                        role:     Some(state.user_form.role.clone()),
                    };

                    match api_users::create_user(api, req).await {

                        Ok(_) => {
                            state.user_modal = UserModal::None;
                            state.user_form  = UserForm::default();
                            state.error      = None;
                        }

                        Err(e) => {
                            state.error = Some(e.to_string());
                            return Ok(());
                        }
                    }
                }

                UserModal::Edit => {

                    let user_id = if let Resource::Success(ref list) = state.users {
                        list.get(state.selected_user_index).map(|u| u.id.clone())
                    } else {
                        None
                    };

                    let Some(id) = user_id else {
                        return Ok(());
                    };

                    let password = if state.user_form.password.is_empty() {
                        None
                    } else {
                        Some(state.user_form.password.clone())
                    };

                    let req = UpdateUserRequest {
                        name:  Some(state.user_form.name.clone()),
                        email: Some(state.user_form.email.clone()),
                        role:  Some(state.user_form.role.clone()),
                    };

                    // password is not included in UpdateUserRequest per API design
                    let _ = password;

                    match api_users::update_user(api, &id, req).await {

                        Ok(_) => {
                            state.user_modal = UserModal::None;
                            state.user_form  = UserForm::default();
                            state.error      = None;
                        }

                        Err(e) => {
                            state.error = Some(e.to_string());
                            return Ok(());
                        }
                    }
                }

                _ => {}
            }

            // reload list after add/edit
            let api = api_client(config, state).unwrap();
            match api_users::list_users(&api).await {
                Ok(list) => {
                    state.selected_user_index = 0;
                    state.users = Resource::Success(list);
                }
                Err(e) => {
                    state.users = Resource::Error(e.to_string());
                }
            }
        }

        Action::ConfirmDeleteUser => {

            let user_id = if let Resource::Success(ref list) = state.users {
                list.get(state.selected_user_index).map(|u| u.id.clone())
            } else {
                None
            };

            let Some(id) = user_id else {
                state.user_modal = UserModal::None;
                return Ok(());
            };

            let Some(ref api) = api_client(config, state) else {
                return Ok(());
            };

            match api_users::delete_user(api, &id).await {

                Ok(_) => {
                    state.user_modal = UserModal::None;
                    state.error      = None;

                    let api2 = api_client(config, state).unwrap();
                    match api_users::list_users(&api2).await {
                        Ok(list) => {
                            state.selected_user_index = 0;
                            state.users = Resource::Success(list);
                        }
                        Err(e) => {
                            state.users = Resource::Error(e.to_string());
                        }
                    }
                }

                Err(e) => {
                    state.error = Some(e.to_string());
                    state.user_modal = UserModal::None;
                }
            }
        }

        // -- focus ---------------------------------------------

        Action::SetFocus(focus) => {
            state.focus = focus;
        }

        // -- not yet implemented -------------------------------

        _ => {}
    }

    Ok(())
}
