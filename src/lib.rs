//! Reusable, research-driven parsers for observed Studio Vision project data.

pub mod analysis;
pub mod app_contract;
pub mod app_service;
pub mod authority120;
pub mod bank_patch166_handoff;
pub mod bounded_conductor;
pub mod bounded_multi_patch;
pub mod bounded_patch_translation;
pub mod bounded_prelude166;
pub mod bounded_routing;
pub mod bounded_sequence;
pub mod bounded_terminal;
pub mod bounded_terminal166;
mod c_abi;
pub mod census;
pub mod channel_pressure;
pub mod comparison;
pub mod compatibility;
pub mod compatibility_profiles;
pub mod context_controller166;
pub mod context_note166;
pub mod controller;
pub mod controller166;
pub mod controller_target120;
pub mod experimental120;
pub mod experimental166;
pub mod experimental_export;
pub(crate) mod export_handoff;
#[allow(dead_code)]
pub(crate) mod identification;
#[allow(dead_code)]
pub(crate) mod inspection;
pub mod instrument_transposition;
pub mod json_transport;
pub mod meter;
pub mod midi_export;
pub mod mixed_event;
mod mixed_partial166;
pub mod multitrack_export;
pub mod numeric_program166;
mod observed_layout120;
pub mod opening;
pub mod patch;
pub mod pitch_bend;
pub mod program_only166;
mod prologue_inspection;
mod readback120;
pub mod recovery120;
pub mod routing_evidence;
pub mod saved_mute;
mod semantic_layout120;
pub mod sequence_container;
pub mod simultaneous_note166;
pub mod smf;
pub mod standalone_bank166;
pub mod sysex166;
pub mod tempo;
pub mod terminal_program166;
pub mod track7;

#[cfg(test)]
#[path = "../tests/support/reference_smf.rs"]
mod reference_smf;
