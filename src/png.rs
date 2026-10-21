//! Minimal PNG encoder (RGBA8, filter type 0, zlib "stored" deflate blocks).
//! Used to emit the heightmap atlas into the resource pack. Zero deps.

fn adler32(data: &[u8]) -> u32 {
    let (mut a, mut b) = (1u32, 0u32);
    for &byte in data {
        a = (a + byte as u32) % 65521;
        b = (b + a) % 65521;
    }
    (b << 16) | a
}

/// Raw DEFLATE stream with only stored (uncompressed) blocks, wrapped in a
/// zlib container. Valid for any length.
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

fn chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    let mut crc_input = Vec::with_capacity(4 + data.len());
    crc_input.extend_from_slice(kind);
    crc_input.extend_from_slice(data);
    out.extend_from_slice(kind);
    // Local fix: the chunk payload was never written, so every PNG emitted by
    // this encoder was the 44-byte skeleton (IHDR/IDAT with no data) and the
    // client could never upload the atlas texture -> LOD never rendered.
    out.extend_from_slice(data);
    out.extend_from_slice(&crc32_bytes(&crc_input));
}

fn crc32_bytes(data: &[u8]) -> [u8; 4] {
    let c = crate::zipw::crc32(data);
    c.to_be_bytes()
}

/// One PNG scanline filter. `prev` is the already-unfiltered previous row.
fn filter_row(kind: u8, row: &[u8], prev: &[u8], out: &mut [u8]) {
    let bpp = 4;
    for i in 0..row.len() {
        let a = if i >= bpp { row[i - bpp] } else { 0 }; // left
        let b = prev[i]; // up
        let c = if i >= bpp { prev[i - bpp] } else { 0 }; // up-left
        let x = row[i];
        out[i] = match kind {
            0 => x,
            1 => x.wrapping_sub(a),
            2 => x.wrapping_sub(b),
            3 => x.wrapping_sub((((a as u16) + (b as u16)) / 2) as u8),
            _ => {
                let p = a as i16 + b as i16 - c as i16;
                let pa = (p - a as i16).abs();
                let pb = (p - b as i16).abs();
                let pc = (p - c as i16).abs();
                let pred = if pa <= pb && pa <= pc {
                    a
                } else if pb <= pc {
                    b
                } else {
                    c
                };
                x.wrapping_sub(pred)
            }
        };
    }
}

/// Encode an RGBA8 image as PNG. `rgba.len()` must be `width * height * 4`.
///
/// Uses adaptive per-scanline filtering + real DEFLATE (miniz_oxide). The old
/// stored-only stream made a 2048² atlas 16 MiB, which is unusable as a
/// server resource pack.
pub fn encode_png(width: u32, height: u32, rgba: &[u8]) -> Vec<u8> {
    assert_eq!(rgba.len(), (width as usize) * (height as usize) * 4);
    let stride = width as usize * 4;
    let mut raw = Vec::with_capacity(height as usize * (stride + 1));
    let mut prev = vec![0u8; stride];
    let mut best = vec![0u8; stride];
    let mut trial = vec![0u8; stride];
    for y in 0..height as usize {
        let row = &rgba[y * stride..(y + 1) * stride];
        let mut best_kind = 0u8;
        let mut best_score = u64::MAX;
        for kind in 0..5u8 {
            filter_row(kind, row, &prev, &mut trial);
            // Classic heuristic: smallest sum of absolute signed residuals.
            let score: u64 = trial.iter().map(|&b| (b as i8).unsigned_abs() as u64).sum();
            if score < best_score {
                best_score = score;
                best_kind = kind;
                best.copy_from_slice(&trial);
            }
        }
        raw.push(best_kind);
        raw.extend_from_slice(&best);
        prev.copy_from_slice(row);
    }
    let idat = miniz_oxide::deflate::compress_to_vec_zlib(&raw, 8);

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

    /// Minimal PNG reader for tests: inflate IDAT and undo the scanline filters.
    fn decode(png: &[u8]) -> (u32, u32, Vec<u8>) {
        let mut p = 8usize;
        let mut w = 0u32;
        let mut h = 0u32;
        let mut idat = Vec::new();
        while p + 8 <= png.len() {
            let len = u32::from_be_bytes([png[p], png[p + 1], png[p + 2], png[p + 3]]) as usize;
            let kind = &png[p + 4..p + 8];
            let data = &png[p + 8..p + 8 + len];
            match kind {
                b"IHDR" => {
                    w = u32::from_be_bytes([data[0], data[1], data[2], data[3]]);
                    h = u32::from_be_bytes([data[4], data[5], data[6], data[7]]);
                }
                b"IDAT" => idat.extend_from_slice(data),
                _ => {}
            }
            p += 12 + len;
        }
        let raw = miniz_oxide::inflate::decompress_to_vec_zlib(&idat).expect("deflate");
        let stride = w as usize * 4;
        assert_eq!(raw.len(), h as usize * (stride + 1));
        let mut out = vec![0u8; stride * h as usize];
        for y in 0..h as usize {
            let kind = raw[y * (stride + 1)];
            let row = &raw[y * (stride + 1) + 1..(y + 1) * (stride + 1)];
            for i in 0..stride {
                let a = if i >= 4 { out[y * stride + i - 4] } else { 0 };
                let b = if y > 0 { out[(y - 1) * stride + i] } else { 0 };
                let c = if y > 0 && i >= 4 { out[(y - 1) * stride + i - 4] } else { 0 };
                let pred = match kind {
                    0 => 0,
                    1 => a,
                    2 => b,
                    3 => (((a as u16) + (b as u16)) / 2) as u8,
                    4 => {
                        let p = a as i16 + b as i16 - c as i16;
                        let pa = (p - a as i16).abs();
                        let pb = (p - b as i16).abs();
                        let pc = (p - c as i16).abs();
                        if pa <= pb && pa <= pc {
                            a
                        } else if pb <= pc {
                            b
                        } else {
                            c
                        }
                    }
                    other => panic!("bad filter {other}"),
                };
                out[y * stride + i] = row[i].wrapping_add(pred);
            }
        }
        (w, h, out)
    }

    #[test]
    fn png_roundtrips_pixels() {
        let (w, h) = (64u32, 32u32);
        let px: Vec<u8> = (0..(w * h * 4)).map(|i| ((i * 7 + i / 13) % 256) as u8).collect();
        let png = encode_png(w, h, &px);
        let (dw, dh, out) = decode(&png);
        assert_eq!((dw, dh), (w, h));
        assert_eq!(out, px);
    }

    #[test]
    fn png_idat_is_compressed() {
        // Regression: stored-deflate made the 2048² atlas a 16 MiB pack.
        let (w, h) = (256u32, 256u32);
        let px = vec![42u8; (w * h * 4) as usize];
        let png = encode_png(w, h, &px);
        assert!(
            png.len() < (w * h * 4) as usize / 8,
            "flat image must compress, got {} bytes",
            png.len()
        );
        let (_, _, out) = decode(&png);
        assert_eq!(out, px);
    }

    #[test]
    fn png_chunks_carry_their_payload() {
        // Regression: `chunk()` used to write the length, tag and CRC but not
        // the data itself, so every atlas texture was a 44-byte skeleton.
        let (w, h) = (16u32, 8u32);
        let px: Vec<u8> = (0..(w * h * 4)).map(|i| (i % 251) as u8).collect();
        let png = encode_png(w, h, &px);

        let mut p = 8usize; // after signature
        let mut seen = Vec::new();
        while p + 8 <= png.len() {
            let len =
                u32::from_be_bytes([png[p], png[p + 1], png[p + 2], png[p + 3]]) as usize;
            let kind = &png[p + 4..p + 8];
            let data = &png[p + 8..p + 8 + len];
            seen.push((String::from_utf8_lossy(kind).to_string(), len));
            if kind == b"IHDR" {
                assert_eq!(data.len(), 13);
                assert_eq!(&data[0..4], &w.to_be_bytes());
                assert_eq!(&data[4..8], &h.to_be_bytes());
                assert_eq!(data[8], 8); // bit depth
                assert_eq!(data[9], 6); // RGBA
            }
            if kind == b"IEND" {
                assert_eq!(len, 0);
            }
            p += 12 + len;
        }
        assert_eq!(p, png.len(), "chunk walk must land exactly on EOF");
        assert_eq!(seen[0].0, "IHDR");
        assert_eq!(seen.last().unwrap().0, "IEND");
        let idat = seen.iter().find(|(k, _)| k == "IDAT").expect("IDAT");
        assert!(idat.1 > 0, "IDAT payload must not be empty");
        let (dw, dh, out) = decode(&png);
        assert_eq!((dw, dh), (w, h));
        assert_eq!(out, px);
    }
}
