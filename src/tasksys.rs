use crate::taskinst::TaskSet;
use std::thread;

pub trait TaskSys {
    fn sys_name(&self) -> &str;
    // Launch bulk tasks
    fn launch_bulk_tasks<T : TaskSet>(&self, task_set : &mut T);
}

// =======================================================
// Setial task system implementation
// =======================================================

/// Serial Task system, run tasks serial
pub struct SerialTaskSys {
    name : String,
}

impl SerialTaskSys {
    pub fn new() -> Self {
        SerialTaskSys {
            // Set the name field to "Serial"
            name: String::from("Serial"),
        }
    }
}

impl TaskSys for SerialTaskSys {

    fn sys_name(&self) -> &str {
        &self.name
    }

    fn launch_bulk_tasks<T : TaskSet>(&self, task_set : &mut T) {
        for i in 0..task_set.get_num_tasks() {
            task_set.run_task_inst(i);
        }
    }
}

// =======================================================
// Single queue thread pool implementation
// =======================================================

struct SingleQueueThreadPoolSys {
    name : String,
    
}