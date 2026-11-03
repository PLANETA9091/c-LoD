#!/usr/bin/env python3
"""Reproduce the #227 control-arm login_disconnect (43 bytes) locally:
boot the EXACT control dist (paper 26.3 + the 5-line server.properties),
then speak the MC login protocol to 127.0.0.1:25565 and decode the
server's login response. Evidence, not guessing."""
import json, socket, struct, subprocess, sys, time, os

DIST = "/tmp/ctl-test/dist"
JAVA = os.path.expanduser("~/tools/jdk-25.0.5+8-jre/bin/java")
LOG = "/tmp/ctl-test/server.log"

def boot():
    subprocess.run(["pkill", "-f", "paper-26.3"], capture_output=True)
    time.sleep(1)
    f = open(LOG, "w")
    p = subprocess.Popen([JAVA, "-Xmx1G", "-jar", "versions/paper-26.3.jar", "--nogui"],
                         cwd=DIST, stdout=f, stderr=subprocess.STDOUT, stdin=subprocess.DEVNULL)
    for i in range(120):
        time.sleep(2)
        try:
            if "Done (" in open(LOG).read():
                print("server booted in", (i + 1) * 2, "s")
                return p
        except Exception:
            pass
        if p.poll() is not None:
            print("SERVER DIED during boot:"); print(open(LOG).read()[-1500:]); sys.exit(1)
    print("boot timeout"); print(open(LOG).read()[-1500:]); sys.exit(1)

def varint(n):
    out = b""
    while True:
        b = n & 0x7F
        n >>= 7
        if n: out += bytes([b | 0x80])
        else: return out + bytes([b])

def packet(pid, payload):
    body = varint(pid) + payload
    return varint(len(body)) + body

def read_varint(sock):
    n, shift = 0, 0
    while True:
        b = sock.recv(1)
        if not b: raise EOFError("closed during varint")
        n |= (b[0] & 0x7F) << shift
        if not (b[0] & 0x80): return n
        shift += 7

def read_packet(sock):
    ln = read_varint(sock)
    data = b""
    while len(data) < ln:
        chunk = sock.recv(ln - len(data))
        if not chunk: raise EOFError("closed mid-packet")
        data += chunk
    # data[0] = packet id varint (assume 1 byte for login-state ids 0..3)
    return data[0], data[1:]

def ping_protocol(sock):
    """status ping -> (server version name, protocol number). Handshake:
    protocol(placeholder 0 for status) + addr_len + addr + port + next_state."""
    hs = varint(0) + varint(9) + b"127.0.0.1" + struct.pack(">H", 25565) + varint(1)
    sock.sendall(packet(0x00, hs))
    sock.sendall(packet(0x00, b""))  # status request
    pid, payload = read_packet(sock)
    js = json.loads(payload[1:].decode())  # skip 1-byte string len prefix... varint actually
    # safer: parse varint string length
    def rd_varint_buf(buf, pos):
        n, shift = 0, 0
        while True:
            b = buf[pos]; pos += 1
            n |= (b & 0x7F) << shift
            if not (b & 0x80): return n, pos
            shift += 7
    ln, pos = rd_varint_buf(payload, 0)
    js = json.loads(payload[pos:pos + ln].decode())
    return js["version"]["name"], js["version"]["protocol"]

def try_login(sock, proto, name="Stepan"):
    hs = varint(proto) + varint(9) + b"127.0.0.1" + struct.pack(">H", 25565) + varint(2)
    sock.sendall(packet(0x00, hs))
    hello_payload = varint(len(name)) + name.encode() + uuid_bytes()
    sock.sendall(packet(0x00, hello_payload))
    pid, payload = read_packet(sock)
    return pid, payload

def uuid_bytes():
    import uuid
    return uuid.uuid4().bytes

def rd_varint_buf(buf, pos):
    n, shift = 0, 0
    while True:
        b = buf[pos]; pos += 1
        n |= (b & 0x7F) << shift
        if not (b & 0x80): return n, pos
        shift += 7

p = boot()
try:
    s = socket.create_connection(("127.0.0.1", 25565), timeout=10)
    s.settimeout(10)
    vname, proto = ping_protocol(s)
    print(f"STATUS: server version {vname!r}, protocol {proto}")
    s.close()
    s = socket.create_connection(("127.0.0.1", 25565), timeout=10)
    s.settimeout(10)
    pid, payload = try_login(s, proto)
    print(f"LOGIN response: packet_id={pid}, size={len(payload)} bytes")
    if pid == 0:
        ln, pos = rd_varint_buf(payload, 0)
        msg = payload[pos:pos + ln].decode(errors="replace")
        print("LOGIN_DISCONNECT MESSAGE (exact):")
        try:
            print(json.dumps(json.loads(msg), indent=2))
        except Exception:
            print(msg)
    else:
        print("unexpected packet (not disconnect):", payload[:80])
    s.close()
finally:
    subprocess.run(["pkill", "-f", "paper-26.3"], capture_output=True)
    print("server stopped")
