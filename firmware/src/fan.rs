//! Spindle fan PLC logic.
//!
//! This implements a simple hold-on/hold-off controller for a spindle
//! cooling fan.
use fugit::ExtU32;
use crate::simpletimer::SimpleTimer;

#[derive(Default)]
enum FanFSMState {
    #[default]
    FOff,
    FHoldOff(SimpleTimer),
    FOn,
    FHoldOn(SimpleTimer),
}

#[derive(Default)]
pub struct FanControl {
    state: FanFSMState,
}

const FAN_HOLDOFF_SECS: u32 = 60;
const FAN_HOLDON_SECS: u32 = 300;

impl FanControl {
    pub fn update(&mut self, spindle_on: bool, now: i64) {
        match &self.state {
            FanFSMState::FOff => {
                if spindle_on {
                    self.state =
                        FanFSMState::FHoldOff(SimpleTimer::start(now, FAN_HOLDOFF_SECS.secs()))
                }
            }
            FanFSMState::FHoldOff(holdoff) => {
                if !spindle_on {
                    self.state = FanFSMState::FOff
                } else if holdoff.expired(now) {
                    self.state = FanFSMState::FOn
                }
            }
            FanFSMState::FOn => {
                if !spindle_on {
                    self.state =
                        FanFSMState::FHoldOn(SimpleTimer::start(now, FAN_HOLDON_SECS.secs()))
                }
            }
            FanFSMState::FHoldOn(holdon) => {
                if spindle_on {
                    self.state = FanFSMState::FOn
                } else if holdon.expired(now) {
                    self.state = FanFSMState::FOff
                }
            }
        }
    }

    pub fn fan_state(&self) -> bool {
        match self.state {
            FanFSMState::FOff => false,
            FanFSMState::FHoldOff(_) => false,
            FanFSMState::FOn => true,
            FanFSMState::FHoldOn(_) => true,
        }
    }

    pub fn status_char(&self) -> char {
        match self.state {
            FanFSMState::FOff => 'N',
            FanFSMState::FHoldOff(_) => 'D',
            FanFSMState::FOn => 'R',
            FanFSMState::FHoldOn(_) => 'U',
        }
    }
}

