//! Initial-envelope parity only; never compiled into production recovery.
use super::*;
use crate::{meter_key_map, readback120, smf, tempo};

#[path = "../tests/support/conductor_fixture.rs"]
mod conductor_fixture;
use conductor_fixture::{fixture, payloads, reframe, variant_fixture};

struct InitialMaps<'a> {
    tempo: tempo::BoundedTempoMap<'a>,
    meter_key: meter_key_map::BoundedMeterKeyMap<'a>,
}

fn parity_bridge<'a>(
    source: &'a [u8],
    ranges: [Range<usize>; 4],
    name: &[u8],
) -> Result<InitialMaps<'a>, String> {
    // Revalidate all four records against this exact source before deriving any
    // map bounds. Never accept a detached conductor with caller-replaced bytes.
    let conductor = decode_compact_conductor(source, ranges).map_err(|e| format!("{e:?}"))?;
    let key = conductor.key();
    let meter_range = conductor.meter().event_range.clone();
    let start = key.map_or(meter_range.start, |k| k.source_range.start);
    // These counts are already proved by the complete initial-only envelope.
    let maps = InitialMaps {
        tempo: tempo::decode_bounded_tempo_map(
            source,
            tempo::TempoMapBounds {
                event_range: conductor.tempo().event_range.clone(),
                declared_count: 1,
            },
        )
        .map_err(|e| format!("{e:?}"))?,
        meter_key: meter_key_map::decode_bounded_meter_key_map(
            source,
            meter_key_map::MeterKeyMapBounds {
                event_range: start..meter_range.end,
                declared_count: if key.is_some() { 2 } else { 1 },
            },
        )
        .map_err(|e| format!("{e:?}"))?,
    };
    let adapted = conductor.adapt(name).map_err(|e| format!("{e:?}"))?;
    let plan =
        smf::conductor_map::prepare(&maps.tempo, &maps.meter_key).map_err(|e| format!("{e:?}"))?;
    let track = smf::conductor_map::serialize(&adapted.sequence_name, &plan)
        .map_err(|e| format!("{e:?}"))?;
    let expected = readback120::conductor_map_expected(&adapted.sequence_name, &plan)
        .map_err(|e| format!("{e:?}"))?;
    let midi = smf::serialize_format1(adapted.ppqn, std::slice::from_ref(&track))
        .map_err(|e| format!("{e:?}"))?;
    readback120::verify(&midi, adapted.ppqn, &[expected]).map_err(str::to_owned)?;
    let legacy = smf::serialize_conductor_track_with_key(
        &adapted.sequence_name,
        adapted.tempo_mpqn,
        adapted.time_signature,
        key.map(|k| (k.accidentals, k.minor)),
    )
    .map_err(|e| format!("{e:?}"))?;
    if track.as_bytes() != legacy.as_bytes() {
        return Err("initial conductor byte parity mismatch".into());
    }
    Ok(maps)
}

#[test]
fn accepted_initial_envelopes_preserve_bounds_counts_bytes_and_readback() {
    for (source, ranges) in [
        fixture([7, 161, 32], [253, 1]),
        fixture([8, 0, 1], [2, 0]),
        variant_fixture(false),
        variant_fixture(true),
    ] {
        let c = decode_compact_conductor(&source, ranges.clone()).unwrap();
        let maps = parity_bridge(&source, ranges.clone(), b"Synthetic parity").unwrap();
        assert_eq!(maps.tempo.event_range(), c.tempo().event_range);
        assert_eq!(maps.tempo.events().len(), 1);
        assert_eq!(maps.tempo.events()[0].source_range, c.tempo().event_range);
        let meter_range = c.meter().event_range.clone();
        let start = c.key().map_or(meter_range.start, |k| k.source_range.start);
        assert_eq!(maps.meter_key.event_range(), start..meter_range.end);
        let declared = u32::from_be_bytes(
            source[ranges[0].start + 7..ranges[0].start + 11]
                .try_into()
                .unwrap(),
        );
        assert_eq!(maps.meter_key.declared_count(), declared);
        assert_eq!(maps.meter_key.events().len(), declared as usize);
        let mut cursor = start;
        for event in maps.meter_key.events() {
            assert_eq!(event.source.range.start, cursor);
            assert_eq!(event.source.bytes, &source[event.source.range.clone()]);
            assert_eq!(
                event.source.bytes.as_ptr(),
                source[event.source.range.clone()].as_ptr()
            );
            assert_eq!(event.absolute_tick, 0);
            cursor = event.source.range.end;
        }
        assert_eq!(cursor, meter_range.end);
        let tags: Vec<_> = maps
            .meter_key
            .events()
            .iter()
            .map(|e| e.event_tag.value)
            .collect();
        assert_eq!(
            tags,
            if c.key().is_some() {
                vec![0x59, 0x58]
            } else {
                vec![0x58]
            }
        );
    }
}

fn same_refusal(source: &[u8], ranges: [Range<usize>; 4]) {
    let legacy = decode_compact_conductor(source, ranges.clone()).unwrap_err();
    let bridged = parity_bridge(source, ranges, b"Synthetic").err().unwrap();
    assert_eq!(bridged, format!("{legacy:?}"));
}

#[test]
fn contradictory_secondary_copies_and_unsupported_layouts_keep_refusals() {
    for padded in [false, true] {
        let (source, ranges) = variant_fixture(padded);
        for record in [1, 3] {
            let mut changed = source.clone();
            changed[ranges[record].end - 1] ^= 1;
            same_refusal(&changed, ranges.clone());
        }
        for n in 0..source.len() {
            same_refusal(&source[..n], ranges.clone());
        }
        let mut swapped = ranges.clone();
        swapped.swap(0, 2);
        same_refusal(&source, swapped);
        let mut parts = payloads(&source, &ranges);
        parts[0][14] = 1; // Nonzero initial timing is still unsupported.
        let (bytes, bounds) = reframe(&parts);
        same_refusal(&bytes, bounds);
    }
}

#[test]
fn extra_counted_events_or_events_hidden_in_padding_still_refuse() {
    let (source, ranges) = variant_fixture(true);
    let original = payloads(&source, &ranges);
    for extra in [
        vec![0, 255, 81, 3, 8, 0, 1],
        vec![0, 255, 88, 4, 4, 2, 8, 8],
        vec![0, 255, 89, 2, 0, 0],
    ] {
        for record in [0, 2] {
            let end = if record == 0 { 22 } else { 21 };
            let mut parts = original.clone();
            parts[record][end + 7..end + 7 + extra.len()].copy_from_slice(&extra);
            let (bytes, bounds) = reframe(&parts);
            same_refusal(&bytes, bounds);
            let mut parts = original.clone();
            parts[record].splice(end..end, extra.iter().copied());
            parts[record][2..6].copy_from_slice(&2u32.to_be_bytes());
            parts[record + 1][6..10].copy_from_slice(&2u32.to_be_bytes());
            parts[record + 1][10..14]
                .copy_from_slice(&((end - 14 + extra.len() + 4) as u32).to_be_bytes());
            let (bytes, bounds) = reframe(&parts);
            same_refusal(&bytes, bounds);
        }
    }
}

#[test]
fn complete_conductor_diagnostics_preserve_refusal_and_secondary_locations() {
    let (source, ranges) = variant_fixture(true);
    for (record, local, predicate, expected) in [
        (0, 1, "header byte 1", 1..2),
        (0, 5, "declared count", 2..6),
        (0, 29, "zero padding", 29..38),
        (1, 6, "secondary framing/count/length", 0..19),
        (1, 24, "secondary value copy/zero fields", 20..29),
    ] {
        let mut changed = source.clone();
        changed[ranges[record].start + 5 + local] ^= 0x40;
        let old = decode_compact_conductor(&changed, ranges.clone()).unwrap_err();
        let (failure, context) =
            decode_compact_conductor_diagnostic(&changed, ranges.clone()).unwrap_err();
        assert_eq!(failure, old);
        let context = context.unwrap();
        assert_eq!(context.predicate, predicate);
        assert_eq!(context.record_range, ranges[record]);
        assert_eq!(
            context.field_range,
            ranges[record].start + 5 + expected.start..ranges[record].start + 5 + expected.end
        );
        assert_eq!(
            context.role,
            if record == 0 {
                "Meter primary"
            } else {
                "Meter secondary"
            }
        );
        assert!(context.describe().contains("applicability unresolved"));
    }
    let mut wrong = ranges.clone();
    wrong[0].start += 1;
    let (failure, context) = decode_compact_conductor_diagnostic(&source, wrong).unwrap_err();
    assert_eq!(failure, ConductorRefusal::RecordBounds);
    assert!(context.is_none()); // No fabricated field provenance when framing fails.
}
