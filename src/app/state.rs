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

// ---- Login form -----------------------------------------------

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

// ---- User modal / form ----------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum UserModal {
    None,
    Add,
    Edit,
    ConfirmDelete,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UserFormField {
    Name,
    Email,
    Password,
    Role,
}

#[derive(Debug, Clone)]
pub struct UserForm {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub active_field: UserFormField,
}

impl Default for UserForm {
    fn default() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            password: String::new(),
            role: "student".to_string(),
            active_field: UserFormField::Name,
        }
    }
}

// ---- App state ------------------------------------------------

pub struct AppState {

    pub route: Route,

    pub focus: Focus,

    pub session: Option<Session>,

    pub login_form: LoginForm,

    pub users: Resource<Vec<User>>,
    pub classes: Resource<Vec<ClassRoom>>,
    pub tasks: Resource<Vec<Task>>,

    pub sidebar_index: usize,

    pub selected_user_index: usize,
    pub user_modal: UserModal,
    pub user_form: UserForm,

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

            selected_user_index: 0,
            user_modal: UserModal::None,
            user_form: UserForm::default(),

            loading: false,

            error: None,
        }
    }
}
