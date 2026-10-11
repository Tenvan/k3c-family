# WZ2 · DEV · Planungs-Werkzeuge: eindeutige Nummern, Fahrplan, Flags, Prüfläufe

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-387, B-360, B-366, B-363, B-372, B-367, B-341, B-400, B-401, B-402
- **Start-Commit:** 2b0d0891
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-10-11, 🧑 im Chat, Revision 3 (Revision 2 am selben Tag; B-401 Revision 2 nach Review M5)

## Ausgangslage

Zehn offene Tickets des Projekts WZG betreffen die Planungs-Werkzeuge von k3c-dev: Ticket-Nummern kollidieren zwischen Worktrees (B-387), `plan_set` meldet bei fehlendem Feld stillen Erfolg (B-360), der Fahrplan kennt die Spalte `Projekt` nicht (B-366), die Planungsseite kann Flags nicht ändern (B-363), `check_run` läuft bei langen Läufen in den Timeout (B-372), Doku und Glossar nennen alte Begriffe (B-367), die Header-Ursache aus Worktrees ist gemessen, aber nicht abgeschlossen (B-341). Neu in Revision 2 (🧑 2026-10-11): die Planungsseite merkt die Stellung des Splitters zwischen Backlog und Details (B-400), trennt den Backlog-Filter vom Projekt-Filter (B-401) und zeigt in den Session-Zeilen den Titel (B-402). Die Ablage bleibt wie sie ist (B-368 verworfen, 🧑 2026-10-11). Details stehen in den Tickets.

## Ziel

Die Planungs-Werkzeuge vergeben eindeutige Nummern, melden nie stillen Erfolg, pflegen den Fahrplan nach Projekt-Rang, bedienen Flags in der Planungsseite, halten lange Prüfläufe aus, und die Planungsseite merkt ihren Splitter, trennt die Filter für Backlog und Projekte und zeigt die Titel der Sessions.

**Am Ende sichtbar:** `plan_create` aus zwei Worktrees liefert verschiedene Ticket-Nummern; `check_run task:check:go` endet mit Exit-Code statt Timeout; die Planungsseite ändert Prio, Umgebung und Projekt eines Tickets, merkt die Stellung des Splitters, filtert Backlog und Projekte getrennt und zeigt in jeder Session-Zeile den Titel; neue Fahrplan-Zeilen tragen Projekt und stehen nach Rang.

## Beteiligte und Zielgruppen

Agenten und 🧑 beim Planen und Prüfen über k3c-dev. 🧑 gibt die Spec frei, klärt die offenen Fragen zu B-400 und B-401 und nimmt die Planungsseite am Bildschirm ab (B-363, B-400, B-401).

## Anforderungen

Alle Anforderungen stehen in den Ticket-Specs: `B-387 › Anforderungen`, `B-360 › Anforderungen`, `B-366 › Anforderungen`, `B-363 › Anforderungen`, `B-372 › Anforderungen`, `B-367 › Anforderungen`, `B-341 › Anforderungen`, `B-400 › Anforderungen`, `B-401 › Anforderungen`, `B-402 › Anforderungen`. Sprint-eigene Anforderung: Jede Session lässt `task check` (bzw. `check_run`) grün und ändert Golden-Daten nicht.

## Nicht-Ziele

Umzug der Planung auf die ErpApi-Ablage (B-368 verworfen); Seiten Dienste & Logs, MCP und Tasks (Branch `feat/workbench-seiten`); Testläufe mit Bots und Lasttest (Projekt TST); Go-Tests beschleunigen (B-372); Umnummerieren bestehender Tickets (B-387).

## Regeln und Einschränkungen

- Code nur in `tools/k3c-dev/` (Go und Frontend) sowie Doku/Glossar (B-367); Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.
- Die heutige Ablage und die Tools `plan_create`, `plan_set`, `plan_section`, `plan_delete` bleiben; Änderungen daran sind rückwärtskompatibel.
- Firewall-Regel: Proben nur auf Loopback.

## Beispiele

- Wurzel hat B-381, ein Worktree B-385 ungetrackt → `plan_create` legt B-388 an (`B-387/AC-01`).
- Neuer Sprint im Projekt GRA → Fahrplan-Zeile mit Projekt an der Rang-Stelle (`B-366/AC-01`).
- `check_run task:check:go` nach ~90 s → `exit 0 · 90 s` statt Timeout (`B-372/AC-01`).
- Splitter nach links gezogen, Seite neu geladen → die Breite bleibt (`B-400/AC-02`).
- Backlog nach Prio filtern → die Projekte bleiben unverändert (`B-401/AC-01`).

## Ausnahme- und Fehlerfälle

- Worktree nicht lesbar → übersprungen und genannt (`B-387/AC-02`).
- Feld fehlt in der Datei → kein stiller Erfolg (`B-360/AC-01`).
- k3c-dev läuft nicht → keine Live-Messung, B-341 bleibt offen.

## Akzeptanzkriterien

- **AC-01** Ticket-Nummern sind über Worktrees und Remote-Refs eindeutig (`B-387/AC-01`–`AC-04`).
- **AC-02** `plan_set` lehnt ein fehlendes Feld mit Grund ab und meldet keinen stillen Erfolg (`B-360/AC-01`).
- **AC-03** Fahrplan-Zeilen tragen Projekt und stehen nach Rang (`B-366/AC-01`, `AC-02`).
- **AC-04** Prio, Umgebung, Projekt (Ticket) sowie Agent, Umgebung (Session, nur offene) sind in der Planungsseite änderbar, abgelehnte Werte ändern nichts (`B-363/AC-01`–`AC-03`).
- **AC-05** `check_run` liefert bei Läufen > 60 s ein Ergebnis; `console_tail`-Doku stimmt; `plan_set` an Sprints ohne `Prio` und Session-IDs mit Buchstaben funktionieren (`B-372/AC-01`–`AC-04`).
- **AC-06** Alte Begriffe sind ersetzt, Glossar kennt „Ziel-Ticket“ und „ABN“ (`B-367/AC-01`, `AC-02`).
- **AC-07** Header-Ursache und Abhilfe stehen in B-341 (`B-341/AC-01`, `AC-02`).
- **AC-08** Die Sprint-/Backlog-Ansicht der Planungsseite hat einen Splitter zwischen Backlog und Details mit einem gemeinsamen, gemerkten Anteil; die Projekt-Ansicht hat keinen (`B-400/AC-01`, `AC-02`).
- **AC-09** Der Backlog-Filter (Suche, Prio, Umgebung, Projekt, Ticket-Schnellfilter) wirkt auf Backlog- und Sprint-Liste, der Projekt-Filter (Status, Domäne, Sprint-Schnellfilter) nur auf die Projekte; beide werden getrennt gemerkt (`B-401/AC-01`–`AC-04`).
- **AC-10** Session-Zeilen zeigen den Titel der Session aus ihrer Datei (`B-402/AC-01`, `AC-02`).

## Offene Fragen

Entschieden von 🧑 am 2026-10-11:

- B-360: Fehlendes Feld wird **abgelehnt**, nicht eingefügt.
- B-363: Agent/Umgebung nur bei **offenen** Sessions änderbar; **keine** Felder am Sprint.
- B-387: Remote-Refs (`origin/develop`, `origin/sprint/*`) zählen mit, aus dem lokalen Stand, ohne Fetch.
- B-367: Bleibt bei **WZG**, kein PRZ-Folgeprojekt.
- B-372: Lösungsweg **Hintergrundlauf mit Lauf-ID**; woher der Timeout kommt (MCP-Client oder k3c-dev), misst WZ2.4 und trägt es in B-372 ein.
- Domänen: Alle Sessions laufen unter **DEV**, wie der Sprint (Begründung unter Sessions).
- B-400, B-401, B-402: In den Sprint aufgenommen (Revision 2), Session WZ2.7.
- B-400: Splitter **nur in der Sprint-/Backlog-Ansicht**, **ein gemeinsamer** Anteil.
- B-401: **Projekt-Filter** Status und Domäne; **Backlog-Filter** Prio, Umgebung, Projekt und Suche; die **Sprint-Liste folgt dem Backlog-Filter**.
- B-402: **Nur Session-Titel**.

Offen: keine. Der dritte Punkt der Wunschliste vom 2026-10-11 war leer; kommt er, wird er ein eigenes Ticket.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ2.1 | `WZ2.1-ticket-nummern-feld-ablehnen.md` | Umsetzung | autonom | fertig |
| WZ2.2 | `WZ2.2-fahrplan-projekt.md` | Umsetzung | autonom | fertig |
| WZ2.3 | `WZ2.3-planungsseite-flags.md` | Umsetzung | autonom | fertig |
| WZ2.4 | `WZ2.4-check-run-hintergrund.md` | Umsetzung | autonom | fertig |
| WZ2.5 | `WZ2.5-begriffe-glossar.md` | Umsetzung | autonom | fertig |
| WZ2.6 | `WZ2.6-header-abschluss.md` | Umsetzung | autonom | fertig |
| WZ2.7 | `WZ2.7-planungsseite-splitter-filter.md` | Umsetzung | autonom | fertig |
| WZ2.8 | `WZ2.8-review.md` | Review | autonom | fertig |

Reihenfolge nach Abhängigkeit, je Session ein Commit. WZ2.1 B-387 + B-360, WZ2.2 B-366, WZ2.3 B-363, WZ2.4 B-372, WZ2.5 B-367, WZ2.6 B-341, WZ2.7 B-400 + B-401 + B-402, WZ2.8 Review.

Alle Sessions tragen die Domäne des Sprints (DEV), weil ein Sprint in seiner Domäne bleibt (`CLAUDE.md`, `docs/arbeitsweise.md`). Die Tickets B-372 (SRV) und B-367 (INF) behalten ihre eigene Domäne; ihr Code liegt ohnehin in `tools/k3c-dev/` bzw. ist reine Doku und Glossar der Planungs-Werkzeuge (🧑 2026-10-11).

## Abnahme

- 2026-10-11, Review WZ2.8 (unabhängiger Reviewer-Agent): AC-01, AC-03, AC-05 bis AC-07, AC-10 umgesetzt und geprüft; AC-02 und AC-04 nach Befund nachgebessert und getestet; AC-09 nach H1, M2, M3 nachgebessert.
- Offen für 🧑: Bildschirm-Abnahme der Planungsseite (AC-04, AC-08, AC-09, AC-10) und die Entscheidungen M1, M5 und Splitter-Breite in B-403; Herkunft des Timeouts (B-372 › Notizen) ungemessen.
- Behobene Befunde: H1, M2, M3, M4, B-360-Dateimeldung, Ticket-Test B-363, N2, N3, Glossar. Neue Tickets: B-403.
