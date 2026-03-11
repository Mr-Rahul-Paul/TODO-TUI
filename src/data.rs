use chrono::{NaiveDate};

pub enum Priority{
    Low,
    Med,
    High,
} 
pub struct Task{
    pub id: u32,
    pub title: String,
    pub done: bool,
    pub prioirty: Priority,
    pub cateogry: String, 
    pub date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub string:  Vec<String>,
    pub created_at: NaiveDate,
    pub completed_at: Option<NaiveDate>,
}