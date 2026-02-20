//! Morse code status output
//!
//! Used to blink a LED with a status charater
use crate::simpletimer::SimpleTimer;
use fugit::ExtU32;

#[derive(Default)]
pub struct Morse {
    timer: Option<SimpleTimer>,
    code_len: i32,
    code: u32,
    phase: i32,
    state: bool,
}

const MORSE_UNIT_MS: u32 = 300;
const MORSE_DASH: u32 = 3 * MORSE_UNIT_MS;
const MORSE_DOT: u32 = 1 * MORSE_UNIT_MS;
const MORSE_INTERVAL: u32 = 1 * MORSE_UNIT_MS;
const MORSE_GAP: u32 = 7 * MORSE_UNIT_MS;

impl Morse {
    pub fn set_char(&mut self, sym: char) {
        let (code_len, code) = Morse::morse(sym);
        // Setting the current code shouldn't restart it.
        if self.code_len != code_len || self.code != code {
            self.code_len = code_len;
            self.code = code;
            self.timer = None;
        }
    }

    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.code_len = 0;
        self.timer = None;
    }

    pub fn update(&mut self, now: i64) {
        if self.code_len == 0 {
            // Invalid or unset code.
            // TODO: better indication of invalid codes.
            self.state = false;
            return;
        }

        if let Some(timer) = &self.timer {
            if !timer.expired(now) {
                // Nothing interesting happened since last call.
                return;
            }
            // Timer expired; advance phase.
            self.phase += 1;
            self.phase %= self.code_len * 2;
        } else {
            // Just started.
            self.phase = 0;
        }

        // Phase 0 is the inter-symbol gap.
        if self.phase == 0 {
            self.timer = Some(SimpleTimer::start(now, MORSE_GAP.millis()));
            self.state = false;
            return;
        }

        // Other even-numbered phases are brief intervals between dots/dashes.
        if (self.phase & 1) == 0 {
            self.timer = Some(SimpleTimer::start(now, MORSE_INTERVAL.millis()));
            self.state = false;
            return;
        }

        // Odd phases are the dots and dashes themselves.
        if ((self.code >> (self.phase / 2)) & 1) == 0 {
            self.timer = Some(SimpleTimer::start(now, MORSE_DOT.millis()));
        } else {
            self.timer = Some(SimpleTimer::start(now, MORSE_DASH.millis()));
        }
        self.state = true;
    }

    pub fn output(&self) -> bool {
        self.timer.is_some() && self.state
    }

    #[allow(dead_code)]
    pub fn is_gap(&self) -> bool {
        self.timer.is_none() || self.phase == 0
    }

    const fn morse(sym: char) -> (i32, u32) {
        match sym {
            // Letters.
            'A' => (2, 0b0010),    // .-
            'B' => (4, 0b0001),  // -...
            'C' => (4, 0b0101),  // -.-.
            'D' => (3, 0b0001),  // -..
            'E' => (1, 0b0000),  // .
            'F' => (4, 0b0100),  // ..-.
            'G' => (3, 0b0011),  // --.
            'H' => (4, 0b0000),  // ....
            'I' => (2, 0b0000),  // ..
            'J' => (4, 0b1110),  // .---
            'K' => (3, 0b0101),  // -.-
            'L' => (4, 0b0010),  // .-..
            'M' => (2, 0b0011),  // --
            'N' => (2, 0b0001),  // -.
            'O' => (3, 0b0111),  // ---
            'P' => (4, 0b0110),  // .--.
            'Q' => (4, 0b1011),  // --.-
            'R' => (3, 0b0010),  // .-.
            'S' => (3, 0b0000),  // ...
            'T' => (1, 0b0001),  // -
            'U' => (3, 0b0100),  // ..-
            'V' => (4, 0b1000),  // ...-
            'W' => (3, 0b0110),  // .--
            'X' => (4, 0b1001),  // -..-
            'Y' => (4, 0b1101),  // -.--
            'Z' => (4, 0b0011),  // --..
            // Numbers.
            '0' => (5, 0b11111), // -----
            '1' => (5, 0b11110), // .----
            '2' => (5, 0b11100), // ..---
            '3' => (5, 0b11000), // ...--
            '4' => (5, 0b10000), // ....-
            '5' => (5, 0b00000), // .....
            '6' => (5, 0b00001), // -....
            '7' => (5, 0b00011), // --...
            '8' => (5, 0b00111), // ---..
            '9' => (5, 0b01111), // ----.
            // TODO: other symbols.
            _ => (0, 0),
        }
    }
}
