use sqlx::SqlitePool;

use crate::{
    api::{
        classes as api_classes,
        client::ApiClient,
        tasks as api_tasks,
        users as api_users,
    },
    app::{
        actions::Action,
        resources::Resource,
        routes::Route,
        state::{
            AppState,
            ClassForm,
            ClassFormField,
            ClassModal,
            FIBONACCI_SCORES,
            LoginField,
            LoginForm,
            PICKER_VISIBLE_ROWS,
            TaskForm,
            TaskFormField,
            TaskModal,
            UserForm,
            UserFormField,
            UserModal,
        },
    },
    config::Config,
    database::session_repository,
    models::{
        class::{
            CreateClassRequest,
            UpdateClassRequest,
        },
        user::{
            CreateUserRequest,
            UpdateUserRequest,
        },
    },
    services::auth_service,
};

const SIDEBAR_MAX_ADMIN:   usize = 3;
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

// ---- Helpers para pickers ------------------------------------

/// Converte o índice selecionado no teacher picker para o UUID do professor.
fn resolve_teacher_id(state: &AppState) -> Option<String> {
    let idx = state.class_form.selected_teacher?;
    if let Resource::Success(ref teachers) = state.available_teachers {
        teachers.get(idx).map(|u| u.id.clone())
    } else {
        None
    }
}

/// Converte os índices selecionados no student picker para uma lista de UUIDs.
fn resolve_student_ids(state: &AppState) -> Vec<String> {
    if let Resource::Success(ref students) = state.available_students {
        state.class_form.selected_students
            .iter()
            .filter_map(|&idx| students.get(idx).map(|u| u.id.clone()))
            .collect()
    } else {
        Vec::new()
    }
}

/// Converte o índice selecionado no class picker da task para o UUID da turma.
fn resolve_task_class_id(state: &AppState) -> Option<String> {
    let idx = state.task_form.selected_class?;
    if let Resource::Success(ref classes) = state.available_task_classes {
        classes.get(idx).map(|c| c.id.clone())
    } else {
        None
    }
}

// ---- Reducer -------------------------------------------------

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

        // -- focus ---------------------------------------------

        Action::SetFocus(focus) => {
            state.focus = focus;
        }

        // ======================================================
        // USERS
        // ======================================================

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

        Action::SelectUser(index) => {
            state.selected_user_index = index;
        }

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

                    let req = UpdateUserRequest {
                        name:  Some(state.user_form.name.clone()),
                        email: Some(state.user_form.email.clone()),
                        role:  Some(state.user_form.role.clone()),
                    };

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

        // ======================================================
        // CLASSES
        // ======================================================

        Action::LoadClasses => {

            let Some(session) = &state.session else {
                return Ok(());
            };

            let api = ApiClient::new(
                &config.api_url,
                Some(session.access_token.clone()),
            );

            state.classes = Resource::Loading;

            match api_classes::list_classes(&api).await {

                Ok(list) => {
                    state.selected_class_index = 0;
                    state.classes = Resource::Success(list);
                }

                Err(e) => {
                    state.classes = Resource::Error(e.to_string());
                }
            }
        }

        Action::SelectClass(index) => {
            state.selected_class_index = index;
        }

        // -- classes: modals -----------------------------------

        Action::OpenAddClassModal => {

            let is_admin = state.session.as_ref().map(|s| s.role == "admin").unwrap_or(false);
            if !is_admin {
                return Ok(());
            }

            state.class_form          = ClassForm::default();
            state.class_modal         = ClassModal::Add;
            state.available_teachers  = Resource::Loading;
            state.available_students  = Resource::Loading;

            let Some(api) = api_client(config, state) else {
                return Ok(());
            };

            // Carrega professores
            match api_users::list_users_by_role(&api, "teacher").await {
                Ok(list) => { state.available_teachers = Resource::Success(list); }
                Err(e)   => { state.available_teachers = Resource::Error(e.to_string()); }
            }

            // Carrega alunos
            match api_users::list_users_by_role(&api, "student").await {
                Ok(list) => { state.available_students = Resource::Success(list); }
                Err(e)   => { state.available_students = Resource::Error(e.to_string()); }
            }
        }

        Action::OpenEditClassModal => {

            let is_admin = state.session.as_ref().map(|s| s.role == "admin").unwrap_or(false);
            if !is_admin {
                return Ok(());
            }

            // Extrai dados da turma selecionada antes de mutarmos o estado
            let (period, grade, teacher_id, class_student_ids) =
                if let Resource::Success(ref list) = state.classes {
                    if let Some(class) = list.get(state.selected_class_index) {
                        let sids: Vec<String> = class.students
                            .iter()
                            .map(|u| u.id.clone())
                            .collect();
                        (
                            class.period.clone(),
                            class.grade.clone(),
                            class.teacher.id.clone(),
                            sids,
                        )
                    } else {
                        return Ok(());
                    }
                } else {
                    return Ok(());
                };

            state.class_modal        = ClassModal::Edit;
            state.available_teachers = Resource::Loading;
            state.available_students = Resource::Loading;

            let Some(api) = api_client(config, state) else {
                return Ok(());
            };

            // Carrega professores e alunos
            match api_users::list_users_by_role(&api, "teacher").await {
                Ok(list) => { state.available_teachers = Resource::Success(list); }
                Err(e)   => { state.available_teachers = Resource::Error(e.to_string()); }
            }

            match api_users::list_users_by_role(&api, "student").await {
                Ok(list) => { state.available_students = Resource::Success(list); }
                Err(e)   => { state.available_students = Resource::Error(e.to_string()); }
            }

            // Pré-seleciona professor pelo ID
            let selected_teacher = if let Resource::Success(ref teachers) = state.available_teachers {
                teachers.iter().position(|u| u.id == teacher_id)
            } else {
                None
            };

            let teacher_cursor = selected_teacher.unwrap_or(0);

            // Pré-seleciona alunos pelo ID
            let selected_students = if let Resource::Success(ref students) = state.available_students {
                class_student_ids
                    .iter()
                    .filter_map(|id| students.iter().position(|u| &u.id == id))
                    .collect()
            } else {
                Vec::new()
            };

            state.class_form = ClassForm {
                period,
                grade,
                teacher_cursor,
                teacher_scroll: 0,
                selected_teacher,
                student_cursor: 0,
                student_scroll: 0,
                selected_students,
                active_field: ClassFormField::Period,
            };
        }

        Action::OpenConfirmDeleteClassModal => {

            let is_admin = state.session.as_ref().map(|s| s.role == "admin").unwrap_or(false);
            if is_admin {
                if let Resource::Success(ref list) = state.classes {
                    if list.get(state.selected_class_index).is_some() {
                        state.class_modal = ClassModal::ConfirmDelete;
                    }
                }
            }
        }

        Action::CloseClassModal => {
            state.class_modal        = ClassModal::None;
            state.class_form         = ClassForm::default();
            state.available_teachers = Resource::Idle;
            state.available_students = Resource::Idle;
            state.error              = None;
        }

        // -- classes: form input (apenas campos de texto) ------

        Action::ClassFormChar(c) => {

            match state.class_form.active_field {
                ClassFormField::Period        => state.class_form.period.push(c),
                ClassFormField::Grade         => state.class_form.grade.push(c),
                ClassFormField::TeacherPicker => {}
                ClassFormField::StudentPicker => {}
            }
        }

        Action::ClassFormBackspace => {

            match state.class_form.active_field {
                ClassFormField::Period        => { state.class_form.period.pop(); }
                ClassFormField::Grade         => { state.class_form.grade.pop(); }
                ClassFormField::TeacherPicker => {}
                ClassFormField::StudentPicker => {}
            }
        }

        Action::ClassFormNextField => {

            state.class_form.active_field = match state.class_form.active_field {
                ClassFormField::Period        => ClassFormField::Grade,
                ClassFormField::Grade         => ClassFormField::TeacherPicker,
                ClassFormField::TeacherPicker => ClassFormField::StudentPicker,
                ClassFormField::StudentPicker => ClassFormField::Period,
            };
        }

        // -- classes: picker navigation ------------------------

        Action::ClassPickerUp => {

            match state.class_form.active_field {

                ClassFormField::TeacherPicker => {
                    if state.class_form.teacher_cursor > 0 {
                        state.class_form.teacher_cursor -= 1;
                        if state.class_form.teacher_cursor < state.class_form.teacher_scroll {
                            state.class_form.teacher_scroll = state.class_form.teacher_cursor;
                        }
                    }
                }

                ClassFormField::StudentPicker => {
                    if state.class_form.student_cursor > 0 {
                        state.class_form.student_cursor -= 1;
                        if state.class_form.student_cursor < state.class_form.student_scroll {
                            state.class_form.student_scroll = state.class_form.student_cursor;
                        }
                    }
                }

                _ => {}
            }
        }

        Action::ClassPickerDown => {

            match state.class_form.active_field {

                ClassFormField::TeacherPicker => {
                    let len = if let Resource::Success(ref list) = state.available_teachers {
                        list.len()
                    } else {
                        0
                    };
                    if len > 0 && state.class_form.teacher_cursor < len - 1 {
                        state.class_form.teacher_cursor += 1;
                        if state.class_form.teacher_cursor
                            >= state.class_form.teacher_scroll + PICKER_VISIBLE_ROWS
                        {
                            state.class_form.teacher_scroll =
                                state.class_form.teacher_cursor + 1 - PICKER_VISIBLE_ROWS;
                        }
                    }
                }

                ClassFormField::StudentPicker => {
                    let len = if let Resource::Success(ref list) = state.available_students {
                        list.len()
                    } else {
                        0
                    };
                    if len > 0 && state.class_form.student_cursor < len - 1 {
                        state.class_form.student_cursor += 1;
                        if state.class_form.student_cursor
                            >= state.class_form.student_scroll + PICKER_VISIBLE_ROWS
                        {
                            state.class_form.student_scroll =
                                state.class_form.student_cursor + 1 - PICKER_VISIBLE_ROWS;
                        }
                    }
                }

                _ => {}
            }
        }

        Action::ClassPickerSelect => {

            match state.class_form.active_field {

                ClassFormField::TeacherPicker => {
                    if let Resource::Success(ref list) = state.available_teachers {
                        if list.get(state.class_form.teacher_cursor).is_some() {
                            state.class_form.selected_teacher =
                                Some(state.class_form.teacher_cursor);
                        }
                    }
                }

                ClassFormField::StudentPicker => {
                    if let Resource::Success(ref list) = state.available_students {
                        let cursor = state.class_form.student_cursor;
                        if list.get(cursor).is_some() {
                            if let Some(pos) = state.class_form.selected_students
                                .iter()
                                .position(|&i| i == cursor)
                            {
                                // Já selecionado — remove (toggle off)
                                state.class_form.selected_students.remove(pos);
                            } else {
                                // Não selecionado — adiciona (toggle on)
                                state.class_form.selected_students.push(cursor);
                            }
                        }
                    }
                }

                _ => {}
            }
        }

        // -- classes: submit -----------------------------------

        Action::SubmitClassForm => {

            let is_admin = state.session.as_ref().map(|s| s.role == "admin").unwrap_or(false);
            if !is_admin {
                state.error = Some("Somente admin pode criar/editar turmas.".to_string());
                return Ok(());
            }

            let Some(ref api) = api_client(config, state) else {
                return Ok(());
            };

            let modal = state.class_modal.clone();

            match modal {

                ClassModal::Add => {

                    let Some(teacher_id) = resolve_teacher_id(state) else {
                        state.error = Some("Selecione um professor.".to_string());
                        return Ok(());
                    };

                    let student_ids = resolve_student_ids(state);
                    if student_ids.is_empty() {
                        state.error = Some("Selecione ao menos um aluno.".to_string());
                        return Ok(());
                    }

                    let req = CreateClassRequest {
                        period:      state.class_form.period.clone(),
                        grade:       state.class_form.grade.clone(),
                        teacher_id,
                        student_ids,
                    };

                    match api_classes::create_class(api, req).await {

                        Ok(_) => {
                            state.class_modal        = ClassModal::None;
                            state.class_form         = ClassForm::default();
                            state.available_teachers = Resource::Idle;
                            state.available_students = Resource::Idle;
                            state.error              = None;
                        }

                        Err(e) => {
                            state.error = Some(e.to_string());
                            return Ok(());
                        }
                    }
                }

                ClassModal::Edit => {

                    let class_id = if let Resource::Success(ref list) = state.classes {
                        list.get(state.selected_class_index).map(|c| c.id.clone())
                    } else {
                        None
                    };

                    let Some(id) = class_id else {
                        return Ok(());
                    };

                    // Campos opcionais: só envia se houver seleção/valor
                    let teacher_id  = resolve_teacher_id(state);
                    let student_ids = {
                        let ids = resolve_student_ids(state);
                        if ids.is_empty() { None } else { Some(ids) }
                    };
                    let period = if state.class_form.period.is_empty() {
                        None
                    } else {
                        Some(state.class_form.period.clone())
                    };
                    let grade = if state.class_form.grade.is_empty() {
                        None
                    } else {
                        Some(state.class_form.grade.clone())
                    };

                    let req = UpdateClassRequest {
                        period,
                        grade,
                        teacher_id,
                        student_ids,
                    };

                    match api_classes::update_class(api, &id, req).await {

                        Ok(_) => {
                            state.class_modal        = ClassModal::None;
                            state.class_form         = ClassForm::default();
                            state.available_teachers = Resource::Idle;
                            state.available_students = Resource::Idle;
                            state.error              = None;
                        }

                        Err(e) => {
                            state.error = Some(e.to_string());
                            return Ok(());
                        }
                    }
                }

                _ => {}
            }

            // Recarrega a lista de turmas após add/edit
            let api = api_client(config, state).unwrap();
            match api_classes::list_classes(&api).await {
                Ok(list) => {
                    state.selected_class_index = 0;
                    state.classes = Resource::Success(list);
                }
                Err(e) => {
                    state.classes = Resource::Error(e.to_string());
                }
            }
        }

        // -- classes: delete -----------------------------------

        Action::ConfirmDeleteClass => {

            let class_id = if let Resource::Success(ref list) = state.classes {
                list.get(state.selected_class_index).map(|c| c.id.clone())
            } else {
                None
            };

            let Some(id) = class_id else {
                state.class_modal = ClassModal::None;
                return Ok(());
            };

            let Some(ref api) = api_client(config, state) else {
                return Ok(());
            };

            match api_classes::delete_class(api, &id).await {

                Ok(_) => {
                    state.class_modal = ClassModal::None;
                    state.error       = None;

                    let api2 = api_client(config, state).unwrap();
                    match api_classes::list_classes(&api2).await {
                        Ok(list) => {
                            state.selected_class_index = 0;
                            state.classes = Resource::Success(list);
                        }
                        Err(e) => {
                            state.classes = Resource::Error(e.to_string());
                        }
                    }
                }

                Err(e) => {
                    state.error = Some(e.to_string());
                    state.class_modal = ClassModal::None;
                }
            }
        }

        // ======================================================
        // TASKS
        // ======================================================

        Action::LoadTasks => {

            let Some(session) = &state.session else {
                return Ok(());
            };

            let api = ApiClient::new(
                &config.api_url,
                Some(session.access_token.clone()),
            );

            state.tasks = Resource::Loading;

            match api_tasks::list_tasks(&api).await {

                Ok(list) => {
                    state.selected_task_index = 0;
                    state.tasks = Resource::Success(list);
                }

                Err(e) => {
                    state.tasks = Resource::Error(e.to_string());
                }
            }
        }

        Action::SelectTask(index) => {
            state.selected_task_index = index;
        }

        Action::OpenAddTaskModal => {

            let can_create = state.session.as_ref()
                .map(|s| s.role == "admin" || s.role == "teacher")
                .unwrap_or(false);

            if !can_create {
                return Ok(());
            }

            state.task_form              = TaskForm::default();
            state.task_modal             = TaskModal::Add;
            state.available_task_classes = Resource::Loading;

            let Some(api) = api_client(config, state) else {
                return Ok(());
            };

            match api_classes::list_classes(&api).await {
                Ok(list) => { state.available_task_classes = Resource::Success(list); }
                Err(e)   => { state.available_task_classes = Resource::Error(e.to_string()); }
            }
        }

        Action::OpenEditTaskModal => {

            let can_edit = state.session.as_ref()
                .map(|s| s.role == "admin" || s.role == "teacher")
                .unwrap_or(false);

            if !can_edit {
                return Ok(());
            }

            if let Resource::Success(ref list) = state.tasks {
                if let Some(task) = list.get(state.selected_task_index) {

                    let score_index = FIBONACCI_SCORES
                        .iter()
                        .position(|&v| v == task.score as u32)
                        .unwrap_or(0);

                    state.task_form = TaskForm {
                        title:        task.title.clone(),
                        description:  task.description.clone().unwrap_or_default(),
                        score_index,
                        expires_at:   task.expires_at.clone().unwrap_or_default(),
                        active_field: TaskFormField::Title,
                        class_cursor: 0,
                        class_scroll: 0,
                        selected_class: None,
                    };

                    state.task_modal = TaskModal::Edit;
                }
            }
        }

        Action::OpenConfirmDeleteTaskModal => {

            let can_delete = state.session.as_ref()
                .map(|s| s.role == "admin" || s.role == "teacher")
                .unwrap_or(false);

            if can_delete {
                if let Resource::Success(ref list) = state.tasks {
                    if list.get(state.selected_task_index).is_some() {
                        state.task_modal = TaskModal::ConfirmDelete;
                    }
                }
            }
        }

        Action::CloseTaskModal => {
            state.task_modal             = TaskModal::None;
            state.task_form              = TaskForm::default();
            state.available_task_classes = Resource::Idle;
            state.error                  = None;
        }

        // -- tasks: form input ---------------------------------

        Action::TaskFormChar(c) => {

            match state.task_form.active_field {
                TaskFormField::Title       => state.task_form.title.push(c),
                TaskFormField::Description => state.task_form.description.push(c),
                TaskFormField::ExpiresAt   => state.task_form.expires_at.push(c),
                TaskFormField::Score
                | TaskFormField::ClassPicker => {}
            }
        }

        Action::TaskFormBackspace => {

            match state.task_form.active_field {
                TaskFormField::Title       => { state.task_form.title.pop(); }
                TaskFormField::Description => { state.task_form.description.pop(); }
                TaskFormField::ExpiresAt   => { state.task_form.expires_at.pop(); }
                TaskFormField::Score
                | TaskFormField::ClassPicker => {}
            }
        }

        Action::TaskFormNextField => {

            let modal = state.task_modal.clone();

            state.task_form.active_field = match state.task_form.active_field {
                TaskFormField::Title       => TaskFormField::Description,
                TaskFormField::Description => TaskFormField::Score,
                TaskFormField::Score       => TaskFormField::ExpiresAt,
                TaskFormField::ExpiresAt   => {
                    if modal == TaskModal::Add {
                        TaskFormField::ClassPicker
                    } else {
                        TaskFormField::Title
                    }
                }
                TaskFormField::ClassPicker => TaskFormField::Title,
            };
        }

        Action::TaskFormCycleScore => {

            let max = FIBONACCI_SCORES.len() - 1;
            state.task_form.score_index =
                if state.task_form.score_index < max {
                    state.task_form.score_index + 1
                } else {
                    0
                };
        }

        // -- tasks: picker navigation --------------------------

        Action::TaskPickerUp => {

            if state.task_form.class_cursor > 0 {
                state.task_form.class_cursor -= 1;
                if state.task_form.class_cursor < state.task_form.class_scroll {
                    state.task_form.class_scroll = state.task_form.class_cursor;
                }
            }
        }

        Action::TaskPickerDown => {

            let len = if let Resource::Success(ref list) = state.available_task_classes {
                list.len()
            } else {
                0
            };

            if len > 0 && state.task_form.class_cursor < len - 1 {
                state.task_form.class_cursor += 1;
                if state.task_form.class_cursor
                    >= state.task_form.class_scroll + PICKER_VISIBLE_ROWS
                {
                    state.task_form.class_scroll =
                        state.task_form.class_cursor + 1 - PICKER_VISIBLE_ROWS;
                }
            }
        }

        Action::TaskPickerSelect => {

            if let Resource::Success(ref list) = state.available_task_classes {
                if list.get(state.task_form.class_cursor).is_some() {
                    state.task_form.selected_class = Some(state.task_form.class_cursor);
                }
            }
        }

        // -- tasks: submit -------------------------------------

        Action::SubmitTaskForm => {

            let can_mutate = state.session.as_ref()
                .map(|s| s.role == "admin" || s.role == "teacher")
                .unwrap_or(false);

            if !can_mutate {
                return Ok(());
            }

            let Some(ref api) = api_client(config, state) else {
                return Ok(());
            };

            let modal = state.task_modal.clone();

            match modal {

                TaskModal::Add => {

                    if state.task_form.title.is_empty() {
                        state.error = Some("Título é obrigatório.".to_string());
                        return Ok(());
                    }

                    let Some(class_id) = resolve_task_class_id(state) else {
                        state.error = Some("Selecione uma turma.".to_string());
                        return Ok(());
                    };

                    let score = FIBONACCI_SCORES[state.task_form.score_index].to_string();

                    let mut fields = vec![
                        ("classId".to_string(), class_id),
                        ("title".to_string(),   state.task_form.title.clone()),
                        ("score".to_string(),   score),
                    ];

                    if !state.task_form.description.is_empty() {
                        fields.push(("description".to_string(), state.task_form.description.clone()));
                    }

                    if !state.task_form.expires_at.is_empty() {
                        fields.push(("expiresAt".to_string(), state.task_form.expires_at.clone()));
                    }

                    match api_tasks::create_task(api, fields).await {

                        Ok(_) => {
                            state.task_modal             = TaskModal::None;
                            state.task_form              = TaskForm::default();
                            state.available_task_classes = Resource::Idle;
                            state.error                  = None;
                        }

                        Err(e) => {
                            state.error = Some(e.to_string());
                            return Ok(());
                        }
                    }
                }

                TaskModal::Edit => {

                    let task_id = if let Resource::Success(ref list) = state.tasks {
                        list.get(state.selected_task_index).map(|t| t.id.clone())
                    } else {
                        None
                    };

                    let Some(id) = task_id else {
                        return Ok(());
                    };

                    let score = FIBONACCI_SCORES[state.task_form.score_index].to_string();

                    let mut fields = vec![("score".to_string(), score)];

                    if !state.task_form.title.is_empty() {
                        fields.push(("title".to_string(), state.task_form.title.clone()));
                    }

                    if !state.task_form.description.is_empty() {
                        fields.push(("description".to_string(), state.task_form.description.clone()));
                    }

                    if !state.task_form.expires_at.is_empty() {
                        fields.push(("expiresAt".to_string(), state.task_form.expires_at.clone()));
                    }

                    match api_tasks::update_task(api, &id, fields).await {

                        Ok(_) => {
                            state.task_modal = TaskModal::None;
                            state.task_form  = TaskForm::default();
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

            // Recarrega lista após add/edit
            let api2 = api_client(config, state).unwrap();
            match api_tasks::list_tasks(&api2).await {
                Ok(list) => {
                    state.selected_task_index = 0;
                    state.tasks = Resource::Success(list);
                }
                Err(e) => {
                    state.tasks = Resource::Error(e.to_string());
                }
            }
        }

        // -- tasks: delete -------------------------------------

        Action::ConfirmDeleteTask => {

            let task_id = if let Resource::Success(ref list) = state.tasks {
                list.get(state.selected_task_index).map(|t| t.id.clone())
            } else {
                None
            };

            let Some(id) = task_id else {
                state.task_modal = TaskModal::None;
                return Ok(());
            };

            let Some(ref api) = api_client(config, state) else {
                return Ok(());
            };

            match api_tasks::delete_task(api, &id).await {

                Ok(_) => {
                    state.task_modal = TaskModal::None;
                    state.error      = None;

                    let api2 = api_client(config, state).unwrap();
                    match api_tasks::list_tasks(&api2).await {
                        Ok(list) => {
                            state.selected_task_index = 0;
                            state.tasks = Resource::Success(list);
                        }
                        Err(e) => {
                            state.tasks = Resource::Error(e.to_string());
                        }
                    }
                }

                Err(e) => {
                    state.error      = Some(e.to_string());
                    state.task_modal = TaskModal::None;
                }
            }
        }

        // ======================================================
        // não implementado
        // ======================================================

        _ => {}
    }

    Ok(())
}
