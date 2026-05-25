use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "role", rename_all = "lowercase")]
pub enum DashboardData {
    Admin(AdminDashboard),
    Teacher(TeacherDashboard),
    Student(StudentDashboard),
}

// ---- Admin ----------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct AdminDashboard {
    pub users:   AdminUsers,
    pub classes: AdminClasses,
    pub tasks:   AdminTasks,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AdminUsers {
    pub total:    u32,
    pub admins:   u32,
    pub teachers: u32,
    pub students: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AdminClasses {
    pub total: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdminTasks {
    pub total:         u32,
    pub submissions:   u32,
    pub pending_grades: u32,
}

// ---- Teacher --------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct TeacherDashboard {
    pub overview: TeacherOverview,
    pub classes:  Vec<TeacherClass>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeacherOverview {
    pub total_classes:  u32,
    pub total_students: u32,
    pub total_tasks:    u32,
    pub pending_grades: u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeacherClass {
    pub id:            String,
    pub code:          String,
    pub period:        String,
    pub grade:         String,
    pub student_count: u32,
    pub task_count:    u32,
    pub pending_grades: u32,
}

// ---- Student --------------------------------------------------

#[derive(Debug, Clone, Deserialize)]
pub struct StudentDashboard {
    pub tasks:  StudentTasks,
    pub score:  StudentScore,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StudentTasks {
    pub total:     u32,
    pub submitted: u32,
    pub pending:   u32,
    pub expired:   u32,
    pub graded:    u32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudentScore {
    pub total_earned:   u32,
    pub total_possible: u32,
    pub average:        f64,
}
