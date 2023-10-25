use conquer_once::spin::OnceCell;
use core::{pin::Pin, task::Context as Cx, task::Poll};
use crossbeam_queue::ArrayQueue;
use futures_util::{stream::Stream, task::AtomicWaker};
static QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();
static WAKER: AtomicWaker = AtomicWaker::new();

pub mod scancode {
    use super::{ScancodeStream, QUEUE, WAKER};
    use crate::{eprintln, print, wprintln};
    use futures_util::stream::StreamExt;
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    pub fn add(scancode: u8) {
        match QUEUE.try_get() {
            Err(_) => {
                eprintln!("Scancode queue uninitalized");
            }
            Ok(queue) => match queue.push(scancode) {
                Err(_) => {
                    wprintln!("Scancode queue full; dropping key input");
                }
                Ok(_) => WAKER.wake(),
            },
        }
    }
    pub async fn print_keys() {
        let mut codes = ScancodeStream::new();
        let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);
        while let Some(code) = codes.next().await {
            if let Ok(Some(event)) = keyboard.add_byte(code) {
                if let Some(key) = keyboard.process_keyevent(event) {
                    match key {
                        DecodedKey::Unicode(r#char) => print!("{}", r#char),
                        DecodedKey::RawKey(key) => print!("{:?}", key),
                    }
                }
            }
        }
    }
}

pub struct ScancodeStream {
    _priv: (),
}
impl ScancodeStream {
    pub fn new() -> Self {
        QUEUE
            .try_init_once(|| ArrayQueue::new(100))
            .expect("Failed to initalize ScancodeStream");
        Self { _priv: () }
    }
}
impl Stream for ScancodeStream {
    type Item = u8;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Cx<'_>) -> Poll<Option<Self::Item>> {
        let queue = QUEUE.try_get().expect("QUEUE not initalized");
        if let Some(code) = queue.pop() {
            return Poll::Ready(Some(code));
        }
        WAKER.register(&cx.waker());
        match queue.pop() {
            Some(code) => {
                WAKER.take();
                return Poll::Ready(Some(code));
            }
            None => Poll::Pending,
        }
    }
}
