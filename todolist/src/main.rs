use chrono::{DateTime, Local};
use dialoguer::{Input, Select, theme::ColorfulTheme};
use std::fmt;
use std::time::SystemTime;

struct Task {
    text: String,
    is_done: bool,
    date_created: SystemTime,
}

impl Task {
    fn new(text: String, is_done: bool) -> Task {
        return Task {
            text,
            is_done,
            date_created: SystemTime::now(),
        };
    }

    fn toggle_state(&mut self) {
        self.is_done = !self.is_done;
    }
}

impl fmt::Display for Task {
    fn fmt(&self , f: &mut fmt::Formatter) -> fmt::Result {
        let state = if self.is_done { "✅" } else { "⏰" };
        let datetime: DateTime<Local> = self.date_created.into();
        let dateformatted = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
        write!(f, "{state} {}, created at {}", self.text, dateformatted)
    }
}


fn main() {

    let mut tasks: Vec<Task> = vec![];

    loop {
        clearscreen::clear().unwrap();

        let selections = &["➕  Add a Task", "🗑️  Delete a task", "✅  Change a task state", "🚪 Exit"];

        if tasks.len() == 0 {
            println!("No tasks for you")
        } else {
            println!(
                "Tasks:\n{}",
                tasks.iter().map(|item| item.to_string()).collect::<Vec<_>>().join("\n")
            );
        }

        println!("\n");

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick what you want")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();

        match selection {
            0 => {
                let input: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Write the task")
                    .interact_text()
                    .unwrap();
                let new_task = Task::new(input, false);
                tasks.push(new_task);
            }
            1 => {
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("choose the task to delete")
                    .items(
                        &tasks[..]
                            .iter()
                            .map(|item| item.to_string())
                            .collect::<Vec<_>>()[..],
                    )
                    .interact()
                    .unwrap();

                tasks.remove(selection);
            }
            2 => {
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("choose the task to delete")
                    .items(
                        &tasks
                            .iter()
                            .map(|item| item.to_string())
                            .collect::<Vec<_>>()[..],
                    )
                    .interact()
                    .unwrap();

                tasks[selection].toggle_state();
            },
            3 => {
                break;
            }
            _ => (),
        }
    }
}
