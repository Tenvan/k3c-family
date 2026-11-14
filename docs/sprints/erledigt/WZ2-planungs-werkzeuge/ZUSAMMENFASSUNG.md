# WZ2 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Planungs-Werkzeuge: eindeutige Nummern, Fahrplan, Flags, Prüfläufe
- **Projekt:** WZG

## Ziel

Die Planungs-Werkzeuge vergeben eindeutige Nummern, melden nie stillen Erfolg, pflegen den Fahrplan nach Projekt-Rang, bedienen Flags in der Planungsseite, halten lange Prüfläufe aus, und die Planungsseite merkt ihren Splitter, trennt die Filter für Backlog und Projekte und zeigt die Titel der Sessions. **Am Ende sichtbar:** `plan_create` aus zwei Worktrees liefert verschiedene Ticket-Nummern; `ch …

## Ergebnis

- 2026-10-11, Review WZ2.8 (unabhängiger Reviewer-Agent): AC-01, AC-03, AC-05 bis AC-07, AC-10 umgesetzt und geprüft; AC-02 und AC-04 nach Befund nachgebessert und getestet; AC-09 nach H1, M2, M3 nachgebessert. - Offen für 🧑: Bildschirm-Abnahme der Planungsseite (AC-04, AC-08, AC-09, AC-10) und die Entscheidungen M1, M5 und Splitter-Breite in B-403; Herkunft des Timeouts (B-372 › Notizen) ungemessen. - Behobene Befunde: H1, M2, M3, M4, B-360-Dateimeldung, Ticket-Test B-363, N2, N3, Glossar. Neue Tickets: B-403.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Ticket-Nummern sind über Worktrees und Remote-Refs eindeutig (`B-387/AC-01`–`AC-04`). |
| AC-02 | – | `plan_set` lehnt ein fehlendes Feld mit Grund ab und meldet keinen stillen Erfolg (`B-360/ … |
| AC-03 | – | Fahrplan-Zeilen tragen Projekt und stehen nach Rang (`B-366/AC-01`, `AC-02`). |
| AC-04 | – | Prio, Umgebung, Projekt (Ticket) sowie Agent, Umgebung (Session, nur offene) sind in der P … |
| AC-05 | – | `check_run` liefert bei Läufen > 60 s ein Ergebnis; `console_tail`-Doku stimmt; `plan_set` … |
| AC-06 | – | Alte Begriffe sind ersetzt, Glossar kennt „Ziel-Ticket“ und „ABN“ (`B-367/AC-01`, `AC-02`) … |
| AC-07 | geprüft | Header-Ursache und Abhilfe stehen in B-341 (`B-341/AC-01`, `AC-02`). |
| AC-08 | verschoben | Die Sprint-/Backlog-Ansicht der Planungsseite hat einen Splitter zwischen Backlog und Deta … |
| AC-09 | – | Der Backlog-Filter (Suche, Prio, Umgebung, Projekt, Ticket-Schnellfilter) wirkt auf Backlo … |
| AC-10 | – | Session-Zeilen zeigen den Titel der Session aus ihrer Datei (`B-402/AC-01`, `AC-02`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-387 | erledigt | AC-01 – | Ticket-Nummern bleiben zwischen parallelen Worktrees eindeutig |
| B-360 | erledigt | AC-02 – | plan_set lehnt ein Feld ab, das in der Datei fehlt, statt es still zu übergehen |
| B-366 | erledigt | AC-03 – | k3c-workbench füllt im Fahrplan die Spalte Projekt und ordnet nach Rang |
| B-363 | eingeplant | AC-04 – | Die Planungsseite ändert Prio, Umgebung, Agent und Projekt direkt im Detail |
| B-372 | erledigt | AC-05 – | check_run go:test und task:check:go brechen im MCP-Aufruf mit Timeout ab |
| B-367 | erledigt | AC-06 – | Begriffe aus den alten Bahnen und Spuren sind nach PJ3 ersetzt und neue Projekt- … |
| B-341 | erledigt | AC-07 geprüft | Die Ursache für den fehlenden Header `X-K3C-Root` aus Worktrees ist live gemesse … |
| B-400 | eingeplant | AC-08 verschoben | Die Planungsseite trennt Backlog und Details mit einem Splitter, dessen Stellung … |
| B-401 | eingeplant | AC-09 – | Der Backlog-Filter wirkt nur auf den Backlog, der Projekt-Filter nur auf die Pro … |
| B-402 | eingeplant | AC-10 – | Die Planungsseite zeigt in den Session-Zeilen den Titel der Session |
