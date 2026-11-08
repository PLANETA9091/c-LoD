#!/usr/bin/env python3
"""Scenario grid for the v6.20.0 ray-true SSAO probe vs the v6.8.1 disc probe.

The #305 crush needed 2D-varying terrain (cell-scale steps in every
direction): the xz-projected tangent disc compresses the along-slope spread,
so 1D steps stay mild. Real terraces/forest vary in both axes -> most samples
land 0.3-1 m across a border with dh 1-3 m -> per-sample 0.25-0.42 -> ao 3-5
-> factor 0.26-0.5 (the dark tower band)."""
import math

CELL = 2.0
SSAO_STEPS = 12
SSAO_PHI1 = 1.6180339887
SSAO_TAU = 6.2831853
SSAO_RADIUS = 1.5


def norm(v):
    l = math.sqrt(v[0] ** 2 + v[1] ** 2 + v[2] ** 2)
    return (v[0] / l, v[1] / l, v[2] / l)


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def sub(a, b):
    return (a[0] - b[0], a[1] - b[1], a[2] - b[2])


class Field:
    def __init__(self, hfun):
        self.h = hfun

    def ground_at(self, x, z):
        ci = math.floor(x / CELL)
        cj = math.floor(z / CELL)
        if ci < 0 or cj < 0 or ci > 255 or cj > 255:
            return 0.0, False
        return self.h(ci, cj), True


def ssao_tbn(n):
    if n[1] == 1.0:
        t0 = (1.0, 0.0, 0.0)
    else:
        t0 = norm((n[2], 0.0, -n[0]))  # cross((0,1,0), n)
    t1 = norm((t0[1] * n[2] - t0[2] * n[1],
               t0[2] * n[0] - t0[0] * n[2],
               t0[0] * n[1] - t0[1] * n[0]))
    return t0, t1


def r1(n):
    return math.modf(0.5 + n / SSAO_PHI1)[0]


def gradient_normal(field, x, z):
    hC, _ = field.ground_at(x, z)
    hL, _ = field.ground_at(x - CELL, z)
    hR, _ = field.ground_at(x + CELL, z)
    hD, _ = field.ground_at(x, z - CELL)
    hU, _ = field.ground_at(x, z + CELL)
    return norm((hL - hR, 2.0 * CELL, hD - hU))


def march_ssao_new(field, pos, n, cam, dirv, F, t_hit):
    t0, t1 = ssao_tbn(n)
    zP = t_hit * dot(dirv, F)
    ao = 0.0
    for i in range(SSAO_STEPS):
        a = (i + 0.5) * (1.0 / SSAO_STEPS)
        r = math.sqrt(r1(i))
        theta = a * SSAO_TAU
        ox, oy = r * math.cos(theta), r * math.sin(theta)
        T = tuple((t0[k] * ox + t1[k] * oy) * SSAO_RADIUS for k in range(3))
        d = dot(T, dirv)
        T = tuple(T[k] - dirv[k] * d for k in range(3))
        ds = norm(tuple(dirv[k] + T[k] / max(t_hit, 1.0) for k in range(3)))
        dF = max(dot(ds, F), 1e-4)
        tp = zP / dF
        g1, ok1 = field.ground_at(cam[0] + ds[0] * tp, cam[2] + ds[2] * tp)
        if not ok1:
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
        if abs(dot(sub(occ, cam), F) - zP) < 0.5:
            continue
        v = sub(occ, pos)
        slen = dot(v, v)
        rlen = 1.0 / math.sqrt(max(slen, 1e-6))
        cos_t = max(0.0, min(1.0, dot(v, n) * rlen))
        falloff = 1.0 / (1.0 + (1.0 / rlen) * (1.0 / SSAO_RADIUS))
        ao += cos_t * falloff
    return math.pow(max(0.0, min(1.0, 1.0 - ao / SSAO_STEPS)), 2.5)


def march_ssao_old(field, pos, n):
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
    return math.pow(max(0.0, min(1.0, 1.0 - ao / SSAO_STEPS)), 2.5)


def probe(name, field, pos, cam):
    dirv = norm(sub(pos, cam))
    t_hit = math.sqrt(sum((pos[k] - cam[k]) ** 2 for k in range(3)))
    n = gradient_normal(field, pos[0], pos[2])
    new = march_ssao_new(field, pos, n, cam, dirv, dirv, t_hit)
    old = march_ssao_old(field, pos, n)
    lit_new = 102 * 0.85 * new  # grass-ish albedo x face, day light = 1
    lit_old = 102 * 0.85 * old
    print(f"{name}: ao_new={new:.3f} (lit~{lit_new:.0f})  ao_old={old:.3f} (lit~{lit_old:.0f})")


print("=== diagonal terraces (+2m per cell in BOTH axes), graze view from W ===")
ter = Field(lambda i, j: 64.0 + 2.0 * i + 2.0 * j)
cam = (-45.0, 70.0, 0.0)
for px in (2.0, 6.0, 10.0, 14.0):
    pz = px * 0.5
    py = 64.0 + 2.0 * math.floor(px / CELL) + 2.0 * math.floor(pz / CELL) + 1.0
    probe(f"  terrace@({px:.0f},{py:.0f},{pz:.0f})", ter, (px, py, pz), cam)

print("=== forest: ground 64, towers +10 on even-even cells, aerial view ===")
def forest(i, j):
    return 74.0 if (i % 2 == 0 and j % 2 == 0) else 64.0
forF = Field(forest)
camA = (5.0, 130.0, 5.0)
# shaded points: a gap cell top (64) and a tower top (74)
probe("  gap top (64)", forF, (7.0, 64.0, 7.0), camA)
probe("  tower top (74)", forF, (5.0, 74.0, 5.0), camA)

print("=== flat ground, aerial ===")
flat = Field(lambda i, j: 64.0)
probe("  flat", flat, (10.0, 64.0, 0.0), (10.0, 130.0, 0.0))

print("=== smooth hill (6m/26-cell sine), graze ===")
hill = Field(lambda i, j: 64.0 + 6.0 * math.sin(i * 0.24) * math.cos(j * 0.24))
probe("  hill mid", hill, (10.0, hill.h(5, 0) + 0.5, 0.0), (-40.0, 72.0, 0.0))
