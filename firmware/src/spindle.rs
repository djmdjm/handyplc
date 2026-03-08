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
    Off,
    WaitBrakeOff(SimpleTimer),
    Running,
    WaitBrakeOn(SimpleTimer),
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
            SpindleFSMState::Off => {
                if spindle_on && !spindle_inhibit {
                    self.state = SpindleFSMState::WaitBrakeOff(SimpleTimer::start(
                        now,
                        BRAKE_OFF_MS.millis(),
                    ))
                }
            }
            SpindleFSMState::WaitBrakeOff(timer) => {
                if !spindle_on || spindle_inhibit {
                    self.state = SpindleFSMState::Off;
                } else if timer.expired(now) {
                    self.state = SpindleFSMState::Running;
                }
            }
            SpindleFSMState::Running => {
                if !spindle_on || spindle_inhibit {
                    self.state =
                        SpindleFSMState::WaitBrakeOn(SimpleTimer::start(now, BRAKE_ON_MS.millis()))
                }
            }
            SpindleFSMState::WaitBrakeOn(timer) => {
                if spindle_on && !spindle_inhibit {
                    self.state = SpindleFSMState::Running;
                } else if timer.expired(now) {
                    self.state = SpindleFSMState::Off;
                }
            }
        }
    }

    pub fn spindle_on(&self) -> bool {
        match self.state {
            SpindleFSMState::Off => false,
            SpindleFSMState::WaitBrakeOff(_) => false,
            SpindleFSMState::Running => true,
            SpindleFSMState::WaitBrakeOn(_) => false,
        }
    }

    pub fn brake_on(&self) -> bool {
        match self.state {
            SpindleFSMState::Off => true,
            SpindleFSMState::WaitBrakeOff(_) => false,
            SpindleFSMState::Running => false,
            SpindleFSMState::WaitBrakeOn(_) => false,
        }
    }

    #[allow(dead_code)]
    pub fn status_char(&self) -> char {
        match self.state {
            SpindleFSMState::Off => 'O',
            SpindleFSMState::WaitBrakeOff(_) => 'D',
            SpindleFSMState::Running => 'R',
            SpindleFSMState::WaitBrakeOn(_) => 'B',
        }
    }
}
