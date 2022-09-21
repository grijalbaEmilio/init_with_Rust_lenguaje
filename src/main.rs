mod in_memory_task_repo;
mod abstract_repo;
mod task;
use task::Task;
use chrono::NaiveDate;
use in_memory_task_repo::InMemoryTaskRepo;
use abstract_repo::AbstractRepo;

/// axum backend
/// axis backend
/// rocket backend

fn main() {
    //let date: NaiveDate = NaiveDate::from_ymd(2022, 09, 20);
    let task_one = Task::new("task 001".to_string(), false, None);
    task_one.print_create_date();

    let mut task_repo = InMemoryTaskRepo::new();
    task_repo.add(task_one);
    // task_repo.add(task_one);

    println!("\n{}\n", task_repo);
}