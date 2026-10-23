# B-531 · Wald hat mehr Bäume je Abschnitt, Höhle bekommt Deko-Büsche

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GEB14
- **Projekt:** GEB
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

GEB10 hat die Bäume 11,25 Kacheln hoch und 2–4 Kacheln breit gemacht; dichter wirkt der Wald nur über die breiteren Kronen. Wie viele Bäume ein Abschnitt hat, steht in `data/biomes/forest.json` › `resourcesPerChunk` (z. B. `forest: tree [4, 7]`). Der Level-Generator setzt Büsche nur im Wald (`clearing`), der `hoehlenbusch` aus GEB10 erscheint deshalb im Spiel nicht.

## Ziel

Der Wald steht sichtbar dichter (mehr Bäume je Abschnitt), die Höhle zeigt Deko-Büsche (Höhlenbusch).

## Beteiligte und Zielgruppen

Spielende (Wald, Höhle); 🧑 legt die Werte fest (B-540); Agent REG für `data/`, CLI nur falls der Höhlenbusch nicht erscheint.

## Anforderungen

- Baumzahl je Abschnitt im Wald nach 🧑 (B-540), deterministisch über den Level-Generator.
- Höhle (optional Mine) mit `bush` als Deko; Holz-Balancing mit `sim_test` im Rahmen.

## Nicht-Ziele

Neue Grafik; Baumgröße (B-541).

## Regeln und Einschränkungen

Werte in `data/biomes/*.json`; Golden nur mit Begründung (`task go:golden`); Testläufe nur über `sim_test`.

## Beispiele

Wald `tree [6, 9]` → sichtbar dichter, Holz je Nacht steigt moderat.

## Ausnahme- und Fehlerfälle

🧑 sagt „so lassen“ → Ticket verworfen.

## Akzeptanzkriterien

- **AC-01** 🧑 legt die Baumzahl je Abschnitt im Wald fest (Werte in `data/biomes/forest.json`), Balancing (Holz) bleibt im Rahmen.
- **AC-02** Höhle (und auf Wunsch Mine) bekommt `bush` als Deko in `resourcesPerChunk`; der Client zeigt dort den Höhlenbusch (GEB10.3).

## Offene Fragen

keine (Entscheidung D3 in B-540 gefallen, 2026-10-23).

## Notizen

Geplant in GEB12.2 (2026-10-22): wird nach B-540 umgesetzt, Rang im Projekt GEB nach GEB12.

**Werte 🧑 (KR2.1, 2026-10-23, B-540):** Wald `tree [6, 9]` statt `[4, 7]`, Höhle `bush [1, 3]`, Mine ohne Büsche. Umsetzung REG mit `task go:golden` (Begründung in den Commit) und `sim_test` für Holz.
