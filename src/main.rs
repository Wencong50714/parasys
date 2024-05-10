use std::process::exit;
use std::time::Instant;

use parasys::tasksys::{SerialTaskSys, TaskSys};
use parasys::taskinst::{FibonacciTask, TaskSet};

/// The result for task system execution
/// 
/// # Parameter:
/// - time: the time to execute tasks
/// - pass: is result meet the expectation
/// - num_turn: execution times
struct Result {
    time : u128,
    pass : bool,
    num_turn : usize,
}

fn main() {
    
    let mut sys_vector = Vec::new();
    sys_vector.push(SerialTaskSys::new());
    // Add New sys implementation Here !
    
    let mut task_vector= Vec::new();
    task_vector.push(FibonacciTask::default());
    // Add New workload Here!


    // Execute Differents tasks in Different System 
    for task in task_vector.iter_mut() {
        for sys in sys_vector.iter() {
            run_tasks(sys, task);
            println!();
        }
    }
}

fn run_tasks<T : TaskSys, I : TaskSet>(sys : &T, inst : &mut I) {
    let mut result = Result {
        time: 0, 
        pass : true,
        num_turn : 5,
    };

    println!("========================================");
    println!("| Task {}", inst.task_name());
    println!("========================================");

    // Compute the execution time
    let start_time = Instant::now();
    sys.launch_bulk_tasks(inst);
    let end_time = Instant::now();

    result.time = (end_time - start_time).as_millis();
    result.pass = inst.valid_defualt_correct();

    if result.pass {
        println!("| {}: use {:.2} ms",sys.sys_name(), result.time);
    } else {
        println!("test failed!");
        exit(0);
    }
}
