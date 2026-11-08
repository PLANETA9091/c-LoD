#!/usr/bin/env python3
"""Numeric check of the v6.20.0 ray-true SSAO probe against the #305 failure
classes. Ports lod_terrain.fsh march_ssao + ground_at to Python on synthetic
heightfields and prints the ao factor per scenario.

Scenarios (the #305 evidence):
  A flat ground            -> voxy/skip semantics: ao == 1.0
  B terraced slope (graze) -> ao ~ 0.8-1.0 (the old disc probe: ~0.26 crush)
  C forest gap from above  -> ao mild (towers damped by falloff)
  D pit rim                -> mild occlusion kept (relief shadows survive)
"""
import math

CELL = 2.0
SSAO_STEPS = 12
SSAO_PHI1 = 1.6180339887
SSAO_TAU = 6.2831853
SSAO_RADIUS = 1.5


class Field:
    def __init__(self, hfun):
        self.h = hfun

    def ground_at(self, x, z):
        ci = math.floor(x / CELL)
        cj = math.floor(z / CELL)
        if ci < 0 or cj < 0 or ci > 255 or cj > 255:
            return 0.0, False
        return self.h(ci, cj), True


def norm(v):
    l = math.sqrt(v[0] * v[0] + v[1] * v[1] + v[2] * v[2])
    return (v[0] / l, v[1] / l, v[2] / l)


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


def ssao_tbn(n):
    if n[1] == 1.0:
        t0 = (1.0, 0.0, 0.0)
    else:
        # cross((0,1,0), n)
        t0 = norm((1.0 * n[2], 0.0, -1.0 * n[0]))
    t1 = norm((t0[1] * n[2] - t0[2] * n[1],
               t0[2] * n[0] - t0[0] * n[2],
               t0[0] * n[1] - t0[1] * n[0]))
    return t0, t1


def r1(n):
    return math.modf(0.5 + n / SSAO_PHI1)[0]


def march_ssao(field, pos, n, cam, dirv, F, t_hit):
    t0, t1 = ssao_tbn(n)
    zP = t_hit * dot(dirv, F)
    ao = 0.0
    skips = {"nodata": 0, "samedepth": 0, "dotkill": 0, "occ": 0}
    for i in range(SSAO_STEPS):
        a = (i + 0.5) * (1.0 / SSAO_STEPS)
        r = math.sqrt(r1(i))
        theta = a * SSAO_TAU
        ox, oy = r * math.cos(theta), r * math.sin(theta)
        T = tuple((t0[k] * ox + t1[k] * oy) * SSAO_RADIUS for k in range(3))
        d = dot(T, dirv)
        T = (T[0] - dirv[0] * d, T[1] - dirv[1] * d, T[2] - dirv[2] * d)
        ds = norm(tuple(dirv[k] + T[k] / max(t_hit, 1.0) for k in range(3)))
        dF = max(dot(ds, F), 1e-4)
        tp = zP / dF
        g1, ok1 = field.ground_at(cam[0] + ds[0] * tp, cam[2] + ds[2] * tp)
        if not ok1:
            skips["nodata"] += 1
            continue
        f1 = (cam[1] + ds[1] * tp) - g1
        t2 = tp * 1.15 + 1.0
        g2, ok2 = field.ground_at(cam[0] + ds[0] * t2, cam[2] + ds[2] * t2)
        f2 = ((cam[1] + ds[1] * t2) - g2) if ok2 else f1
        ts = tp
        if abs(f2 - f1) > 1e-3:
            ts = tp - f1 * (t2 - tp) / (f2 - f1)
            ts = max(tp * 0.4, min(ts, tp * 3.0))
        if f1 <= 0.0:
            occ = (cam[0] + ds[0] * tp, g1, cam[2] + ds[2] * tp)
        elif f2 <= 0.0:
            occ = tuple(cam[k] + ds[k] * ts for k in range(3))
        else:
            occ = (cam[0] + ds[0] * tp, g1, cam[2] + ds[2] * tp)
        zOcc = dot(sub(occ, cam), F)
        if abs(zOcc - zP) < 0.5:
            skips["samedepth"] += 1
            continue
        v = sub(occ, pos)
        slen = dot(v, v)
        rlen = 1.0 / math.sqrt(max(slen, 1e-6))
        cos_t = max(0.0, min(1.0, dot(v, n) * rlen))
        if cos_t <= 0.0:
            skips["dotkill"] += 1
        else:
            skips["occ"] += 1
        falloff = 1.0 / (1.0 + (1.0 / rlen) * (1.0 / SSAO_RADIUS))
        ao += cos_t * falloff
    return math.pow(max(0.0, min(1.0, 1.0 - ao * (1.0 / SSAO_STEPS))), 2.5), skips


def gradient_normal(field, x, z):
    hC, _ = field.ground_at(x, z)
    hL, _ = field.ground_at(x - CELL, z)
    hR, _ = field.ground_at(x + CELL, z)
    hD, _ = field.ground_at(x, z - CELL)
    hU, _ = field.ground_at(x, z + CELL)
    for h in (hL, hR, hD, hU):
        pass
    return norm((hL - hR, 2.0 * CELL, hD - hU))


def old_disc_ssao(field, pos, n):
    """The v6.8.1 flat world-disc probe (the #305 behavior) for comparison."""
    t0, t1 = ssao_tbn(n)
    ao = 0.0
    for i in range(SSAO_STEPS):
        a = (i + 0.5) * (1.0 / SSAO_STEPS)
        r = math.sqrt(r1(i))
        theta = a * SSAO_TAU
        ox, oy = r * math.cos(theta), r * math.sin(theta)
        qx = pos[0] + (t0[0] * ox + t1[0] * oy) * SSAO_RADIUS
        qz = pos[2] + (t0[2] * ox + t1[2] * oy) * SSAO_RADIUS
        hq, ok = field.ground_at(qx, qz)
        if not ok:
            continue
        v = (qx - pos[0], hq - pos[1], qz - pos[2])
        slen = dot(v, v)
        rlen = 1.0 / math.sqrt(max(slen, 1e-6))
        cos_t = max(0.0, min(1.0, dot(v, n) * rlen))
        falloff = 1.0 / (1.0 + (1.0 / rlen) * (1.0 / SSAO_RADIUS))
        ao += cos_t * falloff
    return math.pow(max(0.0, min(1.0, 1.0 - ao * (1.0 / SSAO_STEPS))), 2.5)


def run(name, field, cam, target, F_yaw_deg, pitch_deg):
    az = math.radians(F_yaw_deg)
    el = math.radians(pitch_deg)
    F = (math.cos(el) * math.cos(az), math.sin(el), math.cos(el) * math.sin(az))
    fov = 70.0
    tanH = math.tan(math.radians(fov) * 0.5)
    # the pixel at the screen center: dir = F
    dirv = F
    # march the center ray to its first entering crossing (coarse, fine here:
    # analytic-ish stepping)
    t = 1.0
    dt = 1.0
    t_hit = -1.0
    prev_above = None
    for _ in range(4000):
        t += dt
        dt = max(CELL * 0.5, t * (2.0 * tanH / 720.0))
        x, y, z = cam[0] + dirv[0] * t, cam[1] + dirv[1] * t, cam[2] + dirv[2] * t
        g, ok = field.ground_at(x, z)
        if not ok:
            continue
        above = g - y
        if above > 0 and prev_above is not None and prev_above <= 0:
            lo, hi = t - dt, t
            for _b in range(30):
                mid = (lo + hi) * 0.5
                ym = cam[1] + dirv[1] * mid
                gm, _ = field.ground_at(cam[0] + dirv[0] * mid, cam[2] + dirv[2] * mid)
                if gm - ym > 0:
                    hi = mid
                else:
                    lo = mid
            t_hit = (lo + hi) * 0.5
            break
        prev_above = above
    if t_hit < 0:
        print(f"{name}: ray never hits (skip)")
        return
    pos = tuple(cam[k] + dirv[k] * t_hit for k in range(3))
    n = gradient_normal(field, pos[0], pos[2])
    new, skips = march_ssao(field, pos, n, cam, dirv, F, t_hit)
    old = old_disc_ssao(field, pos, n)
    print(f"{name}: t_hit={t_hit:.1f} pos=({pos[0]:.1f},{pos[1]:.1f},{pos[2]:.1f}) "
          f"ao_new={new:.3f} ao_old(disc)={old:.3f} skips={skips}")


print("=== A: flat ground (h=64) viewed from above ===")
run("A flat", Field(lambda i, j: 64.0), (0.0, 100.0, 0.0), None, 0.0, -60.0)

print("=== B: terraced slope (+2m per cell in x), eye WEST at graze ===")
run("B terrace", Field(lambda i, j: 64.0 + 2.0 * i), (-40.0, 72.0, 0.0), None, 0.0, -8.0)

print("=== B2: terraced slope, eye higher (aerial-ish) ===")
run("B2 terrace hi", Field(lambda i, j: 64.0 + 2.0 * i), (-40.0, 96.0, 0.0), None, 0.0, -20.0)

print("=== C: forest gap (ground 64, towers +10 on even cells), eye above ===")
def forest(i, j):
    base = 64.0
    if (i % 2 == 0) and (j % 2 == 0):
        return base + 10.0
    return base
run("C forest", Field(forest), (1.0, 120.0, 1.0), None, 0.0, -80.0)

print("=== D: pit (h=64, a 8m-deep pit 3 cells wide), eye above the rim ===")
def pit(i, j):
    if 5 <= i <= 8 and 5 <= j <= 8:
        return 56.0
    return 64.0
run("D pit", Field(pit), (0.0, 110.0, 7.0), None, 0.0, -55.0)

print("=== E: smooth hill (sinusoidal, 6m amplitude over 40 cells) ===")
import math as _m
run("E hill", Field(lambda i, j: 64.0 + 6.0 * _m.sin(i * 0.15)), (-60.0, 74.0, 0.0), None, 0.0, -5.0)
