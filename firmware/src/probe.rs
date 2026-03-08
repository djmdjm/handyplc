//! Wireless touch probe PLC logic
//!
//! This is mostly to implement probe detection, alarm logic and spindle
//! inhibition. The probe accepts an enable signal sent by the CNC controller
//! when the right tool is selected. It emits a touch detection signal which
//! goes directly to the CNC controller, as well as alarm and low-battery
//! signals that we combine here to a "probe detect" output that also goes
//! back to the CNC controller.
//!
//! The probe's wireless connection takes a moment to establish, so there
//! is a little finesse around the other signals becoming valid. Also, we
//! want to disable the spindle signal whenever the probe is active to
//! prevent stupid accident.
use crate::simpletimer::SimpleTimer;
use fugit::ExtU32;

#[derive(Default)]
enum ProbeFSMState {
    #[default]
    Off,
    WaitReady(SimpleTimer),
    Active,
    Error,
}

#[derive(Default)]
pub struct ProbeControl {
    state: ProbeFSMState,
}

const PROBE_WAIT_MS: u32 = 500;

impl ProbeControl {
    pub fn update(&mut self, probe_enable: bool, probe_alarm: bool, probe_lowbatt: bool, now: i64) {
        match &self.state {
            ProbeFSMState::Off => {
                if probe_enable {
                    self.state =
                        ProbeFSMState::WaitReady(SimpleTimer::start(now, PROBE_WAIT_MS.millis()))
                }
            }
            ProbeFSMState::WaitReady(timer) => {
                if !probe_enable {
                    self.state = ProbeFSMState::Off;
                } else if timer.expired(now) {
                    if probe_alarm || probe_lowbatt {
                        self.state = ProbeFSMState::Error;
                    } else {
                        self.state = ProbeFSMState::Active;
                    }
                }
            }
            ProbeFSMState::Active => {
                if !probe_enable {
                    self.state = ProbeFSMState::Off;
                } else if probe_alarm || probe_lowbatt {
                    self.state = ProbeFSMState::Error;
                }
            }
            ProbeFSMState::Error => {
                if !probe_enable {
                    self.state = ProbeFSMState::Off;
                } else if !probe_alarm && !probe_lowbatt {
                    self.state =
                        ProbeFSMState::WaitReady(SimpleTimer::start(now, PROBE_WAIT_MS.millis()))
                }
            }
        }
    }

    pub fn probe_power(&self) -> bool {
        match self.state {
            ProbeFSMState::Off => false,
            ProbeFSMState::WaitReady(_) => true,
            ProbeFSMState::Active => true,
            ProbeFSMState::Error => true,
        }
    }

    pub fn probe_detect(&self) -> bool {
        match self.state {
            ProbeFSMState::Off => false,
            ProbeFSMState::WaitReady(_) => false,
            ProbeFSMState::Active => true,
            ProbeFSMState::Error => false,
        }
    }

    pub fn spindle_inhibit(&self) -> bool {
        match self.state {
            ProbeFSMState::Off => false,
            ProbeFSMState::WaitReady(_) => true,
            ProbeFSMState::Active => true,
            ProbeFSMState::Error => true,
        }
    }

    pub fn status_char(&self) -> char {
        match self.state {
            ProbeFSMState::Off => 'O',
            ProbeFSMState::WaitReady(_) => 'W',
            ProbeFSMState::Active => 'A',
            ProbeFSMState::Error => 'X',
        }
    }
}
