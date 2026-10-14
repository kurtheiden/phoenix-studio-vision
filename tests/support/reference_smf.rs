// Independent SMF readback: no Phoenix source decoder or readback implementation.
pub fn midi_tracks(b: &[u8]) -> Vec<Vec<(u32, u8, Vec<u8>)>> {
    fn vlq(b: &[u8], p: &mut usize) -> u32 {
        let mut v = 0;
        for _ in 0..4 {
            let x = b[*p];
            *p += 1;
            v = v * 128 + u32::from(x & 127);
            if x < 128 {
                return v;
            }
        }
        panic!("VLQ")
    }
    let mut p = 14;
    let mut result = vec![];
    for _ in 0..u16::from_be_bytes([b[10], b[11]]) {
        assert_eq!(&b[p..p + 4], b"MTrk");
        let n = u32::from_be_bytes(b[p + 4..p + 8].try_into().unwrap()) as usize;
        p += 8;
        let d = &b[p..p + n];
        p += n;
        let mut q = 0;
        let mut tick = 0;
        let mut events = vec![];
        let mut running = 0;
        while q < d.len() {
            tick += vlq(d, &mut q);
            let mut status = d[q];
            if status >= 128 {
                q += 1;
            } else {
                status = running;
            }
            if status < 240 {
                running = status;
            }
            let n = if status == 255 {
                status = d[q];
                q += 1;
                vlq(d, &mut q) as usize
            } else if status == 240 {
                vlq(d, &mut q) as usize
            } else if status >> 4 == 12 || status >> 4 == 13 {
                1
            } else {
                2
            };
            let data = d[q..q + n].to_vec();
            q += n;
            events.push((tick, status, data));
        }
        assert_eq!(events.last().unwrap().1, 47);
        result.push(events);
    }
    assert_eq!(p, b.len());
    result
}
