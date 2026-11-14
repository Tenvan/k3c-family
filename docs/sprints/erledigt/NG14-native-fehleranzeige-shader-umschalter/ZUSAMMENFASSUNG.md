# NG14 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Natives Spiel: Fehler- und Ladeanzeige, Shader-Umschalter
- **Projekt:** NAT

## Ziel

Wer das Spiel startet, sieht Fehler und Ladevorgänge im Fenster; Fluss-Darstellung und Leistungsanzeige lassen sich im Spiel umschalten. Am Ende sichtbar: Tests in `game`, `world`, `view`, `local` grün, Notiz zu stderr der EXE in B-810, Abnahme 🧑 am PC (NG14.5).

## Ergebnis

2026-11-12 Review (Claude, `k3c-reviewer`): Kriterien laut Ergebnis der Sessions, Befunde behoben oder weitergegeben (siehe Review-Session). Version: keine vorgeschlagen. Sprint bleibt aktiv bis NG14.5 (Abnahme am PC, 🧑).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Fehlende Grafik liefert Fehlerzustand mit Text statt stillem Exit (`B-810/AC-01`). |
| AC-02 | – | Stufenwechsel setzt vor dem Laden „lädt“ und zeigt die Anzeige (`B-810/AC-02`). |
| AC-03 | geprüft | stderr der gebauten EXE geprüft, Ergebnis in B-810 › Notizen (`B-810/AC-03`). |
| AC-04 | – | Abnahme 🧑 am PC: Grafikordner umbenannt, Meldung gesehen (`B-810/AC-04`). |
| AC-05 | – | Umschalten ändert den Zustand und zeichnet im anderen Modus (`B-816/AC-01`). |
| AC-06 | – | Anzeige aus → keine Zeitnahme im Bildpfad (`B-816/AC-02`). |
| AC-07 | – | Anzeige an → Bildrate und Rechenzeit im Frame-Text (`B-816/AC-03`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-810 | erledigt | AC-01 –, AC-02 –, AC-03 geprüft, AC-04 – | Das native Spiel meldet Startfehler und fehlende Grafik im Fenster und zeigt bei … |
| B-816 | erledigt | AC-05 –, AC-06 –, AC-07 – | Fluss-Shader und Kacheln lassen sich im Spiel umschalten, Bildrate und Rechenzei … |
