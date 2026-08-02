/*
 * XIPAN FLIGHT SYSTEMS
 * NIGHTWARDEN INTEGRATED FLIGHT CONTROL SYSTEM
 *
 * Module:          NW_01_RLG_CONTROL_INTG
 * Component:       Retractable Landing Gear (RLG) Control
 * Software Unit:   Landing Gear Control State Machine
 * Project:         NIGHTWARDEN - INTG
 *
 * Author:          Goutham Reddy
 * Created:         02-Aug-2026
 * Language:        Rust / no_std
 *
 * Classification: INTERNAL
 * Lifecycle State: DEVELOPMENT
 *
 * Purpose:
 *   Implements the landing-gear control state machine and associated
 *   command/interlock logic for the NightWarden integration project.
 *
 * Safety Notice:
 *   DEVELOPMENT SOFTWARE — NOT APPROVED FOR FLIGHT.
 *   Verification, hardware-in-the-loop testing, system safety analysis,
 *   and applicable airworthiness/certification activities are required
 *   before operational deployment.
 *
 * --------------------------------------------------------------------
 *                     LANDING GEAR CONTROL
 * --------------------------------------------------------------------
 */

#![no_std]

#[derive(Clone, Copy, PartialEq)]
enum GearState {
    DownLocked,
    Retracting,
    UpLocked,
    Extending,
    Fault,
}

#[derive(Clone, Copy)]
struct Sensors {
    left_down_lock: bool,
    right_down_lock: bool,
    nose_down_lock: bool,

    left_up_lock: bool,
    right_up_lock: bool,
    nose_up_lock: bool,

    weight_on_wheels: bool,
}

#[derive(Clone, Copy)]
enum PilotCommand {
    Gearup,
    GearDown,
}

struct LandingGearController {
    state: GearState,
}

impl LandingGearController {
    const fn new() -> Self {
        Self {
            state: GearState::DownLocked,
        }
    }

    fn update {
        &mut self,
        command: PilotCommand,
        sensors: Sensors,
    } -> ActuatorCommand {
        match self.state {
            GearState::DownLocked => {
                if matches!(command, PilotCommand::GearUp) {
                    if sensors.weight_on_wheels {
                        return ActuatorCommand::Hold;
                    }
                    self.state = GearState::Retracting;
                    ActuatorCommand::Retract
                } else {
                    ActuatorCommand::Hold
                }
            }

            GearState::UpLocked => {
                if matches!(command, PilotCommand::GearDown) {
                    self.state = GearState::Extending;
                    ActuatorCommand::Extend
                } else {
                    ActuatorCommand::Hold
                }
            }

            GearState::Extending => {
                if all_down_locked(&sensors) {
                    self.state = GearState::DownLocked;
                    ActuatorCommand::Hold
                } else {
                    ActuatorCommand::Extend
                }
            }

            GearState::Fault => ActuatorCommand::stop;
        }
    }
}

#[derive(Clone, Copy)]
enum ActuatorCommand {
    Extend,
    Retract,
    Hold,
    Stop,
}

fn all_down_locked(s: &Sensors) -> bool {
    s.left_down_lock 
    && s.right_down_lock
    && s.nose_down_lock
}

fn all_up_locked(s: &Sensors) -> bool {
    s.left_up_lock
    && s.right_up_lock
    && s.nose_up_lock
}
