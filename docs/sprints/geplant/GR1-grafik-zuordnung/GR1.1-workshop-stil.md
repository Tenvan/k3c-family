# GR1.1 · Workshop: Grafik-Stil je Pack bestätigen

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Branch:** gr1/1-workshop-stil
- **Abhängig von:** –
- **Tickets:** B-161
- **Kriterien:** AC-05

## Ziel

`docs/assets/zuordnung.md` existiert mit einem Kopf, der den Stilbeschluss Q13 wörtlich nennt, und einer Pack-Tabelle, in der 🧑 je Pack „passt“ oder „passt nicht (Lücke)“ bestätigt hat.

## Kontext

- Stilbeschluss Q13 (`docs/fragenkatalog.md` › Beschlüsse vom 2026-10-03): Grundraster 16/32 px, Skalierung ×2 bis ×3; nicht passende Packs sind Lücken, Palettenbruch nur bei Hintergründen. Damit ist der Stil entschieden; der Workshop bestätigt nur die Anwendung je Pack (Plan § 11.2, Welle W3: „Q13 ist entschieden: nur bestätigen“).
- Packs: zwölf Umgebungs-Packs unter `public/grafik/` (Liste und Lizenzen in `public/grafik/CREDITS.md`, Bildgrößen in `public/grafik/index.json`, Gruppen tileset-einzeln, ebenen, props, icons, umgebung, ressourcen, muenzen, portale, vorschau) und die Figuren-Packs unter `public/sprites/` (LuizMelo, Gothicvania, LPC-Reittiere; `public/sprites/CREDITS.md`). Ansehen: Testseite `grafiken.html` (Umgebung) und `figuren.html` (Figuren).
- Ein Agent bereitet im Workshop vor: je Pack Raster (aus Kachel- bzw. Frame-Größe in `index.json` oder den PNG), nötige Skalierung für die Spielgröße (`UNIT_PX` = 32 px), Palette grob (hell/dunkel, gesättigt), Vorschlag „passt“ oder „passt nicht“ mit einem Satz Grund. 🧑 entscheidet. Dauer etwa 20 Minuten.

## Erlaubte Dateien

- `docs/assets/zuordnung.md` (neu: nur Kopf und Pack-Tabelle; die Objekt-Zeilen folgen in GR1.2 und GR1.3)
- `docs/sprints/` (nur Status und Ergebnis dieser Session)

## Nicht-Ziele

Zuordnung einzelner Spielobjekte (GR1.2, GR1.3), Test (GR1.2), neue Assets (GR2), Einbau (GR3).

## Schritte

1. Agent legt `docs/assets/zuordnung.md` mit Kopf (Q13 wörtlich, Datum) und Pack-Tabelle (Pack, Raster, Skalierung, Palette, Vorschlag, Grund) an.
2. 🧑 geht die Packs durch, ändert oder bestätigt; der Agent trägt mit.
3. Im Kopf „Bestätigt von 🧑 am <Datum>“ eintragen; unbestätigte Packs bleiben „Vorschlag“ und gelten bis zur Klärung als Lücke.
4. Ergebnis eintragen: welche Packs passen, welche nicht.

## Fertig, wenn

- [ ] AC-05 (Teil Kopf): Kopf von `docs/assets/zuordnung.md` nennt den Stilbeschluss Q13 und „Bestätigt von 🧑“ mit Datum.
- [ ] Pack-Tabelle mit Raster, Skalierung, Palette und Entscheidung je Pack.

## Prüfen

Manuell durch 🧑 (Gespräch); danach `task check`.

## Ergebnis

–
