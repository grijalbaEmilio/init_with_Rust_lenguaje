use crate::task::Task;

pub trait AbstractRepo{

    /// Adds a task
    fn add(&mut self, task: Task);

    /// List all tasks
    fn list(&self) -> Vec<Task>;
}