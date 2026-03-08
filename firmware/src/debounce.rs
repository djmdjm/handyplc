//! Generic debouncer.
//!
//! Debounce inputs such as pushbuttons.
use crate::simpletimer::SimpleTimer;
use fugit::ExtU32;

#[derive(Default)]
enum DebounceFSMState {
    #[default]
    Off,
    DebounceOn(SimpleTimer),
    On,
    DebounceOff(SimpleTimer),
}

pub struct Debouncer {
    state: DebounceFSMState,
    posedge_read: bool,
    holdoff_time: fugit::Duration<u32, 1, 1_000>,
    holdon_time: fugit::Duration<u32, 1, 1_000>,
}

const DEBOUNCE_ON_MS: u32 = 2;
const DEBOUNCE_OFF_MS: u32 = 10;

impl Debouncer {
    pub fn default() -> Self {
        Debouncer {
            state: DebounceFSMState::Off,
            posedge_read: false,
            holdoff_time: DEBOUNCE_ON_MS.millis(),
            holdon_time: DEBOUNCE_OFF_MS.millis(),
        }
    }
    pub fn new(
        holdoff_time: fugit::Duration<u32, 1, 1_000>,
        holdon_time: fugit::Duration<u32, 1, 1_000>,
    ) -> Self {
        Debouncer {
            state: DebounceFSMState::Off,
            posedge_read: false,
            holdoff_time,
            holdon_time,
        }
    }
    pub fn update(&mut self, input: bool, now: i64) {
        match &self.state {
            DebounceFSMState::Off => {
                if input {
                    self.state =
                        DebounceFSMState::DebounceOn(SimpleTimer::start(now, self.holdoff_time))
                }
            }
            DebounceFSMState::DebounceOn(timer) => {
                if !input {
                    self.state = DebounceFSMState::Off;
                } else if timer.expired(now) {
                    self.state = DebounceFSMState::On;
                    self.posedge_read = false;
                }
            }
            DebounceFSMState::On => {
                if !input {
                    self.state =
                        DebounceFSMState::DebounceOff(SimpleTimer::start(now, self.holdon_time))
                }
            }
            DebounceFSMState::DebounceOff(timer) => {
                if input {
                    self.state = DebounceFSMState::On;
                } else if timer.expired(now) {
                    self.state = DebounceFSMState::Off;
                }
            }
        }
    }

    pub fn is_on(&self) -> bool {
        match self.state {
            DebounceFSMState::Off => false,
            DebounceFSMState::DebounceOn(_) => false,
            DebounceFSMState::On => true,
            DebounceFSMState::DebounceOff(_) => true,
        }
    }

    pub fn posedge(&mut self) -> bool {
        match self.state {
            DebounceFSMState::Off => false,
            DebounceFSMState::DebounceOn(_) => false,
            DebounceFSMState::On | DebounceFSMState::DebounceOff(_) => {
                if self.posedge_read {
                    false
                } else {
                    self.posedge_read = true;
                    true
                }
            }
        }
    }
}
