# B-774 · Abnahme-Läufe LG1

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** verworfen
- **Sprint:** LG1
- **Projekt:** LST
- **Erstellt:** 2026-11-10
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)

## Ausgangslage

Sprint LG1 (Remote-Logs, Texturbudget, Textur-Testseite) hat in Sessions weder Sim-, Balance- noch Gen-Läufe gefahren (B-742). Berührt wurden `engine/net` (lesende Dev-Endpunkte), `tools/k3c-workbench` (MCP-Log-Tools), `src/core` (Texturbudget im Render-Log) und `texturetest.html`.

## Ziel

Die Abnahme-Läufe, die LG1 braucht, laufen autonom (nachts) und sind grün.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

Nur der Race-Lauf ist für LG1 relevant: `engine/net` hat neue nebenläufige Zugriffe (Log-Slots `devLogSlots`, Dateileser).

## Nicht-Ziele

Keine neuen Funktionen, keine Sim- oder Balance-Tests.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Sim: nicht relevant: LG1 ändert weder `engine/sim` noch Raum-Takt oder Protokoll; die Dev-Endpunkte lesen nur Dateien.
- **AC-02** Balance: nicht relevant: keine Werte aus `data/` für Spielregeln geändert (`data/render.json` betrifft nur den Client).
- **AC-03** Gen: nicht relevant: keine Papierschnitt-Spec, kein Sheet, kein Golden berührt (`texturetest.html` ist eine Werkzeugseite).
- **AC-04** Race: `check_run task:check:race` grün (ohne C-Compiler übersprungen, im Ergebnis so nennen).

## Offene Fragen

keine

## Notizen

Angelegt von der Review-Session LG1.5 (2026-11-10).

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009).
