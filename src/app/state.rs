use crate::{
    app::{
        focus::Focus,
        resources::Resource,
        routes::Route,
    },
    models::{
        auth::Session,
        user::User,
        class::ClassRoom,
        task::Task,
    },
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoginField {
    Email,
    Password,
}

#[derive(Debug, Clone)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
    pub active_field: LoginField,
}

impl Default for LoginForm {
    fn default() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
            active_field: LoginField::Email,
        }
    }
}

pub struct AppState {

    pub route: Route,

    pub focus: Focus,

    pub session: Option<Session>,

    pub login_form: LoginForm,

    pub users: Resource<Vec<User>>,
    pub classes: Resource<Vec<ClassRoom>>,
    pub tasks: Resource<Vec<Task>>,

    pub sidebar_index: usize,

    pub loading: bool,

    pub error: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            route: Route::Login,

            focus: Focus::Sidebar,

            session: None,

            login_form: LoginForm::default(),

            users: Resource::Idle,
            classes: Resource::Idle,
            tasks: Resource::Idle,

            sidebar_index: 0,

            loading: false,

            error: None,
        }
    }
}
