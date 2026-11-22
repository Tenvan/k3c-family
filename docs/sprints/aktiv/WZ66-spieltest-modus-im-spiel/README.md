# WZ66 · CLI · Spieltest-Modus im Spiel: REC, Pad-Marker mit Spielkontext, Ruckler-Marker, Fragebogen

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** CLI
- **Reife:** bereit
- **Tickets:** B-1220, B-1221, B-1222
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, Chat 2026-11-22

## Ausgangslage

Das Rust-Spiel hat keine Verbindung zur Workbench außer der Pipe zum Cheat-Fenster und kennt keinen Spieltest-Modus. Die Aufnahme in WZ64 braucht Marker vom Sofa und ein sichtbares Aufnahme-Zeichen.

## Ziel

Mit `--playtest <ordner>` zeigt das Spiel REC, Spieler setzen Marker per Pad, Ruckler markieren sich selbst, jeder Pad-Marker hat einen Spielstand, und am Ende fragt ein Kurz-Fragebogen ab.

Am Ende sichtbar: REC-Punkt am TV, „Markiert“ nach Back+Y, Fragebogen beim Beenden; im Ordner `markers.jsonl`, `saves/`, `survey.json`.

## Beteiligte und Zielgruppen

Spieler am Sofa (bis 2 Pads), Ralf (🧑) bei Auswertung und Abnahme; Claude setzt um.

## Anforderungen

- B-1220 › Anforderungen (Flag, REC, Pad-Marker mit Kontext).
- B-1221 › Anforderungen (Ruckler-Automarker, Spielstand am Marker).
- B-1222 › Anforderungen (Fragebogen am Spielende).

## Nicht-Ziele

- Ton- oder Bildaufnahme im Spiel (WZ64).
- Neue Mechanik, Änderung an Simulation oder Spielstand-Format.
- Neues Protokoll zur Workbench: Austausch nur über Dateien im Ordner.

## Regeln und Einschränkungen

- Domäne CLI (`rust/crates/k3c-game`); Logik im Modul `playtest/`, das Spiel zeichnet nur.
- B bleibt zurück im Menü, im Spiel unbelegt; alles mit 2 Spielern.
- Frame-Budget 60 Hz mit zwei Ansichten.
- Ordner-Vertrag: B-1218 › Anforderungen.

- **Bereich:** `rust/crates/k3c-game/src/playtest/**`, `rust/crates/k3c-game/src/main.rs`, `rust/crates/k3c-game/src/input/**`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/menu/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Sperre:** –

## Beispiele

- B-1220, B-1221, B-1222 › Beispiele.

## Ausnahme- und Fehlerfälle

- B-1220, B-1221, B-1222 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Flag, REC und Pad-Marker mit Kontext (B-1220/AC-01 bis AC-03).
- **AC-02** Ruckler-Automarker und Spielstand am Marker, ohne messbaren Frame-Verlust (B-1221/AC-01 bis AC-03).
- **AC-03** Fragebogen mit 2 Pads (B-1222/AC-01 bis AC-03).
- **AC-04** `check_run task:check:diff` grün.

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ66.1 | `WZ66.1-flag-rec-pad-marker.md` | Umsetzung | autonom | fertig |
| WZ66.2 | `WZ66.2-ruckler-marker-spielstand.md` | Umsetzung | autonom | fertig |
| WZ66.3 | `WZ66.3-fragebogen-spielende.md` | Umsetzung | autonom | offen |
| WZ66.4 | `WZ66.4-review.md` | Review | autonom | offen |

- WZ66.1 Flag, REC, Pad-Marker (B-1220, AC-01)
- WZ66.2 Ruckler-Automarker und Spielstand am Marker (B-1221, AC-02)
- WZ66.3 Fragebogen am Spielende (B-1222, AC-03)
- WZ66.4 Review (AC-04)

Ralf prüft im Spiel: Back+Y bequem, REC stört nicht, Fragebogen lesbar am TV.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
