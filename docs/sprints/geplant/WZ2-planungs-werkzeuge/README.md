# WZ2 · DEV · Planungs-Werkzeuge: eindeutige Nummern, Fahrplan, Flags, Prüfläufe

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-387, B-360, B-366, B-363, B-372, B-367, B-341
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Sieben offene Tickets des Projekts WZG betreffen die Planungs-Werkzeuge von k3c-dev: Ticket-Nummern kollidieren zwischen Worktrees (B-387), `plan_set` meldet bei fehlendem Feld stillen Erfolg (B-360), der Fahrplan kennt die Spalte `Projekt` nicht (B-366), die Planungsseite kann Flags nicht ändern (B-363), `check_run` läuft bei langen Läufen in den Timeout (B-372), Doku und Glossar nennen alte Begriffe (B-367), die Header-Ursache aus Worktrees ist gemessen, aber nicht abgeschlossen (B-341). Die Ablage bleibt wie sie ist (B-368 verworfen, 🧑 2026-10-11). Details stehen in den Tickets.

## Ziel

Die Planungs-Werkzeuge vergeben eindeutige Nummern, melden nie stillen Erfolg, pflegen den Fahrplan nach Projekt-Rang, bedienen Flags in der Planungsseite, und halten lange Prüfläufe aus.

**Am Ende sichtbar:** `plan_create` aus zwei Worktrees liefert verschiedene Ticket-Nummern; `check_run task:check:go` endet mit Exit-Code statt Timeout; die Planungsseite ändert Prio, Umgebung und Projekt eines Tickets; neue Fahrplan-Zeilen tragen Projekt und stehen nach Rang.

## Beteiligte und Zielgruppen

Agenten und 🧑 beim Planen und Prüfen über k3c-dev. 🧑 entscheidet die offenen Fragen (B-360, B-363), gibt die Spec frei und nimmt die Planungsseite am Bildschirm ab (B-363).

## Anforderungen

Alle Anforderungen stehen in den Ticket-Specs: `B-387 › Anforderungen`, `B-360 › Anforderungen`, `B-366 › Anforderungen`, `B-363 › Anforderungen`, `B-372 › Anforderungen`, `B-367 › Anforderungen`, `B-341 › Anforderungen`. Sprint-eigene Anforderung: Jede Session lässt `task check` (bzw. `check_run`) grün und ändert Golden-Daten nicht.

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

## Offene Fragen

Entschieden von 🧑 am 2026-10-11:

- B-360: Fehlendes Feld wird **abgelehnt**, nicht eingefügt.
- B-363: Agent/Umgebung nur bei **offenen** Sessions änderbar; **keine** Felder am Sprint.
- B-387: Remote-Refs (`origin/develop`, `origin/sprint/*`) zählen mit, aus dem lokalen Stand, ohne Fetch.
- B-367: Bleibt bei **WZG**, kein PRZ-Folgeprojekt.
- B-372: Lösungsweg **Hintergrundlauf mit Lauf-ID**; woher der Timeout kommt (MCP-Client oder k3c-dev), misst WZ2.4 und trägt es in B-372 ein.
- Domänen: WZ2.4 und WZ2.5 laufen unter **DEV**, wie der Sprint (Begründung unter Sessions).

Offen: keine.

## Sessions

Reife `Entwurf`: Stichpunkte statt Session-Dateien (Kriterien in Klammern). Reihenfolge nach Abhängigkeit, je Session ein Commit.

Alle Sessions tragen die Domäne des Sprints (DEV), weil ein Sprint in seiner Domäne bleibt (`CLAUDE.md`, `docs/arbeitsweise.md`). Die Tickets B-372 (SRV) und B-367 (INF) behalten ihre eigene Domäne; ihr Code liegt ohnehin in `tools/k3c-dev/` bzw. ist reine Doku und Glossar der Planungs-Werkzeuge (🧑 2026-10-11).

| Nr. | Inhalt | Domäne | Typ | Agent | Status |
|---|---|---|---|---|---|
| WZ2.1 | B-387 Ticket-Nummern über alle Worktrees und Remote-Refs, B-360 stiller Erfolg bei `plan_set` (AC-01, AC-02) | DEV | Umsetzung | autonom | offen |
| WZ2.2 | B-366 Fahrplan: Spalte Projekt, Einsortieren nach Rang (AC-03) | DEV | Umsetzung | autonom | offen |
| WZ2.3 | B-363 Planungsseite: Flags bearbeiten (AC-04) | DEV | Umsetzung | autonom | offen |
| WZ2.4 | B-372 `check_run` Hintergrundlauf, `console_tail`-Doku, `plan_set` Sprint ohne Prio, Session-IDs mit Buchstaben (AC-05) | DEV | Umsetzung | autonom | offen |
| WZ2.5 | B-367 Begriffe und Glossar (AC-06) | DEV | Umsetzung | autonom | offen |
| WZ2.6 | B-341 Abschluss: AC-01/AC-02 gegen die Notizen prüfen, Ergebnis eintragen (AC-07) | DEV | Umsetzung | autonom | offen |
| WZ2.7 | Review gegen alle Kriterien | DEV | Review | autonom | offen |

Vor dem Aktivieren wird jede Session als eigene Datei nach `docs/vorlagen/session.md` angelegt (`plan_create`).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
