# B-1136 · In-Game-Cheat-Menü (F9) entfernen, sobald das Cheat-Fenster abgenommen ist

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU56
- **Projekt:** RSU
- **Erstellt:** 2026-11-19
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Nach RU56 gibt es zwei Wege zu denselben Cheats: das F9-Menü im Bild (`rust/crates/k3c-game/src/devtools/cheatmenu.rs`, `cheatlist.rs`, `drawcheats.rs`) und das Cheat-Fenster. Zwei Wege kosten Pflege und Bildpfad-Code.

## Ziel

Das F9-Menü im Bild ist entfernt, das Cheat-Fenster ist der einzige Weg; Debug-Overlay (F10) und Tastenübersicht (F1) bleiben.

## Beteiligte und Zielgruppen

Umsetzung Claude (CLI) nach 🧑-Abnahme von RU56 (Freigabe und Zeitpunkt entscheidet 🧑).

## Anforderungen

- F9 und die Overlay-Rolle `DevOverlayRole` für Cheats, `cheatmenu.rs`, `cheatlist.rs` (die Listen leben im Fenster), `drawcheats.rs` und ihre Tests entfallen; `DevAction` und `DevSession` bleiben.
- Pad-Kombination View + Y (`ActionCheats`) entfällt oder öffnet das Fenster (Entscheidung 🧑).
- Nicht Teil der Sessions von RU56; wird erst nach der Abnahme von B-1132 bis B-1135 freigegeben.

## Nicht-Ziele

Debug-Overlay F10, Tastenübersicht F1.

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/src/devtools/**`, `rust/crates/k3c-game/src/view/devkeys.rs`, `rust/crates/k3c-game/src/input/**`
- **Abhängig von:** RU56
- **Braucht 🧑:** Freigabe, Entscheidung
- **Größe:** S
- **Sperre:** –

## Beispiele

F9 gedrückt → nichts mehr (oder Fenster öffnet sich).

## Ausnahme- und Fehlerfälle

Ohne `k3c-devwin.exe` (anderes Betriebssystem) gibt es keine Cheats mehr; akzeptiert, Windows ist das Ziel.

## Akzeptanzkriterien

- **AC-01** F9-Menü und Code entfernt, `task rust:lint` und `cargo test --workspace` grün.
- **AC-02** Alle Einträge des alten Menüs sind im Fenster vorhanden (Abgleich mit B-1134).

## Offene Fragen

Pad-Kombination View + Y entfernen oder auf das Fenster legen? 🧑 entscheidet bei der Freigabe; Default: entfernen.

## Notizen

Ticket aus dem Auftrag von Ralf (2026-11-19): in-game devtools-Menüs bleiben vorerst.
