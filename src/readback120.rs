//! Independent strict readback of Phoenix's explicit-status SMF output.
//! No Studio Vision interpretation. Compare every parsed event to the handoff.
use crate::midi_export::ConductorResult;
use crate::smf::{ChannelMessage, ScheduledEvent};
pub(crate) type Events = Vec<(u32, u8, Vec<u8>)>; // tick, channel status or meta type, data
pub(crate) fn conductor_expected(c: &ConductorResult, key: Option<(i8, bool)>) -> Events {
    let tempo = c.tempo_mpqn.to_be_bytes();
    let mut e = vec![
        (0, 3, c.sequence_name.clone()),
        (0, 0x51, tempo[1..].to_vec()),
    ];
    if let Some((a, m)) = key {
        e.push((0, 0x59, vec![a as u8, u8::from(m)]));
    }
    e.push((
        0,
        0x58,
        crate::smf::serialize_time_signature(c.time_signature)[3..].to_vec(),
    ));
    e.push((0, 0x2f, vec![]));
    e
}
pub(crate) fn musical_expected(name: &[u8], scheduled: &[ScheduledEvent]) -> Events {
    let mut events = scheduled.to_vec();
    events.sort_by_key(|e| (e.absolute_tick, e.stable_ordinal));
    let mut out = vec![(0, 3, name.to_vec())];
    for e in &events {
        use ChannelMessage::*;
        let (s, d) = match &e.message {
            SysEx { payload } => (0xf0, payload.as_bytes().to_vec()),
            NoteOn {
                channel,
                key,
                attack_velocity,
            } => (
                0x90 + channel.get() - 1,
                vec![key.get(), attack_velocity.get()],
            ),
            NoteOff {
                channel,
                key,
                release_velocity,
            } => (
                0x80 + channel.get() - 1,
                vec![key.get(), release_velocity.get()],
            ),
            ControlChange {
                channel,
                controller,
                value,
            } => (
                0xb0 + channel.get() - 1,
                vec![controller.get(), value.get()],
            ),
            ProgramChange { channel, program } => (0xc0 + channel.get() - 1, vec![program.get()]),
            ChannelPressure { channel, pressure } => {
                (0xd0 + channel.get() - 1, vec![pressure.get()])
            }
            PitchBend { channel, lsb, msb } => {
                (0xe0 + channel.get() - 1, vec![lsb.get(), msb.get()])
            }
        };
        out.push((e.absolute_tick, s, d));
    }
    out.push((events.last().map_or(0, |e| e.absolute_tick), 0x2f, vec![]));
    out
}
struct Reader<'a> {
    b: &'a [u8],
    p: usize,
}
impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], &'static str> {
        let end = self.p.checked_add(n).ok_or("length overflow")?;
        let b = self.b.get(self.p..end).ok_or("truncation")?;
        self.p = end;
        Ok(b)
    }
    fn byte(&mut self) -> Result<u8, &'static str> {
        Ok(self.take(1)?[0])
    }
    fn vlq(&mut self) -> Result<u32, &'static str> {
        let mut v = 0;
        for _ in 0..4 {
            let b = self.byte()?;
            v = (v << 7) | u32::from(b & 127);
            if b & 128 == 0 {
                return Ok(v);
            }
        }
        Err("invalid VLQ")
    }
}
pub(crate) fn verify(bytes: &[u8], ppqn: u16, expected: &[Events]) -> Result<(), &'static str> {
    let mut r = Reader { b: bytes, p: 0 };
    if r.take(8)? != b"MThd\0\0\0\x06" {
        return Err("header");
    }
    let h = r.take(6)?;
    if h[..2] != [0, 1]
        || usize::from(u16::from_be_bytes([h[2], h[3]])) != expected.len()
        || u16::from_be_bytes([h[4], h[5]]) != ppqn
    {
        return Err("header values");
    }
    for want in expected {
        if r.take(4)? != b"MTrk" {
            return Err("track chunk");
        }
        let len = u32::from_be_bytes(r.take(4)?.try_into().map_err(|_| "length")?) as usize;
        let mut t = Reader {
            b: r.take(len)?,
            p: 0,
        };
        let mut tick = 0u32;
        let mut events = Vec::new();
        let mut ended = false;
        while t.p < t.b.len() {
            if ended {
                return Err("bytes after EOT");
            }
            tick = tick.checked_add(t.vlq()?).ok_or("tick overflow")?;
            let status = t.byte()?;
            if status == 0xff {
                let ty = t.byte()?;
                let n = t.vlq()? as usize;
                let data = t.take(n)?.to_vec();
                if ty == 0x2f {
                    if !data.is_empty() {
                        return Err("EOT payload");
                    }
                    ended = true;
                }
                events.push((tick, ty, data));
            } else if status == 0xf0 {
                let n = t.vlq()? as usize;
                let data = t.take(n)?.to_vec();
                if data.len() < 2
                    || data.last() != Some(&0xf7)
                    || data[..data.len() - 1].iter().any(|b| *b > 127)
                {
                    return Err("invalid SysEx payload");
                }
                events.push((tick, status, data));
            } else {
                let n = match status >> 4 {
                    8 | 9 | 10 | 11 | 14 => 2,
                    12 | 13 => 1,
                    _ => return Err("unsupported status"),
                };
                let data = t.take(n)?.to_vec();
                if data.iter().any(|v| *v > 127) {
                    return Err("data byte");
                }
                events.push((tick, status, data));
            }
        }
        if !ended || &events != want {
            return Err("handoff/event/EOT mismatch");
        }
    }
    if r.p != bytes.len() {
        return Err("trailing bytes");
    }
    Ok(())
}
