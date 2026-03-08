//! Generic debouncer.
//!
//! Debounce inputs such as pushbuttons.
use crate::simpletimer::SimpleTimer;
use fugit::ExtU32;

#[derive(Default)]
enum DebounceFSMState {
    #[default]
    SOff,
    SDebounceOn(SimpleTimer),
    SOn,
    SDebounceOff(SimpleTimer),
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
            state: DebounceFSMState::SOff,
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
            state: DebounceFSMState::SOff,
            posedge_read: false,
            holdoff_time: holdoff_time,
            holdon_time: holdon_time,
        }
    }
    pub fn update(&mut self, input: bool, now: i64) {
        match &self.state {
            DebounceFSMState::SOff => {
                if input {
                    self.state =
                        DebounceFSMState::SDebounceOn(SimpleTimer::start(now, self.holdoff_time))
                }
            }
            DebounceFSMState::SDebounceOn(timer) => {
                if !input {
                    self.state = DebounceFSMState::SOff;
                } else if timer.expired(now) {
                    self.state = DebounceFSMState::SOn;
                    self.posedge_read = false;
                }
            }
            DebounceFSMState::SOn => {
                if !input {
                    self.state =
                        DebounceFSMState::SDebounceOff(SimpleTimer::start(now, self.holdon_time))
                }
            }
            DebounceFSMState::SDebounceOff(timer) => {
                if input {
                    self.state = DebounceFSMState::SOn;
                } else if timer.expired(now) {
                    self.state = DebounceFSMState::SOff;
                }
            }
        }
    }

    pub fn is_on(&self) -> bool {
        match self.state {
            DebounceFSMState::SOff => false,
            DebounceFSMState::SDebounceOn(_) => false,
            DebounceFSMState::SOn => true,
            DebounceFSMState::SDebounceOff(_) => true,
        }
    }

    pub fn posedge(&mut self) -> bool {
        match self.state {
            DebounceFSMState::SOff => false,
            DebounceFSMState::SDebounceOn(_) => false,
            DebounceFSMState::SOn | DebounceFSMState::SDebounceOff(_) => {
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
