pub trait TaskSet {
    // Task name
    fn task_name(&self) -> &str;

    // Run task instance
    fn run_task_inst(&mut self, id : usize);

    fn valid_defualt_correct(&self) -> bool;

    fn get_num_tasks(&self) -> usize;
}

/// Compute the Fibonacci number
/// 
/// #Tag: Compute-incentive
pub struct FibonacciTask {
    num_tasks : usize,
    idx : usize,
    output : Vec<usize>,
    name : String,
}

impl FibonacciTask {
    pub fn force_compute(n : usize) ->usize {
        if n < 2 {
            1
        } else {
            Self::force_compute(n-1) + Self::force_compute(n-2)
        }
    }
}

/// Defualt workload for FibonacciTask
/// 
/// Compute the 25th Fibonacci recursively, each with 256 tasks
/// 
/// It's a very compute-intensive task instance
/// 
impl Default for FibonacciTask {
    fn default() -> Self {
        FibonacciTask {
            num_tasks : 256,
            idx : 25,
            output: vec![0; 256],
            name : String::from("Fibonacci"),
        }
    }
}

impl TaskSet for FibonacciTask {

    fn task_name(&self) -> &str {
        &self.name
    }

    fn run_task_inst(&mut self, id : usize) {
        self.output[id] = Self::force_compute(self.idx);
    }

    fn valid_defualt_correct(&self) -> bool {
        for output in &self.output {
            if *output != 121393 {
                return false;
            }
        }
        true
    }

    fn get_num_tasks(&self) -> usize {
        self.num_tasks
    }
}
