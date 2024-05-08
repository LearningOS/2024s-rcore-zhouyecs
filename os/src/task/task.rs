//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;
use crate::timer::get_time_ms;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// The task info
    pub task_info: TaskInfo,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}

/// The task information
#[derive(Copy, Clone, PartialEq)]
pub struct TaskInfo {
    /// The control block information (TaskStatus includes this)
    /// pub status: TaskStatus,
    /// The number of syscalls called by the task
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
    /// The time from now to the first syscall of task
    pub time: usize,
}

impl TaskInfo {
    /// Create a new TaskInfo instance.
    pub fn init() -> Self {
        Self {
            // status: TaskStatus::Ready,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: get_time_ms(),
        }
    }
}
