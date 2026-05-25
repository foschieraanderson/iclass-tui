use crate::{
    app::{
        focus::Focus,
        resources::Resource,
        routes::Route,
    },
    models::{
        auth::Session,
        class::ClassRoom,
        dashboard::DashboardData,
        report::ClassReport,
        submission::Submission,
        task::Task,
        user::User,
    },
};

/// Valores Fibonacci válidos para score de tarefa.
pub const FIBONACCI_SCORES: &[u32] = &[1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];

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

// ---- Submission modal / forms ---------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum SubmissionModal {
    None,
    Submit,
    List,
    Grade,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SubmitFormField {
    FilePath,
    Content,
}

#[derive(Debug, Clone)]
pub struct SubmitForm {
    pub file_path:    String,
    pub content:      String,
    pub active_field: SubmitFormField,
}

impl Default for SubmitForm {
    fn default() -> Self {
        Self {
            file_path:    String::new(),
            content:      String::new(),
            active_field: SubmitFormField::FilePath,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GradeFormField {
    Grade,
    Feedback,
}

#[derive(Debug, Clone)]
pub struct GradeForm {
    pub grade:        String,
    pub feedback:     String,
    pub active_field: GradeFormField,
}

impl Default for GradeForm {
    fn default() -> Self {
        Self {
            grade:        String::new(),
            feedback:     String::new(),
            active_field: GradeFormField::Grade,
        }
    }
}

// ---- Task modal / form ----------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum TaskModal {
    None,
    Add,
    Edit,
    ConfirmDelete,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskFormField {
    Title,
    Description,
    Score,
    ExpiresAt,
    ClassPicker,
}

/// Formulário de criação/edição de tarefa.
///
/// `score_index` aponta para `FIBONACCI_SCORES`.
/// `selected_class` referencia índice em `AppState::available_task_classes` (só usado no Add).
#[derive(Debug, Clone)]
pub struct TaskForm {
    pub title: String,
    pub description: String,
    pub score_index: usize,
    pub expires_at: String,
    pub active_field: TaskFormField,
    // class picker (single-select, só no modal Add)
    pub class_cursor: usize,
    pub class_scroll: usize,
    pub selected_class: Option<usize>,
}

impl Default for TaskForm {
    fn default() -> Self {
        Self {
            title: String::new(),
            description: String::new(),
            score_index: 0,
            expires_at: String::new(),
            active_field: TaskFormField::Title,
            class_cursor: 0,
            class_scroll: 0,
            selected_class: None,
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
    pub selected_task_index: usize,
    pub task_modal: TaskModal,
    pub task_form: TaskForm,

    /// Turmas disponíveis para o picker no modal de criação de tarefa.
    pub available_task_classes: Resource<Vec<ClassRoom>>,

    // -- dashboard ----------------------------------------------

    pub dashboard: Resource<DashboardData>,

    // -- reports ------------------------------------------------

    pub reports: Resource<Vec<ClassReport>>,

    // -- submissions --------------------------------------------

    pub submission_modal:          SubmissionModal,
    pub submissions:               Resource<Vec<Submission>>,
    pub selected_submission_index: usize,
    pub submit_form:               SubmitForm,
    pub grade_form:                GradeForm,

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
            selected_task_index: 0,
            task_modal: TaskModal::None,
            task_form: TaskForm::default(),
            available_task_classes: Resource::Idle,

            dashboard: Resource::Idle,

            reports: Resource::Idle,

            submission_modal:          SubmissionModal::None,
            submissions:               Resource::Idle,
            selected_submission_index: 0,
            submit_form:               SubmitForm::default(),
            grade_form:                GradeForm::default(),

            sidebar_index: 0,

            loading: false,

            error: None,
        }
    }
}
