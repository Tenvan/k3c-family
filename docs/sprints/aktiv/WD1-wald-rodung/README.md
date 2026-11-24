# WD1 · SIM, CLI · Wald: voller Baumwuchs, Lichtungen und Rodungsregeln

- **Status:** aktiv
- **Projekt:** WIR
- **Domäne:** SIM, CLI
- **Reife:** bereit
- **Tickets:** B-1204
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-21

## Ausgangslage

Die Oberfläche hat Bäume nur im Chunk-Typ `forest`, jeder Baum ist einzeln markierbar. Details und Annahmen: B-1204.

## Ziel

Dichter Wald auf der Oberfläche mit Hub-Streifen und Lichtungen; Roden nur von den Rändern der Waldstücke (äußere 5 je Seite, kleine Gruppen höchstens 5 zugleich). Am Ende sichtbar: grün schraffierte Waldstücke im Debug-Overlay, in der Workbench und im Cheat-Fenster.

## Beteiligte und Zielgruppen

Ralf (Abnahme am PC mit Pad), Hauptrechner (Lane erst).

## Anforderungen

B-1204 › Anforderungen (AC-01 bis AC-05).

## Nicht-Ziele

Balancing, Frame-Budget-Maßnahmen, neue Baum-Grafik, andere Biome (B-1204 › Nicht-Ziele).

## Regeln und Einschränkungen

Nur Rust (Entscheidung 013); Level und Daten mit Go geteilt, darum alles hinter `native_rules` oder Rust-only-Werte; Go-Golden unverändert.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `rust/crates/k3c-level/**`, `rust/crates/k3c-sim/**`, `rust/crates/k3c-data/**`, `rust/crates/k3c-game/src/devtools/**`, `rust/crates/k3c-game/src/props/**`, `rust/crates/k3c-devwin/**`, `rust/crates/k3c-devproto/**`, `tools/k3c-workbench/frontend/src/world/**`, `data/sizes.json`, `docs/rules/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** golden | spielstand

## Beispiele

Siehe B-1204 › Beispiele.

## Ausnahme- und Fehlerfälle

Siehe B-1204 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** (B-1204/AC-01, AC-02) Dichte, Hub-Streifen und Lichtungen.
- **AC-02** (B-1204/AC-03, AC-04) Waldstücke und Rodungsregel.
- **AC-03** (B-1204/AC-05) Anzeige.

## Offene Fragen

keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WD1.1 | `WD1.1-level-dichte-lichtungen.md` | Umsetzung | autonom | fertig |
| WD1.2 | `WD1.2-waldstuecke-rodung.md` | Umsetzung | autonom | fertig |
| WD1.3 | `WD1.3-anzeige-waldstuecke.md` | Umsetzung | autonom | fertig |
| WD1.4 | `WD1.4-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
