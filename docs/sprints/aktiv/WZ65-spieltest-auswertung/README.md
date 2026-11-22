# WZ65 · DEV · Spieltest-Auswertung: Transkript, Marker-Bilder, Befunde werden Tickets

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-1223, B-1224, B-1225
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Chat 2026-11-22

## Ausgangslage

Nach WZ64 und WZ66 liegen je Spieltest Sprache, Video, Marker mit Kontext, Spielstände und Fragebogen lokal vor. Ausgewertet wird noch nichts. ffmpeg 9.0.2 hat den whisper-Filter eingebaut.

## Ziel

Ralf löst im Nachgang per Knopf Transkript und Auswertung aus und bekommt aus angehakten Befunden Tickets, ohne Live-KI und ohne API-Key in der Workbench.

Am Ende sichtbar: Zeitleiste mit Transkript, Markern und Bildern; Befund-Liste mit Haken; neue Tickets mit Zitat, Bild und Spielkontext.

## Beteiligte und Zielgruppen

Ralf (🧑) löst aus und entscheidet; Claude wertet über MCP aus und legt Tickets an.

## Anforderungen

- B-1223 › Anforderungen (Modell laden, Transkribieren mit ffmpeg-whisper, Bild je Marker).
- B-1224 › Anforderungen (MCP `playtest_*`, Prompt-Knöpfe Auswerten und Tickets anlegen, Befunde mit Haken, Duplikat-Abgleich).
- B-1225 › Akzeptanzkriterien (Abnahme der ganzen Kette).

## Nicht-Ziele

- Live-Transkription oder Live-Auswertung.
- Cloud-Dienste, Python, Anthropic-API aus der Workbench (verworfen 🧑 2026-11-22).
- Auswertung über mehrere Sessions.

## Regeln und Einschränkungen

- Domäne DEV.
- Neue Abhängigkeit (whisper-Modell, ffmpeg ≥ 8): Freigabe durch 🧑.
- Tickets nur über `plan_create`, immer mit Projekt und Sprint.

- **Bereich:** `tools/k3c-workbench/internal/playtest/**`, `tools/k3c-workbench/app_playtest.go`, `tools/k3c-workbench/frontend/src/playtest/**`, `tools/k3c-workbench/internal/mcpsrv/tools_playtest.go`, `tools/k3c-workbench/internal/mcpsrv/tools.go`, `requirements.md`
- **Abhängig von:** WZ64, WZ66
- **Braucht 🧑:** Freigabe, Abnahme
- **Sperre:** –

## Beispiele

- B-1223 und B-1224 › Beispiele.

## Ausnahme- und Fehlerfälle

- B-1223 und B-1224 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Transkript mit Zeitstempeln und Bild je Marker per Knopf (B-1223/AC-01 bis AC-03).
- **AC-02** MCP-Tools, Prompt-Knöpfe, Befunde mit Haken und Duplikat-Abgleich (B-1224/AC-01 bis AC-03).
- **AC-03** `check_run task:check:diff` grün.
- **AC-04** Abnahme der Kette am TV (B-1225).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ65.1 | `WZ65.1-transkript-bilder.md` | Umsetzung | autonom | fertig |
| WZ65.2 | `WZ65.2-mcp-befunde-tickets.md` | Umsetzung | autonom | fertig |
| WZ65.3 | `WZ65.3-review.md` | Review | autonom | offen |

- WZ65.1 Modell laden, Transkribieren, Bilder je Marker (B-1223, AC-01)
- WZ65.2 MCP `playtest_*`, Prompt-Knöpfe, Befunde (B-1224, AC-02)
- WZ65.3 Review (AC-03); danach Abnahme-Ticket B-1225 mit 🧑 (AC-04)

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
