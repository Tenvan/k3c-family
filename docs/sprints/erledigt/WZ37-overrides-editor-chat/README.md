# WZ37 · DEV · Welt-Overrides im Welten-Editor und im Studio-Chat

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-682, B-683
- **Start-Commit:** dc31130a
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-31, Ralf im Chat (Revision 1)

## Ausgangslage

Sobald Welten Werte übersteuern können (BAL8), fehlt die Bedienung im Welten-Editor (B-682) und im Studio-Chat (B-683).

## Ziel

Im Welten-Editor und im Chat lassen sich Overrides einer Welt setzen, zurücksetzen und gegen den Basiswert vergleichen. Am Ende sichtbar: Editor zeigt je Wert „geerbt“ oder „überschrieben“; Chat setzt einen Override als Aktionskarte.

## Beteiligte und Zielgruppen

Wer Welten pflegt; Agent DEV; 🧑 nimmt am PC ab.

## Anforderungen

`B-682 › Anforderungen`, `B-683 › Anforderungen`.

## Nicht-Ziele

Regeln und Simulation der Overrides (BAL8).

## Regeln und Einschränkungen

Beginnt erst nach BAL8; Oberfläche über `@webkit`; Studio schreibt `data/**`; Port 5196 für Tests.

## Beispiele

Basiswert 100, Override 150 → Editor zeigt beide, Zurücksetzen entfernt den Override.

## Ausnahme- und Fehlerfälle

Ungültiger Override → Editor lehnt ab, nennt den Pfad.

## Akzeptanzkriterien

- **AC-01** Der Editor übersteuert und setzt zurück, zeigt geerbt und überschrieben (`B-682/AC-01` bis `AC-03`).
- **AC-02** Der Chat liest, setzt und entfernt Overrides als Aktionskarte (`B-683/AC-01`, `AC-02`).

## Offene Fragen

keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ37.1 | `WZ37.1-editor-overrides.md` | Umsetzung | autonom | fertig |
| WZ37.2 | `WZ37.2-chat-overrides.md` | Umsetzung | autonom | fertig |
| WZ37.3 | `WZ37.3-review.md` | Review | autonom | fertig |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- WZ37.1 (DEV) Editor für Overrides (AC-01).
- WZ37.2 (DEV) Chat-Aktionen (AC-02).
- WZ37.3 Review, alle Kriterien, schließt ab.

## Abnahme

- 2026-11-02, Review WZ37.3: AC-01 und AC-02 belegt (Details in `WZ37.1` und `WZ37.2` › Ergebnis, Gegenprobe in `WZ37.3` › Ergebnis).
- Studio-Go-Tests und Vitest grün; ein Lastflake in `ChunkCount.test.tsx` (isoliert grün, nicht aus WZ37).
- Offen bei Ralf: B-682/AC-03 (Abnahme im Studio am PC). Keine Befunde, keine neuen Tickets.
