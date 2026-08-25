//! Exact-bounded walking for the authenticated Studio Vision mixed-event form.
//!
//! The caller supplies an exact track event range. This module never discovers
//! bounds, scans for a later signature, backtracks, or probes fallback decoders.

use std::{fmt, ops::Range};

use crate::{
    channel_pressure::{
        decode_channel_pressure_entry_at, ChannelPressureEntry, ChannelPressureEntryError,
        DecodedChannelPressureEntry,
    },
    controller::{
        decode_bounded_controller_record, BoundedControllerError, BoundedControllerRecord,
        ControllerRecordBounds,
    },
    patch::{
        decode_bounded_patch_core, decode_bounded_patch_representation, BoundedPatchCore,
        BoundedPatchError, BoundedPatchRepresentation, LocatedByte, LocatedBytes, LocatedVlq,
        PatchCoreBounds, PatchRepresentationBounds,
    },
    pitch_bend::{
        decode_pitch_bend_entry_at, DecodedPitchBendEntry, PitchBendEntry, PitchBendEntryError,
    },
    track7::{
        decode_7bit_be_vlq, decode_note_at, decode_note_body_at, BoundedNoteBody, BoundedNoteError,
        BoundedNoteEvent, VlqError,
    },
};

const CONTROLLER_REMAINDER_LENGTH: usize = 8;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MixedEventBounds {
    pub event_range: Range<usize>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MixedEventTimingBasis {
    pub previous_event_position: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveEventState {
    None,
    Note,
    ChannelPressure,
    PitchBend,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PositionedEvent<'a> {
    pub position: u32,
    pub event: MixedEventKind<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MixedEventKind<'a> {
    Note(BoundedNoteEvent<'a>),
    Controller(BoundedControllerRecord<'a>),
    ChannelPressure {
        entry: ChannelPressureEntry<'a>,
        entry_tag: Option<LocatedByte>,
    },
    PitchBend {
        entry: PitchBendEntry<'a>,
        entry_tag: Option<LocatedByte>,
    },
    ContextMediatedNote(BoundedContextMediatedNoteEntry<'a>),
    DoubleContextMediatedNote(BoundedDoubleContextMediatedNoteEntry<'a>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedFf60Context<'a> {
    pub range: Range<usize>,
    pub tag_range: Range<usize>,
    pub payload_length: LocatedByte,
    pub payload: LocatedBytes<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedContextMediatedNoteEntry<'a> {
    pub representation_range: Range<usize>,
    pub leading_timing: LocatedVlq<'a>,
    pub context: BoundedFf60Context<'a>,
    pub final_timing: LocatedVlq<'a>,
    pub note: BoundedNoteBody<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedDoubleContextMediatedNoteEntry<'a> {
    pub representation_range: Range<usize>,
    pub leading_timing: LocatedVlq<'a>,
    pub first_context: BoundedFf60Context<'a>,
    pub inter_context_timing: LocatedVlq<'a>,
    pub second_context: BoundedFf60Context<'a>,
    pub final_timing: LocatedVlq<'a>,
    pub note: BoundedNoteBody<'a>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoundedPatchToNoteTransition<'a> {
    pub representation_range: Range<usize>,
    pub patch: BoundedPatchRepresentation<'a>,
    pub context: Option<BoundedFf60Context<'a>>,
    pub final_timing: Option<LocatedVlq<'a>>,
    pub first_note: BoundedNoteBody<'a>,
    pub patch_position: u32,
    pub first_note_position: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MixedEventItem<'a> {
    Event(Box<PositionedEvent<'a>>),
    Patch(Box<BoundedPatchCore<'a>>),
    PatchToNote(Box<BoundedPatchToNoteTransition<'a>>),
}

impl MixedEventItem<'_> {
    pub fn logical_event_count(&self) -> usize {
        match self {
            Self::Event(_) => 1,
            Self::Patch(_) => 1,
            Self::PatchToNote(_) => 2,
        }
    }
}

struct PatchDispatch<'a> {
    items: Vec<MixedEventItem<'a>>,
    next: usize,
    next_position: u32,
    next_state: ActiveEventState,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MixedEventWalk<'a> {
    pub event_range: Range<usize>,
    pub items: Vec<MixedEventItem<'a>>,
    pub consumed_range: Range<usize>,
}

impl MixedEventWalk<'_> {
    pub fn logical_event_count(&self) -> usize {
        self.items
            .iter()
            .map(MixedEventItem::logical_event_count)
            .sum()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MixedEventWalkError {
    InvalidEventBounds {
        start: usize,
        end: usize,
        size: usize,
    },
    TimingVlq {
        cursor: usize,
        source: VlqError,
    },
    PositionOverflow {
        cursor: usize,
        left: u32,
        right: u32,
    },
    MissingDispatchByte {
        cursor: usize,
        offset: usize,
    },
    DataWithoutActiveState {
        cursor: usize,
        offset: usize,
        observed: u8,
    },
    UnsupportedStatus {
        cursor: usize,
        offset: usize,
        observed: u8,
    },
    UnsupportedFfTag {
        cursor: usize,
        offset: usize,
        observed: Option<u8>,
    },
    MalformedKnownTag {
        cursor: usize,
        offset: usize,
    },
    MalformedNote {
        cursor: usize,
        source: BoundedNoteError,
    },
    MalformedController {
        cursor: usize,
        source: BoundedControllerError,
    },
    MalformedChannelPressure {
        cursor: usize,
        source: ChannelPressureEntryError,
    },
    MalformedPitchBend {
        cursor: usize,
        source: PitchBendEntryError,
    },
    MalformedPatch {
        cursor: usize,
        source: BoundedPatchError,
    },
    PatchContextMismatch {
        cursor: usize,
        offset: usize,
        observed: Option<u8>,
    },
    ContextLengthMismatch {
        cursor: usize,
        offset: usize,
        expected: u8,
        observed: u8,
    },
    EventPastBound {
        cursor: usize,
        required_end: usize,
        event_end: usize,
    },
    HighBitData {
        cursor: usize,
        offset: usize,
        observed: u8,
    },
    CursorDidNotAdvance {
        cursor: usize,
    },
}

impl fmt::Display for MixedEventWalkError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "bounded mixed-event walk failed: {self:?}")
    }
}

impl std::error::Error for MixedEventWalkError {}

pub fn walk_bounded_mixed_events(
    bytes: &[u8],
    bounds: MixedEventBounds,
    timing_basis: MixedEventTimingBasis,
) -> Result<MixedEventWalk<'_>, MixedEventWalkError> {
    let start = bounds.event_range.start;
    let end = bounds.event_range.end;
    if start > end || end > bytes.len() {
        return Err(MixedEventWalkError::InvalidEventBounds {
            start,
            end,
            size: bytes.len(),
        });
    }

    let mut cursor = start;
    let mut previous_position = timing_basis.previous_event_position;
    let mut state = ActiveEventState::None;
    let mut items = Vec::new();

    while cursor != end {
        let decoded_timing = decode_7bit_be_vlq(bytes, cursor, end)
            .map_err(|source| MixedEventWalkError::TimingVlq { cursor, source })?;
        let timing_end = cursor.checked_add(decoded_timing.bytes_consumed).ok_or(
            MixedEventWalkError::EventPastBound {
                cursor,
                required_end: usize::MAX,
                event_end: end,
            },
        )?;
        let Some(first) = bytes.get(timing_end).copied().filter(|_| timing_end < end) else {
            return Err(MixedEventWalkError::MissingDispatchByte {
                cursor,
                offset: timing_end,
            });
        };

        let tag_offset = timing_end.checked_add(1);
        if first == 0xff && tag_offset.and_then(|offset| bytes.get(offset).copied()) == Some(0x7c) {
            let outcome = decode_patch_transition(bytes, cursor, end)?;
            require_advance(cursor, outcome.next, end)?;
            items.extend(outcome.items);
            cursor = outcome.next;
            previous_position = outcome.next_position;
            state = outcome.next_state;
            continue;
        }

        let (item, next, next_position, next_state) = match first {
            0x00..=0x7f => match state {
                ActiveEventState::None => {
                    return Err(MixedEventWalkError::DataWithoutActiveState {
                        cursor,
                        offset: timing_end,
                        observed: first,
                    });
                }
                ActiveEventState::Note => {
                    let (note, next) = decode_note_at(bytes, cursor, end, false)
                        .map_err(|source| MixedEventWalkError::MalformedNote { cursor, source })?;
                    let position = add_position(previous_position, note.timing.value, cursor)?;
                    (
                        MixedEventItem::Event(Box::new(PositionedEvent {
                            position,
                            event: MixedEventKind::Note(note),
                        })),
                        next,
                        position,
                        ActiveEventState::Note,
                    )
                }
                ActiveEventState::ChannelPressure => {
                    let (decoded, next) = decode_channel_pressure_entry_at(
                        bytes, cursor, end, false,
                    )
                    .map_err(|source| {
                        MixedEventWalkError::MalformedChannelPressure { cursor, source }
                    })?;
                    require_pressure_data(&decoded, cursor)?;
                    let position =
                        add_position(previous_position, decoded.entry.timing_delta.value, cursor)?;
                    (
                        pressure_item(position, decoded),
                        next,
                        position,
                        ActiveEventState::ChannelPressure,
                    )
                }
                ActiveEventState::PitchBend => {
                    let (decoded, next) =
                        decode_pitch_bend_entry_at(bytes, cursor, end, false).map_err(
                            |source| MixedEventWalkError::MalformedPitchBend { cursor, source },
                        )?;
                    require_bend_data(&decoded, cursor)?;
                    let position =
                        add_position(previous_position, decoded.entry.timing_delta.value, cursor)?;
                    (
                        bend_item(position, decoded),
                        next,
                        position,
                        ActiveEventState::PitchBend,
                    )
                }
            },
            0xff => dispatch_ff(bytes, cursor, timing_end, end, previous_position)?,
            0x90 => {
                let (note, next) = decode_note_at(bytes, cursor, end, true)
                    .map_err(|source| MixedEventWalkError::MalformedNote { cursor, source })?;
                let position = add_position(previous_position, note.timing.value, cursor)?;
                (
                    MixedEventItem::Event(Box::new(PositionedEvent {
                        position,
                        event: MixedEventKind::Note(note),
                    })),
                    next,
                    position,
                    ActiveEventState::Note,
                )
            }
            0xd0 => {
                let (decoded, next) =
                    decode_channel_pressure_entry_at(bytes, cursor, end, true).map_err(
                        |source| MixedEventWalkError::MalformedChannelPressure { cursor, source },
                    )?;
                require_pressure_data(&decoded, cursor)?;
                let position =
                    add_position(previous_position, decoded.entry.timing_delta.value, cursor)?;
                (
                    pressure_item(position, decoded),
                    next,
                    position,
                    ActiveEventState::ChannelPressure,
                )
            }
            0xe0 => {
                let (decoded, next) = decode_pitch_bend_entry_at(bytes, cursor, end, true)
                    .map_err(|source| MixedEventWalkError::MalformedPitchBend { cursor, source })?;
                require_bend_data(&decoded, cursor)?;
                let position =
                    add_position(previous_position, decoded.entry.timing_delta.value, cursor)?;
                (
                    bend_item(position, decoded),
                    next,
                    position,
                    ActiveEventState::PitchBend,
                )
            }
            observed => {
                return Err(MixedEventWalkError::UnsupportedStatus {
                    cursor,
                    offset: timing_end,
                    observed,
                });
            }
        };

        require_advance(cursor, next, end)?;
        items.push(item);
        cursor = next;
        previous_position = next_position;
        state = next_state;
    }

    Ok(MixedEventWalk {
        event_range: bounds.event_range.clone(),
        items,
        consumed_range: bounds.event_range,
    })
}

fn dispatch_ff<'a>(
    bytes: &'a [u8],
    cursor: usize,
    ff_offset: usize,
    event_end: usize,
    previous_position: u32,
) -> Result<(MixedEventItem<'a>, usize, u32, ActiveEventState), MixedEventWalkError> {
    let tag_offset = ff_offset
        .checked_add(1)
        .ok_or(MixedEventWalkError::MalformedKnownTag {
            cursor,
            offset: ff_offset,
        })?;
    let observed = bytes
        .get(tag_offset)
        .copied()
        .filter(|_| tag_offset < event_end);
    match observed {
        Some(0x41) => {
            let required_end = ff_offset.checked_add(CONTROLLER_REMAINDER_LENGTH).ok_or(
                MixedEventWalkError::EventPastBound {
                    cursor,
                    required_end: usize::MAX,
                    event_end,
                },
            )?;
            if required_end > event_end {
                return Err(MixedEventWalkError::EventPastBound {
                    cursor,
                    required_end,
                    event_end,
                });
            }
            let controller = decode_bounded_controller_record(
                bytes,
                ControllerRecordBounds {
                    record_range: cursor..required_end,
                },
            )
            .map_err(|source| MixedEventWalkError::MalformedController { cursor, source })?;
            let position = add_position(previous_position, controller.timing_delta.value, cursor)?;
            Ok((
                MixedEventItem::Event(Box::new(PositionedEvent {
                    position,
                    event: MixedEventKind::Controller(controller),
                })),
                required_end,
                position,
                ActiveEventState::None,
            ))
        }
        Some(0x60) => {
            decode_context_mediated_note(bytes, cursor, ff_offset, event_end, previous_position)
        }
        _ => Err(MixedEventWalkError::UnsupportedFfTag {
            cursor,
            offset: tag_offset,
            observed,
        }),
    }
}

fn decode_patch_transition(
    bytes: &[u8],
    cursor: usize,
    event_end: usize,
) -> Result<PatchDispatch<'_>, MixedEventWalkError> {
    let core = decode_bounded_patch_core(
        bytes,
        PatchCoreBounds {
            position_start: cursor,
            end: event_end,
        },
    )
    .map_err(|source| MixedEventWalkError::MalformedPatch { cursor, source })?;
    let payload_end = core.representation_range.end;
    let post_pc = located_vlq(bytes, payload_end, event_end, cursor)?;
    let mut transition_cursor = post_pc.range.end;

    let controller_tag_end = transition_cursor.checked_add(2);
    if controller_tag_end.and_then(|end| bytes.get(transition_cursor..end)) == Some(&[0xff, 0x41]) {
        return decode_patch_controller_note(bytes, event_end, core);
    }

    let (context, final_timing) = match bytes.get(transition_cursor).copied() {
        Some(0x90) if transition_cursor < event_end => (None, None),
        Some(0xff) if transition_cursor < event_end => {
            let (context, after_context) =
                decode_ff60_context(bytes, transition_cursor, event_end, cursor)?;
            let final_timing = located_vlq(bytes, after_context, event_end, cursor)?;
            transition_cursor = final_timing.range.end;
            (Some(context), Some(final_timing))
        }
        observed => {
            return Err(MixedEventWalkError::PatchContextMismatch {
                cursor,
                offset: transition_cursor,
                observed,
            });
        }
    };
    if bytes
        .get(transition_cursor)
        .copied()
        .filter(|_| transition_cursor < event_end)
        != Some(0x90)
    {
        return Err(MixedEventWalkError::PatchContextMismatch {
            cursor,
            offset: transition_cursor,
            observed: bytes.get(transition_cursor).copied(),
        });
    }

    let patch = decode_bounded_patch_representation(
        bytes,
        PatchRepresentationBounds {
            position_start: cursor,
            note_status_end: transition_cursor + 1,
        },
    )
    .map_err(|source| MixedEventWalkError::MalformedPatch { cursor, source })?;
    let (first_note, next) = decode_note_body_at(bytes, transition_cursor, event_end, true)
        .map_err(|source| MixedEventWalkError::MalformedNote { cursor, source })?;
    let interval = if let Some(final_timing) = &final_timing {
        add_position(post_pc.value, final_timing.value, cursor)?
    } else {
        post_pc.value
    };
    let first_note_position = add_position(patch.position.value, interval, cursor)?;
    let transition = BoundedPatchToNoteTransition {
        representation_range: cursor..next,
        patch_position: patch.position.value,
        first_note_position,
        patch,
        context,
        final_timing,
        first_note,
    };
    Ok(PatchDispatch {
        items: vec![MixedEventItem::PatchToNote(Box::new(transition))],
        next,
        next_position: first_note_position,
        next_state: ActiveEventState::Note,
    })
}

fn decode_patch_controller_note<'a>(
    bytes: &'a [u8],
    event_end: usize,
    patch: BoundedPatchCore<'a>,
) -> Result<PatchDispatch<'a>, MixedEventWalkError> {
    let controller_cursor = patch.representation_range.end;
    let timing = decode_7bit_be_vlq(bytes, controller_cursor, event_end).map_err(|source| {
        MixedEventWalkError::TimingVlq {
            cursor: controller_cursor,
            source,
        }
    })?;
    let controller_tag = controller_cursor.checked_add(timing.bytes_consumed).ok_or(
        MixedEventWalkError::EventPastBound {
            cursor: controller_cursor,
            required_end: usize::MAX,
            event_end,
        },
    )?;
    let controller_end = controller_tag
        .checked_add(CONTROLLER_REMAINDER_LENGTH)
        .ok_or(MixedEventWalkError::EventPastBound {
            cursor: controller_cursor,
            required_end: usize::MAX,
            event_end,
        })?;
    if controller_end > event_end {
        return Err(MixedEventWalkError::EventPastBound {
            cursor: controller_cursor,
            required_end: controller_end,
            event_end,
        });
    }
    let controller = decode_bounded_controller_record(
        bytes,
        ControllerRecordBounds {
            record_range: controller_cursor..controller_end,
        },
    )
    .map_err(|source| MixedEventWalkError::MalformedController {
        cursor: controller_cursor,
        source,
    })?;
    let controller_position = add_position(
        patch.position.value,
        controller.timing_delta.value,
        controller_cursor,
    )?;

    let note_cursor = controller_end;
    let (note, next) = decode_note_at(bytes, note_cursor, event_end, true).map_err(|source| {
        MixedEventWalkError::MalformedNote {
            cursor: note_cursor,
            source,
        }
    })?;
    let note_position = add_position(controller_position, note.timing.value, note_cursor)?;

    Ok(PatchDispatch {
        items: vec![
            MixedEventItem::Patch(Box::new(patch)),
            MixedEventItem::Event(Box::new(PositionedEvent {
                position: controller_position,
                event: MixedEventKind::Controller(controller),
            })),
            MixedEventItem::Event(Box::new(PositionedEvent {
                position: note_position,
                event: MixedEventKind::Note(note),
            })),
        ],
        next,
        next_position: note_position,
        next_state: ActiveEventState::Note,
    })
}

fn require_advance(
    cursor: usize,
    next: usize,
    event_end: usize,
) -> Result<(), MixedEventWalkError> {
    if next <= cursor {
        return Err(MixedEventWalkError::CursorDidNotAdvance { cursor });
    }
    if next > event_end {
        return Err(MixedEventWalkError::EventPastBound {
            cursor,
            required_end: next,
            event_end,
        });
    }
    Ok(())
}

fn decode_context_mediated_note(
    bytes: &[u8],
    cursor: usize,
    context_start: usize,
    event_end: usize,
    previous_position: u32,
) -> Result<(MixedEventItem<'_>, usize, u32, ActiveEventState), MixedEventWalkError> {
    let leading_timing = located_vlq(bytes, cursor, event_end, cursor)?;
    let (context, after_context) = decode_ff60_context(bytes, context_start, event_end, cursor)?;
    let following_timing = located_vlq(bytes, after_context, event_end, cursor)?;
    let following_offset = following_timing.range.end;

    if bytes
        .get(following_offset)
        .copied()
        .filter(|_| following_offset < event_end)
        == Some(0x90)
    {
        let (note, next) = decode_note_body_at(bytes, following_offset, event_end, true)
            .map_err(|source| MixedEventWalkError::MalformedNote { cursor, source })?;
        let total_delta = add_position(leading_timing.value, following_timing.value, cursor)?;
        let position = add_position(previous_position, total_delta, cursor)?;
        let representation = BoundedContextMediatedNoteEntry {
            representation_range: cursor..next,
            leading_timing,
            context,
            final_timing: following_timing,
            note,
        };
        return Ok((
            MixedEventItem::Event(Box::new(PositionedEvent {
                position,
                event: MixedEventKind::ContextMediatedNote(representation),
            })),
            next,
            position,
            ActiveEventState::Note,
        ));
    }

    let second_tag_end = following_offset.checked_add(2);
    if second_tag_end.and_then(|end| {
        bytes
            .get(following_offset..end)
            .filter(|_| end <= event_end)
    }) == Some(&[0xff, 0x60])
    {
        return decode_double_context_mediated_note(
            bytes,
            cursor,
            event_end,
            previous_position,
            leading_timing,
            context,
            following_timing,
        );
    }

    Err(MixedEventWalkError::PatchContextMismatch {
        cursor,
        offset: following_offset,
        observed: bytes.get(following_offset).copied(),
    })
}

fn decode_double_context_mediated_note<'a>(
    bytes: &'a [u8],
    cursor: usize,
    event_end: usize,
    previous_position: u32,
    leading_timing: LocatedVlq<'a>,
    first_context: BoundedFf60Context<'a>,
    inter_context_timing: LocatedVlq<'a>,
) -> Result<(MixedEventItem<'a>, usize, u32, ActiveEventState), MixedEventWalkError> {
    if first_context.payload_length.value != 6 {
        return Err(MixedEventWalkError::ContextLengthMismatch {
            cursor,
            offset: first_context.payload_length.offset,
            expected: 6,
            observed: first_context.payload_length.value,
        });
    }

    let second_context_start = inter_context_timing.range.end;
    let (second_context, after_second_context) =
        decode_ff60_context(bytes, second_context_start, event_end, cursor)?;
    if second_context.payload_length.value != 7 {
        return Err(MixedEventWalkError::ContextLengthMismatch {
            cursor,
            offset: second_context.payload_length.offset,
            expected: 7,
            observed: second_context.payload_length.value,
        });
    }

    let final_timing = located_vlq(bytes, after_second_context, event_end, cursor)?;
    let status_offset = final_timing.range.end;
    if bytes
        .get(status_offset)
        .copied()
        .filter(|_| status_offset < event_end)
        != Some(0x90)
    {
        return Err(MixedEventWalkError::PatchContextMismatch {
            cursor,
            offset: status_offset,
            observed: bytes.get(status_offset).copied(),
        });
    }

    let (note, next) = decode_note_body_at(bytes, status_offset, event_end, true)
        .map_err(|source| MixedEventWalkError::MalformedNote { cursor, source })?;
    let first_sum = add_position(leading_timing.value, inter_context_timing.value, cursor)?;
    let total_delta = add_position(first_sum, final_timing.value, cursor)?;
    let position = add_position(previous_position, total_delta, cursor)?;
    let representation = BoundedDoubleContextMediatedNoteEntry {
        representation_range: cursor..next,
        leading_timing,
        first_context,
        inter_context_timing,
        second_context,
        final_timing,
        note,
    };
    Ok((
        MixedEventItem::Event(Box::new(PositionedEvent {
            position,
            event: MixedEventKind::DoubleContextMediatedNote(representation),
        })),
        next,
        position,
        ActiveEventState::Note,
    ))
}

fn decode_ff60_context(
    bytes: &[u8],
    start: usize,
    event_end: usize,
    cursor: usize,
) -> Result<(BoundedFf60Context<'_>, usize), MixedEventWalkError> {
    let tag_end = start
        .checked_add(2)
        .ok_or(MixedEventWalkError::MalformedKnownTag {
            cursor,
            offset: start,
        })?;
    if tag_end > event_end || bytes.get(start..tag_end) != Some(&[0xff, 0x60]) {
        return Err(MixedEventWalkError::PatchContextMismatch {
            cursor,
            offset: start,
            observed: bytes.get(start).copied(),
        });
    }
    let length_offset = tag_end;
    let Some(length) = bytes
        .get(length_offset)
        .copied()
        .filter(|_| length_offset < event_end)
    else {
        return Err(MixedEventWalkError::MalformedKnownTag {
            cursor,
            offset: length_offset,
        });
    };
    let payload_start = length_offset + 1;
    let payload_end = payload_start.checked_add(usize::from(length)).ok_or(
        MixedEventWalkError::EventPastBound {
            cursor,
            required_end: usize::MAX,
            event_end,
        },
    )?;
    let Some(payload) = bytes
        .get(payload_start..payload_end)
        .filter(|_| payload_end <= event_end)
    else {
        return Err(MixedEventWalkError::EventPastBound {
            cursor,
            required_end: payload_end,
            event_end,
        });
    };
    Ok((
        BoundedFf60Context {
            range: start..payload_end,
            tag_range: start..tag_end,
            payload_length: LocatedByte {
                value: length,
                offset: length_offset,
            },
            payload: LocatedBytes {
                bytes: payload,
                range: payload_start..payload_end,
            },
        },
        payload_end,
    ))
}

fn located_vlq(
    bytes: &[u8],
    start: usize,
    end: usize,
    cursor: usize,
) -> Result<LocatedVlq<'_>, MixedEventWalkError> {
    let decoded = decode_7bit_be_vlq(bytes, start, end)
        .map_err(|source| MixedEventWalkError::TimingVlq { cursor, source })?;
    let vlq_end =
        start
            .checked_add(decoded.bytes_consumed)
            .ok_or(MixedEventWalkError::EventPastBound {
                cursor,
                required_end: usize::MAX,
                event_end: end,
            })?;
    Ok(LocatedVlq {
        value: decoded.value,
        bytes: &bytes[start..vlq_end],
        range: start..vlq_end,
    })
}

fn add_position(left: u32, right: u32, cursor: usize) -> Result<u32, MixedEventWalkError> {
    left.checked_add(right)
        .ok_or(MixedEventWalkError::PositionOverflow {
            cursor,
            left,
            right,
        })
}

fn require_pressure_data(
    decoded: &DecodedChannelPressureEntry<'_>,
    cursor: usize,
) -> Result<(), MixedEventWalkError> {
    let byte = decoded.entry.pressure_value;
    if byte.value >= 0x80 {
        return Err(MixedEventWalkError::HighBitData {
            cursor,
            offset: byte.offset,
            observed: byte.value,
        });
    }
    Ok(())
}

fn require_bend_data(
    decoded: &DecodedPitchBendEntry<'_>,
    cursor: usize,
) -> Result<(), MixedEventWalkError> {
    for byte in [decoded.entry.pitch_lsb, decoded.entry.pitch_msb] {
        if byte.value >= 0x80 {
            return Err(MixedEventWalkError::HighBitData {
                cursor,
                offset: byte.offset,
                observed: byte.value,
            });
        }
    }
    Ok(())
}

fn pressure_item(position: u32, decoded: DecodedChannelPressureEntry<'_>) -> MixedEventItem<'_> {
    MixedEventItem::Event(Box::new(PositionedEvent {
        position,
        event: MixedEventKind::ChannelPressure {
            entry: decoded.entry,
            entry_tag: decoded.entry_tag,
        },
    }))
}

fn bend_item(position: u32, decoded: DecodedPitchBendEntry<'_>) -> MixedEventItem<'_> {
    MixedEventItem::Event(Box::new(PositionedEvent {
        position,
        event: MixedEventKind::PitchBend {
            entry: decoded.entry,
            entry_tag: decoded.entry_tag,
        },
    }))
}
