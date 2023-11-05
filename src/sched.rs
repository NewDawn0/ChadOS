//     ____ _               _  ___  ____
//    / ___| |__   __ _  __| |/ _ \/ ___|
//   | |   | '_ \ / _` |/ _` | | | \___ \
//   | |___| | | | (_| | (_| | |_| |___) |
//    \____|_| |_|\__,_|\__,_|\___/|____/
//    https://github.com/NewDawn0/ChadOS
//
//   @Author: NewDawn0
//   @Contributors: -
//   @License: MIT
//
//   File: src/sched.rs
//   Desc: Scheduling implementation

// RustDoc
//! # ChadOS Scheduling Implementation
//!
//! This module provides the implementation of the task scheduler for ChadOS, an operating system
//! implemented in Rust. It includes the `Exec` struct and related types for managing and scheduling tasks.
//!
//! For more information about ChadOS, visit [the ChadOS GitHub repository](https://github.com/NewDawn0/ChadOS).
//!
//! ## Author
//!
//! - [NewDawn0](https://github.com/NewDawn0)
//!
//! ## License
//!
//! This code is licensed under the MIT License. See the MIT License section below for details.
//!
//! # File: src/sched.rs
//!
//! This file contains the task scheduling implementation.

// Imports
use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, task::Wake};
#[cfg(feature = "unused-task")]
use core::sync::atomic::{AtomicU64, Ordering};
use core::{
    arch::asm,
    future::Future,
    pin::Pin,
    task::{Context as Cx, Poll, Waker},
};
use crossbeam_queue::ArrayQueue;

/// The main task scheduler for ChadOS.
pub struct Exec {
    tasks: BTreeMap<TaskId, Task>,
    queue: Arc<ArrayQueue<TaskId>>,
    cache: BTreeMap<TaskId, Waker>,
}

/// Represents a task that can be scheduled and executed.
pub struct Task {
    _id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    /// Creates a new task from a given future.
    ///
    /// # Parameters
    ///
    /// - `future`: A future representing the task.
    #[cfg(feature = "unused-task")]
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            _id: TaskId::new(),
            future: Box::pin(future),
        }
    }

    /// Polls the task to check if it is ready to execute.
    ///
    /// # Parameters
    ///
    /// - `cx`: The task's context.
    fn poll(&mut self, cx: &mut Cx) -> Poll<()> {
        self.future.as_mut().poll(cx)
    }
}

/// Represents a task waker that can wake up a specific task.
struct TaskWaker {
    id: TaskId,
    queue: Arc<ArrayQueue<TaskId>>,
}

impl TaskWaker {
    /// Creates a new task waker for a specific task.
    ///
    /// # Parameters
    ///
    /// - `id`: The ID of the task to wake.
    /// - `queue`: The task queue.
    #[allow(clippy::new_ret_no_self)]
    fn new(id: TaskId, queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(Self { id, queue }))
    }

    /// Wakes up the associated task.
    fn wake_task(&self) {
        self.queue.push(self.id).expect("Queue full");
    }
}

impl Wake for TaskWaker {
    fn wake(self: Arc<Self>) {
        self.wake_task()
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.wake_task()
    }
}

/// Represents a task ID.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct TaskId(u64);
#[cfg(feature = "unused-task")]
impl TaskId {
    /// Generates a new unique task ID.
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }
}

impl Exec {
    /// Creates a new `Exec` instance for task scheduling.
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            queue: Arc::new(ArrayQueue::new(100)),
            cache: BTreeMap::new(),
        }
    }

    /// Spawns a new task for execution.
    ///
    /// # Parameters
    ///
    /// - `task`: The task to be spawned.
    #[cfg(feature = "unused-task")]
    pub fn spawn(&mut self, task: Task) {
        let id = task.id;
        match self.tasks.insert(task.id, task) {
            Some(_) => panic!("Task with same id exists"),
            None => self.queue.push(id).expect("Queue full"),
        }
    }

    /// Runs the task scheduler.
    pub fn run(&mut self) -> ! {
        loop {
            self.ready();
            self.sleep();
        }
    }
    fn ready(&mut self) {
        let Self {
            tasks,
            queue,
            cache,
        } = self;

        while let Some(id) = queue.pop() {
            let task = match tasks.get_mut(&id) {
                Some(task) => task,
                None => continue, // task no longer exists
            };
            let waker = cache
                .entry(id)
                .or_insert_with(|| TaskWaker::new(id, queue.clone()));
            let mut cx = Cx::from_waker(waker);
            match task.poll(&mut cx) {
                Poll::Ready(()) => {
                    // task done -> remove it and its cached waker
                    tasks.remove(&id);
                    cache.remove(&id);
                }
                Poll::Pending => {}
            }
        }
    }
    fn sleep(&self) {
        unsafe {
            asm!("cli");
            if self.queue.is_empty() {
                asm!("sti");
                asm!("hlt");
            }
            asm!("sti");
        }
    }
}
