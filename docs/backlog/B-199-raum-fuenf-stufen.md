# B-199 · Ein neuer Raum legt die Insel mit allen Stufen an, für die es ein Biom gibt

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** SV1
- **Projekt:** WRT
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`engine/room/manager.go` › `open` erzeugt eine neue Insel fest mit `sim.CreateIsland(name, []int{0, 1, 2}, 1)` (Zeile 166) und lehnt eine Startstufe über 2 ab (`depth < 0 || depth > 2`, Zeile 163; Kommentar Zeile 150 „Stufen Tiefe 0, 1, 2“). Nach W2.2 (B-115) gibt es fünf Biome (`ironhold.json` Tiefe 3, `crystal.json` Tiefe 4), die Sim baut fünf Stufen, aber ein Raum sieht Eisenstollen und Kristallhöhle nicht. W2.2 darf `engine/room/` nicht ändern (SRV) und verweist auf dieses Ticket.

## Ziel

Jeder neue Raum spielt die ganze Insel mit allen vorhandenen Stufen; neue Biome brauchen keine Code-Änderung im Raum.

## Beteiligte und Zielgruppen

Spieler (erreichen Tiefe 3 und 4 im echten Spiel), Entwickler (SRV); 🧑 gibt die Spec frei.

## Anforderungen

- Die Tiefen der neuen Insel kommen aus den geladenen Biomen (alle Tiefen von 0 bis zur tiefsten lückenlos), nicht aus einer festen Liste im Raum.
- Die gültige Startstufe beim Anlegen folgt derselben Liste (Tiefe 0 bis tiefste).
- Geladene Spielstände behalten ihre gespeicherten Stufen.
- Deterministisch, 2+ Spieler in verschiedenen Stufen wie bisher.

## Nicht-Ziele

`data/islands.json`, mehrere Inseln und Inselwechsel (B-103, K2); Biome, Generator und Sim der neuen Stufen (B-115, W2.2); Protokoll-Felder (W5).

## Regeln und Einschränkungen

`CLAUDE.md` (Struktur `engine/room/`, Datei ≤ 400 Zeilen, Funktion ≤ 60), Domäne SRV, Schichtgrenzen aus `docs/arbeitsweise.md`. Kein Spielstand-Formatwechsel.

## Beispiele

Nach W2.2 einen neuen Raum anlegen → die Insel hat fünf Stufen; ein Spieler mit Startstufe 4 startet in der Kristallhöhle.

## Ausnahme- und Fehlerfälle

Startstufe tiefer als die tiefste vorhandene → Anlegen abgelehnt (`ErrBadRequest`) wie heute bei 3. Lücke in den Tiefen (z. B. Tiefe 3 fehlt) → Fehler beim Laden der Biome, nicht beim Spielen.

## Akzeptanzkriterien

- **AC-01** Test in `engine/room/`: Ein neu angelegter Raum hat so viele Stufen, wie es Biome gibt (nach W2.2: fünf).
- **AC-02** Test: Startstufe 4 wird angenommen, Startstufe 5 abgelehnt; ein gespeicherter Stand mit drei Stufen lädt mit drei.
- **AC-03** `task check:go` grün.

## Offene Fragen

keine

## Notizen

Entstanden bei der Vorbereitung von W2.2 (Kontext „Insel mit fünf Stufen“). `engine/net/level_test.go` (Zeile 63) prüft fest `{"forest", "cave", "mine"}`; das ist kein Fehler, solange es grün bleibt, kann aber mit diesem Ticket auf alle Biome erweitert werden.
