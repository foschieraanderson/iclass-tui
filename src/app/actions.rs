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

    // -- login form input ----------------------------------------

    InputChar(char),
    InputBackspace,
    InputToggleField,

    // -- general navigation --------------------------------------

    LoadUsers,
    LoadClasses,
    LoadTasks,

    NavigateUp,
    NavigateDown,

    ChangeRoute(Route),

    Logout,

    // -- users screen --------------------------------------------

    SelectUser(usize),

    OpenAddUserModal,
    OpenEditUserModal,
    OpenConfirmDeleteModal,
    CloseUserModal,

    UserFormChar(char),
    UserFormBackspace,
    UserFormNextField,
    UserFormCycleRole,

    SubmitUserForm,
    ConfirmDeleteUser,
}
