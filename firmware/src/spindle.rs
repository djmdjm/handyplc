//! Spindle controller.
//!
//! This sequences spindle startup with brake disengagement/reengagement.
//! In the future this will also be used to sequence active spindle braking,
//! which for some reason doesn't work via the signals I'm currently using on
//! my servo.
use crate::simpletimer::SimpleTimer;
use fugit::ExtU32;

#[derive(Default)]
enum SpindleFSMState {
    #[default]
    SOff,
    SWaitBrakeOff(SimpleTimer),
    SRunning,
    SWaitBrakeOn(SimpleTimer),
}

#[derive(Default)]
pub struct SpindleControl {
    state: SpindleFSMState,
}

const BRAKE_OFF_MS: u32 = 50;
const BRAKE_ON_MS: u32 = 1000;

impl SpindleControl {
    pub fn update(&mut self, spindle_on: bool, spindle_inhibit: bool, now: i64) {
        match &self.state {
            SpindleFSMState::SOff => {
                if spindle_on && !spindle_inhibit {
                    self.state = SpindleFSMState::SWaitBrakeOff(SimpleTimer::start(
                        now,
                        BRAKE_OFF_MS.millis(),
                    ))
                }
            }
            SpindleFSMState::SWaitBrakeOff(timer) => {
                if !spindle_on || spindle_inhibit {
                    self.state = SpindleFSMState::SOff;
                } else if timer.expired(now) {
                    self.state = SpindleFSMState::SRunning;
                }
            }
            SpindleFSMState::SRunning => {
                if !spindle_on || spindle_inhibit {
                    self.state =
                        SpindleFSMState::SWaitBrakeOn(SimpleTimer::start(now, BRAKE_ON_MS.millis()))
                }
            }
            SpindleFSMState::SWaitBrakeOn(timer) => {
                if spindle_on && !spindle_inhibit {
                    self.state = SpindleFSMState::SRunning;
                } else if timer.expired(now) {
                    self.state = SpindleFSMState::SOff;
                }
            }
        }
    }

    pub fn spindle_on(&self) -> bool {
        match self.state {
            SpindleFSMState::SOff => false,
            SpindleFSMState::SWaitBrakeOff(_) => false,
            SpindleFSMState::SRunning => true,
            SpindleFSMState::SWaitBrakeOn(_) => false,
        }
    }

    pub fn brake_on(&self) -> bool {
        match self.state {
            SpindleFSMState::SOff => true,
            SpindleFSMState::SWaitBrakeOff(_) => false,
            SpindleFSMState::SRunning => false,
            SpindleFSMState::SWaitBrakeOn(_) => false,
        }
    }

    #[allow(dead_code)]
    pub fn status_char(&self) -> char {
        match self.state {
            SpindleFSMState::SOff => 'O',
            SpindleFSMState::SWaitBrakeOff(_) => 'D',
            SpindleFSMState::SRunning => 'R',
            SpindleFSMState::SWaitBrakeOn(_) => 'B',
        }
    }
}
