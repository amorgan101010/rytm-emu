//! The computer keyboard as the front panel, laid out as the other Elektron
//! emulators here lay theirs out: F1..F8 and 1..8 are the sixteen trig keys, a
//! held letter picks a knob that `-` `=` turn (`[` `]` with its switch pressed),
//! Ctrl is FUNC, and Shift latches what is pressed until Shift is let go.
//!
//! The Rytm and the A4 have more keys than the keyboard has left over, so ` is
//! a layer: held, F1..F8 are BANK A..H and 1..4 the keys in `*_LAYER`. On the A4
//! the layer's letters are its one-octave keyboard, and Tab keeps them that way.

use super::Control;
use crate::panel;
use minifb::Key;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Machine {
    Digitakt,
    Rytm,
    A4,
}

impl Machine {
    /// Identify the instrument from its loaded panel tables and machine flag.
    pub fn of(rytm: bool) -> Machine {
        if panel::a4_panel() || panel::a4mk1_panel() {
            Machine::A4
        } else if rytm || panel::rytmmk1_panel() {
            Machine::Rytm
        } else {
            Machine::Digitakt
        }
    }
}

/// The force a pad is struck with from the keyboard (the mouse gives 30..255).
pub const PAD_FORCE: u8 = 200;

/// The trig keys, 1..16.
const STEPS: [Key; 16] = [
    Key::F1,
    Key::F2,
    Key::F3,
    Key::F4,
    Key::F5,
    Key::F6,
    Key::F7,
    Key::F8,
    Key::Key1,
    Key::Key2,
    Key::Key3,
    Key::Key4,
    Key::Key5,
    Key::Key6,
    Key::Key7,
    Key::Key8,
];
const TRIGS: [&str; 16] = [
    "T1", "T2", "T3", "T4", "T5", "T6", "T7", "T8", "T9", "T10", "T11", "T12", "T13", "T14", "T15",
    "T16",
];
const BANKS: [&str; 8] = [
    "BANK_A", "BANK_B", "BANK_C", "BANK_D", "BANK_E", "BANK_F", "BANK_G", "BANK_H",
];
const PUSHES: [&str; 10] = [
    "PUSH_A", "PUSH_B", "PUSH_C", "PUSH_D", "PUSH_E", "PUSH_F", "PUSH_G", "PUSH_H", "PUSH_I",
    "PUSH_J",
];

/// Keys every machine has, under the same names.
const SHARED: [(Key, &str); 13] = [
    (Key::Backspace, "NO"),
    (Key::Enter, "YES"),
    (Key::Up, "UP"),
    (Key::Down, "DOWN"),
    (Key::Left, "LEFT"),
    (Key::Right, "RIGHT"),
    (Key::Space, "PLAY"),
    (Key::End, "STOP"),
    (Key::Home, "RECORD"),
    (Key::PageUp, "PAGE"),
    (Key::PageDown, "PAGE"),
    (Key::LeftCtrl, "FUNC"),
    (Key::RightCtrl, "FUNC"),
];

/// The Digitakt, as digiemu maps it.
const DIGITAKT: [(Key, &str); 12] = [
    (Key::Q, "TRK"),
    (Key::Backquote, "BANK"),
    (Key::U, "TEMPO"),
    (Key::I, "GLOBAL"),
    (Key::O, "SAMPLE"),
    (Key::P, "PTN"),
    (Key::M, "SONG"),
    (Key::J, "TRIG"),
    (Key::K, "SRC"),
    (Key::L, "FLTR"),
    (Key::Semicolon, "AMP"),
    (Key::Apostrophe, "LFO"),
];

/// The Rytm: the six pages on Q..Y, and the pads under the right hand in the
/// panel's three rows of four (U I O P / J K L ; / M , . /).
const RYTM: [(Key, &str); 30] = [
    (Key::F9, "MUTE"),
    (Key::F10, "CHRO"),
    (Key::F11, "SCNE"),
    (Key::F12, "PERF"),
    (Key::Key9, "PLAYMODE"),
    (Key::Key0, "SAMPLER"),
    (Key::Tab, "TRK"),
    (Key::H, "RTRG"),
    (Key::Q, "TRIG"),
    (Key::W, "SRC"),
    (Key::E, "SMPL"),
    (Key::R, "FLTR"),
    (Key::T, "AMP"),
    (Key::Y, "LFO"),
    (Key::U, "CH"),
    (Key::I, "OH"),
    (Key::O, "CY"),
    (Key::P, "CB"),
    (Key::J, "BT"),
    (Key::K, "LT"),
    (Key::L, "MT"),
    (Key::Semicolon, "HT"),
    (Key::M, "BD"),
    (Key::Comma, "SD"),
    (Key::Period, "RS"),
    (Key::Slash, "CP"),
    (Key::Backslash, "TEMPO"),
    (Key::Apostrophe, "FILL"),
    (Key::B, "FX"),
    (Key::N, "GLOBAL"),
];
const RYTM_LAYER: [(Key, &str); 4] = [
    (Key::Key1, "CHAIN"),
    (Key::Key2, "SONG"),
    (Key::Key3, "FIX"),
    (Key::Key4, "QPER"),
];

/// The A4: the track keys on Q..Y (where the Monomachine keeps its six), the
/// six PARAMETER pages on H..' and PERF, ARP, NOTE under them.
const A4: [(Key, &str); 27] = [
    (Key::F9, "KIT"),
    (Key::F10, "TRK"),
    (Key::F11, "SONG"),
    (Key::F12, "MIX"),
    (Key::Key9, "OCT_DOWN"),
    (Key::Key0, "OCT_UP"),
    (Key::Q, "TRACK1"),
    (Key::W, "TRACK2"),
    (Key::E, "TRACK3"),
    (Key::R, "TRACK4"),
    (Key::T, "FX"),
    (Key::Y, "CV"),
    (Key::U, "TEMPO"),
    (Key::I, "GLOBAL"),
    (Key::O, "SND"),
    (Key::P, "PTN"),
    (Key::Backslash, "TRNS"),
    (Key::H, "OSC1"),
    (Key::J, "OSC2"),
    (Key::K, "FILTERS"),
    (Key::L, "AMP"),
    (Key::Semicolon, "ENV"),
    (Key::Apostrophe, "LFO"),
    (Key::M, "PERF"),
    (Key::Comma, "ARP"),
    (Key::Period, "NOTE"),
    (Key::Slash, "FILL"),
];
const A4_LAYER: [(Key, &str); 3] = [
    (Key::Key1, "CHAIN"),
    (Key::Key2, "SONGMODE"),
    (Key::Key4, "QPER"),
];
/// The A4's keyboard, C1..C2, on the two letter rows a tracker uses.
const A4_PIANO: [(Key, &str); 13] = [
    (Key::A, "KB_C1"),
    (Key::W, "KB_C#1"),
    (Key::S, "KB_D1"),
    (Key::E, "KB_D#1"),
    (Key::D, "KB_E1"),
    (Key::F, "KB_F1"),
    (Key::T, "KB_F#1"),
    (Key::G, "KB_G1"),
    (Key::Y, "KB_G#1"),
    (Key::H, "KB_A1"),
    (Key::U, "KB_A#1"),
    (Key::J, "KB_B1"),
    (Key::K, "KB_C2"),
];

/// The knob keys, (key, encoder): the panel's two rows of four, then LEVEL.
const KNOBS: [(Key, u8); 9] = [
    (Key::A, 0),
    (Key::S, 1),
    (Key::D, 2),
    (Key::F, 3),
    (Key::Z, 4),
    (Key::X, 5),
    (Key::C, 6),
    (Key::V, 7),
    (Key::G, 8),
];
/// The A4's two rows of five, then LEVEL.
const A4_KNOBS: [(Key, u8); 11] = [
    (Key::A, 0),
    (Key::S, 1),
    (Key::D, 2),
    (Key::F, 3),
    (Key::G, 4),
    (Key::Z, 5),
    (Key::X, 6),
    (Key::C, 7),
    (Key::V, 8),
    (Key::B, 9),
    (Key::N, super::A4_ENC_LEVEL),
];

fn code(m: Machine, name: &str) -> u8 {
    let at = |t: &[(&str, u8, u8, u8)]| {
        t.iter()
            .position(|k| k.0 == name)
            .map(|i| panel::RYTM_CODE_BASE + i as u8)
    };
    match m {
        Machine::Digitakt => (1..48).find(|&c| panel::key_name(c) == Some(name)),
        Machine::Rytm => at(&panel::RYTM_KEYS),
        Machine::A4 => at(&panel::A4_KEYS),
    }
    .unwrap_or_else(|| panic!("no key {name}"))
}

/// Whether `c` is one of the BANK keys the ` layer puts on F1..F8.
fn is_bank(m: Machine, c: u8) -> bool {
    m != Machine::Digitakt && BANKS.iter().any(|n| code(m, n) == c)
}

/// The key code of encoder `enc`'s push switch, if it has one.
fn push(m: Machine, enc: u8) -> Option<u8> {
    let level = if m == Machine::A4 {
        super::A4_ENC_LEVEL
    } else {
        8
    };
    match m {
        Machine::Digitakt => (enc < 8).then_some(40 + enc),
        _ if enc == level => Some(code(m, "PUSH_LEVEL")),
        _ => Some(code(m, PUSHES[enc as usize])),
    }
}

/// What computer key `key` is on machine `m`, with the ` layer held (`layer`)
/// or the A4's keyboard switched on (`piano`). On the A4 the layer plays the
/// keyboard too, so a note is ` and a letter without switching modes.
pub fn control(m: Machine, key: Key, layer: bool, piano: bool) -> Option<Control> {
    let find = |t: &[(Key, &'static str)]| t.iter().find(|e| e.0 == key).map(|e| e.1);
    let step = STEPS.iter().position(|&k| k == key);
    let (map, upper, knobs): (&[(Key, &'static str)], &[(Key, &'static str)], &[(Key, u8)]) =
        match m {
            Machine::Digitakt => (&DIGITAKT, &[], &KNOBS),
            Machine::Rytm => (&RYTM, &RYTM_LAYER, &KNOBS),
            Machine::A4 => (&A4, &A4_LAYER, &A4_KNOBS),
        };
    if m == Machine::A4 && (piano || layer) {
        if let Some(n) = find(&A4_PIANO) {
            return Some(Control::Key(code(m, n)));
        }
    }
    if layer {
        if let Some(i) = step.filter(|&i| i < 8) {
            return Some(Control::Key(code(m, BANKS[i])));
        }
        if let Some(n) = find(upper) {
            return Some(Control::Key(code(m, n)));
        }
    }
    if let Some(i) = step {
        return Some(Control::Key(if m == Machine::Digitakt {
            24 + i as u8
        } else {
            code(m, TRIGS[i])
        }));
    }
    if let Some(&(_, e)) = knobs.iter().find(|e| e.0 == key) {
        return Some(Control::Knob(e, push(m, e)));
    }
    find(map)
        .or_else(|| find(&SHARED))
        .map(|n| Control::Key(code(m, n)))
}

/// What the keyboard asks of the machine.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Out {
    Key(u8, bool),
    /// `steps` value steps on an encoder, in a direction.
    Nudge(u8, i8, u8),
    Save,
}

/// The knob key held down.
struct Held {
    key: Key,
    enc: u8,
    push: Option<u8>,
    turned: bool,
    pushed: bool,
}

/// Frames a tapped knob's switch stays down.
const CLICK_FRAMES: u8 = 4;

/// The keyboard's state from frame to frame.
pub struct Keyboard {
    machine: Machine,
    prev: Vec<Key>,
    /// Panel keys held: the computer key and the code it pressed.
    down: Vec<(Key, u8)>,
    /// Codes let go while Shift was held; released when Shift is.
    latched: Vec<u8>,
    knob: Option<Held>,
    /// Push switches clicked, and the frames until each is let go.
    clicks: Vec<(u8, u8)>,
    /// The A4's letters play its keyboard (Tab).
    pub piano: bool,
}

impl Keyboard {
    pub fn new(machine: Machine) -> Keyboard {
        Keyboard {
            machine,
            prev: Vec::new(),
            down: Vec::new(),
            latched: Vec::new(),
            knob: None,
            clicks: Vec::new(),
            piano: false,
        }
    }

    /// The codes held from the keyboard, latched ones included, for the window.
    pub fn held(&self) -> Vec<u8> {
        let mut v: Vec<u8> = self
            .down
            .iter()
            .map(|d| d.1)
            .chain(self.latched.iter().copied())
            .collect();
        v.extend(self.knob.iter().filter(|h| h.pushed).filter_map(|h| h.push));
        v.extend(self.clicks.iter().map(|c| c.0));
        v
    }

    pub fn latched(&self) -> &[u8] {
        &self.latched
    }

    /// Forget everything held (the machine was told to release all).
    pub fn clear(&mut self) {
        self.down.clear();
        self.latched.clear();
        self.knob = None;
        self.clicks.clear();
    }

    fn release_all(&mut self, out: &mut Vec<Out>) {
        for c in self.held() {
            out.push(Out::Key(c, false));
        }
        self.clear();
    }

    /// One frame: `keys` are down now, `repeats` were pressed or auto-repeated
    /// since the last frame, `selected` is the knob last clicked with the mouse.
    pub fn frame(
        &mut self,
        keys: Vec<Key>,
        repeats: &[Key],
        active: bool,
        selected: Option<u8>,
    ) -> Vec<Out> {
        let mut out = Vec::new();
        self.clicks.retain_mut(|c| {
            c.1 -= 1;
            if c.1 == 0 {
                out.push(Out::Key(c.0, false));
            }
            c.1 > 0
        });
        // Another window has the keyboard: let go, as a hand leaving the panel.
        if !active {
            self.release_all(&mut out);
            self.prev.clear();
            return out;
        }
        let has = |k: Key| keys.contains(&k);
        let shift = has(Key::LeftShift) || has(Key::RightShift);
        let alt = has(Key::LeftAlt) || has(Key::RightAlt);
        let layer = self.machine != Machine::Digitakt && has(Key::Backquote);
        let ups: Vec<Key> = self
            .prev
            .iter()
            .copied()
            .filter(|k| !keys.contains(k))
            .collect();
        let downs: Vec<Key> = keys
            .iter()
            .copied()
            .filter(|k| !self.prev.contains(k))
            .collect();
        for k in ups {
            if let Some(i) = self.down.iter().position(|d| d.0 == k) {
                let (_, c) = self.down.remove(i);
                if shift {
                    self.latched.push(c);
                } else {
                    out.push(Out::Key(c, false));
                }
            }
            if self.knob.as_ref().is_some_and(|h| h.key == k) {
                let h = self.knob.take().unwrap();
                self.let_go(h, &mut out);
            }
        }
        if !shift {
            for c in self.latched.drain(..) {
                out.push(Out::Key(c, false));
            }
        }
        for k in downs {
            if alt {
                if k == Key::S {
                    out.push(Out::Save);
                }
                continue;
            }
            if k == Key::Tab && self.machine == Machine::A4 {
                self.piano = !self.piano;
                continue;
            }
            // A bank key held on the layer: the trig keys are trigs again, so
            // ` + F3 held and F5 tapped is BANK C + trig 5.
            let layer = layer && !self.down.iter().any(|d| is_bank(self.machine, d.1));
            match control(self.machine, k, layer, self.piano) {
                Some(Control::Key(c)) => {
                    self.latched.retain(|&l| l != c);
                    self.down.push((k, c));
                    out.push(Out::Key(c, true));
                }
                Some(Control::Knob(enc, push)) => {
                    if let Some(h) = self.knob.take() {
                        self.let_go(h, &mut out);
                    }
                    self.knob = Some(Held {
                        key: k,
                        enc,
                        push,
                        turned: false,
                        pushed: false,
                    });
                }
                None => {}
            }
        }
        for k in repeats {
            let (dir, press) = match k {
                Key::Minus => (-1, false),
                Key::Equal => (1, false),
                Key::LeftBracket => (-1, true),
                Key::RightBracket => (1, true),
                _ => continue,
            };
            match &mut self.knob {
                Some(h) => {
                    if let Some(p) = h.push.filter(|_| press != h.pushed) {
                        out.push(Out::Key(p, press));
                        h.pushed = press;
                    }
                    h.turned = true;
                    out.push(Out::Nudge(h.enc, dir, 1));
                }
                None => {
                    // No knob key held: the knob last clicked, `[` `]` eight steps.
                    if let Some(e) = selected {
                        out.push(Out::Nudge(e, dir, if press { 8 } else { 1 }));
                    }
                }
            }
        }
        self.prev = keys;
        out
    }

    /// A knob key released: its switch up if `[` `]` held it, or a click if the
    /// knob was never turned.
    fn let_go(&mut self, h: Held, out: &mut Vec<Out>) {
        match h.push {
            Some(p) if h.pushed => out.push(Out::Key(p, false)),
            Some(p) if !h.turned => {
                out.push(Out::Key(p, true));
                self.clicks.push((p, CLICK_FRAMES));
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn names(tables: &[&[(Key, &'static str)]], knobs: usize) -> Vec<&'static str> {
        let mut v: Vec<&str> = tables.iter().flat_map(|t| t.iter().map(|e| e.1)).collect();
        v.extend(
            TRIGS
                .iter()
                .chain(BANKS.iter())
                .chain(PUSHES[..knobs].iter()),
        );
        v.push("PUSH_LEVEL");
        v
    }

    /// No computer key is given two jobs in one table set.
    fn unique(tables: &[&[(Key, &'static str)]], knobs: &[(Key, u8)]) {
        let mut seen: Vec<Key> = STEPS.to_vec();
        for k in tables
            .iter()
            .flat_map(|t| t.iter().map(|e| e.0))
            .chain(knobs.iter().map(|e| e.0))
        {
            assert!(!seen.contains(&k), "{k:?} is mapped twice");
            seen.push(k);
        }
    }

    #[test]
    fn every_name_is_a_panel_key() {
        for n in names(&[&SHARED, &RYTM, &RYTM_LAYER], 8) {
            assert!(panel::RYTM_KEYS.iter().any(|k| k.0 == n), "Rytm has no {n}");
        }
        for n in names(&[&SHARED, &A4, &A4_LAYER, &A4_PIANO], 10) {
            assert!(panel::A4_KEYS.iter().any(|k| k.0 == n), "A4 has no {n}");
        }
        for (_, n) in SHARED.iter().chain(DIGITAKT.iter()) {
            code(Machine::Digitakt, n);
        }
        unique(&[&SHARED, &RYTM], &KNOBS);
        unique(&[&SHARED, &A4], &A4_KNOBS);
        unique(&[&SHARED, &DIGITAKT], &KNOBS);
    }

    #[test]
    fn knob_keys_turn_click_and_press_turn() {
        let mut kb = Keyboard::new(Machine::Digitakt);
        // Hold A, tap =: a turn, and no click when A comes up.
        assert_eq!(kb.frame(vec![Key::A], &[Key::A], true, None), vec![]);
        assert_eq!(
            kb.frame(vec![Key::A, Key::Equal], &[Key::Equal], true, None),
            vec![Out::Nudge(0, 1, 1)]
        );
        assert_eq!(kb.frame(vec![], &[], true, None), vec![]);
        // Tap A alone: its switch goes down, and up a few frames later.
        kb.frame(vec![Key::A], &[], true, None);
        assert_eq!(kb.frame(vec![], &[], true, None), vec![Out::Key(40, true)]);
        let later: Vec<Out> = (0..CLICK_FRAMES)
            .flat_map(|_| kb.frame(vec![], &[], true, None))
            .collect();
        assert_eq!(later, vec![Out::Key(40, false)]);
        // [ turns with the switch held until the knob key is let go.
        kb.frame(vec![Key::S], &[], true, None);
        assert_eq!(
            kb.frame(vec![Key::S], &[Key::LeftBracket], true, None),
            vec![Out::Key(41, true), Out::Nudge(1, -1, 1)]
        );
        assert_eq!(kb.frame(vec![], &[], true, None), vec![Out::Key(41, false)]);
    }

    #[test]
    fn shift_latches_until_released_and_focus_loss_lets_go() {
        let mut kb = Keyboard::new(Machine::Digitakt);
        assert_eq!(
            kb.frame(vec![Key::LeftShift, Key::F3], &[], true, None),
            vec![Out::Key(26, true)]
        );
        assert_eq!(kb.frame(vec![Key::LeftShift], &[], true, None), vec![]);
        assert_eq!(kb.frame(vec![], &[], true, None), vec![Out::Key(26, false)]);
        kb.frame(vec![Key::Space], &[], true, None);
        assert_eq!(
            kb.frame(vec![Key::Space], &[], false, None),
            vec![Out::Key(10, false)]
        );
    }

    #[test]
    fn layer_reaches_the_banks_and_a_held_bank_takes_a_trig() {
        let m = Machine::A4;
        let mut kb = Keyboard::new(m);
        assert_eq!(kb.frame(vec![Key::Backquote], &[], true, None), vec![]);
        let hold = vec![Key::Backquote, Key::F3];
        assert_eq!(
            kb.frame(hold.clone(), &[], true, None),
            vec![Out::Key(code(m, "BANK_C"), true)]
        );
        assert_eq!(
            kb.frame(vec![Key::Backquote, Key::F3, Key::F5], &[], true, None),
            vec![Out::Key(code(m, "T5"), true)]
        );
        assert_eq!(
            kb.frame(hold, &[], true, None),
            vec![Out::Key(code(m, "T5"), false)]
        );
        assert_eq!(
            kb.frame(vec![Key::Backquote], &[], true, None),
            vec![Out::Key(code(m, "BANK_C"), false)]
        );
        assert_eq!(
            kb.frame(vec![Key::Backquote, Key::Key1], &[], true, None),
            vec![Out::Key(code(m, "CHAIN"), true)]
        );
    }

    #[test]
    fn a4_layer_letters_play_the_keyboard() {
        let m = Machine::A4;
        let mut kb = Keyboard::new(m);
        kb.frame(vec![Key::Backquote], &[], true, None);
        assert_eq!(
            kb.frame(vec![Key::Backquote, Key::A], &[], true, None),
            vec![Out::Key(code(m, "KB_C1"), true)]
        );
        assert_eq!(
            kb.frame(vec![Key::Backquote], &[], true, None),
            vec![Out::Key(code(m, "KB_C1"), false)]
        );
    }
}
