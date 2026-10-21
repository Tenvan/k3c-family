# WZ24 · REG · Geplante Chunks in Simulation und Spielstand: Regeln geklärt, Planung im Stand, eine Prüfung der Inselgröße

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** REG
- **Reife:** Entwurf
- **Tickets:** B-475, B-489, B-490, B-479, B-497
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Zwei Stellen von `docs/rules/geplante-chunks.md` sind mehrdeutig, WZ8.2 hat nach Annahmen entschieden (B-475). Spielstände bauen ihre Stufen beim Laden mit Planung und Chunk-Anzahl aus den **aktuellen** Daten (`savedIslandOptions`, `engine/sim/island_data.go`); eine spätere Datenänderung verschiebt alte Stände (B-489). Die Prüfung der Inselgröße steht dreimal, die Vorschau lehnt Chunk-Anzahl mit Kandidaten-`hub.json` ab (B-490). `sim.ValidateHub` prüft die geplanten Chunks nicht mit (`NotePlannedChunksUnchecked`, B-479). Für große Inseln fehlen Lastmessung, Umwandlungstests in allen Biomen und ein Schutz von `PlanSave` gegen parallele Aufrufe (B-497).

## Ziel

Geplante Chunks haben eindeutige Regeln und verhalten sich in Prüfung, Vorschau und Spielstand gleich: Ein alter Stand lädt so, wie er gebaut wurde. Am Ende sichtbar: Regeltext ohne Mehrdeutigkeit, `task check:go` grün mit Tests für Laden alter Stände nach Datenänderung, kein Hinweis „geplante Chunks nicht geprüft“ mehr.

## Beteiligte und Zielgruppen

🧑 entscheidet die Regelfragen im Workshop; Agent in REG und SIM; Spielende profitieren von stabilen Spielständen.

## Anforderungen

B-475, B-489, B-490, B-479 › Ziel bzw. Ausgangslage; B-497 Punkte (2) und (3), Punkt (1) nur, wenn 🧑 die Lastmessung hier haben will. Specs noch Entwurf.

## Nicht-Ziele

Ruinen und Gegnerlager (WZ14, B-468, B-517); neue Inselgrößen; Migration alter Spielstände (Beschluss B-288).

## Regeln und Einschränkungen

Reihenfolge REG → SIM → SRV → DEV. **Abhängigkeit:** nach WZ14 (beide ändern `docs/rules/geplante-chunks.md` und `engine/level/planned*.go`). Spielstand-Format darf steigen, ohne Migration (`docs/arbeitsweise.md` › Spielstand-Format ändern). Deterministisch (`engine/rng`); Golden nur bei gewollter Änderung mit Begründung. Testläufe nur über `sim_test`.

## Beispiele

Stand gespeichert mit 24 Chunks, danach `data/islands.json` auf 40 Chunks geändert → Laden baut die Stufe mit 24 Chunks und demselben Haupt-Strom. Vorschau mit Kandidaten-`hub.json` und Chunk-Anzahl 40 → angenommen.

## Ausnahme- und Fehlerfälle

Stand ohne gespeicherte Chunk-Anzahl → 0 = Biom-Länge, wie alte Stände gebaut wurden. Ungültige Inselgröße → eine Meldung aus einer Funktion (`sim.CheckChunkCount`).

## Akzeptanzkriterien

- **AC-01** 🧑 hat beide Fragen aus B-475 entschieden; Regeltext § 3 und § 5 und `engine/level/planned*.go` stimmen überein (B-475).
- **AC-02** Ein Spielstand lädt seine Stufen mit der Planung und Chunk-Anzahl, mit der sie gebaut wurden (Go-Test mit geänderten Daten, B-489).
- **AC-03** Die Prüfung der Inselgröße lebt in einer exportierten Funktion in `engine/sim`, die Vorschau (`engine/room`) und das Studio nutzen sie; die Vorschau nimmt die Chunk-Anzahl auch mit Kandidaten-`hub.json` (B-490).
- **AC-04** `sim.ValidateHub` prüft die geplanten Chunks gegen die Kandidaten-Linien mit, der Hinweis `NotePlannedChunksUnchecked` entfällt (B-479).
- **AC-05** Umwandlung eines Seed-Chunks ist in allen Biomen und Stufen 0 bis 4 getestet (B-497 Punkt 2).
- **AC-06** `PlanSave` im Studio ist gegen parallele Aufrufe geschützt (B-497 Punkt 3).

## Offene Fragen

🧑: B-475 (1) und (2), Vorschlag von Claude (ungeprüft): wie umgesetzt, Regeltext anpassen. B-489: Planung als Kopie oder Hash im Stand? B-497 (1) Lastmessung großer Inseln hier oder in einem eigenen Größen-Projekt?

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- WZ24.1 (REG, Mensch) Workshop: Fragen aus B-475 und B-497 entscheiden, Regeltext angleichen (AC-01).
- WZ24.2 (SIM) Spielstand hält Planung und Chunk-Anzahl (AC-02).
- WZ24.3 (SIM) Exportierte Prüfung der Inselgröße, geplante Chunks in `ValidateHub`, Umwandlung in allen Biomen (AC-01, AC-03, AC-04, AC-05).
- WZ24.4 (SRV) Vorschau nimmt Chunk-Anzahl mit Kandidaten-`hub.json`, nutzt die Prüfung aus `sim` (AC-03).
- WZ24.5 (DEV) Studio nutzt die Prüfung aus `sim`, `PlanSave` geschützt (AC-03, AC-06).
- WZ24.6 (DEV) Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
