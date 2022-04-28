use chrono::DateTime;
use chrono::Local;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;
use url::Url;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Term {
    pub id: i64,
    pub name: String,
    #[serde(rename = "start_at")]
    pub start_at: DateTime<Local>,
    #[serde(rename = "end_at")]
    pub end_at: Option<DateTime<Local>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CourseProgress {
    #[serde(rename = "requirement_count")]
    pub requirement_count: i64,
    #[serde(rename = "requirement_completed_count")]
    pub requirement_completed_count: i64,
    #[serde(rename = "next_requirement_url")]
    pub next_requirement_url: Option<Url>,
    #[serde(rename = "completed_at")]
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    pub id: i64,
    #[serde(rename = "sis_course_id")]
    pub sis_course_id: Option<Value>,
    pub uuid: String,
    #[serde(rename = "integration_id")]
    pub integration_id: Option<Value>,
    #[serde(rename = "sis_import_id")]
    pub sis_import_id: Option<i64>,
    pub name: String,
    #[serde(rename = "course_code")]
    pub course_code: String,
    #[serde(rename = "original_name")]
    pub original_name: Option<String>,
    #[serde(rename = "workflow_state")]
    pub workflow_state: Option<WorkflowState>,
    #[serde(rename = "account_id")]
    pub account_id: i64,
    #[serde(rename = "root_account_id")]
    pub root_account_id: i64,
    #[serde(rename = "enrollment_term_id")]
    pub enrollment_term_id: i64,
    #[serde(rename = "grading_periods")]
    pub grading_periods: Value,
    #[serde(rename = "grading_standard_id")]
    pub grading_standard_id: i64,
    #[serde(rename = "grade_passback_setting")]
    pub grade_passback_setting: String,
    #[serde(rename = "created_at")]
    pub created_at: DateTime<Local>,
    #[serde(rename = "start_at")]
    pub start_at: DateTime<Local>,
    #[serde(rename = "end_at")]
    pub end_at: DateTime<Local>,
    pub locale: String,
    pub enrollments: Value,
    #[serde(rename = "total_students")]
    pub total_students: Option<i64>,
    pub calendar: Value,
    #[serde(rename = "default_view")]
    pub default_view: DefaultView,
    #[serde(rename = "syllabus_body")]
    pub syllabus_body: String,
    #[serde(rename = "needs_grading_count")]
    pub needs_grading_count: Option<i64>,
    pub term: Option<Term>,
    #[serde(rename = "course_progress")]
    pub course_progress: Option<CourseProgress>,
    #[serde(rename = "apply_assignment_group_weights")]
    pub apply_assignment_group_weights: bool,
    pub permissions: Option<Permissions>,
    #[serde(rename = "is_public")]
    pub is_public: bool,
    #[serde(rename = "is_public_to_auth_users")]
    pub is_public_to_auth_users: bool,
    #[serde(rename = "public_syllabus")]
    pub public_syllabus: bool,
    #[serde(rename = "public_syllabus_to_auth")]
    pub public_syllabus_to_auth: bool,
    #[serde(rename = "public_description")]
    pub public_description: Option<String>,
    #[serde(rename = "storage_quota_mb")]
    pub storage_quota_mb: i64,
    #[serde(rename = "storage_quota_used_mb")]
    pub storage_quota_used_mb: i64,
    #[serde(rename = "hide_final_grades")]
    pub hide_final_grades: bool,
    pub license: String,
    #[serde(rename = "allow_student_assignment_edits")]
    pub allow_student_assignment_edits: bool,
    #[serde(rename = "allow_wiki_comments")]
    pub allow_wiki_comments: bool,
    #[serde(rename = "allow_student_forum_attachments")]
    pub allow_student_forum_attachments: bool,
    #[serde(rename = "open_enrollment")]
    pub open_enrollment: bool,
    #[serde(rename = "self_enrollment")]
    pub self_enrollment: bool,
    #[serde(rename = "restrict_enrollments_to_course_dates")]
    pub restrict_enrollments_to_course_dates: bool,
    #[serde(rename = "course_format")]
    pub course_format: String,
    #[serde(rename = "access_restricted_by_date")]
    pub access_restricted_by_date: Option<bool>,
    #[serde(rename = "time_zone")]
    pub time_zone: String,
    pub blueprint: Option<bool>,
    #[serde(rename = "blueprint_restrictions")]
    pub blueprint_restrictions: Option<BlueprintRestrictions>,
    #[serde(rename = "blueprint_restrictions_by_object_type")]
    pub blueprint_restrictions_by_object_type: Option<BlueprintRestrictionsByObjectType>,
    pub template: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Permissions {
    #[serde(rename = "create_discussion_topic")]
    pub create_discussion_topic: bool,
    #[serde(rename = "create_announcement")]
    pub create_announcement: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlueprintRestrictions {
    pub content: bool,
    pub points: bool,
    #[serde(rename = "due_dates")]
    pub due_dates: bool,
    #[serde(rename = "availability_dates")]
    pub availability_dates: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlueprintRestrictionsByObjectType {
    pub assignment: Assignment,
    #[serde(rename = "wiki_page")]
    pub wiki_page: WikiPage,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Assignment {
    pub content: bool,
    pub points: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WikiPage {
    pub content: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "UPPERCASE"))]
pub enum WorkflowState {
    Unpublished,
    Available,
    Completed,
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase", deserialize = "UPPERCASE"))]
pub enum DefaultView {
    Feed,
    Wiki,
    Modules,
    Assignments,
    Syllabus,
}

pub struct Page<T> {
    pub items: Vec<T>,
    pub current: Url,
    pub next: Option<Url>,
    pub prev: Option<Url>,
    pub first: Url,
    pub last: Option<Url>,
}
