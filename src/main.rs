use std::{
    fs::File,
    io::{self, Write},
    ops::Deref,
};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, EnumIter)]
pub enum Action {
    PickAction,
    Add,
    AddSubtask,
    MakeSubtask,
    Toggle,
    Delete,
    Display,
    Quit,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    Todo,
    Done,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tasks(Vec<Task>);
impl Tasks {
    fn get_from_id(&self, id: &Uuid) -> Option<&Task> {
        for task in self.0.iter() {
            if &task.id == id {
                return Some(task);
            }
        }
        None
    }

    fn get_from_id_mut(&mut self, id: &Uuid) -> Option<&mut Task> {
        for task in self.0.iter_mut() {
            if &task.id == id {
                return Some(task);
            }
        }
        None
    }

    fn get_index_from_id(&self, id: &Uuid) -> Option<usize> {
        let task_from_id = self.get_from_id(id).unwrap();

        for (idx, task) in self.0.iter().enumerate() {
            if task.id == task_from_id.id {
                return Some(idx);
            }
        }

        None
    }

    fn remove_task_from_subtasks(&mut self, id: &Uuid, task: *const Task) {
        self.0
            .iter_mut()
            .for_each(|x| x.subtasks.retain(|t| !std::ptr::addr_eq(t, task)));
    }

    fn remove_task(&mut self, task: *const Task, task_id: Uuid) {
        self.0.retain(|t| !std::ptr::addr_eq(t, task));
        self.remove_task_from_subtasks(&task_id, task);
    }

    fn display(&self, id: &Uuid) {
        let task = self.get_from_id(id).unwrap();

        println!("Task: {}", task.name);
        println!("  {}", task.description);
        println!("  Status: {:?}", task.status);
        for tasks in task.subtasks.iter() {
            if let Some(subtask) = self.get_from_id(tasks) {
                if !task.subtasks.is_empty() {
                    println!("  Has the subtasks:")
                }

                println!("  > Sub Task: {}", subtask.name);
                println!("  > {}", subtask.description);
                println!("  > Status: {:?}", subtask.status);
            }
        }
        println!()
    }

    fn check_if_subtasks_done(&self, id: &Uuid) -> bool {
        let task = self.get_from_id(id).unwrap();
        for tasks in task.subtasks.iter() {
            match self.get_from_id(tasks).unwrap().status {
                Status::Todo => {
                    return false;
                }
                Status::Done => {}
            }
        }

        true
    }

    fn mark_task_toggle(&mut self, id: Uuid) {
        for task in self.0.iter_mut() {
            if task.id == id {
                task.status = match task.status {
                    Status::Todo => Status::Done,
                    Status::Done => Status::Todo,
                }
            }
        }
    }

    fn add_task(&mut self, task: Task) {
        self.0.push(task);
    }

    fn make_subtask(&mut self, to_task: &Uuid, subtask: &Uuid) {
        for add_task in self.0.iter_mut() {
            if &add_task.id == to_task {
                add_task.subtasks.push(*subtask);
            }
        }
    }

    fn add_subtask(&mut self, add_to: &Uuid, task: Task) {
        let id = task.id;

        for add_task in self.0.iter_mut() {
            if &add_task.id == add_to {
                add_task.subtasks.push(id);
            }
        }

        self.add_task(task);
    }
}

impl Deref for Tasks {
    type Target = Vec<Task>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    id: Uuid,
    name: String,
    description: String,
    status: Status,
    subtasks: Vec<Uuid>,
}

fn main() {
    let mut tasks = load();

    loop {
        let action = menu();

        match action {
            Action::PickAction => {}
            Action::Add => {
                add_task(&mut tasks);
                save(&tasks);
            }
            Action::AddSubtask => {
                add_subtask(&mut tasks);
                save(&tasks);
            }
            Action::MakeSubtask => {
                make_subtask(&mut tasks);
                save(&tasks);
            }
            Action::Toggle => {
                toggle_task(&mut tasks);
                save(&tasks);
            }
            Action::Delete => {
                remove_task(&mut tasks);
                save(&tasks);
            }
            Action::Display => {
                for task in tasks.0.iter() {
                    tasks.display(&task.id);
                }
                pause();
            }
            Action::Quit => {
                save(&tasks);
                return;
            }
        }
    }
}

fn select(tasks: &Tasks) -> Option<&Task> {
    for (idx, task) in tasks.iter().enumerate() {
        print!("{idx}) ");
        tasks.display(&task.id);
    }
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // Read input into the `input` variable
        .expect("Failed to read input");
    let selection = input.trim().parse::<usize>().unwrap();
    for (idx, task) in tasks.iter().enumerate() {
        if idx == selection {
            return Some(task);
        }
    }
    None
}

fn pause() {
    println!("Press any key to continue.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // Read input into the `input` variable
        .expect("Failed to read input");
}

fn confirm() -> bool {
    println!("[y/N]: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // Read input into the `input` variable
        .expect("Failed to read input");
    let selection = input.trim().chars().nth(0).unwrap_or('n');
    if selection == 'y' {
        return true;
    }

    false
}

fn remove_task(tasks: &mut Tasks) {
    let selection = select(&tasks).unwrap();
    println!("Are you sure you wish to delete? ");
    if confirm() {
        let selection_id = selection.id.clone();
        tasks.remove_task(selection, selection_id);
        println!("Task Deleted.");
    }
}

fn add_task(tasks: &mut Tasks) {
    println!("Task Name: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // Read input into the `input` variable
        .expect("Failed to read input");
    let name = input.trim();

    println!("Description: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // Read input into the `input` variable
        .expect("Failed to read input");
    let description = input.trim();

    tasks.0.push(Task {
        id: Uuid::new_v4(),
        name: name.to_owned(),
        description: description.to_owned(),
        status: Status::Todo,
        subtasks: vec![],
    });
}

fn add_subtask(tasks: &mut Tasks) {
    println!("Add subtask to:");
    let add_to = select(&tasks).unwrap().id;
    println!("Task Name: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // Read input into the `input` variable
        .expect("Failed to read input");
    let name = input.trim();

    println!("Description: ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // Read input into the `input` variable
        .expect("Failed to read input");
    let description = input.trim();

    let uuid = Uuid::new_v4();

    let task = Task {
        id: uuid,
        name: name.to_owned(),
        description: description.to_owned(),
        status: Status::Todo,
        subtasks: vec![],
    };

    tasks.add_subtask(&add_to, task);
}

fn make_subtask(tasks: &mut Tasks) {
    println!("Select task to add as a subtask: ");
    let make_subtask = select(tasks).unwrap().id;

    println!("Select task to add subtask to: ");
    let to_task = select(tasks).unwrap().id;

    tasks.make_subtask(&to_task, &make_subtask);
}

fn toggle_task(tasks: &mut Tasks) {
    println!("Select task to toggle status of: ");
    let to_toggle = select(tasks).unwrap().id;

    if tasks.check_if_subtasks_done(&to_toggle) {
        tasks.mark_task_toggle(to_toggle);
    } else {
        println!("You need to complete all subtasks of this task first!");
    }
}

fn menu() -> Action {
    for (idx, action) in Action::iter().enumerate() {
        println!("{idx}) {action:?}");
    }
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input) // Read input into the `input` variable
        .expect("Failed to read input");
    let selection = input.trim().parse::<usize>().unwrap();
    for (idx, action) in Action::iter().enumerate() {
        if idx == selection {
            return action;
        }
    }

    Action::PickAction
}

fn save(tasks: &Tasks) {
    let json_data = serde_json::to_string_pretty(tasks).unwrap();
    let mut file = File::create("tasks.json").expect("Could not open task file");
    file.write_all(json_data.as_bytes())
        .expect("Could not write to task file");
}

fn load() -> Tasks {
    if let Ok(file) = File::open("tasks.json") {
        serde_json::from_reader(file).unwrap_or(Tasks(vec![]))
    } else {
        File::create("tasks.json").expect("Could not create task file");
        Tasks(vec![])
    }
}
