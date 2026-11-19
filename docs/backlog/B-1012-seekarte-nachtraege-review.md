# B-1012 · Seekarten-Nachträge aus dem Review: Reihenfolge von completed abgesichert, gehaltenes A nach der Fahrt, Stufen der alten Insel im Wechsel-Frame

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** RU60
- **Projekt:** RSU
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Review GS2.4 (2026-11-15) ließ vier Punkte offen, die außerhalb der erlaubten Dateien lagen: (1) Die Seekarte nimmt den letzten Eintrag von `islandSwitch.completed` als aktuelle Insel; diese Reihenfolge aus `engine/sim/world_state_mirror.go` ist ungetestet. (2) Vermutet: Ein über die Schiffsfahrt gehaltenes A wirkt im ersten Takt auf der Zielinsel. (3) `native/k3c-game/local/step.go` führt im Frame des Inselwechsels einen Takt lang Stufen der alten Insel. (4) `hud/seekarte_look.go` doppelt die Chunk-Grenzen 16/100 aus `data/island-sizes.json`; kein Test prüft die Namen aus `data/paper.json` › `seekarte` gegen den Index der Gruppe objekte.

## Ziel

Die stillen Abhängigkeiten der Seekarte sind abgesichert, die Doppel entfernt.

## Beteiligte und Zielgruppen

Entwickler; Spieler (Punkt 2).

## Anforderungen

- Test in `engine/sim`: `completed` führt die aktuelle Insel als letzten Eintrag; Vertrag in `docs/protocol.md` genannt.
- Punkt 2 am Rechner nachstellen; trifft er zu, bleibt die Eingabe bis zum Loslassen verbraucht (wie B-1006).
- Der Frame des Inselwechsels führt keine Stufen der alten Insel mehr.
- Chunk-Grenzen aus `data/island-sizes.json`; Test der Seekarten-Namen gegen den Index.

## Nicht-Ziele

Änderung an Regeln oder Spielstand.

## Regeln und Einschränkungen

- Umgehängt auf Rust (B-1143, 2026-11-19): Die Pendants der Go-Dateien liegen in `rust/crates/k3c-sim`, `rust/crates/k3c-game/src/local/step.rs`, `game/voyage.rs`, `hud/` und `art/`; die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-sim/**`, `docs/protocol.md`, `rust/crates/k3c-game/src/local/**`, `rust/crates/k3c-game/src/game/**`, `rust/crates/k3c-game/src/hud/**`, `rust/crates/k3c-game/src/art/**`
- **Abhängig von:** GS2.4
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –

## Beispiele

Hafen in Stufe 1, Wechsel zur nächsten Insel → der erste Frame am Ziel zeigt nur Stufen der neuen Insel.

## Ausnahme- und Fehlerfälle

Punkt 2 lässt sich nicht nachstellen → im Ergebnis festhalten, kein Code.

## Akzeptanzkriterien

- **AC-01** Sim-Test zur Reihenfolge von `completed` grün, `protocol.md` nennt den Vertrag.
- **AC-02** Test: gehaltenes A über die Fahrt löst am Ziel nichts aus (oder Befund: nicht nachstellbar).
- **AC-03** Test: Frame des Wechsels ohne Stufen der alten Insel.
- **AC-04** Chunk-Grenzen aus den Daten; Namens-Test grün.

## Offene Fragen

keine

## Notizen

Folgetickets 1 bis 4 aus dem Bericht GS2.4.
