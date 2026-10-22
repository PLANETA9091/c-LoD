//! Minimal PNG encoder (RGBA8, per-row None/Sub filtering, real DEFLATE:
//! LZ77 + fixed-Huffman blocks, stored-block fallback). Used to emit the
//! heightmap atlas into the resource pack. Zero deps.
//!
//! v4.5: the encoder previously emitted STORED (uncompressed) deflate
//! blocks — the atlas PNG was 32.5 MB raw and the whole pack ~50 MB, so
//! every pack change cost the client a 50 MB download. A hand-rolled
//! fixed-Huffman LZ77 deflate gets the same pixels down to a few MB.

fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

// ---------------------------------------------------------------------------
// Bit writer (DEFLATE packs bits LSB-first)
// ---------------------------------------------------------------------------
#[derive(Default)]
struct BitWriter {
    out: Vec<u8>,
    acc: u64,
    nbits: u32,
}

impl BitWriter {
    /// Write `n` bits of `value`, least-significant bit first.
    fn put(&mut self, value: u32, n: u32) {
        self.acc |= ((value as u64) & ((1u64 << n) - 1)) << self.nbits;
        self.nbits += n;
        while self.nbits >= 8 {
            self.out.push((self.acc & 0xFF) as u8);
            self.acc >>= 8;
            self.nbits -= 8;
        }
    }
    /// Write a Huffman code, most-significant bit first (RFC 1951 s3.1.1).
    fn put_code(&mut self, code: u32, n: u32) {
        for i in (0..n).rev() {
            self.put((code >> i) & 1, 1);
        }
    }
    /// Pad the last byte with zeros and return the stream.
    fn finish(mut self) -> Vec<u8> {
        if self.nbits > 0 {
            self.out.push((self.acc & 0xFF) as u8);
        }
        self.out
    }
}

/// Fixed Huffman code for a literal / length symbol (0..=285, RFC 1951 3.2.6).
fn put_symbol(bw: &mut BitWriter, sym: u32) {
    match sym {
        0..=143 => bw.put_code(0x30 + sym, 8),
        144..=255 => bw.put_code(0x190 + (sym - 144), 9),
        256..=279 => bw.put_code(sym - 256, 7),
        _ => bw.put_code(0xC0 + (sym - 280), 8),
    }
}

/// RFC 1951 3.2.5 length/distance base + extra-bit tables.
const LEN_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LEN_EBITS: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 6, 6, 0,
];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EBITS: [u8; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];

/// Length 3..=258 -> (symbol 257..=285, extra bits, extra value).
fn length_code(len: usize) -> (u32, u32, u32) {
    let mut sym = 28usize;
    while LEN_BASE[sym] as usize > len {
        sym -= 1;
    }
    (
        257 + sym as u32,
        LEN_EBITS[sym] as u32,
        (len - LEN_BASE[sym] as usize) as u32,
    )
}

/// Dist 1..=32768 -> (symbol 0..=29, extra bits, extra value).
fn dist_code(dist: usize) -> (u32, u32, u32) {
    let mut sym = 29usize;
    while DIST_BASE[sym] as usize > dist {
        sym -= 1;
    }
    (
        sym as u32,
        DIST_EBITS[sym] as u32,
        (dist - DIST_BASE[sym] as usize) as u32,
    )
}

/// Raw DEFLATE with fixed Huffman codes + greedy LZ77 (hash chains).
fn deflate_fixed(data: &[u8]) -> Vec<u8> {
    const MIN_MATCH: usize = 3;
    const MAX_MATCH: usize = 258;
    const WINDOW: i64 = 32768;
    const MAX_CHAIN: usize = 48;
    let n = data.len();
    let mut bw = BitWriter::default();
    bw.put(1, 1); // BFINAL
    bw.put(1, 2); // BTYPE = 01 (fixed Huffman)

    let head_size = 1usize << 15;
    let mut head = vec![-1i32; head_size];
    let mut prev = vec![-1i32; n];
    let hash = |d: &[u8]| -> usize {
        (((d[0] as usize) << 10) ^ ((d[1] as usize) << 5) ^ (d[2] as usize)) & (head_size - 1)
    };

    let mut i = 0usize;
    while i < n {
        let mut best_len = 0usize;
        let mut best_dist = 0usize;
        if i + MIN_MATCH <= n {
            let h = hash(&data[i..]);
            let mut cand = head[h];
            let mut chain = 0usize;
            let maxlen = (n - i).min(MAX_MATCH);
            while cand >= 0 && i as i64 - cand as i64 <= WINDOW && chain < MAX_CHAIN {
                let c = cand as usize;
                let mut l = 0usize;
                while l < maxlen && data[c + l] == data[i + l] {
                    l += 1;
                }
                if l > best_len {
                    best_len = l;
                    best_dist = i - c;
                    if l >= maxlen {
                        break;
                    }
                }
                cand = prev[cand as usize];
                chain += 1;
            }
        }
        if best_len >= MIN_MATCH {
            let (lsym, lebits, leval) = length_code(best_len);
            put_symbol(&mut bw, lsym);
            if lebits > 0 {
                bw.put(leval, lebits);
            }
            let (dsym, debits, deval) = dist_code(best_dist);
            bw.put_code(dsym, 5);
            if debits > 0 {
                bw.put(deval, debits);
            }
            // Insert every covered position so later matches can reference
            // the middle of this one.
            let end = i + best_len;
            while i < end {
                if i + MIN_MATCH <= n {
                    let h = hash(&data[i..]);
                    prev[i] = head[h];
                    head[h] = i as i32;
                }
                i += 1;
            }
        } else {
            put_symbol(&mut bw, data[i] as u32);
            if i + MIN_MATCH <= n {
                let h = hash(&data[i..]);
                prev[i] = head[h];
                head[h] = i as i32;
            }
            i += 1;
        }
    }
    put_symbol(&mut bw, 256); // end of block
    bw.finish()
}

/// Raw DEFLATE stream with only stored (uncompressed) blocks, wrapped in a
/// zlib container. Valid for any length. Kept as the size fallback.
pub fn zlib_store(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() + data.len() / 65535 * 5 + 16);
    // 0x78 0x01: CM=8 (deflate), 32K window; FCHECK valid (30721 % 31 == 0).
    out.push(0x78);
    out.push(0x01);
    let mut chunks = data.chunks(65535).peekable();
    if data.is_empty() {
        // One final empty stored block.
        out.extend_from_slice(&[0x01, 0x00, 0x00, 0xFF, 0xFF]);
    }
    while let Some(chunk) = chunks.next() {
        let last = chunks.peek().is_none();
        out.push(if last { 0x01 } else { 0x00 }); // BFINAL, BTYPE=00
        let len = chunk.len() as u16;
        out.extend_from_slice(&len.to_le_bytes());
        out.extend_from_slice(&(!len).to_le_bytes());
        out.extend_from_slice(chunk);
    }
    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

/// zlib container: CMF/FLG + deflate + adler32. Uses the fixed-Huffman LZ77
/// deflate and falls back to stored blocks when they happen to be smaller.
fn zlib_deflate(data: &[u8]) -> Vec<u8> {
    let fixed = deflate_fixed(data);
    let stored = zlib_store(data);
    // Both start with CMF/FLG; pick the smaller payload.
    let (body, cmf, flg) = if fixed.len() <= stored.len() {
        (fixed, 0x78u8, 0x9Cu8)
    } else {
        (stored, 0x78u8, 0x01u8)
    };
    let mut out = Vec::with_capacity(body.len() + 8);
    out.push(cmf);
    out.push(flg);
    out.extend_from_slice(&body[2..]); // strip the temp header, keep the stream
    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let mut crc_input = Vec::with_capacity(4 + data.len());
    crc_input.extend_from_slice(kind);
    crc_input.extend_from_slice(data);
    out.extend_from_slice(kind);
    // v4.2 CRITICAL FIX: the chunk PAYLOAD was never written (length + type
    // + CRC only) — every atlas PNG ever shipped decoded as garbage on the
    // client, and the whole v4.0/v4.1 "LOD band" was the missing-texture
    // checkerboard, not the heightfield. Write the data.
    out.extend_from_slice(data);
    out.extend_from_slice(&crc32_bytes(&crc_input));
}

fn crc32_bytes(data: &[u8]) -> [u8; 4] {
    let c = crate::zipw::crc32(data);
    c.to_be_bytes()
}

/// Filter one scanline with type 0 (None) or type 1 (Sub), picking the
/// smaller standard heuristic cost (sum of |signed byte|).
fn filter_row(row: &[u8], bpp: usize) -> (u8, Vec<u8>) {
    let none_cost: i64 = row.iter().map(|&b| i64::from((b as i8).unsigned_abs())).sum();
    let mut sub = Vec::with_capacity(row.len());
    for (x, &b) in row.iter().enumerate() {
        let left = if x >= bpp { row[x - bpp] } else { 0 };
        sub.push(b.wrapping_sub(left));
    }
    let sub_cost: i64 = sub.iter().map(|&b| i64::from((b as i8).unsigned_abs())).sum();
    if sub_cost < none_cost {
        (1, sub)
    } else {
        (0, row.to_vec())
    }
}

/// Encode an RGBA8 image as PNG. `rgba.len()` must be `width * height * 4`.
pub fn encode_png(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
    assert_eq!(rgba.len(), (width as usize) * (height as usize) * 4);
    // Filtered scanlines: per row, None (0) or Sub (1) — whichever the
    // standard cost heuristic prefers. LZ77 then crushes the horizontal
    // repetitions the Sub filter surfaces.
    let bpp = 4usize;
    let stride = width as usize * bpp;
    let mut raw = Vec::with_capacity(height as usize * (stride + 1));
    for y in 0..height as usize {
        let row = &rgba[y * stride..(y + 1) * stride];
        let (f, bytes) = filter_row(row, bpp);
        raw.push(f);
        raw.extend_from_slice(&bytes);
    }
    let idat = zlib_deflate(&raw);

    let mut out = Vec::with_capacity(idat.len() + 128);
    out.extend_from_slice(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.push(8); // bit depth
    ihdr.push(6); // color type RGBA
    ihdr.push(0); // compression
    ihdr.push(0); // filter
    ihdr.push(0); // interlace
    chunk(&mut out, b"IHDR", &ihdr);
    chunk(&mut out, b"IDAT", &idat);
    chunk(&mut out, b"IEND", &[]);
    out
}

// ---------------------------------------------------------------------------
// Tests: the encoder's own fixed-Huffman inflater round-trips every pixel.
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zlib_stored_roundtrip_length() {
        let data: Vec<u8> = (0..200_000u32).map(|i| (i % 13) as u8).collect();
        let z = zlib_store(&data);
        // Header
        assert_eq!(z[0], 0x78);
        assert_eq!(((z[0] as u32) * 256 + (z[1] as u32)) % 31, 0);
        // Adler at the end
        let n = z.len();
        let adler = u32::from_be_bytes([z[n - 4], z[n - 3], z[n - 2], z[n - 1]]);
        assert_eq!(adler, adler32(&data));
    }

    #[test]
    fn png_signature_and_chunks() {
        let px = vec![128u8; 4 * 4 * 4]; // 4x4 RGBA
        let png = encode_png(4, 4, &px);
        assert_eq!(&png[..8], &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]);
        // IHDR length 13
        assert_eq!(&png[8..12], &13u32.to_be_bytes());
        assert_eq!(&png[12..16], b"IHDR");
        // Must end with IEND CRC
        let n = png.len();
        assert_eq!(&png[n - 8..n - 4], b"IEND");
    }

    /// Full structural decode: IHDR carries the right w/h and the IDAT
    /// zlib stream inflates + unfilters back to the EXACT input pixels.
    /// Guards the v4.5 deflate (bit order, Huffman tables, LZ77 window)
    /// against silently producing garbage the client would reject.
    #[test]
    fn png_roundtrips_pixels_through_deflate() {
        let (w, h) = (128u32, 128u32);
        let mut px = vec![0u8; (w * h * 4) as usize];
        for (i, b) in px.iter_mut().enumerate() {
            // Mixed content: smooth gradients + blocky repeats + noise.
            let x = (i / 4) % w as usize;
            let y = (i / 4) / w as usize;
            *b = match i % 4 {
                0 => (x * 2) as u8,
                1 => if (x / 16) % 2 == 0 { 0x29 } else { 0x91 },
                2 => (y ^ x) as u8,
                _ => ((x * 7 + y * 13) % 251) as u8,
            };
        }
        let png = encode_png(w, h, &px);

        let mut pos = 8usize;
        let mut ihdr_data: Option<Vec<u8>> = None;
        let mut idat_data: Option<Vec<u8>> = None;
        while pos + 12 <= png.len() {
            let len =
                u32::from_be_bytes([png[pos], png[pos + 1], png[pos + 2], png[pos + 3]]) as usize;
            let kind = &png[pos + 4..pos + 8];
            let payload = png[pos + 8..pos + 8 + len].to_vec();
            match kind {
                b"IHDR" => ihdr_data = Some(payload),
                b"IDAT" => idat_data = Some(payload),
                _ => {}
            }
            pos += 12 + len;
        }
        let ihdr = ihdr_data.expect("IHDR payload present");
        assert_eq!(ihdr.len(), 13);
        assert_eq!(u32::from_be_bytes([ihdr[0], ihdr[1], ihdr[2], ihdr[3]]), w);
        assert_eq!(u32::from_be_bytes([ihdr[4], ihdr[5], ihdr[6], ihdr[7]]), h);

        let idat = idat_data.expect("IDAT payload present");
        let raw = inflate_zlib(&idat);
        assert_eq!(raw.len(), h as usize * (1 + w as usize * 4), "raw size");
        // Unfilter (types 0 and 1 only) and compare with the input.
        let stride = w as usize * 4;
        for y in 0..h as usize {
            let row = &raw[y * (stride + 1)..(y + 1) * (stride + 1)];
            let f = row[0];
            let data = &row[1..];
            let mut out = vec![0u8; stride];
            for x in 0..stride {
                out[x] = match f {
                    0 => data[x],
                    1 => data[x].wrapping_add(if x >= 4 { out[x - 4] } else { 0 }),
                    other => panic!("unexpected filter {other}"),
                };
            }
            assert_eq!(out[..], px[y * stride..(y + 1) * stride], "row {y}");
        }
    }

    /// Deflate must actually compress: a smooth-gradient + repeat image
    /// should shrink far below its stored size (the 50 MB pack regression).
    #[test]
    fn deflate_shrinks_smooth_and_repetitive_data() {
        let stride = 512usize;
        let mut data = vec![0u8; 300 * (stride + 1)];
        for y in 0..300 {
            data[y * (stride + 1)] = 1; // Sub filter rows
            for x in 0..stride {
                data[y * (stride + 1) + 1 + x] = ((x / 4) as u8) ^ ((y / 8) as u8);
            }
        }
        let z = zlib_deflate(&data);
        assert!(
            z.len() < data.len() / 3,
            "deflate {} vs raw {}",
            z.len(),
            data.len()
        );
    }

    /// Bit reader (LSB-first) + canonical fixed-Huffman walker.
    struct R<'a> {
        z: &'a [u8],
        p: usize,
        bit: u32,
    }
    impl<'a> R<'a> {
        fn bit(&mut self) -> u32 {
            let b = ((self.z[self.p] >> self.bit) & 1) as u32;
            self.bit += 1;
            if self.bit == 8 {
                self.bit = 0;
                self.p += 1;
            }
            b
        }
        fn bits(&mut self, n: u32) -> u32 {
            let mut v = 0u32;
            for i in 0..n {
                v |= self.bit() << i;
            }
            v
        }
        fn symbol(&mut self) -> u32 {
            let mut code = 0u32;
            let mut len = 0u32;
            loop {
                code = (code << 1) | self.bit();
                len += 1;
                match len {
                    7 if code <= 23 => return 256 + code,
                    8 => {
                        if (0x30..=0xBF).contains(&code) {
                            return code - 0x30;
                        }
                        if (0xC0..=0xC7).contains(&code) {
                            return 280 + (code - 0xC0);
                        }
                    }
                    9 => return 144 + (code - 0x190),
                    _ => {}
                }
                assert!(len < 10, "bad huffman stream");
            }
        }
        fn dist5(&mut self) -> u32 {
            let mut code = 0u32;
            for _ in 0..5 {
                code = (code << 1) | self.bit();
            }
            code
        }
    }

    /// Minimal zlib inflater for the encoder's own output (fixed-Huffman
    /// LZ77 + stored fallback), verifying the adler32 trailer.
    fn inflate_zlib(z: &[u8]) -> Vec<u8> {
        assert_eq!(z[0], 0x78, "zlib CMF");
        assert_eq!(((z[0] as u32) * 256 + (z[1] as u32)) % 31, 0, "FCHECK");
        let mut r = R { z, p: 2, bit: 0 };
        let mut out: Vec<u8> = Vec::new();
        loop {
            let bfinal = r.bit() == 1;
            let btype = r.bits(2);
            if btype == 0 {
                if r.bit != 0 {
                    r.p += 1;
                    r.bit = 0;
                }
                let len = u16::from_le_bytes([r.z[r.p], r.z[r.p + 1]]) as usize;
                let nlen = u16::from_le_bytes([r.z[r.p + 2], r.z[r.p + 3]]);
                assert_eq!(!len as u16, nlen, "stored length check");
                out.extend_from_slice(&r.z[r.p + 4..r.p + 4 + len]);
                r.p += 4 + len;
            } else {
                assert_eq!(btype, 1, "only fixed/stored blocks are produced");
                loop {
                    let sym = r.symbol();
                    if sym < 256 {
                        out.push(sym as u8);
                    } else if sym == 256 {
                        break;
                    } else {
                        let s = (sym - 257) as usize;
                        assert!(s < 29, "length symbol {sym}");
                        let len =
                            LEN_BASE[s] as usize + r.bits(LEN_EBITS[s] as u32) as usize;
                        let d = r.dist5() as usize;
                        assert!(d < 30, "dist symbol {d}");
                        let dist = DIST_BASE[d] as usize + r.bits(DIST_EBITS[d] as u32) as usize;
                        assert!(dist <= out.len(), "distance beyond output");
                        let start = out.len() - dist;
                        for k in 0..len {
                            let b = out[start + k];
                            out.push(b);
                        }
                    }
                }
            }
            if bfinal {
                break;
            }
        }
        let n = z.len();
        let adler = u32::from_be_bytes([z[n - 4], z[n - 3], z[n - 2], z[n - 1]]);
        assert_eq!(adler, adler32(&out), "adler32 must match the payload");
        out
    }
}
