use serde::{Deserialize,  Serialize};

#[derive(Serialize, Deserialize)]

pub struct Question {
    pub title : String,
    pub description : String,
}

#[derive(Serialize, Deserialize,Debug,PartialEq,Clone)]
pub struct QuestionDetail{
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionId {
    pub question_uuid: String,  // public `question_uuid` field of type String
}

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub question_uuid: String,  // public `question_uuid` field of type String
    pub content: String,        // public `content` field of type String
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AnswerDetail {
    pub answer_uuid: String,    // public `answer_uuid` field of type String
    pub question_uuid: String,  // public `question_uuid` field of type String
    pub content: String,        // public `content` field of type String
    pub created_at: String,     // public `created_at` field of type String
}

#[derive(Serialize, Deserialize)]
pub struct AnswerId {
    pub answer_uuid: String,    // public `answer_uuid` field of type String
}