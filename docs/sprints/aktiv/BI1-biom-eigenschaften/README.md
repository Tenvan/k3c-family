# BI1 · SIM, DEV, CLI · Biome mit Eigenschaften: Dichte, Chunk-Anteile und Varianten je Welt

- **Status:** aktiv
- **Projekt:** NAT
- **Domäne:** SIM, DEV, CLI
- **Reife:** bereit
- **Tickets:** –
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Ralf, 2026-11-14 (B-891 freigegeben, Erweiterung auf alle Biome)

## Ausgangslage

Die Biome legen die Natur nur über `resourcesPerChunk` und `chunkWeights` fest; zwei Welten mit anderem Seed sehen im selben Biom fast gleich aus (B-891).

## Ziel

Jedes Biom hat Eigenschaften (Dichte, Chunk-Anteile, Darstellung) und Gewichts-Gruppen (Baumarten, Kristallfarben), je Stufe aus dem Seed gewürfelt oder in der Welt fest gesetzt; Welten-Editor zeigt und setzt sie, das native Spiel zeigt die Baumart.

## Beteiligte und Zielgruppen

Ralf (🧑) entscheidet und nimmt den Welten-Editor ab; Spielende sehen abwechslungsreichere Welten.

## Anforderungen

Siehe B-891 › Anforderungen (Tabelle aller Eigenschaften, Datenmodell, Welten-Editor, Grafik).

## Nicht-Ziele

Balancing der Erträge (gesperrt); neue Biome; Darstellung der reinen Darstellungs-Eigenschaften und Kristallfarben (Folge-Tickets).

## Regeln und Einschränkungen

Determinismus nur über `engine/rng`. Golden-Änderungen über `task go:golden` mit Begründung im Commit.

- **Bereich:** `data/biomes/**`, `engine/level/**`, `engine/sim/island*.go`, `engine/sim/save.go`, `engine/sim/types.go`, `engine/sim/world.go`, `engine/sim/campaign.go`, `cmd/k3c-sim/world.go`, `tools/k3c-workbench/internal/world/**`, `tools/k3c-workbench/frontend/src/world/**`, `native/k3c-game/props/nature.go`, `native/k3c-game/props/plan.go`, `testdata/golden/**`, `docs/rules/welten.md`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** golden

## Beispiele

Welt setzt im Wald `treeSpecies` {tanne 0, pappel 3, birke 0} → nur Pappeln, in jedem Seed. Ohne Eintrag: Seed A ergibt Walddichte 5 und Mischwald, Seed B Walddichte 2 und fast nur Tannen.

## Ausnahme- und Fehlerfälle

Alle Varianten einer Gruppe fest auf 0 → Prüfung lehnt die Welt ab. Wert außerhalb des Bereichs oder unbekannte Eigenschaft → abgelehnt mit Namen. Alle Chunk-Gewichte würden 0 → die Gewichte des Bioms bleiben.

## Akzeptanzkriterien

- **AC-01** Gleicher Seed und gleiche Welt ergeben dieselben Werte und dieselbe Natur (Golden-Test, B-891/AC-01).
- **AC-02** Zwei Seeds ergeben im selben Biom messbar verschiedene Werte oder Mischung (Test, B-891/AC-02).
- **AC-03** Ein fest gesetzter Wert in der Welt gilt unabhängig vom Seed (Test, B-891/AC-03).
- **AC-04** Monokultur erzeugt nur diese Art; alle Gewichte 0 lehnt die Datenprüfung ab (B-891/AC-04).
- **AC-05** Der Welten-Editor zeigt und setzt die Werte; Sichtprüfung durch 🧑 (B-891/AC-05).
- **AC-06** Das native Spiel zeigt im Wald die Baumart aus dem Level.

## Offene Fragen

–

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BI1.1 | `BI1.1-datenmodell-generator.md` | Umsetzung | autonom | fertig |
| BI1.2 | `BI1.2-welten-editor-natur.md` | Umsetzung | autonom | fertig |
| BI1.3 | `BI1.3-baumart-darstellung.md` | Umsetzung | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
