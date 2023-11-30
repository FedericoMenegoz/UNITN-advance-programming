use std::{cell::RefCell, collections::VecDeque, rc::Rc};

trait Task {
    fn execute(&self) -> usize;
}
type Queue = Rc<RefCell<VecDeque<Box<dyn Task>>>>;
struct SumTask {
    n1: usize,
    n2: usize,
}

impl SumTask {
    fn new(n1: usize, n2: usize) -> Self {
        SumTask { n1, n2 }
    }
}

impl Task for SumTask {
    fn execute(&self) -> usize {
        self.n1 + self.n2
    }
}

struct LenTask {
    s: String,
}

impl LenTask {
    fn new(s: String) -> LenTask {
        LenTask { s }
    }
}

impl Task for LenTask {
    fn execute(&self) -> usize {
        self.s.len()
    }
}

struct Tasker {
    queue: Queue,
}

impl<'a> Tasker {
    fn new() -> Tasker {
        let queue: Queue = Rc::new(RefCell::new(VecDeque::new()));
        Tasker { queue }
    }

    fn get_tasker(&self) -> Tasker {
        let queue = Rc::clone(&self.queue);
        Tasker { queue }
    }

    fn get_executer(&self) -> Executer {
        let queue = Rc::clone(&self.queue);
        Executer { queue }
    }

    fn schedule_task(&mut self, task: Box<dyn Task>) {
        self.queue.borrow_mut().push_back(task);
    }
}

struct Executer {
    queue: Queue,
}

impl Executer {
    fn execute_task(&mut self) -> Option<usize> {
        self.queue
            .borrow_mut()
            .pop_front()
            .map(|task| task.execute())
    }
}

pub fn es3() {
    println!("Test 1:");
    test_1();
    println!("Test 2:");
    test_2();
}

fn test_1() {
    macro_rules! sum_task {
        (let $task: ident =$n1: literal + $n2: literal) => {
            let $task: Box<dyn Task> = Box::new(SumTask::new($n1, $n2));
        };
    }

    sum_task!(let t1 = 1+1);
    sum_task!(let t2 = 2+2);
    sum_task!(let t3 = 3+3);
    sum_task!(let t4 = 4+4);
    sum_task!(let t5 = 5+5);
    sum_task!(let t6 = 6+6);
    sum_task!(let t7 = 7+7);

    let mut tasker = Tasker::new();
    let mut executer = tasker.get_executer();

    println!("{:?}", executer.execute_task());

    tasker.schedule_task(t1);
    tasker.schedule_task(t2);

    println!("{:?}", executer.execute_task());

    tasker.schedule_task(t3);
    tasker.schedule_task(t4);
    tasker.schedule_task(t5);
    tasker.schedule_task(t6);
    tasker.schedule_task(t7);

    println!("{:?}", executer.execute_task());
    println!("{:?}", executer.execute_task());
    println!("{:?}", executer.execute_task());
    println!("{:?}", executer.execute_task());
    println!("{:?}", executer.execute_task());
    println!("{:?}", executer.execute_task());
    println!("{:?}", executer.execute_task());
}

fn test_2() {
    macro_rules! sum_task {
        (let $task: ident =$n1: literal + $n2: literal) => {
            let $task: Box<dyn Task> = Box::new(SumTask::new($n1, $n2));
        };
    }
    macro_rules! len_task {
        (let $task: ident =$s: literal) => {
            let $task: Box<dyn Task> = Box::new(LenTask::new($s.to_owned()));
        };
    }

    sum_task!(let t1 = 10+1);
    len_task!(let t2 = "four");
    let mut tasker1 = Tasker::new();
    let mut tasker2 = tasker1.get_tasker();
    let mut executer1 = tasker2.get_executer();
    let mut executer2 = tasker1.get_executer();
    tasker1.schedule_task(t1);
    tasker2.schedule_task(t2);
    println!("{:?}", executer1.execute_task());
    println!("{:?}", executer2.execute_task());
}
