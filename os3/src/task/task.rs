//! Types related to task management
use crate::config::MAX_SYSCALL_NUM;

use super::TaskContext;

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    pub task_syscall_cnt: [u32;MAX_SYSCALL_NUM],
    pub task_init_time: usize 
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
