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

#[derive(Default)]
pub struct Debouncer {
    state: DebounceFSMState,
    posedge_read: bool,
}

const DEBOUNCE_ON_MS: u32 = 2;
const DEBOUNCE_OFF_MS: u32 = 10;

impl Debouncer {
    pub fn update(&mut self, input: bool, now: i64) {
        match &self.state {
            DebounceFSMState::SOff => {
                if input {
                    self.state = DebounceFSMState::SDebounceOn(SimpleTimer::start(
                        now,
                        DEBOUNCE_ON_MS.millis(),
                    ))
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
                    self.state = DebounceFSMState::SDebounceOff(SimpleTimer::start(
                        now,
                        DEBOUNCE_OFF_MS.millis(),
                    ))
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

    #[allow(dead_code)]
    pub fn output(&self) -> bool {
        match self.state {
            DebounceFSMState::SOff => false,
            DebounceFSMState::SDebounceOn(_) => false,
            DebounceFSMState::SOn => true,
            DebounceFSMState::SDebounceOff(_) => true,
        }
    }

    #[allow(dead_code)]
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
