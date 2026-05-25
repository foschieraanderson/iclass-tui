use crate::{
    app::{
        focus::Focus,
        resources::Resource,
        routes::Route,
    },
    models::{
        auth::Session,
        class::ClassRoom,
        task::Task,
        user::User,
    },
};

/// Número de linhas visíveis em cada picker (teacher / students).
/// Deve estar em sincronia com o layout do modal em `ui::screens::classes`.
pub const PICKER_VISIBLE_ROWS: usize = 8;

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

// ---- Class modal / form ---------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum ClassModal {
    None,
    Add,
    Edit,
    ConfirmDelete,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ClassFormField {
    Period,
    Grade,
    TeacherPicker,
    StudentPicker,
}

/// Formulário de criação/edição de turma.
///
/// Os campos `period` e `grade` são entradas de texto livres.
/// `selected_teacher` e `selected_students` referenciam índices nas listas
/// `AppState::available_teachers` e `AppState::available_students`.
#[derive(Debug, Clone)]
pub struct ClassForm {
    pub period: String,
    pub grade: String,
    pub active_field: ClassFormField,

    // -- teacher picker (single-select) -------------------------
    pub teacher_cursor: usize,
    pub teacher_scroll: usize,
    pub selected_teacher: Option<usize>,

    // -- student picker (multi-select) --------------------------
    pub student_cursor: usize,
    pub student_scroll: usize,
    pub selected_students: Vec<usize>,
}

impl Default for ClassForm {
    fn default() -> Self {
        Self {
            period: String::new(),
            grade: String::new(),
            active_field: ClassFormField::Period,
            teacher_cursor: 0,
            teacher_scroll: 0,
            selected_teacher: None,
            student_cursor: 0,
            student_scroll: 0,
            selected_students: Vec::new(),
        }
    }
}

// ---- App state ------------------------------------------------

pub struct AppState {

    pub route: Route,

    pub focus: Focus,

    pub session: Option<Session>,

    pub login_form: LoginForm,

    // -- users --------------------------------------------------

    pub users: Resource<Vec<User>>,
    pub selected_user_index: usize,
    pub user_modal: UserModal,
    pub user_form: UserForm,

    // -- classes ------------------------------------------------

    pub classes: Resource<Vec<ClassRoom>>,
    pub selected_class_index: usize,
    pub class_modal: ClassModal,
    pub class_form: ClassForm,

    /// Professores carregados da API ao abrir o modal de turma.
    /// Resetados para `Idle` ao fechar o modal.
    pub available_teachers: Resource<Vec<User>>,

    /// Alunos carregados da API ao abrir o modal de turma.
    /// Resetados para `Idle` ao fechar o modal.
    pub available_students: Resource<Vec<User>>,

    // -- tasks --------------------------------------------------

    pub tasks: Resource<Vec<Task>>,

    // -- shared -------------------------------------------------

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
            selected_user_index: 0,
            user_modal: UserModal::None,
            user_form: UserForm::default(),

            classes: Resource::Idle,
            selected_class_index: 0,
            class_modal: ClassModal::None,
            class_form: ClassForm::default(),
            available_teachers: Resource::Idle,
            available_students: Resource::Idle,

            tasks: Resource::Idle,

            sidebar_index: 0,

            loading: false,

            error: None,
        }
    }
}
