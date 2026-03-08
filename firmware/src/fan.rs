//! Spindle fan PLC logic.
//!
//! This implements a simple hold-on/hold-off controller for a spindle
//! cooling fan.
use crate::simpletimer::SimpleTimer;
use fugit::ExtU32;

#[derive(Default)]
enum FanFSMState {
    #[default]
    Off,
    HoldOff(SimpleTimer),
    On,
    HoldOn(SimpleTimer),
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
            FanFSMState::Off => {
                if spindle_on {
                    self.state =
                        FanFSMState::HoldOff(SimpleTimer::start(now, FAN_HOLDOFF_SECS.secs()))
                }
            }
            FanFSMState::HoldOff(holdoff) => {
                if !spindle_on {
                    self.state = FanFSMState::Off
                } else if holdoff.expired(now) {
                    self.state = FanFSMState::On
                }
            }
            FanFSMState::On => {
                if !spindle_on {
                    self.state =
                        FanFSMState::HoldOn(SimpleTimer::start(now, FAN_HOLDON_SECS.secs()))
                }
            }
            FanFSMState::HoldOn(holdon) => {
                if spindle_on {
                    self.state = FanFSMState::On
                } else if holdon.expired(now) {
                    self.state = FanFSMState::Off
                }
            }
        }
    }

    pub fn fan_state(&self) -> bool {
        match self.state {
            FanFSMState::Off => false,
            FanFSMState::HoldOff(_) => false,
            FanFSMState::On => true,
            FanFSMState::HoldOn(_) => true,
        }
    }

    pub fn status_char(&self) -> char {
        match self.state {
            FanFSMState::Off => 'N',
            FanFSMState::HoldOff(_) => 'D',
            FanFSMState::On => 'R',
            FanFSMState::HoldOn(_) => 'U',
        }
    }
}
