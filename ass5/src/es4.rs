#[derive(Debug, Clone)]
struct Tasks {
    tasks: Vec<Task>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Task {
    name: String,
    priority: i32,
    done: bool,
}

impl Iterator for Tasks {
    type Item = Tasks;
    fn next(&mut self) -> Option<Self::Item> {
        Some(Tasks {
            tasks: self
                .tasks
                .clone()
                .into_iter()
                .filter(|task| !task.done)
                .collect(),
        })
    }
}

pub fn es4() {
    let t1 = Task {
        name: "Go to the loo!".to_owned(),
        priority: 1,
        done: false,
    };

    let t2 = Task {
        name: "Study more!".to_owned(),
        priority: 2,
        done: false,
    };

    let t3 = Task {
        name: "Drink beer!".to_owned(),
        priority: 3,
        done: true,
    };

    let tasks = vec![t1, t2, t3];
    let mut tasks = Tasks { tasks };
    println!("{:?}", tasks);
    let tasks = tasks.next();
    println!("{:?}", tasks);
}
