//! Embedded HTTP server: serves the generated resource pack to vanilla
//! clients. std-only, one accept thread, one thread per connection (packs
//! are large but client downloads are rare).
//!
//! v4.3 streaming: the pack bytes are SWAPPED on every streamed revision
//! (same URL, new SHA-1 — vanilla clients re-download only when the hash
//! differs). Diagnostic endpoints let ops/CI observe the stream without a
//! game client:
//!   GET /clod-pack.zip    the pack (ETag = SHA-1, X-Clod-Revision header)
//!   GET /clod-pack.sha1   current SHA-1 hex (plain text)
//!   GET /clod-pack.info   revision/cells/sha1/bytes summary (plain text)

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};

/// Metadata of the currently served pack (v4.3 streaming diagnostics).
#[derive(Clone, Default)]
pub struct PackMeta {
    pub rev: u64,
    pub sha1_hex: String,
    pub cells: u64,
    pub total_cells: u64,
}

pub struct PackServer {
    pub bytes: RwLock<Arc<Vec<u8>>>,
    pub meta: RwLock<PackMeta>,
}

impl PackServer {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            bytes: RwLock::new(Arc::new(Vec::new())),
            meta: RwLock::new(PackMeta::default()),
        })
    }

    /// Swap in a new pack revision (bytes + metadata) atomically per field.
    pub fn set_pack(&self, zip: Vec<u8>, meta: PackMeta) {
        if let Ok(mut b) = self.bytes.write() {
            *b = Arc::new(zip);
        }
        if let Ok(mut m) = self.meta.write() {
            *m = meta;
        }
    }

    /// Refresh the LIVE stream counters served by /clod-pack.info without
    /// touching the pack bytes or its content-fixed sha1 (v5.2.2: the info
    /// endpoint used to report the boot-time cells=0 forever).
    pub fn set_meta(&self, cells: u64, total_cells: u64) {
        if let Ok(mut m) = self.meta.write() {
            m.cells = cells;
            m.total_cells = total_cells;
        }
    }

    /// Snapshot of the currently served pack bytes (empty until finalize).
    #[allow(dead_code)] // used by tests and diagnostics
    pub fn pack_bytes(&self) -> Vec<u8> {
        self.bytes
            .read()
            .map(|b| b.as_ref().clone())
            .unwrap_or_default()
    }

    #[allow(dead_code)] // used by tests
    pub fn meta(&self) -> PackMeta {
        self.meta.read().map(|m| m.clone()).unwrap_or_default()
    }

    pub fn serve(self: &Arc<Self>, port: u16) -> std::io::Result<()> {
        let listener = TcpListener::bind(("0.0.0.0", port))?;
        crate::log_info(&format!(
            "c-LoD pack server listening on 0.0.0.0:{port} (GET /clod-pack.zip | /clod-pack.sha1 | /clod-pack.info)"
        ));
        for stream in listener.incoming() {
            match stream {
                Ok(s) => {
                    let srv = Arc::clone(self);
                    std::thread::spawn(move || {
                        let _ = handle(s, &srv);
                    });
                }
                Err(_) => continue,
            }
        }
        Ok(())
    }
}

fn handle(mut stream: TcpStream, srv: &PackServer) -> std::io::Result<()> {
    let mut buf = [0u8; 4096];
    let mut req = Vec::new();
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            break;
        }
        req.extend_from_slice(&buf[..n]);
        if req.windows(4).any(|w| w == b"\r\n\r\n") || req.len() > 16 * 1024 {
            break;
        }
    }
    let req = String::from_utf8_lossy(&req);
    let first_line = req.lines().next().unwrap_or("");
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or("");
    let path = parts.next().unwrap_or("");

    let head_only = method == "HEAD";
    if method != "GET" && !head_only {
        return write_status(&mut stream, 405, "method not allowed", b"GET/HEAD only");
    }
    let path = path.split('?').next().unwrap_or(path);

    match path {
        "/clod-pack.zip" => {
            let Some(pack) = srv.bytes.read().ok().map(|b| Arc::clone(&b)) else {
                return write_status(&mut stream, 503, "service unavailable", b"pack not ready");
            };
            let meta = srv.meta();
            stream.write_all(b"HTTP/1.1 200 OK\r\n")?;
            stream.write_all(b"Content-Type: application/zip\r\n")?;
            stream.write_all(format!("Content-Length: {}\r\n", pack.len()).as_bytes())?;
            stream.write_all(b"Cache-Control: no-cache\r\n")?;
            if !meta.sha1_hex.is_empty() {
                stream.write_all(format!("ETag: \"{}\"\r\n", meta.sha1_hex).as_bytes())?;
                stream.write_all(format!("X-Clod-Revision: {}\r\n", meta.rev).as_bytes())?;
            }
            stream.write_all(b"Connection: close\r\n\r\n")?;
            if !head_only {
                stream.write_all(&pack)?;
            }
            stream.flush()
        }
        "/clod-pack.sha1" => {
            let meta = srv.meta();
            if meta.sha1_hex.is_empty() {
                return write_status(&mut stream, 503, "service unavailable", b"pack not ready");
            }
            let body = format!("{}\n", meta.sha1_hex).into_bytes();
            write_plain(&mut stream, &body)
        }
        "/clod-pack.info" => {
            let meta = srv.meta();
            if meta.sha1_hex.is_empty() {
                return write_status(&mut stream, 503, "service unavailable", b"pack not ready");
            }
            let len = srv.bytes.read().map(|b| b.len()).unwrap_or(0);
            let body = format!(
                "revision={}\nsha1={}\ncells={}/{}\nbytes={}\n",
                meta.rev, meta.sha1_hex, meta.cells, meta.total_cells, len
            )
            .into_bytes();
            write_plain(&mut stream, &body)
        }
        _ => write_status(
            &mut stream,
            404,
            "not found",
            b"c-LoD: use /clod-pack.zip, /clod-pack.sha1, /clod-pack.info",
        ),
    }
}

fn write_plain(stream: &mut TcpStream, body: &[u8]) -> std::io::Result<()> {
    stream.write_all(b"HTTP/1.1 200 OK\r\n")?;
    stream.write_all(b"Content-Type: text/plain\r\n")?;
    stream.write_all(format!("Content-Length: {}\r\n", body.len()).as_bytes())?;
    stream.write_all(b"Cache-Control: no-cache\r\n")?;
    stream.write_all(b"Connection: close\r\n\r\n")?;
    stream.write_all(body)?;
    stream.flush()
}

fn write_status(stream: &mut TcpStream, code: u16, msg: &str, body: &[u8]) -> std::io::Result<()> {
    let resp = format!(
        "HTTP/1.1 {code} {msg}\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(resp.as_bytes())?;
    stream.write_all(body)?;
    stream.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpStream as CStream;
    use std::time::Duration;

    fn get(port: u16, path: &str) -> Vec<u8> {
        let mut c = CStream::connect(("127.0.0.1", port)).expect("connect");
        c.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
        c.write_all(format!("GET {path} HTTP/1.1\r\nHost: t\r\n\r\n").as_bytes())
            .unwrap();
        let mut resp = Vec::new();
        let _ = c.read_to_end(&mut resp);
        resp
    }

    #[test]
    fn serves_pack_and_stream_diagnostics() {
        let srv = PackServer::new();
        srv.set_pack(
            vec![1, 2, 3, 4, 5, 6, 7, 8],
            PackMeta {
                rev: 3,
                sha1_hex: "abc123".into(),
                cells: 100,
                total_cells: 200,
            },
        );
        let srv2 = Arc::clone(&srv);
        let h = std::thread::spawn(move || {
            let _ = srv2.serve(18401);
        });
        std::thread::sleep(Duration::from_millis(300));

        let resp = get(18401, "/clod-pack.zip");
        let text = String::from_utf8_lossy(&resp);
        assert!(text.starts_with("HTTP/1.1 200 OK"));
        assert!(text.contains("Content-Length: 8"));
        assert!(text.contains("ETag: \"abc123\""));
        assert!(text.contains("X-Clod-Revision: 3"));
        assert!(resp.ends_with(&[1, 2, 3, 4, 5, 6, 7, 8]));

        let resp = get(18401, "/clod-pack.sha1");
        assert!(String::from_utf8_lossy(&resp).contains("abc123"));
        let resp = get(18401, "/clod-pack.info");
        let text = String::from_utf8_lossy(&resp);
        assert!(text.contains("revision=3"), "{text}");
        assert!(text.contains("cells=100/200"), "{text}");
        assert!(text.contains("bytes=8"), "{text}");

        // 404 path
        let resp = get(18401, "/nope");
        assert!(String::from_utf8_lossy(&resp).starts_with("HTTP/1.1 404"));

        drop(h);
    }

    #[test]
    fn set_meta_updates_live_counters_without_touching_pack() {
        let srv = PackServer::new();
        srv.set_pack(
            vec![9, 9],
            PackMeta {
                rev: 1,
                sha1_hex: "fff".into(),
                cells: 0,
                total_cells: 0,
            },
        );
        // v5.2.2: the engine refreshes live scan counters every 100 ticks;
        // bytes and sha1 must stay untouched.
        srv.set_meta(53034, 65536);
        let m = srv.meta();
        assert_eq!(m.cells, 53034);
        assert_eq!(m.total_cells, 65536);
        assert_eq!(m.rev, 1);
        assert_eq!(m.sha1_hex, "fff");
        assert_eq!(srv.pack_bytes(), vec![9, 9]);
    }
}
