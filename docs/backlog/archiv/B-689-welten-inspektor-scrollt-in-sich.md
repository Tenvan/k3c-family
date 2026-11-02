# B-689 · Welten-Editor: Inspektor scrollt in sich, Höhe mit 24 Chips messen

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** WZ39
- **Projekt:** WZG
- **Erstellt:** 2026-10-31
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-02

## Ausgangslage

Gefunden in WZ38.1 (B-666/AC-05): `.wd-inspector` in `tools/k3c-studio/frontend/src/styles/world.css:181` hat weder `max-height` noch `overflow`. Der Inspektor scrollt nicht in sich, die Seite kann bei vielen Chips (Mauern, 24 Chips) wachsen. Die Messung „höchstens 300 px höher“ fehlt.

## Ziel

Der Inspektor des Welten-Editors scrollt in sich; die Seite wächst bei 24 Chips um höchstens 300 px.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Bei aufgeklappten Mauern (24 Chips) wächst die Seite um höchstens 300 px, der Inspektor scrollt in sich.
- **AC-02** Messung im Browser-Pane (Port 5196, `?mock=1`) in der Notiz festgehalten.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

2026-11-02 (Claude, WZ39): `.wd-stage-mode .wd-inspector { align-self: start; max-height: calc(100vh - 6rem); overflow-y: auto; }`. Messung im Browser-Pane, Port 5196, `?mock=1`, Viewport 1600×1000, Modus „Stufe bearbeiten“, Reiter Gebäude, Mauern aufgeklappt: 24 Chips, Seitenhöhe vorher 1000 px, nachher 1000 px (+0 px); Inspektor 902 px hoch, Inhalt 1162 px, scrollt in sich.
