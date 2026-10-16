# GEB3 · DEV, CLI · Mauer und Turm, Stufen 1 bis 5

- **Status:** aktiv
- **Projekt:** GEB
- **Domäne:** DEV, CLI
- **Reife:** bereit
- **Tickets:** B-435, B-437
- **Start-Commit:** 670eace6
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-16, Chat (Ralf), Revision 1

## Ausgangslage

Mauer und Turm gibt es in fünf Stufen (Holz, Stein, Kupfer, Eisen, Kristall; `rules/materialien-gebaeude.md` § 1–2) aus gemischten Packs: Mauer = gestapelte 16-px-Kachel ×2 (1 × 5 Kacheln, `gothicvania-town/tileset-einzeln/wall.png`, `k3c-paletten/tileset-einzeln/wall-*.png`), Turm = Pack-Bild 112 × 128 ×2 (7 × 8 Kacheln, `k3c-paletten/props/tower-*.png`). `wall:1`, `tower:1` und `tower:5` (Zaubertum) sind Lücken. Regeldatei § 6 lässt offen, wie Stufen aus einer Quelle entstehen. Der Asset-Katalog führt eigene Gebäude-Grafik als Lizenz „unbekannt“ (B-437).

## Ziel

Mauer und Turm haben fünf zusammenpassende Stufen aus einer Geometrie mit Material-Paletten. Am Ende sichtbar: Stufenreihe in der Vorschau, von 🧑 abgenommen.

## Beteiligte und Zielgruppen

🧑 entscheidet Maße und Stufen-Mechanik und nimmt die Stufenreihe ab; Agenten bauen Werkzeug, Daten und Zuordnung; künftige GEB-Sprints (Burg/Hub-Stufen GEB4, Farm-Stufen GEB5) nutzen die Stufen-Mechanik.

## Anforderungen

B-435 › Anforderungen, B-437 › Anforderungen; Regeln aus `docs/standards/gebaeude-grafik.md`. Sprint-eigen:
- **Maße** (Entscheidung 🧑 2026-10-16, Chat): Mauer **1 × 5** Kacheln, Turm **3 × 8** Kacheln (schmal, Bogenschützen stehen oben).
- **Stufen in der Gebäude-JSON** (Entscheidung 🧑 2026-10-16, Chat; gilt für alle Gebäude, Regeldatei § 6): Ein Gebäude mit Ausbaustufen ist **eine** JSON. Ihre Bausteine und ihre Vorlage beschreiben die Grundform (Stufe 1). Ein Feld `stufen` listet je weitere Stufe genau, was sich gegenüber der Grundform ändert: `stufe` (Nummer), `aenderung` (Text: was anders aussieht, Teil der Vorlage), `farben` (Farbtausch alt → neu, DB32) und `zusaetze` (weitere Bausteine). `tools/gebaeude` rendert daraus `<id>-<stufe>-<dichte>.png` und schreibt die Stufen ins `index.json`.
- Materialien je Stufe nach § 1: Holz, Stein, Kupfer, Eisen, Kristall; Stufe 5 des Turms ist der **Zaubertum** (erkennbar an Zusätzen, z. B. Kristallspitze, Leuchten).
- Zuordnung ins Spiel: `wall:1`–`wall:5`, `tower:1`–`tower:5` in Maßstab 1 (32 px je Kachel); die Mauer wird ein Bild statt gestapelter Kachel.
- **Aufräumen** (Projekt GEB › Notizen): ersetzte Mauer- und Turm-Grafik samt Verweisen löschen, sofern nichts anderes sie nutzt.

## Nicht-Ziele

Tor (GEB4); Burg und Hub-Stufen (GEB4); Stufen für Werkstatt, Taverne usw. (folgen in ihren Sprints, Mechanik steht hier); Animationen (Zaubertum-Leuchten); Schadens- oder Baustellen-Looks; Bäume (B-438, GEB6).

## Regeln und Einschränkungen

- Domänen nacheinander: **DEV** (GEB3.1 Katalog, GEB3.2 Stufen im Werkzeug + Mauer, GEB3.3 Turm: `tools/gebaeude/`, `tools/k3c-dev/internal/assets/`, `public/grafik/gebaeude/`, `src/tools/assetvorschau*.ts`, Regeldatei) → **CLI** (GEB3.4: `src/scenes/buildingSprites*.ts`, `src/scenes/siteView.ts`, `data/assets.json`, Löschen alter Grafik). `engine/` bleibt unberührt.
- Determinismus: gleiche JSON → byte-gleiche PNGs; die PNGs der acht vorhandenen Gebäude bleiben unverändert.
- Keine neue Abhängigkeit. Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Beispiele

- `mauer.json` (Grundform Holz-Palisade, 1 × 5) mit `stufen: [{ "stufe": 2, "aenderung": "Steinquader statt Pfählen, Zinnen oben", "farben": { "oiled-cedar": "dim-gray", "rope": "smokey-ash" }, "zusaetze": [ … Zinnen … ] }, …]` → `task gebaeude` schreibt `mauer-1-32.png` … `mauer-5-32.png` (je 32 × 160 px) plus 16/64/128.
- Vorschau: Stufenreihe Mauer 1–5 und Turm 1–5 nebeneinander, mit Vorlage und Stufen-Änderung darunter.
- Im Spiel Turm auf Stufe 5 gebaut → `turm-5-32.png` (Zaubertum) in Maßstab 1, 3 Kacheln breit.

## Ausnahme- und Fehlerfälle

- Stufe ohne `aenderung`, doppelte oder lückenhafte Stufennummer, Farbtausch auf Nicht-DB32-Farbe → `task gebaeude` bricht mit Meldung ab.
- Zusatz außerhalb der Maße → Abbruch wie bei Bausteinen.
- Gebäude ohne `stufen` → wie bisher eine PNG je Dichte (`<id>-<dichte>.png`), Index unverändert.
- Baustelle oder unbekannte Stufe im Spiel → Platzhalter wie heute.

## Akzeptanzkriterien

- **AC-01** Mauer und Turm liegen in den Stufen 1–5 als Daten-JSON und PNG vor; alle Stufen teilen Geometrie und Maße und unterscheiden sich über Palette und Zusätze.
- **AC-02** Die Slots `wall:1`–`wall:5` und `tower:1`–`tower:5` sind zugeordnet, der Zaubertum (Stufe 5) ist erkennbar.
- **AC-03** 🧑 hat die Stufenreihe abgenommen.
- **AC-04** B-437/AC-01: Der Asset-Katalog führt eigene Gebäude-Grafik mit eigener Lizenz statt „unbekannt“.
- **AC-05** `tools/gebaeude` unterstützt `stufen` (Felder `stufe`, `aenderung`, `farben`, `zusaetze`) nach Regeldatei § 6, mit `go test` für Farbtausch, Zusätze, Dateinamen, Index und die Fehlerfälle; Gebäude ohne Stufen bleiben byte-gleich.
- **AC-06** Ersetzte Mauer- und Turm-Grafik ist mit allen Verweisen gelöscht (Suche ohne Treffer außerhalb historischer Doku), sofern keine andere Stelle sie nutzt; `task check` und `task check:go` grün.

## Offene Fragen

- Entschieden 2026-10-16 (Chat, Ralf): Mauer 1 × 5, Turm 3 × 8 Kacheln.
- Entschieden 2026-10-16 (Chat, Ralf): Stufen-Änderungen stehen in der Gebäude-JSON selbst, genau beschrieben, für alle Gebäude.
- Entschieden 2026-10-16 (Chat, Ralf): B-437 als erste Session (GEB3.1).
- Freigabe der Spec setzt 🧑.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB3.1 | `GEB3.1-katalog-eigene-grafik.md` | Umsetzung | autonom | fertig |
| GEB3.2 | `GEB3.2-stufen-werkzeug-mauer.md` | Umsetzung | autonom | offen |
| GEB3.3 | `GEB3.3-turm-stufen.md` | Umsetzung | autonom | offen |
| GEB3.4 | `GEB3.4-zuordnung-aufraeumen.md` | Umsetzung | autonom | offen |
| GEB3.5 | `GEB3.5-review-abnahme.md` | Review | autonom | offen |

- GEB3.1 (DEV) Asset-Katalog kennt eigene Gebäude-Grafik, B-437 (AC-04).
- GEB3.2 (DEV) Stufen im Werkzeug, Regeldatei § 6, Mauer 1–5, Stufenreihe in der Vorschau (AC-01, AC-05).
- GEB3.3 (DEV) Turm 1–5 mit Zaubertum (AC-01).
- GEB3.4 (CLI) Zuordnung im Spiel, alte Grafik löschen (AC-02, AC-06).
- GEB3.5 Review und Abnahme durch 🧑 (AC-03, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
