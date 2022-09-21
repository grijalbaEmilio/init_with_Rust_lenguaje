use crate::task::Task;
use crate::abstract_repo::AbstractRepo;
use std::fmt::Display;

//#[derive(Debug)]
pub struct InMemoryTaskRepo{
    tasks: Vec<Task>
}


impl InMemoryTaskRepo{

    pub fn new() -> InMemoryTaskRepo{
        InMemoryTaskRepo { tasks: vec![] }
    }

}

impl AbstractRepo for InMemoryTaskRepo{

    fn add(&mut self, task: Task){
        self.tasks.push(task);
    }

    fn list(&self) -> Vec<Task>{
        self.tasks.clone()
    }
}

impl Display for InMemoryTaskRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Total task: {}\nfirst task is: {:?}", self.tasks.len(), self.tasks.get(0))
    }
}