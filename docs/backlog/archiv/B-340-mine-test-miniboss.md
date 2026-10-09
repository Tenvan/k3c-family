# B-340 · K2.1a darf den Mine-Test an den Miniboss anpassen

- **Domäne:** SIM
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** K2
- **Projekt:** –
- **Erstellt:** 2026-10-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Mit K2.1a erscheint in der Mine ab Welle 3 die Ratten-Königin (`engine/sim/boss.go`, `data/bosses.json`). `TestMineKupferBisInDenVorrat` (`engine/sim/mine_test.go`) lässt die Mine 300 s mit Aggressionspool laufen; dabei erreicht sie Welle 4, die Königin und ihre Beschwörungen bringen die Burg zu Fall (Vorrat halbiert, Kupfer 32 statt ≥ 10 + 2 Ader-Lieferungen). Der Test stammt aus der Zeit vor den Bossen. `mine_test.go` steht nicht in den Erlaubten Dateien von K2.1a; `task check:go` ist deshalb rot.

## Ziel

`task check:go` ist wieder grün, ohne die Boss-Regel aufzuweichen.

## Beteiligte und Zielgruppen

🧑 entscheidet; SIM setzt um.

## Anforderungen

- Der Mine-Test prüft weiter die Lieferkette Erz und Ader bis in den Vorrat, ohne Störung durch den Miniboss.

## Nicht-Ziele

Boss-Werte und Balancing (BR2, B-099).

## Regeln und Einschränkungen

Nur Test-Code; deterministisch. Erlaubte Dateien der Session K2.1a müssen um `engine/sim/mine_test.go` erweitert werden.

## Beispiele

Vorschlag: In `TestMineKupferBisInDenVorrat` nach `mustIsland` die Königin als besiegt vormerken (`isl.DefeatedBosses = []string{"ratQueen"}`, mit Kommentar). Damit läuft der Test wie vor K2.1a.

## Ausnahme- und Fehlerfälle

nicht relevant: reine Test-Anpassung.

## Akzeptanzkriterien

- **AC-01** `TestMineKupferBisInDenVorrat` und `task check:go` grün, `go test ./engine/sim -run Boss` grün.

## Offene Fragen

- Darf K2.1a `engine/sim/mine_test.go` wie vorgeschlagen ändern (Erlaubte Dateien erweitern)? Entscheidet 🧑.
  - **Entschieden** (🧑, 2026-10-07, Chat): „Test anpassen“. K2.1a ändert nur den Testaufbau von `mine_test.go`, die Prüfungen bleiben; die Boss-Stärke geht als Balancing-Hinweis an BR2 (B-156 › Notizen). Umsetzung: Die Ursache war der Goblin-Anführer auf Stufe 0 (Wald), nicht die Ratten-Königin; der Test merkt daher alle drei Minibosse als besiegt vor.

## Notizen

Gegenprobe: Auch mit einzelnen Ratten statt Schwärmen scheitert der Test; die Ursache ist der Boss an sich, nicht die Auslegung der Beschwörung.
