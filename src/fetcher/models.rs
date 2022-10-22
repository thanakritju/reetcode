use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Problems {
    #[serde(rename = "user_name")]
    pub user_name: String,
    #[serde(rename = "num_solved")]
    pub num_solved: i64,
    #[serde(rename = "num_total")]
    pub num_total: i64,
    #[serde(rename = "ac_easy")]
    pub ac_easy: i64,
    #[serde(rename = "ac_medium")]
    pub ac_medium: i64,
    #[serde(rename = "ac_hard")]
    pub ac_hard: i64,
    #[serde(rename = "stat_status_pairs")]
    pub stat_status_pairs: Vec<StatStatusPair>,
    #[serde(rename = "frequency_high")]
    pub frequency_high: i64,
    #[serde(rename = "frequency_mid")]
    pub frequency_mid: i64,
    #[serde(rename = "category_slug")]
    pub category_slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatStatusPair {
    pub stat: Stat,
    pub status: Option<String>,
    pub difficulty: Difficulty,
    #[serde(rename = "paid_only")]
    pub paid_only: bool,
    #[serde(rename = "is_favor")]
    pub is_favor: bool,
    pub frequency: i64,
    pub progress: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stat {
    #[serde(rename = "question_id")]
    pub question_id: i64,
    #[serde(rename = "question__article__live")]
    pub question_article_live: Option<bool>,
    #[serde(rename = "question__article__slug")]
    pub question_article_slug: Option<String>,
    #[serde(rename = "question__article__has_video_solution")]
    pub question_article_has_video_solution: Option<bool>,
    #[serde(rename = "question__title")]
    pub question_title: String,
    #[serde(rename = "question__title_slug")]
    pub question_title_slug: String,
    #[serde(rename = "question__hide")]
    pub question_hide: bool,
    #[serde(rename = "total_acs")]
    pub total_acs: i64,
    #[serde(rename = "total_submitted")]
    pub total_submitted: i64,
    #[serde(rename = "frontend_question_id")]
    pub frontend_question_id: i64,
    #[serde(rename = "is_new_question")]
    pub is_new_question: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Difficulty {
    pub level: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Problem {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub question: Question,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    pub question_id: String,
    pub question_frontend_id: String,
    pub question_title: String,
    pub enable_debugger: bool,
    pub enable_run_code: bool,
    pub enable_submit: bool,
    pub enable_test_mode: bool,
    pub example_testcase_list: Vec<String>,
    pub meta_data: String,
}
