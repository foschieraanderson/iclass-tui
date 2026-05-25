use crate::{
    app::routes::Route,
    models::auth::Session,
};

pub enum Action {

    Quit,

    Login {
        email: String,
        password: String,
    },

    LoginSuccess(Session),

    SetError(Option<String>),

    InputChar(char),
    InputBackspace,
    InputToggleField,

    LoadUsers,
    LoadClasses,
    LoadTasks,

    NavigateUp,
    NavigateDown,

    ChangeRoute(Route),

    Logout,
}
