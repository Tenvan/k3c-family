# B-850 · Es ist entschieden, ob k3c-server, engine/net, k3c-load und k3c-tui ohne Online-Spiel bleiben oder entfallen

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Teil von B-823 (Entscheidung 009, Punkt 2): Das Online-Spiel über `/ws` entfällt. Offen ist, ob `k3c-server` (Werkzeug-API, Berichte, Spielstände), `engine/net`, `k3c-load` und `k3c-tui` bleiben.

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

Vorschlag Claude: Server als Werkzeug-API behalten (Workbench, Berichte), `/ws`-Spielbetrieb, `k3c-load` und `k3c-tui` entfernen.

## Notizen

Zerlegung B-823 am 2026-11-13 (Claude).
