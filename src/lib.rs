//! Reusable, research-driven parsers for observed Studio Vision project data.

pub mod analysis;
pub mod app_contract;
pub mod app_service;
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
pub mod meter;
pub mod midi_export;
pub mod mixed_event;
pub mod multitrack_export;
pub mod opening;
pub mod patch;
pub mod pitch_bend;
pub mod sequence_container;
pub mod smf;
pub mod tempo;
pub mod track7;
