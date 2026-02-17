//! Wireless touch probe PLC logic
use crate::simpletimer::SimpleTimer;
use fugit::ExtU32;

#[derive(Default)]
enum ProbeFSMState {
    #[default]
    POff,
    PWaitReady(SimpleTimer),
    PActive,
    PErr,
}

#[derive(Default)]
pub struct ProbeControl {
    state: ProbeFSMState,
}

const PROBE_WAIT_MS: u32 = 500;

impl ProbeControl {
    pub fn update(&mut self, probe_enable: bool, probe_alarm: bool, probe_lowbatt: bool, now: i64) {
        match &self.state {
            ProbeFSMState::POff => {
                if probe_enable {
                    self.state =
                        ProbeFSMState::PWaitReady(SimpleTimer::start(now, PROBE_WAIT_MS.millis()))
                }
            }
            ProbeFSMState::PWaitReady(timer) => {
                if !probe_enable {
                    self.state = ProbeFSMState::POff;
                } else if timer.expired(now) {
                    if probe_alarm || probe_lowbatt {
                        self.state = ProbeFSMState::PErr;
                    } else {
                        self.state = ProbeFSMState::PActive;
                    }
                }
            }
            ProbeFSMState::PActive => {
                if !probe_enable {
                    self.state = ProbeFSMState::POff;
                } else if probe_alarm || probe_lowbatt {
                    self.state = ProbeFSMState::PErr;
                }
            }
            ProbeFSMState::PErr => {
                if !probe_enable {
                    self.state = ProbeFSMState::POff;
                } else if !probe_alarm && !probe_lowbatt {
                    self.state =
                        ProbeFSMState::PWaitReady(SimpleTimer::start(now, PROBE_WAIT_MS.millis()))
                }
            }
        }
    }

    pub fn probe_power(&self) -> bool {
        match self.state {
            ProbeFSMState::POff => false,
            ProbeFSMState::PWaitReady(_) => true,
            ProbeFSMState::PActive => true,
            ProbeFSMState::PErr => true,
        }
    }

    pub fn probe_detect(&self) -> bool {
        match self.state {
            ProbeFSMState::POff => false,
            ProbeFSMState::PWaitReady(_) => false,
            ProbeFSMState::PActive => true,
            ProbeFSMState::PErr => false,
        }
    }

    pub fn status_char(&self) -> char {
        match self.state {
            ProbeFSMState::POff => 'O',
            ProbeFSMState::PWaitReady(_) => 'W',
            ProbeFSMState::PActive => 'A',
            ProbeFSMState::PErr => 'X',
        }
    }
}
