//! Reusable, research-driven parsers for observed Studio Vision project data.

pub mod analysis;
pub mod app_contract;
pub mod app_service;
pub mod bounded_multi_patch;
pub mod bounded_patch_translation;
pub mod bounded_routing;
pub mod bounded_sequence;
mod c_abi;
pub mod channel_pressure;
pub mod comparison;
pub mod compatibility;
pub mod compatibility_profiles;
pub mod controller;
pub(crate) mod export_handoff;
#[allow(dead_code)]
pub(crate) mod identification;
#[allow(dead_code)]
pub(crate) mod inspection;
pub mod json_transport;
pub mod meter;
pub mod midi_export;
pub mod mixed_event;
pub mod multitrack_export;
mod observed_layout120;
pub mod opening;
pub mod patch;
pub mod pitch_bend;
mod prologue_inspection;
pub mod routing_evidence;
pub mod saved_mute;
mod semantic_layout120;
pub mod sequence_container;
pub mod smf;
pub mod tempo;
pub mod track7;
