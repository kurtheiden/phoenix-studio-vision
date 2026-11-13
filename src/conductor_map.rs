//! Serialization-only handoff of caller-bounded maps; no envelope/export authority.
use super::*;
use crate::meter_key_map::{BoundedMeterKeyMap, MeterKeyKind};
use crate::tempo::BoundedTempoMap;

pub(crate) type ReferenceEvent = (u32, u8, Vec<u8>);
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum MapError {
    AmbiguousTie { tick: u32 },
    ReferenceMismatch,
    UnsupportedMeter,
    Arithmetic,
    Smf(SmfSerializeError),
}
impl From<SmfSerializeError> for MapError {
    fn from(e: SmfSerializeError) -> Self {
        Self::Smf(e)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Origin {
    Tempo(usize),
    MeterKey(usize),
}

/// Immutable, map-bound merge order. Does not qualify enclosing SV records.
pub(crate) struct ConductorMapPlan<'a> {
    tempo: Option<&'a BoundedTempoMap<'a>>,
    meter_key: &'a BoundedMeterKeyMap<'a>,
    order: Vec<Origin>,
}
impl ConductorMapPlan<'_> {
    pub(crate) fn order(&self) -> &[Origin] {
        &self.order
    }
    pub(crate) fn tempo(&self) -> &BoundedTempoMap<'_> {
        self.tempo.expect("Tempo origin requires a Tempo map")
    }
    pub(crate) fn meter_key(&self) -> &BoundedMeterKeyMap<'_> {
        self.meter_key
    }
}
fn meter_bytes(kind: &MeterKeyKind) -> Result<Vec<u8>, MapError> {
    match kind {
        MeterKeyKind::Key(k) => Ok(vec![0xff, 0x59, 2, k.accidentals_byte.value, k.mode.value]),
        MeterKeyKind::Meter(m) => {
            let (s, warnings) = crate::midi_export::adapt_meter_values(
                m.numerator.value,
                m.denominator_exponent.value,
                m.third_payload.value,
                m.fourth_payload.value,
                crate::midi_export::MeterPolicy::KnownHistoricalOnly,
            )
            .map_err(|_| MapError::UnsupportedMeter)?;
            if !warnings.is_empty() {
                return Err(MapError::UnsupportedMeter);
            }
            Ok(serialize_time_signature(s))
        }
    }
}
fn event(plan: &ConductorMapPlan<'_>, origin: Origin) -> Result<ReferenceEvent, MapError> {
    match origin {
        Origin::Tempo(i) => {
            let e = &plan.tempo().events()[i];
            Ok((
                e.absolute_tick,
                0x51,
                serialize_set_tempo(e.mpqn())?[3..].to_vec(),
            ))
        }
        Origin::MeterKey(i) => {
            let e = &plan.meter_key.events()[i];
            let bytes = meter_bytes(&e.kind)?;
            Ok((e.absolute_tick, bytes[1], bytes[3..].to_vec()))
        }
    }
}
/// A Meter/Key-only serialization handoff. No Tempo event is invented; this
/// does not constitute a complete conductor or source-export plan.
pub(crate) fn prepare_meter_key<'a>(
    meter_key: &'a BoundedMeterKeyMap<'a>,
) -> Result<ConductorMapPlan<'a>, MapError> {
    for e in meter_key.events() {
        meter_bytes(&e.kind)?;
    }
    Ok(ConductorMapPlan {
        tempo: None,
        meter_key,
        order: (0..meter_key.events().len())
            .map(Origin::MeterKey)
            .collect(),
    })
}
/// No general cross-map precedence is inferred. At zero only the established
/// single Tempo then optional Key then single Meter profile resolves a tie.
pub(crate) fn prepare<'a>(
    tempo: &'a BoundedTempoMap<'a>,
    meter_key: &'a BoundedMeterKeyMap<'a>,
) -> Result<ConductorMapPlan<'a>, MapError> {
    prepare_inner(tempo, meter_key, None)
}
/// The caller must authenticate an independently parsed reference and establish
/// its correspondence for the supplied maps. Supply ALL and ONLY
/// Tempo/Meter/Key objects, in reference order. Every value, tick, count and each
/// source stream's order must correlate exactly; this is not an ordering hint.
/// The proof stays bound to these immutable maps; it grants no export authority.
pub(crate) fn correlate_reference<'a>(
    tempo: &'a BoundedTempoMap<'a>,
    meter_key: &'a BoundedMeterKeyMap<'a>,
    reference: &[ReferenceEvent],
) -> Result<ConductorMapPlan<'a>, MapError> {
    prepare_inner(tempo, meter_key, Some(reference))
}
fn prepare_inner<'a>(
    tempo: &'a BoundedTempoMap<'a>,
    meter_key: &'a BoundedMeterKeyMap<'a>,
    reference: Option<&[ReferenceEvent]>,
) -> Result<ConductorMapPlan<'a>, MapError> {
    let total = tempo
        .events()
        .len()
        .checked_add(meter_key.events().len())
        .ok_or(MapError::Arithmetic)?;
    let mut plan = ConductorMapPlan {
        tempo: Some(tempo),
        meter_key,
        order: Vec::new(),
    };
    // Validate even events that would precede an ambiguous tie.
    for e in meter_key.events() {
        meter_bytes(&e.kind)?;
    }
    let zero_t = tempo
        .events()
        .iter()
        .take_while(|e| e.absolute_tick == 0)
        .count();
    let zero_m: Vec<_> = meter_key
        .events()
        .iter()
        .take_while(|e| e.absolute_tick == 0)
        .collect();
    if zero_t > 0 && !zero_m.is_empty() {
        let initial = zero_t == 1
            && (matches!(zero_m.as_slice(), [m] if matches!(m.kind, MeterKeyKind::Meter(_)))
                || matches!(zero_m.as_slice(), [k,m] if matches!(k.kind, MeterKeyKind::Key(_)) && matches!(m.kind, MeterKeyKind::Meter(_))));
        if !initial {
            return Err(MapError::AmbiguousTie { tick: 0 });
        }
    }
    let (mut t, mut m) = (0, 0);
    if let Some(r) = reference {
        if r.len() != total {
            return Err(MapError::ReferenceMismatch);
        }
        let mut previous = 0;
        for wanted in r {
            let origin = match wanted.1 {
                0x51 if t < tempo.events().len() => {
                    let i = t;
                    t += 1;
                    Origin::Tempo(i)
                }
                0x58 | 0x59 if m < meter_key.events().len() => {
                    let i = m;
                    m += 1;
                    Origin::MeterKey(i)
                }
                _ => return Err(MapError::ReferenceMismatch),
            };
            if event(&plan, origin)? != *wanted || wanted.0 < previous {
                return Err(MapError::ReferenceMismatch);
            }
            // Preserve legacy initial precedence even when a reference is supplied.
            if wanted.0 == 0
                && zero_t == 1
                && !zero_m.is_empty()
                && plan.order.is_empty()
                && origin != Origin::Tempo(0)
            {
                return Err(MapError::ReferenceMismatch);
            }
            previous = wanted.0;
            plan.order.push(origin);
        }
    } else {
        while t < tempo.events().len() || m < meter_key.events().len() {
            let tt = tempo.events().get(t).map(|e| e.absolute_tick);
            let mt = meter_key.events().get(m).map(|e| e.absolute_tick);
            let take_t = match (tt, mt) {
                (Some(a), Some(b)) if a == b => {
                    if a != 0 {
                        return Err(MapError::AmbiguousTie { tick: a });
                    }
                    true
                }
                (Some(a), Some(b)) => a < b,
                (Some(_), None) => true,
                _ => false,
            };
            if take_t {
                plan.order.push(Origin::Tempo(t));
                t += 1;
            } else {
                plan.order.push(Origin::MeterKey(m));
                m += 1;
            }
        }
    }
    if t != tempo.events().len() || m != meter_key.events().len() {
        return Err(MapError::ReferenceMismatch);
    }
    Ok(plan)
}
/// Preserve every object, recomputing only SMF deltas between proven absolute
/// positions. No default Tempo/Meter, offset, scaling, deduplication or padding.
pub(crate) fn serialize(
    name: &[u8],
    plan: &ConductorMapPlan<'_>,
) -> Result<SerializedTrack, MapError> {
    let mut payload = vec![0];
    payload.extend(serialize_track_name(name)?);
    let mut previous = 0;
    for origin in &plan.order {
        let (tick, tag, value) = event(plan, *origin)?;
        let delta = tick.checked_sub(previous).ok_or(MapError::Arithmetic)?;
        payload.extend(encode_midi_vlq(delta)?);
        payload.extend([0xff, tag]);
        payload.extend(encode_midi_vlq(
            u32::try_from(value.len()).map_err(|_| MapError::Arithmetic)?,
        )?);
        payload.extend(value);
        previous = tick;
    }
    payload.push(0);
    payload.extend(serialize_end_of_track());
    Ok(make_track_chunk(payload)?)
}
#[cfg(test)]
#[path = "conductor_map_tests.rs"]
mod tests;
