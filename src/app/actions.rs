use crate::{
    app::{
        focus::Focus,
        routes::Route,
    },
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

    LoadDashboard,
    LoadUsers,
    LoadClasses,
    LoadTasks,
    LoadReports,

    NavigateUp,
    NavigateDown,

    ChangeRoute(Route),

    Logout,

    SetFocus(Focus),

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

    // -- classes screen ------------------------------------------

    SelectClass(usize),

    OpenAddClassModal,
    OpenEditClassModal,
    OpenConfirmDeleteClassModal,
    CloseClassModal,

    ClassFormChar(char),
    ClassFormBackspace,
    ClassFormNextField,

    /// Move o cursor uma linha para cima no picker ativo (TeacherPicker ou StudentPicker).
    ClassPickerUp,

    /// Move o cursor uma linha para baixo no picker ativo.
    ClassPickerDown,

    /// Seleciona/alterna o item em destaque no picker ativo.
    /// TeacherPicker: single-select (substitui seleção atual).
    /// StudentPicker: toggle (adiciona ou remove da lista).
    ClassPickerSelect,

    SubmitClassForm,
    ConfirmDeleteClass,

    // -- tasks screen --------------------------------------------

    SelectTask(usize),

    OpenAddTaskModal,
    OpenEditTaskModal,
    OpenConfirmDeleteTaskModal,
    CloseTaskModal,

    TaskFormChar(char),
    TaskFormBackspace,
    TaskFormNextField,

    /// Cicla pelo array FIBONACCI_SCORES no campo Score.
    TaskFormCycleScore,

    /// Move o cursor uma linha para cima no class picker da task.
    TaskPickerUp,

    /// Move o cursor uma linha para baixo no class picker da task.
    TaskPickerDown,

    /// Seleciona a turma em destaque no class picker (single-select).
    TaskPickerSelect,

    SubmitTaskForm,
    ConfirmDeleteTask,

    // -- submissions ---------------------------------------------

    /// student: abre modal de envio de resposta
    OpenSubmitModal,

    /// teacher/admin: abre lista de submissões da tarefa selecionada
    OpenSubmissionsModal,

    CloseSubmissionModal,

    SubmitFormChar(char),
    SubmitFormBackspace,
    SubmitFormNextField,

    /// student: POST /tasks/:id/submissions
    SubmitSubmission,

    /// carrega submissões da tarefa selecionada
    LoadSubmissions,

    SelectSubmission(usize),

    /// abre modal de avaliação para a submissão selecionada
    OpenGradeModal,

    GradeFormChar(char),
    GradeFormBackspace,
    GradeFormNextField,

    /// teacher/admin: PATCH /submissions/:id
    SubmitGrade,

    /// abre task.file_url no browser do sistema
    OpenTaskFile,

    /// abre submission.file_url no browser do sistema
    OpenSubmissionFile,
}
