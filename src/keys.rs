#[cfg(test)]
use crate::test;
use crate::{
    cfg::{interrupt::KEYBOARD_PORT, keys::LAYOUT},
    interrupt::handler::set_irq_handler,
    print,
};
use core::sync::atomic::{AtomicBool, Ordering};
use pc_keyboard::{
    layouts, DecodedKey, Error, HandleControl::MapLettersToUnicode, KeyCode as KC, KeyEvent,
    KeyState as KS, Keyboard, ScancodeSet1,
};
use spin::Mutex;
use x86_64::instructions::port::Port;

static KEYBOARD: Mutex<Layout> = Mutex::new(Layout::new(LAYOUT));
pub static ALT: AtomicBool = AtomicBool::new(false);
pub static CTRL: AtomicBool = AtomicBool::new(false);
pub static SHIFT: AtomicBool = AtomicBool::new(false);
pub static META: AtomicBool = AtomicBool::new(false);

macro_rules! layout {
    ($layout:ident) => {
        Keyboard::new(ScancodeSet1::new(), layouts::$layout, MapLettersToUnicode)
    };
}
#[derive(Copy, Clone)]
pub enum WrappedLayout {
    Us,
    Uk,
    Azerty,
    De,
    Jis,
    Colemak,
    Dvorak,
    Dvp,
}

pub enum Layout {
    Us(Keyboard<layouts::Us104Key, ScancodeSet1>),
    Uk(Keyboard<layouts::Uk105Key, ScancodeSet1>),
    Azerty(Keyboard<layouts::Azerty, ScancodeSet1>),
    De(Keyboard<layouts::De105Key, ScancodeSet1>),
    Jis(Keyboard<layouts::Jis109Key, ScancodeSet1>),
    Colemak(Keyboard<layouts::Colemak, ScancodeSet1>),
    Dvorak(Keyboard<layouts::Dvorak104Key, ScancodeSet1>),
    Dvp(Keyboard<layouts::DVP104Key, ScancodeSet1>),
}

impl Layout {
    pub const fn new(layout: WrappedLayout) -> Self {
        match layout {
            WrappedLayout::Us => Layout::Us(layout!(Us104Key)),
            WrappedLayout::Uk => Layout::Uk(layout!(Uk105Key)),
            WrappedLayout::Azerty => Layout::Azerty(layout!(Azerty)),
            WrappedLayout::De => Layout::De(layout!(De105Key)),
            WrappedLayout::Jis => Layout::Jis(layout!(Jis109Key)),
            WrappedLayout::Colemak => Layout::Colemak(layout!(Colemak)),
            WrappedLayout::Dvorak => Layout::Dvorak(layout!(Dvorak104Key)),
            WrappedLayout::Dvp => Layout::Dvp(layout!(DVP104Key)),
        }
    }
    fn add_byte(&mut self, code: u8) -> Result<Option<KeyEvent>, Error> {
        match self {
            Layout::Us(inner) => inner.add_byte(code),
            Layout::Uk(inner) => inner.add_byte(code),
            Layout::Azerty(inner) => inner.add_byte(code),
            Layout::De(inner) => inner.add_byte(code),
            Layout::Jis(inner) => inner.add_byte(code),
            Layout::Colemak(inner) => inner.add_byte(code),
            Layout::Dvorak(inner) => inner.add_byte(code),
            Layout::Dvp(inner) => inner.add_byte(code),
        }
    }

    fn process_keyevent(&mut self, event: KeyEvent) -> Option<DecodedKey> {
        match self {
            Layout::Us(inner) => inner.process_keyevent(event),
            Layout::Uk(inner) => inner.process_keyevent(event),
            Layout::Azerty(inner) => inner.process_keyevent(event),
            Layout::De(inner) => inner.process_keyevent(event),
            Layout::Jis(inner) => inner.process_keyevent(event),
            Layout::Colemak(inner) => inner.process_keyevent(event),
            Layout::Dvorak(inner) => inner.process_keyevent(event),
            Layout::Dvp(inner) => inner.process_keyevent(event),
        }
    }
}

pub fn init() {
    set_irq_handler(1, key_handler)
}

fn read_scancode() -> u8 {
    unsafe { Port::new(KEYBOARD_PORT).read() }
}

fn send_key(c: char) {
    print!("{}", c);
}

fn send_csi(code: &str) {
    send_key('\x1B'); // ESC
    send_key('[');
    for c in code.chars() {
        send_key(c);
    }
}

fn key_handler() {
    let mut keyboard = KEYBOARD.lock();
    let code = read_scancode();
    if let Ok(Some(event)) = keyboard.add_byte(code) {
        let ord = Ordering::Relaxed;
        match event.code {
            KC::LAlt | KC::RAlt2 => ALT.store(event.state == KS::Down, ord),
            KC::LShift | KC::RShift => SHIFT.store(event.state == KS::Down, ord),
            KC::LControl | KC::RControl | KC::RControl2 => CTRL.store(event.state == KS::Down, ord),
            KC::LWin | KC::RWin => META.store(event.state == KS::Down, ord),
            _ => {}
        }
        let is_alt = ALT.load(ord);
        let is_ctrl = CTRL.load(ord);
        let is_shift = SHIFT.load(ord);
        if let Some(key) = keyboard.process_keyevent(event) {
            match key {
                DecodedKey::RawKey(KC::PageUp) => send_csi("5~"),
                DecodedKey::RawKey(KC::PageDown) => send_csi("6~"),
                DecodedKey::RawKey(KC::ArrowUp) => send_csi("A"),
                DecodedKey::RawKey(KC::ArrowDown) => send_csi("B"),
                DecodedKey::RawKey(KC::ArrowRight) => send_csi("C"),
                DecodedKey::RawKey(KC::ArrowLeft) => send_csi("D"),
                DecodedKey::Unicode('\t') if is_shift => send_csi("Z"), // Convert Shift-Tab into Backtab
                DecodedKey::Unicode(c) => send_key(c),
                _ => {}
            };
        }
    }
}

#[test_case]
fn test_layout_creation() {
    let layouts = [
        WrappedLayout::Us,
        WrappedLayout::Uk,
        WrappedLayout::Azerty,
        WrappedLayout::De,
        WrappedLayout::Jis,
        WrappedLayout::Colemak,
        WrappedLayout::Dvorak,
        WrappedLayout::Dvp,
    ];
    for layout in layouts.iter() {
        let _ = Layout::new(*layout);
    }
    test!("KEYZ create layouts", assert_eq!(1, 1));
}
#[test_case]
fn test_key_handler() {
    use core::mem::ManuallyDrop;
    let mut layout = ManuallyDrop::new(Layout::Us(layout!(Us104Key)));
    let event = KeyEvent {
        state: KS::Down,
        code: KC::A,
    };
    let res = layout.process_keyevent(event);
    test!(
        "KEYZ process_keyevent()",
        assert_eq!(res, Some(DecodedKey::Unicode('a')))
    )
}
