//! Servo reset PLC logic
//!
//! If the my servo fault trips then it needs a reset signal to come out of
//! the fault condition. Unfortunately there's a race condition between the
//! servo's alarm output, which is asserted whenever there's a fault and is
//! used to trip the Safe Torque Off input to the servo, and my CNC
//! controller's reset output, which is deasserted a bit too quickly when
//! clearing a fault.
//!
//! The result of this is that it's impossible to reset the servo out of
//! fault - the servo reset happens before or too quickly after the CNC
//! alarm is deasserted, and the servo's alarm output instantly re-trips
//! the CNC controller.
//!
//! The solution to this is just to hold the reset signal up for long enough
//! that the CNC controller alarm has cleared.
use fugit::ExtU32;
use crate::simpletimer::SimpleTimer;

#[derive(Default)]
enum ServoResetFSMState {
    #[default]
    Roff,
    ROn,
    RHoldOn(SimpleTimer),
}

#[derive(Default)]
pub struct ServoResetControl {
    state: ServoResetFSMState,
}

const RESET_HOLDON_MS: u32 = 50;

impl ServoResetControl {
    pub fn update(&mut self, reset_on: bool, now: i64) {
        match &self.state {
            ServoResetFSMState::Roff => {
                if reset_on {
                    self.state = ServoResetFSMState::ROn;
                }
            }
            ServoResetFSMState::ROn => {
                if !reset_on {
                    self.state =
                        ServoResetFSMState::RHoldOn(SimpleTimer::start(now, RESET_HOLDON_MS.millis()))
                }
            }
            ServoResetFSMState::RHoldOn(holdon) => {
                if reset_on {
                    self.state = ServoResetFSMState::ROn
                } else if holdon.expired(now) {
                    self.state = ServoResetFSMState::Roff
                }
            }
        }
    }

    pub fn reset_state(&self) -> bool {
        match self.state {
            ServoResetFSMState::Roff => false,
            ServoResetFSMState::ROn => true,
            ServoResetFSMState::RHoldOn(_) => true,
        }
    }

    #[allow(dead_code)]
    pub fn status_char(&self) -> char {
        match self.state {
            ServoResetFSMState::Roff => 'L',
            ServoResetFSMState::ROn => 'M',
            ServoResetFSMState::RHoldOn(_) => 'Y',
        }
    }
}
