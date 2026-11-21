# WL1 · CLI, DEV, PAP · Welten im Paket, Gamepad-Hotplug, HUD-Aufräumen und Zäune/Mauern

- **Status:** aktiv
- **Projekt:** WIR
- **Domäne:** CLI, DEV, PAP
- **Reife:** bereit
- **Tickets:** B-1210, B-1211, B-1212, B-1213
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21

## Ausgangslage

Meldungen Ralf 2026-11-21: Welten fehlen im dist-Paket, Repo-Welten sollen beim Start aus dem Repo-dist Vorrang haben, Editor wählt nur die erste Welt, Gamepad-Hotplug, Beitreten-Meldung im HUD, Zaun und Mauern.

## Ziel

Welten im Paket und Dev-Vorrang der Repo-Welten, Editor mit allen Welten, Gamepad-Hotplug ohne Beitreten-Meldung, Jägerzaun und lückenlose, mit der Stufe wachsende Mauern.

## Beteiligte und Zielgruppen

Ralf (Abnahme am PC mit Pad); Hauptrechner (Lane erst: Welten, Editor, Gamepad, HUD), Second Worker (Lane zweit: Zaun/Mauer-Grafik).

## Anforderungen

B-1210, B-1211, B-1212, B-1213 › Anforderungen.

## Nicht-Ziele

Neue Spielregeln für Mauern; Rumble; andere Welt-Editor-Funktionen.

## Regeln und Einschränkungen

Entscheidungen 005 und 013; Assets nur über MCP.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-game/**`, `rust/crates/k3c-platform/**`, `tools/rustpaket/**`, `tools/k3c-workbench/**`, `public/grafik/**`, `data/paper.json`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme | Gerät
- **Sperre:** –

## Beispiele

Siehe Tickets.

## Ausnahme- und Fehlerfälle

Siehe Tickets.

## Akzeptanzkriterien

- **AC-01** (B-1210) Welten im Paket, Repo-Vorrang mit Dev-Symbol.
- **AC-02** (B-1211) Editor wählt alle Welten.
- **AC-03** (B-1212) Gamepad-Hotplug, keine Beitreten-Meldung.
- **AC-04** (B-1213) Zaun, lückenlose Mauern, Höhe nach Stufe.
- **AC-05** (B-1214) Spielstart Dev/Release als Workbench-Dienste mit Log.

## Offene Fragen

keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WL1.1 | `WL1.1-welten-paket-und-dev.md` | Umsetzung | autonom | offen |
| WL1.2 | `WL1.2-editor-welten-waehlbar.md` | Umsetzung | autonom | fertig |
| WL1.3 | `WL1.3-gamepad-hotplug-hud.md` | Umsetzung | autonom | fertig |
| WL1.4 | `WL1.4-zaun-mauern.md` | Umsetzung | autonom | offen |
| WL1.5 | `WL1.5-review.md` | Review | autonom | offen |
| WL1.6 | `WL1.6-workbench-spielstart-dienste.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
