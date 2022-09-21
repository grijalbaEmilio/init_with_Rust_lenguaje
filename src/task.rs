use chrono::{NaiveDate, Utc};

#[derive(Debug, Clone)]
pub struct Task{
    pub name: String,
    pub done: bool,
    create_date: NaiveDate,
    pub due_date: Option<NaiveDate>,
}

impl Task{

    pub fn new(name: String, done: bool, due_date: Option<NaiveDate>) -> Task {
        Task{
            name,
            done,
            due_date,
            create_date: Utc::today().naive_utc()
        }
    }

    pub fn print_create_date(&self){
        println!("{}", self.create_date)
    }



}