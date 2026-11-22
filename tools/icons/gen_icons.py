"""Erzeugt die EXE-Icons von K3C im Papierschnitt-Stil (Pillow): Spiel, Workbench, Studio.

Aufruf (Repo-Wurzel):  python tools/icons/gen_icons.py
Ausgabe je App: appicon.png (256 px) und icon.ico (16, 32, 48, 256 px) in
  tools/k3c-workbench/icon/, tools/k3c-studio/icon/, rust/crates/k3c-game/icon/
Gezeichnet wird in 1024 px und herunterskaliert (glatte Kanten ohne Zusatzabhängigkeit).
"""
import math
import pathlib

from PIL import Image, ImageDraw, ImageFilter

S = 1024
ROOT = pathlib.Path(__file__).resolve().parents[2]

GOLD = (246, 196, 62)
GOLD_DARK = (199, 145, 28)
RED = (214, 64, 69)
INK = (18, 26, 44)


def gradient(top, bottom):
    img = Image.new("RGB", (S, S))
    px = img.load()
    for y in range(S):
        t = y / (S - 1)
        c = tuple(round(top[i] + (bottom[i] - top[i]) * t) for i in range(3))
        for x in range(S):
            px[x, y] = c
    return img


def rounded_mask():
    m = Image.new("L", (S, S), 0)
    ImageDraw.Draw(m).rounded_rectangle((24, 24, S - 24, S - 24), radius=int(S * 0.22), fill=255)
    return m


def shadow(layer_mask, dy=18, blur=10, alpha=110):
    sh = Image.new("L", (S, S), 0)
    sh.paste(layer_mask, (0, dy))
    sh = sh.filter(ImageFilter.GaussianBlur(blur))
    return sh.point(lambda v: v * alpha // 255)


def paste_cut(base, mask, color, dy=18):
    """Papierschnitt: Schatten, dann die Fläche in einer Farbe."""
    base.paste((0, 0, 0), (0, 0), shadow(mask, dy))
    base.paste(color, (0, 0), mask)


def poly_mask(points):
    m = Image.new("L", (S, S), 0)
    ImageDraw.Draw(m).polygon(points, fill=255)
    return m


def crown_mask(cx, cy, w, h):
    """Krone mit drei Zacken, Mittelpunkt (cx, cy), Breite w, Höhe h."""
    x0, x1 = cx - w / 2, cx + w / 2
    top, bot = cy - h / 2, cy + h / 2
    band = bot - h * 0.26
    pts = [(x0, bot), (x0, band), (x0, top + h * 0.12), (x0 + w * 0.25, band - h * 0.1),
           (cx, top), (x1 - w * 0.25, band - h * 0.1), (x1, top + h * 0.12), (x1, bot)]
    m = poly_mask(pts)
    d = ImageDraw.Draw(m)
    for px, py in ((x0, top + h * 0.12), (cx, top), (x1, top + h * 0.12)):
        r = w * 0.065
        d.ellipse((px - r, py - r, px + r, py + r), fill=255)
    return m


def draw_crown(img, cx, cy, w, h):
    m = crown_mask(cx, cy, w, h)
    paste_cut(img, m, GOLD)
    d = ImageDraw.Draw(img)
    bot = cy + h / 2
    d.rectangle((cx - w / 2, bot - h * 0.24, cx + w / 2, bot), fill=GOLD_DARK)
    r = w * 0.07
    gy = bot - h * 0.12
    d.ellipse((cx - r, gy - r, cx + r, gy + r), fill=RED)


def gear_mask(cx, cy, r_out, r_in, teeth, hole):
    m = Image.new("L", (S, S), 0)
    d = ImageDraw.Draw(m)
    pts = []
    for i in range(teeth * 2):
        a0 = math.pi * 2 * i / (teeth * 2)
        a1 = math.pi * 2 * (i + 1) / (teeth * 2)
        r = r_out if i % 2 == 0 else r_in
        pts += [(cx + r * math.cos(a0 + 0.06), cy + r * math.sin(a0 + 0.06)),
                (cx + r * math.cos(a1 - 0.06), cy + r * math.sin(a1 - 0.06))]
    d.polygon(pts, fill=255)
    d.ellipse((cx - r_in, cy - r_in, cx + r_in, cy + r_in), fill=255)
    d.ellipse((cx - hole, cy - hole, cx + hole, cy + hole), fill=0)
    return m


def finish(img, name, outdir):
    out = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    out.paste(img, (0, 0), rounded_mask())
    outdir.mkdir(parents=True, exist_ok=True)
    big = out.resize((256, 256), Image.LANCZOS)
    big.save(outdir / "appicon.png")
    out.save(outdir / "icon.ico", sizes=[(16, 16), (32, 32), (48, 48), (256, 256)])
    print("OK", name, outdir.relative_to(ROOT))


def game():
    img = gradient((36, 58, 110), (13, 22, 48))
    d = ImageDraw.Draw(img)
    for i in range(7):  # Sterne
        x, y = 150 + i * 120 + (i % 2) * 40, 150 + (i * 97) % 200
        d.ellipse((x - 7, y - 7, x + 7, y + 7), fill=(235, 240, 255))
    draw_crown(img, S / 2, S * 0.5, 600, 470)
    return img


def workbench():
    img = gradient((62, 84, 108), (28, 40, 54))
    paste_cut(img, gear_mask(S / 2, S / 2 + 6, 365, 300, 10, 0), (176, 192, 210))
    ImageDraw.Draw(img).ellipse((S / 2 - 225, S / 2 - 219, S / 2 + 225, S / 2 + 231), fill=(40, 56, 74))
    draw_crown(img, S / 2, S / 2 + 6, 330, 260)
    return img


def studio():
    img = gradient((120, 196, 226), (214, 238, 244))
    d = ImageDraw.Draw(img)
    d.ellipse((120, 90, 330, 300), fill=(255, 238, 170))  # Sonne
    sea = Image.new("L", (S, S), 0)
    ImageDraw.Draw(sea).rectangle((0, 640, S, S), fill=255)
    paste_cut(img, sea, (52, 132, 178), dy=-14)
    isl = Image.new("L", (S, S), 0)
    ImageDraw.Draw(isl).ellipse((170, 560, 854, 880), fill=255)
    ImageDraw.Draw(isl).pieslice((250, 380, 774, 820), 180, 360, fill=255)
    paste_cut(img, isl, (96, 176, 96))
    d = ImageDraw.Draw(img)
    d.pieslice((330, 700, 700, 940), 180, 360, fill=(70, 150, 78))
    d.rectangle((492, 520, 532, 640), fill=(120, 80, 48))  # Baumstamm
    tree = poly_mask([(512, 330), (400, 560), (624, 560)])
    paste_cut(img, tree, (44, 120, 70), dy=10)
    draw_crown(img, S / 2, 300, 300, 230)
    # Pinselstrich als Hinweis auf Gestalten
    d.line((690, 760, 880, 560), fill=(150, 98, 52), width=34)
    d.ellipse((860, 520, 930, 590), fill=RED)
    return img


def main():
    finish(game(), "Spiel", ROOT / "rust/crates/k3c-game/icon")
    finish(workbench(), "Workbench", ROOT / "tools/k3c-workbench/icon")
    finish(studio(), "Studio", ROOT / "tools/k3c-studio/icon")


if __name__ == "__main__":
    main()
