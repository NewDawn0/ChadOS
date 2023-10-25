use alloc::{boxed::Box, collections::BTreeMap, sync::Arc, task::Wake};
use core::{
    arch::asm,
    future::Future,
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context as Cx, Poll, Waker},
};
use crossbeam_queue::ArrayQueue;

pub struct Exec {
    tasks: BTreeMap<TaskId, Task>,
    queue: Arc<ArrayQueue<TaskId>>,
    cache: BTreeMap<TaskId, Waker>,
}
pub struct Task {
    id: TaskId,
    future: Pin<Box<dyn Future<Output = ()>>>,
}
impl Task {
    pub fn new(future: impl Future<Output = ()> + 'static) -> Self {
        Self {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }
    fn poll(&mut self, cx: &mut Cx) -> Poll<()> {
        self.future.as_mut().poll(cx)
    }
}
struct TaskWaker {
    id: TaskId,
    queue: Arc<ArrayQueue<TaskId>>,
}
impl TaskWaker {
    fn new(id: TaskId, queue: Arc<ArrayQueue<TaskId>>) -> Waker {
        Waker::from(Arc::new(Self { id, queue }))
    }
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

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct TaskId(u64);
impl TaskId {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        Self(NEXT.fetch_add(1, Ordering::Relaxed))
    }
}

impl Exec {
    pub fn new() -> Self {
        Self {
            tasks: BTreeMap::new(),
            queue: Arc::new(ArrayQueue::new(100)),
            cache: BTreeMap::new(),
        }
    }
    pub fn spawn(&mut self, task: Task) {
        let id = task.id;
        match self.tasks.insert(task.id, task) {
            Some(_) => panic!("Task with same id exists"),
            None => self.queue.push(id).expect("Queue full"),
        }
    }
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
