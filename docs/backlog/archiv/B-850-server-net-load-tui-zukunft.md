# B-850 · Es ist entschieden, ob k3c-server, engine/net, k3c-load und k3c-tui ohne Online-Spiel bleiben oder entfallen

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Teil von B-823 (Entscheidung 009, Punkt 2): Das Online-Spiel über `/ws` entfällt. Offen ist, ob `k3c-server` (Werkzeug-API, Berichte, Spielstände), `engine/net`, `k3c-load`, `k3c-tui` und `k3c-starter` bleiben (Ralf 2026-11-14: „auch die anderen Apps prüfen, ob noch notwendig“). Der Web-Client der Workbench entfällt bereits (B-872).

## Ziel

🧑 hat entschieden; danach Folge-Tickets zum Entfernen oder Behalten.

## Beteiligte und Zielgruppen

🧑 entscheidet; Claude bereitet vor und plant.

## Anforderungen

- Liste der Nutzer jedes Teils (Workbench, Lasttest, TUI, CI) als Entscheidungsgrundlage.
- Entscheidung mit Datum, Folge-Tickets.

## Nicht-Ziele

Umsetzung.

## Regeln und Einschränkungen

Protokoll und Server: Klasse B.

- **Bereich:** `docs/backlog/**`
- **Abhängig von:** –
- **Braucht 🧑:** Entscheidung
- **Größe:** S
- **Sperre:** –

## Beispiele

Entscheidung „Server bleibt als Werkzeug-API, `/ws` und `k3c-load` entfallen“ → zwei Folge-Tickets.

## Ausnahme- und Fehlerfälle

nicht relevant: Entscheidung.

## Akzeptanzkriterien

- **AC-01** Entscheidung 🧑 in den Notizen, Folge-Tickets angelegt.

## Offene Fragen

Vorschlag Claude:
- `k3c-server`: als Werkzeug-API behalten, solange Workbench, Berichte und Spielstände ihn brauchen; `/ws`-Spielbetrieb entfernen.
- `k3c-load` (Lasttest): entfernen, ohne Online-Spiel gibt es keine Last.
- `k3c-tui`: entfernen, ersetzt durch Workbench und natives Spiel.
- `k3c-starter`: entfernen, er startet nur den Server für Browser-Geräte (B-041 dann verwerfen).
- Workbench-EXE und natives Spiel bleiben.

## Notizen

Entscheidung 🧑 Ralf, 2026-11-14: **Alles entfällt** – k3c-server, engine/net, k3c-load und k3c-tui werden entfernt (ohne Online-Spiel kein Bedarf). Umsetzung: neues Ticket für den Rückbau; B-846 und B-746 sind damit überholt und verworfen.
