//! Front-panel artwork, control geometry and mouse hit-testing. The MKII skins
//! are drawn here; the imported hardware layouts in `hw` cover the MKI models.
//!
//! The Analog panels are drawn procedurally, in the same spirit as DigiEmu's
//! generated skins: the reference photographs inform proportions and materials,
//! but no photograph or firmware artwork is shipped with the emulator.

pub mod font;
pub mod keys;
pub mod hw;

use crate::panel;

pub const W: usize = 1180;
pub const H: usize = 600;
/// Both Analog machines need the deeper chassis for pads/keys and the trig row.
pub const H_RYTM: usize = 700;

pub const BG: u32 = 0x0A0B0D;
const RYTM_FACE: u32 = 0x18191C;
const A4_FACE_TOP: u32 = 0xA4A6A8;
const A4_FACE_BOTTOM: u32 = 0x8D9093;
const CAP_TOP: u32 = 0x414347;
const CAP_BOTTOM: u32 = 0x202124;
const CAP_EDGE: u32 = 0x090A0B;
const TEXT: u32 = 0xE4E5E2;
const DIM: u32 = 0x92969C;
const OLED_ON: u32 = 0xEAF3FB;
const OLED_OFF: u32 = 0x030507;
const LATCH: u32 = 0xF0C45A;

pub struct Canvas {
    pub px: Vec<u32>,
    pub w: usize,
    pub h: usize,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Canvas {
        Canvas {
            px: vec![BG; w * h],
            w,
            h,
        }
    }

    #[inline]
    pub fn put(&mut self, x: i32, y: i32, c: u32) {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h {
            self.px[y as usize * self.w + x as usize] = c;
        }
    }

    pub fn fill(&mut self, x: i32, y: i32, w: i32, h: i32, c: u32) {
        for yy in y.max(0)..(y + h).min(self.h as i32) {
            for xx in x.max(0)..(x + w).min(self.w as i32) {
                self.px[yy as usize * self.w + xx as usize] = c;
            }
        }
    }

    pub fn frame(&mut self, x: i32, y: i32, w: i32, h: i32, c: u32) {
        self.fill(x, y, w, 1, c);
        self.fill(x, y + h - 1, w, 1, c);
        self.fill(x, y, 1, h, c);
        self.fill(x + w - 1, y, 1, h, c);
    }

    fn round_inset(y: i32, h: i32, r: i32) -> i32 {
        if r <= 0 {
            return 0;
        }
        let d = if y < r {
            r - y - 1
        } else if y >= h - r {
            y - (h - r)
        } else {
            0
        };
        if d == 0 {
            0
        } else {
            r - (((r * r - d * d).max(0) as f32).sqrt() as i32)
        }
    }

    pub fn rounded_rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: i32, c: u32) {
        for row in 0..h {
            let inset = Self::round_inset(row, h, r.min(w / 2).min(h / 2));
            self.fill(x + inset, y + row, w - inset * 2, 1, c);
        }
    }

    pub fn rounded_frame(&mut self, x: i32, y: i32, w: i32, h: i32, r: i32, width: i32, c: u32) {
        let center = self.sample(x + w / 2, y + h / 2);
        self.rounded_rect(x, y, w, h, r, c);
        if w > width * 2 && h > width * 2 {
            self.rounded_rect(
                x + width,
                y + width,
                w - width * 2,
                h - width * 2,
                (r - width).max(0),
                center,
            );
        }
    }

    fn sample(&self, x: i32, y: i32) -> u32 {
        if x >= 0 && y >= 0 && (x as usize) < self.w && (y as usize) < self.h {
            self.px[y as usize * self.w + x as usize]
        } else {
            BG
        }
    }

    pub fn vertical_gradient(
        &mut self,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        r: i32,
        top: u32,
        bottom: u32,
    ) {
        for row in 0..h {
            let a = if h <= 1 {
                0
            } else {
                row as u32 * 255 / (h - 1) as u32
            };
            let c = mix(top, bottom, a);
            let inset = Self::round_inset(row, h, r.min(w / 2).min(h / 2));
            self.fill(x + inset, y + row, w - inset * 2, 1, c);
        }
    }

    pub fn disc(&mut self, cx: i32, cy: i32, r: i32, c: u32) {
        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    self.put(cx + dx, cy + dy, c);
                }
            }
        }
    }

    pub fn ring(&mut self, cx: i32, cy: i32, r: i32, width: i32, c: u32) {
        let inner = (r - width).max(0);
        for dy in -r..=r {
            for dx in -r..=r {
                let d = dx * dx + dy * dy;
                if d <= r * r && d >= inner * inner {
                    self.put(cx + dx, cy + dy, c);
                }
            }
        }
    }

    pub fn line(&mut self, mut x0: i32, mut y0: i32, x1: i32, y1: i32, c: u32) {
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.put(x0, y0, c);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    /// Text in the built-in 5x7 utility face at `scale`; returns its width.
    pub fn text(&mut self, x: i32, y: i32, s: &str, scale: i32, c: u32) -> i32 {
        let mut cx = x;
        for ch in s.chars() {
            let g = font::glyph(ch);
            for (col, bits) in g.iter().enumerate() {
                for row in 0..7 {
                    if bits >> row & 1 != 0 {
                        self.fill(cx + col as i32 * scale, y + row * scale, scale, scale, c);
                    }
                }
            }
            cx += 6 * scale;
        }
        cx - x
    }

    pub fn text_centered(&mut self, cx: i32, y: i32, s: &str, scale: i32, c: u32) {
        let w = s.chars().count() as i32 * 6 * scale - scale;
        self.text(cx - w / 2, y, s, scale, c);
    }
}

#[derive(Clone, Copy)]
pub enum Control {
    Key(u8),
    /// Encoder index and its push switch, when it has one.
    Knob(u8, Option<u8>),
}

#[derive(Clone, Copy)]
pub struct Place {
    pub c: Control,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Place {
    pub fn hit(&self, x: i32, y: i32) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }
}

/// Pixels of movement before a knob press becomes a turn.
pub const DRAG_START: f32 = 4.0;
/// Pixels of movement per encoder detent.
pub const DRAG_PX: f32 = 6.0;

/// State for a mouse press on a knob. A click pushes it; dragging turns it.
#[derive(Clone, Copy, Debug)]
pub struct KnobDrag {
    pub enc: u8,
    /// Push switch held during a ctrl-click, if any.
    pub pushed: Option<u8>,
    push: Option<u8>,
    x0: f32,
    y0: f32,
    moved: bool,
    used: f32,
}

impl KnobDrag {
    pub fn new(enc: u8, push: Option<u8>, ctrl: bool, x: f32, y: f32) -> Self {
        Self { enc, pushed: push.filter(|_| ctrl), push, x0: x, y0: y, moved: false, used: 0.0 }
    }

    pub fn motion(&mut self, x: f32, y: f32) -> Option<i8> {
        if !self.moved {
            if (x - self.x0).abs() + (y - self.y0).abs() < DRAG_START {
                return None;
            }
            self.moved = true;
        }
        let travel = (x - self.x0) + (self.y0 - y);
        let steps = ((travel - self.used) / DRAG_PX).trunc().clamp(-127.0, 127.0);
        if steps == 0.0 {
            return None;
        }
        self.used += steps * DRAG_PX;
        Some(steps as i8)
    }

    pub fn release(&self) -> DragEnd {
        match (self.pushed, self.push) {
            (Some(c), _) => DragEnd::PushUp(c),
            (None, Some(c)) if !self.moved => DragEnd::Click(c),
            _ => DragEnd::None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragEnd {
    None,
    Click(u8),
    PushUp(u8),
}

/// Legacy Digitakt display geometry. Analog panels use device-specific geometry.
pub const OLED_X: i32 = 170;
pub const OLED_Y: i32 = 44;
pub const OLED_SCALE: i32 = 4;

pub fn layout_for(rytm: bool, a4: bool, mk1: bool) -> Vec<Place> {
    if mk1 {
        let dev = hw::dev_of(rytm, a4, mk1).expect("MK1 panel needs a MK1 device");
        hw::layout(dev)
    } else if a4 {
        layout_a4()
    } else if rytm {
        layout_rytm()
    } else {
        layout()
    }
}

/// Window dimensions for the loaded front panel.
pub fn window_size(rytm: bool, a4: bool, mk1: bool) -> (usize, usize) {
    if mk1 {
        let dev = hw::dev_of(rytm, a4, mk1).expect("MK1 panel needs a MK1 device");
        hw::size(dev)
    } else {
        (W, if rytm || a4 { H_RYTM } else { H })
    }
}

/// The A4 panel wire numbers A..J as 0..9, with TRACK LEVEL at 10.
pub const A4_ENC_LEVEL: u8 = 10;

/// Analog Four MKII geometry, following the proportions of the physical panel.
pub fn layout_a4() -> Vec<Place> {
    let mut v = Vec::new();
    let k = |name: &str, x: i32, y: i32, w: i32, h: i32| Place {
        c: Control::Key(panel::rytm_code(name)),
        x,
        y,
        w,
        h,
    };
    let push = |name: &str| Some(panel::rytm_code(name));

    v.push(Place {
        c: Control::Knob(A4_ENC_LEVEL, push("PUSH_LEVEL")),
        x: 261,
        y: 59,
        w: 70,
        h: 82,
    });
    for (name, x) in [("GLOBAL", 60), ("QPER", 168), ("TEMPO", 276)] {
        v.push(k(name, x, 160, 54, 38));
    }
    v.push(k("FUNC", 58, 260, 60, 52));
    for (name, x) in [("KIT", 166), ("SND", 222), ("MIX", 278)] {
        v.push(k(name, x, 260, 48, 38));
    }
    v.push(k("PERF", 60, 338, 54, 40));
    for (name, x) in [("TRK", 166), ("PTN", 222), ("SONG", 278)] {
        v.push(k(name, x, 338, 48, 38));
    }
    for i in 0..8 {
        let name = format!("BANK_{}", (b'A' + i as u8) as char);
        v.push(k(&name, 58 + (i % 4) * 58, 438 + (i / 4) * 58, 50, 40));
    }

    v.push(k("YES", 405, 288, 48, 40));
    v.push(k("NO", 405, 340, 48, 40));
    v.push(k("UP", 493, 288, 48, 40));
    v.push(k("LEFT", 458, 340, 48, 40));
    v.push(k("DOWN", 511, 340, 48, 40));
    v.push(k("RIGHT", 564, 340, 48, 40));
    v.push(k("SONGMODE", 405, 416, 50, 38));
    v.push(k("CHAIN", 463, 416, 50, 38));
    v.push(k("RECORD", 405, 486, 54, 48));
    v.push(k("PLAY", 467, 486, 54, 48));
    v.push(k("STOP", 529, 486, 54, 48));

    for (i, name) in [
        "ARP", "NOTE", "OSC1", "OSC2", "FILTERS", "AMP", "ENV", "LFO",
    ]
    .iter()
    .enumerate()
    {
        v.push(k(name, 638 + i as i32 * 56, 288, 50, 40));
    }
    for i in 0..10u8 {
        let col = (i % 5) as i32;
        let row = (i / 5) as i32;
        let name = format!("PUSH_{}", (b'A' + i) as char);
        v.push(Place {
            c: Control::Knob(i, push(&name)),
            x: 632 + col * 94,
            y: 58 + row * 98,
            w: 72,
            h: 80,
        });
    }
    v.push(k("OCT_DOWN", 708, 351, 52, 38));
    v.push(k("OCT_UP", 936, 351, 52, 38));

    // One-octave button keyboard. White caps are drawn first; black caps sit on top.
    let white = ["C1", "D1", "E1", "F1", "G1", "A1", "B1", "C2"];
    for (i, name) in white.iter().enumerate() {
        v.push(k(&format!("KB_{name}"), 638 + i as i32 * 50, 405, 44, 128));
    }
    for (slot, name) in [(0, "C#1"), (1, "D#1"), (3, "F#1"), (4, "G#1"), (5, "A#1")] {
        v.push(k(&format!("KB_{name}"), 669 + slot * 50, 405, 28, 70));
    }

    for (i, name) in ["TRACK1", "TRACK2", "TRACK3", "TRACK4", "FX", "CV"]
        .iter()
        .enumerate()
    {
        v.push(k(name, 1085, 82 + i as i32 * 52, 58, 38));
    }
    v.push(k("TRNS", 994, 351, 54, 38));
    v.push(k("FILL", 1085, 510, 58, 40));
    v.push(k("PAGE", 1085, 592, 58, 40));

    for i in 0..16 {
        v.push(k(&format!("T{}", i + 1), 38 + i * 64, 590, 56, 50));
    }
    v
}

/// Analog Rytm MKII geometry, including its 4x3 pressure-pad surface.
pub fn layout_rytm() -> Vec<Place> {
    let mut v = Vec::new();
    let k = |name: &str, x: i32, y: i32, w: i32, h: i32| Place {
        c: Control::Key(panel::rytm_code(name)),
        x,
        y,
        w,
        h,
    };
    let push = |name: &str| Some(panel::rytm_code(name));

    v.push(Place {
        c: Control::Knob(8, push("PUSH_LEVEL")),
        x: 460,
        y: 58,
        w: 70,
        h: 84,
    });
    v.push(k("GLOBAL", 50, 151, 58, 42));
    v.push(k("SAMPLER", 140, 146, 58, 38));
    v.push(k("QPER", 236, 146, 58, 38));
    for (i, name) in ["PLAYMODE", "MUTE", "CHRO", "SCNE", "PERF"]
        .iter()
        .enumerate()
    {
        v.push(k(name, 140 + i as i32 * 60, 204, 54, 38));
    }
    v.push(k("FIX", 443, 204, 45, 38));
    v.push(k("TEMPO", 493, 204, 34, 38));
    v.push(k("FUNC", 50, 265, 66, 52));
    v.push(k("TRK", 50, 386, 66, 48));
    v.push(k("RTRG", 50, 500, 66, 48));

    let pads = [
        ["CH", "OH", "CY", "CB"],
        ["BT", "LT", "MT", "HT"],
        ["BD", "SD", "RS", "CP"],
    ];
    for (row, names) in pads.iter().enumerate() {
        for (col, name) in names.iter().enumerate() {
            v.push(k(
                name,
                140 + col as i32 * 101,
                272 + row as i32 * 98,
                90,
                76,
            ));
        }
    }

    for (i, name) in [
        "PUSH_A", "PUSH_B", "PUSH_C", "PUSH_D", "PUSH_E", "PUSH_F", "PUSH_G", "PUSH_H",
    ]
    .iter()
    .enumerate()
    {
        let col = (i % 4) as i32;
        let row = (i / 4) as i32;
        v.push(Place {
            c: Control::Knob(i as u8, push(name)),
            x: 832 + col * 84,
            y: 58 + row * 99,
            w: 70,
            h: 82,
        });
    }

    v.push(k("YES", 575, 286, 50, 40));
    v.push(k("NO", 575, 338, 50, 40));
    v.push(k("UP", 661, 286, 50, 40));
    v.push(k("LEFT", 628, 338, 50, 40));
    v.push(k("DOWN", 684, 338, 50, 40));
    v.push(k("RIGHT", 740, 338, 50, 40));
    for (i, name) in ["TRIG", "SRC", "SMPL", "FLTR", "AMP", "LFO"]
        .iter()
        .enumerate()
    {
        v.push(k(name, 824 + i as i32 * 57, 282, 52, 40));
    }
    for i in 0..8 {
        let name = format!("BANK_{}", (b'A' + i as u8) as char);
        v.push(k(&name, 575 + (i % 4) * 58, 430 + (i / 4) * 54, 50, 38));
    }
    v.push(k("CHAIN", 834, 405, 54, 40));
    v.push(k("SONG", 895, 405, 54, 40));
    v.push(k("FX", 1084, 405, 58, 40));
    v.push(k("RECORD", 834, 474, 58, 48));
    v.push(k("PLAY", 900, 474, 58, 48));
    v.push(k("STOP", 966, 474, 58, 48));
    v.push(k("FILL", 1084, 493, 58, 40));
    v.push(k("PAGE", 1084, 590, 58, 40));
    for i in 0..16 {
        v.push(k(&format!("T{}", i + 1), 34 + i * 65, 590, 56, 50));
    }
    v
}

/// The legacy Digitakt layout retained for firmware profiles supported by this tree.
pub fn layout() -> Vec<Place> {
    let mut v = Vec::new();
    let key = |code: u8, x: i32, y: i32, w: i32| Place {
        c: Control::Key(code),
        x,
        y,
        w,
        h: 40,
    };
    v.push(Place {
        c: Control::Knob(8, None),
        x: 40,
        y: 70,
        w: 90,
        h: 90,
    });
    for i in 0..8u8 {
        v.push(Place {
            c: Control::Knob(i, Some(40 + i)),
            x: 720 + (i % 4) as i32 * 110,
            y: 50 + (i / 4) as i32 * 130,
            w: 80,
            h: 80,
        });
    }
    v.push(key(2, 20, 190, 64));
    v.push(key(3, 90, 190, 64));
    v.push(key(4, 20, 240, 64));
    v.push(key(5, 90, 240, 64));
    for (i, &c) in [6u8, 7, 8, 18].iter().enumerate() {
        v.push(key(c, 164 + i as i32 * 88, 330, 82));
    }
    v.push(key(13, 540, 330, 64));
    v.push(key(12, 610, 330, 64));
    v.push(key(14, 610, 380, 64));
    v.push(key(16, 540, 430, 64));
    v.push(key(15, 610, 430, 64));
    v.push(key(17, 680, 430, 64));
    for (i, &c) in [19u8, 20, 21, 22, 23].iter().enumerate() {
        v.push(key(c, 770 + i as i32 * 80, 380, 74));
    }
    v.push(key(1, 20, 520, 76));
    v.push(key(9, 102, 520, 80));
    v.push(key(10, 188, 520, 76));
    v.push(key(11, 270, 520, 76));
    for i in 0..16u8 {
        v.push(key(24 + i, 360 + i as i32 * 50, 520, 45));
    }
    v
}

#[derive(Clone)]
pub struct View {
    pub rytm: bool,
    pub a4: bool,
    pub mk1: bool,
    pub oled: Vec<u8>,
    pub leds: Vec<Option<(u8, u8, u8)>>,
    pub status: String,
}

fn mix(a: u32, b: u32, alpha: u32) -> u32 {
    let channel = |shift: u32| {
        let x = (a >> shift) & 0xFF;
        let y = (b >> shift) & 0xFF;
        (x * (255 - alpha.min(255)) + y * alpha.min(255)) / 255
    };
    channel(16) << 16 | channel(8) << 8 | channel(0)
}

fn rgb((r, g, b): (u8, u8, u8)) -> u32 {
    (r as u32) << 16 | (g as u32) << 8 | b as u32
}

fn shade(c: u32, amount: i32) -> u32 {
    let f = |shift: u32| (((c >> shift) & 0xFFu32) as i32 + amount).clamp(0, 255) as u32;
    f(16) << 16 | f(8) << 8 | f(0)
}

fn emission(c: (u8, u8, u8)) -> u32 {
    mix(rgb(c), 0xFFFFFF, 54)
}

fn panel_ink(view: &View) -> u32 {
    if view.a4 {
        0x28312B
    } else {
        0xD0D1CE
    }
}

fn accent_ink(view: &View) -> u32 {
    if view.a4 {
        0x29462F
    } else {
        0xC7C8C4
    }
}

fn draw_chassis(cv: &mut Canvas, view: &View) {
    cv.fill(0, 0, cv.w as i32, cv.h as i32, BG);
    let ph = cv.h as i32 - 50;
    cv.rounded_rect(8, 35, cv.w as i32 - 16, ph, 7, 0x030405);
    if view.a4 {
        cv.vertical_gradient(10, 30, cv.w as i32 - 20, ph, 6, A4_FACE_TOP, A4_FACE_BOTTOM);
        cv.fill(13, 34, 2, ph - 8, 0x55585B);
        cv.fill(cv.w as i32 - 15, 34, 2, ph - 8, 0xC3C4C5);
    } else {
        cv.vertical_gradient(
            10,
            30,
            cv.w as i32 - 20,
            ph,
            6,
            shade(RYTM_FACE, 7),
            RYTM_FACE,
        );
        cv.frame(11, 31, cv.w as i32 - 22, ph - 1, 0x34363A);
        // Sparse, deterministic powder-coat highlights.
        for y in (36..(30 + ph - 5)).step_by(5) {
            for x in ((13 + (y * 17 % 7))..(cv.w as i32 - 13)).step_by(11) {
                if (x * 13 + y * 7) % 23 == 0 {
                    cv.put(x, y, 0x202125);
                }
            }
        }
    }
}

fn draw_knob_shape(cv: &mut Canvas, cx: i32, cy: i32, r: i32, pressed: bool, selected: bool) {
    cv.disc(cx + 3, cy + 5, r + 2, 0x08090A);
    cv.disc(cx, cy + 2, r + 2, 0x111214);
    cv.ring(cx, cy + 1, r + 1, 2, 0x08090A);
    let lift = if pressed { 1 } else { 0 };
    for dy in -r..=r {
        for dx in -r..=r {
            if dx * dx + dy * dy <= r * r {
                let v = (54 - dx / 5 - dy / 4).clamp(31, 70) as u32;
                cv.put(cx + dx, cy + dy + lift, v << 16 | (v + 1) << 8 | (v + 2));
            }
        }
    }
    cv.ring(cx, cy + lift, r, 1, 0x17181A);
    cv.disc(cx - r / 3, cy - r * 2 / 3 + lift, (r / 8).max(2), 0xECEDEA);
    if selected {
        cv.ring(cx, cy + 1, r + 5, 2, LATCH);
    }
}

fn printed(cv: &mut Canvas, x: i32, y: i32, value: &str, color: u32) {
    cv.text_centered(x, y, value, 1, color);
}

fn draw_fixed_hardware(cv: &mut Canvas, view: &View) {
    let ink = panel_ink(view);
    if view.a4 {
        draw_knob_shape(cv, 87, 91, 22, false, false);
        draw_knob_shape(cv, 195, 91, 22, false, false);
        printed(cv, 87, 121, "MAIN VOLUME", ink);
        printed(cv, 195, 121, "QUICK PERF", ink);
        cv.text(18, 40, "ANALOG FOUR MKII", 1, shade(ink, -18));
    } else {
        draw_knob_shape(cv, 72, 91, 22, false, false);
        draw_knob_shape(cv, 363, 91, 22, false, false);
        printed(cv, 72, 121, "MAIN VOLUME", ink);
        printed(cv, 363, 121, "QUICK PERF", ink);
        cv.text(18, 40, "ANALOG RYTM MKII", 1, 0xAEB0B1);
    }
}

fn draw_oled(cv: &mut Canvas, view: &View) {
    let (x, y, bx, by, bw, bh, legend, product) = if view.a4 {
        (
            359,
            86,
            342,
            52,
            290,
            207,
            "4 VOICE ANALOG TABLETOP SYNTHESIZER",
            "ANALOG FOUR MKII",
        )
    } else if view.rytm {
        (
            548,
            86,
            531,
            52,
            291,
            207,
            "8 VOICE ANALOG DRUM COMPUTER + SAMPLER",
            "ANALOG RYTM MKII",
        )
    } else {
        (
            OLED_X,
            OLED_Y,
            OLED_X - 7,
            OLED_Y - 7,
            128 * OLED_SCALE + 14,
            64 * OLED_SCALE + 14,
            "DRUM COMPUTER + SAMPLER",
            "DIGITAKT",
        )
    };
    let scale = if view.rytm || view.a4 { 2 } else { OLED_SCALE };
    cv.rounded_rect(bx + 3, by + 5, bw, bh, 6, 0x08090A);
    cv.rounded_rect(bx, by, bw, bh, 5, 0x050607);
    cv.rounded_frame(bx, by, bw, bh, 5, 2, 0x303236);
    cv.text_centered(bx + bw / 2, by + 11, legend, 1, 0xC4C6C5);
    cv.fill(x - 4, y - 4, 128 * scale + 8, 64 * scale + 8, 0x010203);
    for py in 0..64usize {
        for px in 0..128usize {
            let byte = view.oled.get((7 - py / 8) + 8 * px).copied().unwrap_or(0);
            let on = (byte >> (py % 8)) & 1 != 0;
            cv.fill(
                x + px as i32 * scale,
                y + py as i32 * scale,
                scale,
                scale,
                if on { OLED_ON } else { OLED_OFF },
            );
        }
    }
    if view.rytm || view.a4 {
        cv.text_centered(bx + bw / 2, by + bh - 18, product, 2, 0xE5E6E3);
    }
}

fn raw_name(code: u8) -> &'static str {
    panel::key_name(code).unwrap_or("?")
}

fn is_step(name: &str) -> bool {
    name.strip_prefix('T')
        .is_some_and(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
}

fn is_pad(name: &str) -> bool {
    matches!(
        name,
        "BD" | "SD" | "RS" | "CP" | "BT" | "LT" | "MT" | "HT" | "CH" | "OH" | "CY" | "CB"
    )
}

fn pad_number(name: &str) -> usize {
    [
        "BD", "SD", "RS", "CP", "BT", "LT", "MT", "HT", "CH", "OH", "CY", "CB",
    ]
    .iter()
    .position(|&n| n == name)
    .map(|i| i + 1)
    .unwrap_or(0)
}

fn secondary(view: &View, name: &str) -> Option<&'static str> {
    if is_step(name) {
        let i = name[1..].parse::<usize>().ok()?.checked_sub(1)?;
        const R: [&str; 16] = [
            "BD", "SD", "RS", "CP", "BT", "LT", "MT", "HT", "CH", "OH", "CY", "CB", "", "", "", "",
        ];
        const A: [&str; 16] = [
            "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P",
        ];
        return if view.a4 {
            A.get(i).copied()
        } else {
            R.get(i).copied()
        };
    }
    let s = if view.a4 {
        match name {
            "GLOBAL" => "SAVE PROJECT",
            "QPER" => "MUTE",
            "TEMPO" => "TAP TEMPO",
            "PERF" => "SETUP",
            "KIT" => "SAVE KIT",
            "SND" => "SETTINGS",
            "MIX" => "POLY CONFIG",
            "TRK" => "METRONOME",
            "PTN" => "SWING",
            "SONG" => "SAVE SONG",
            "YES" => "SAVE",
            "NO" => "RELOAD",
            "LEFT" => "TIME-",
            "RIGHT" => "TIME+",
            "CHAIN" => "NEW CHAIN",
            "SONGMODE" => "EDIT SONG",
            "ARP" | "NOTE" => "SETUP",
            "OSC1" => "EXT/CV A",
            "OSC2" => "CV B",
            "FILTERS" => "CHOR/C",
            "AMP" => "DELAY/D",
            "ENV" => "REVERB",
            "LFO" => "LFO",
            "OCT_DOWN" | "OCT_UP" => "OCTAVE",
            "TRNS" => "LOCK",
            "TRACK1" | "TRACK2" | "TRACK3" | "TRACK4" | "FX" | "CV" => "MUTE",
            "RECORD" => "COPY",
            "PLAY" => "CLEAR",
            "STOP" => "PASTE",
            "FILL" => "CUE FILL",
            "PAGE" => "SCALE",
            "BANK_E" => "TRIG MUTE",
            "BANK_F" => "ACCENT",
            "BANK_G" => "NOTE SLD",
            "BANK_H" => "PARAM SLD",
            _ => return None,
        }
    } else {
        match name {
            "GLOBAL" => "SAVE PROJECT",
            "SAMPLER" => "DIRECT",
            "QPER" => "MUTE",
            "PLAYMODE" => "KIT",
            "MUTE" => "SOUND",
            "CHRO" => "TRACK",
            "SCNE" => "PATTERN",
            "PERF" => "SONG",
            "FIX" => "SETUP",
            "TEMPO" => "TAP TEMPO",
            "TRK" => "SAVE KIT",
            "RTRG" => "METRONOME",
            "YES" => "SAVE",
            "NO" => "RELOAD",
            "UP" => "RETRIG+",
            "DOWN" => "RETRIG-",
            "LEFT" => "TIME-",
            "RIGHT" => "TIME+",
            "TRIG" => "QUANTIZE",
            "SRC" => "DELAY",
            "SMPL" => "REVERB",
            "FLTR" => "DIST",
            "AMP" => "COMP",
            "LFO" => "LFO",
            "CHAIN" => "NEW CHAIN",
            "SONG" => "EDIT SONG",
            "FX" => "MIDI CFG",
            "RECORD" => "COPY",
            "PLAY" => "CLEAR",
            "STOP" => "PASTE",
            "FILL" => "CUE FILL",
            "PAGE" => "SCALE",
            "BANK_E" => "TRIG MUTE",
            "BANK_F" => "ACCENT",
            "BANK_G" => "SWING",
            "BANK_H" => "SLIDE",
            _ => return None,
        }
    };
    Some(s)
}

fn led_for(view: &View, code: u8) -> Option<(u8, u8, u8)> {
    panel::led_for_key(code).and_then(|i| view.leds.get(i).copied().flatten())
}

fn draw_arrow(cv: &mut Canvas, cx: i32, cy: i32, name: &str, c: u32) {
    let pts = match name {
        "UP" => [(-9, 6), (0, -8), (9, 6)],
        "DOWN" => [(-9, -6), (0, 8), (9, -6)],
        "LEFT" => [(6, -9), (-8, 0), (6, 9)],
        _ => [(-6, -9), (8, 0), (-6, 9)],
    };
    cv.line(
        cx + pts[0].0,
        cy + pts[0].1,
        cx + pts[1].0,
        cy + pts[1].1,
        c,
    );
    cv.line(
        cx + pts[1].0,
        cy + pts[1].1,
        cx + pts[2].0,
        cy + pts[2].1,
        c,
    );
    cv.line(
        cx + pts[0].0 + 1,
        cy + pts[0].1,
        cx + pts[1].0 + 1,
        cy + pts[1].1,
        c,
    );
    cv.line(
        cx + pts[1].0 + 1,
        cy + pts[1].1,
        cx + pts[2].0 + 1,
        cy + pts[2].1,
        c,
    );
}

fn draw_key_icon(cv: &mut Canvas, p: &Place, name: &str, label: &str, c: u32, shift: i32) {
    let cx = p.x + p.w / 2;
    let cy = p.y + p.h / 2 + shift;
    match name {
        "UP" | "DOWN" | "LEFT" | "RIGHT" => draw_arrow(cv, cx, cy, name, c),
        "RECORD" => cv.ring(cx, cy, 10, 2, c),
        "STOP" => cv.rounded_frame(cx - 9, cy - 9, 18, 18, 3, 2, c),
        "PLAY" => {
            cv.line(cx - 7, cy - 10, cx + 10, cy, c);
            cv.line(cx + 10, cy, cx - 7, cy + 10, c);
            cv.line(cx - 7, cy + 10, cx - 7, cy - 10, c);
        }
        _ => {
            let scale = if is_step(name) || is_pad(name) { 2 } else { 1 };
            cv.text_centered(cx, cy - 7 * scale / 2, label, scale, c);
            if is_step(name) {
                cv.fill(cx - 8, cy + 13, 16, 2, c);
            }
        }
    }
}

fn draw_key(cv: &mut Canvas, view: &View, p: &Place, code: u8, held: &[u8], latched: &[u8]) {
    let name = raw_name(code);
    let label = panel::key_label(code).unwrap_or("?");
    let down = held.contains(&code);
    let lit = led_for(view, code);
    let piano = view.a4 && name.starts_with("KB_");
    let black_piano = piano && name.contains('#');
    let pad = view.rytm && is_pad(name);
    let step = is_step(name);
    let shift = if down { 2 } else { 0 };
    let radius = if pad { 7 } else { 6 };

    cv.rounded_rect(
        p.x + 3,
        p.y + 5,
        p.w,
        p.h,
        radius,
        if view.a4 { 0x55575A } else { 0x050607 },
    );
    cv.rounded_rect(p.x - 1, p.y, p.w + 2, p.h + 2, radius + 1, CAP_EDGE);

    let (top, bottom) = if piano && !black_piano {
        (0xE1E2E0, 0xB7B9BA)
    } else if pad {
        (0xBEC2C3, 0x909598)
    } else if view.a4 && name == "FUNC" {
        (0x4D5945, 0x293128)
    } else {
        (CAP_TOP, CAP_BOTTOM)
    };
    let top = if down { shade(top, 14) } else { top };
    let bottom = if down { shade(bottom, 8) } else { bottom };
    cv.vertical_gradient(
        p.x + 2,
        p.y + 3 + shift,
        p.w - 4,
        p.h - 6,
        radius - 1,
        top,
        bottom,
    );
    cv.fill(
        p.x + 9,
        p.y + 4 + shift,
        (p.w - 18).max(1),
        1,
        shade(top, 18),
    );

    // Keep legends legible; the LED belongs on the cap/rim, not on the text.
    let ink = if piano && !black_piano {
        0x55585B
    } else {
        TEXT
    };
    if let Some(color) = lit {
        let e = emission(color);
        if pad {
            cv.vertical_gradient(
                p.x + 4,
                p.y + 5 + shift,
                p.w - 8,
                p.h - 10,
                radius - 2,
                mix(top, e, 90),
                mix(bottom, rgb(color), 76),
            );
        } else if piano {
            cv.rounded_rect(
                p.x + p.w / 2 - 2,
                p.y + 12 + shift,
                4,
                (p.h - 25).max(8),
                2,
                e,
            );
        } else {
            cv.rounded_frame(p.x + 4, p.y + 5 + shift, p.w - 8, p.h - 10, 3, 2, e);
        }
    } else if piano {
        cv.rounded_rect(
            p.x + p.w / 2 - 2,
            p.y + 12 + shift,
            4,
            (p.h - 25).max(8),
            2,
            if black_piano { 0x77797A } else { 0xF0F0EC },
        );
    }

    if latched.contains(&code) {
        cv.rounded_frame(p.x - 2, p.y - 2, p.w + 4, p.h + 4, radius + 2, 2, LATCH);
    }
    if step && matches!(name, "T1" | "T5" | "T9" | "T13") {
        cv.rounded_frame(
            p.x + 7,
            p.y + 6 + shift,
            p.w - 14,
            p.h - 12,
            3,
            1,
            if lit.is_some() { ink } else { 0x777A7D },
        );
    }

    if pad {
        let n = pad_number(name);
        cv.text_centered(
            p.x + p.w / 2,
            p.y + p.h + 5,
            &format!("{n}  {name}"),
            1,
            accent_ink(view),
        );
    } else if !piano {
        draw_key_icon(cv, p, name, label, ink, shift);
        if let Some(sub) = secondary(view, name).filter(|s| !s.is_empty()) {
            cv.text_centered(p.x + p.w / 2, p.y + p.h + 4, sub, 1, accent_ink(view));
        }
    }
}

fn draw_knob(
    cv: &mut Canvas,
    view: &View,
    p: &Place,
    enc: u8,
    push: Option<u8>,
    held: &[u8],
    selected: Option<u8>,
) {
    let r = (p.w.min(p.h) / 2 - 9).max(12);
    let cx = p.x + p.w / 2;
    let cy = p.y + r + 5;
    let pressed = push.is_some_and(|c| held.contains(&c));
    draw_knob_shape(cv, cx, cy, r, pressed, selected == Some(enc));
    let label = match enc {
        A4_ENC_LEVEL if view.a4 => "TRACK LEVEL".to_string(),
        8 if view.rytm => "TRACK LEVEL".to_string(),
        8 if !view.a4 => "LEVEL/DATA".to_string(),
        _ => ((b'A' + enc) as char).to_string(),
    };
    cv.text_centered(cx, p.y + p.h - 8, &label, 1, panel_ink(view));
    if (view.a4 && enc == A4_ENC_LEVEL) || (view.rytm && enc == 8) {
        cv.text_centered(cx, p.y + p.h + 4, "SOUND BROWSER", 1, accent_ink(view));
    }
}

fn draw_lens(cv: &mut Canvas, view: &View, x: i32, y: i32, led: usize, label: Option<&str>) {
    cv.disc(x + 1, y + 2, 5, if view.a4 { 0x696B6C } else { 0x080909 });
    cv.disc(x, y, 4, 0x242526);
    if let Some(c) = view.leds.get(led).copied().flatten() {
        cv.disc(x, y, 3, emission(c));
        cv.disc(x - 1, y - 1, 1, 0xFFFFFF);
    } else {
        cv.disc(x, y, 2, 0x4A2B27);
    }
    if let Some(s) = label {
        cv.text_centered(x, y + 8, s, 1, accent_ink(view));
    }
}

fn draw_panel_leds(cv: &mut Canvas, view: &View) {
    if view.a4 {
        for (i, &led) in panel::A4_OCTAVE_LEDS.iter().enumerate() {
            draw_lens(
                cv,
                view,
                785 + i as i32 * 22,
                370,
                led,
                Some(["-3", "-2", "-1", "+1", "+2", "+3", "+4"][i]),
            );
        }
        for (i, &led) in panel::A4_MODE_LEDS.iter().enumerate() {
            draw_lens(
                cv,
                view,
                82 + i as i32 * 58,
                421,
                led,
                Some(panel::MODE_LABELS[i]),
            );
        }
        for (i, &led) in panel::A4_PAGE_LEDS.iter().enumerate() {
            draw_lens(
                cv,
                view,
                1058 + i as i32 * 21,
                575,
                led,
                Some(["1", "2", "3", "4"][i]),
            );
        }
    } else if view.rytm {
        for (i, &led) in panel::RYTM_MODE_LEDS.iter().enumerate() {
            draw_lens(
                cv,
                view,
                599 + i as i32 * 58,
                415,
                led,
                Some(panel::MODE_LABELS[i]),
            );
        }
        for (i, &led) in panel::RYTM_PAGE_LEDS.iter().enumerate() {
            draw_lens(
                cv,
                view,
                1058 + i as i32 * 21,
                575,
                led,
                Some(["1", "2", "3", "4"][i]),
            );
        }
    } else {
        for (i, &led) in panel::PAGE_LEDS.iter().enumerate() {
            draw_lens(cv, view, 444 + i as i32 * 16, 376, led, None);
        }
    }
}

pub fn draw(cv: &mut Canvas, view: &View, places: &[Place], held: &[u8], latched: &[u8]) {
    draw_sel(cv, view, places, held, latched, None)
}

pub fn draw_sel(
    cv: &mut Canvas,
    view: &View,
    places: &[Place],
    held: &[u8],
    latched: &[u8],
    selected: Option<u8>,
) {
    if view.mk1 {
        if let Some(dev) = hw::dev_of(view.rytm, view.a4, view.mk1) {
            return hw::draw(cv, dev, view, places, held, latched, selected);
        }
    }
    draw_chassis(cv, view);
    cv.fill(0, 0, cv.w as i32, 29, 0x101216);
    cv.text(
        12,
        8,
        if view.a4 {
            "ANALOG FOUR MKII"
        } else if view.rytm {
            "ANALOG RYTM MKII"
        } else {
            "DIGITAKT"
        },
        2,
        TEXT,
    );
    cv.text(
        if view.rytm || view.a4 { 230 } else { 130 },
        12,
        &view.status,
        1,
        DIM,
    );
    draw_fixed_hardware(cv, view);
    draw_oled(cv, view);
    draw_panel_leds(cv, view);
    for p in places {
        match p.c {
            Control::Key(code) => draw_key(cv, view, p, code, held, latched),
            Control::Knob(enc, push) => draw_knob(cv, view, p, enc, push, held, selected),
        }
    }
    cv.fill(0, cv.h as i32 - 20, cv.w as i32, 20, 0x101216);
    cv.text(12, cv.h as i32 - 14, "CLICK PRESS   SHIFT-CLICK LATCH   DRAG/WHEEL TURN   -/= 1 STEP   [/ ] PUSH+TURN   ESC RELEASE ALL", 1, DIM);
}

/// Write the software canvas as an RGB PNG.
pub fn save_png(cv: &Canvas, path: &std::path::Path) -> std::io::Result<()> {
    let f = std::fs::File::create(path)?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(f), cv.w as u32, cv.h as u32);
    enc.set_color(png::ColorType::Rgb);
    enc.set_depth(png::BitDepth::Eight);
    let mut w = enc.write_header().map_err(std::io::Error::other)?;
    let mut data = Vec::with_capacity(cv.w * cv.h * 3);
    for &p in &cv.px {
        data.extend_from_slice(&[(p >> 16) as u8, (p >> 8) as u8, p as u8]);
    }
    w.write_image_data(&data).map_err(std::io::Error::other)
}
