# B-201 · Der Spielstand stellt Plantage, Berufe, Krieger, Elite, Rüstung, Schwerter und Händler nach dem Laden wieder her

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

W2 bis W4 führen neue Zustände ein und ändern das Spielstand-Format bewusst nicht (W2.1, W2.3, W4.2, W4.3a, W4.3b › Kontext „Spielstand“). Heute speichert `TroopSave` (`engine/sim/save.go`, Zeile 53) nur `kind`, `x`, `anchorX`; beim Laden (Zeile 221–229) wird jede Truppe als Landstreicher erzeugt und nur `archer` und `peasant` befördert. Ein gespeicherter Krieger oder Elite-Kämpfer käme also als **Landstreicher** zurück, ein Bergmann als einfacher Bauer. `SiteSave` kennt nur `bows`/`bowPaidGold`, keine Schwerter, Elite- oder Rüstungs-Fortschritte. Plantage-Bäume haben keine Level-Position und fehlen in `nodesGone`/`nodesMarked` (`kind@x`), ein Händler und die Rüstungsstufe sind nirgends gespeichert.

## Ziel

Speichern und Laden verliert nichts, was Spieler in W2 bis W4 aufgebaut haben: Plantage-Bäume und ihr Wachstum, Berufe, Krieger und Elite, Rüstungsstufe, Schwerter im Regal und Fortschritte an Werkstatt, Schmiede und Rüstkammer, der anwesende Händler.

## Beteiligte und Zielgruppen

Spieler (Abbruch mitten im Spiel, Beschluss Q10: Speichern alle 60 s); Entwickler (SIM); 🧑 gibt die Spec frei.

## Anforderungen

- Neue Felder im Spielstand für die oben genannten Zustände, alte Stände laden weiter (fehlende Felder = Startzustand: kein Beruf, Rüstungsstufe 0, kein Händler, Plantage leer).
- Versionssprung nach `docs/arbeitsweise.md` › „Spielstand-Format ändern“: `IslandSaveVersion` erhöhen, Fixture `testdata/saves/v<n>/` aus dem Code erzeugen, Fall in `engine/sim/save_migration_test.go`, alte Fixtures unverändert.
- Deterministisch: Laden erzeugt bei gleichem Stand dieselbe Welt (keine Map-Iteration).

## Nicht-Ziele

Zustand des Wiederbelebens (W4.1: ein geladener Spieler steht lebendig an der Burg, bleibt so); Protokoll (W5); Formatänderungen von S1 und W1 (B-202).

## Regeln und Einschränkungen

Migrationsregel B-137 (`docs/arbeitsweise.md` › „Spielstand-Format ändern“), Golden nach „Golden aktualisieren“; Datei ≤ 400 Zeilen (`save.go`, `island_save.go`), Funktion ≤ 60. Version **4**: S1.4 und W1.3 teilen sich Version 3, B-201 folgt mit v4 (`testdata/saves/v4/`, Migration aus v3) (Beschluss Q42, 2026-10-04). Die Zuordnung der Plätze über `kind@x` muss wandernde Offsets abdecken (Hub wächst mit dem Ausbau, Beschluss Q26, 2026-10-04); das regelt die gemeinsame v3 (B-202).

## Beispiele

Stand mit 2 Kriegern, 1 Elite-Bogenschützen, 1 Bergmann, Rüstungsstufe 1 und 4 Plantage-Bäumen speichern → laden → dieselben Truppen mit Beruf und HP-Bonus, 4 Bäume an der Farm.

## Ausnahme- und Fehlerfälle

Gespeicherte Truppenart, die der Server nicht kennt → Ladefehler mit Meldung, keine stille Umwandlung in einen Landstreicher. Farm im Stand zerstört → Plantage-Bäume werden nicht hergestellt.

## Akzeptanzkriterien

- **AC-01** Rundlauf-Test: Speichern → Laden → Speichern ergibt denselben Stand für alle oben genannten Zustände mit 2 Spielern.
- **AC-02** Fixture der neuen Version liegt unter `testdata/saves/v<n>/`, `TestJedeVersionHatFixture` und `save_migration_test.go` sind grün; alle älteren Fixtures laden.
- **AC-03** Test: unbekannte Truppenart im Stand ergibt einen Fehler statt eines Landstreichers.
- **AC-04** `task check:go` grün.

## Offene Fragen

Zeitpunkt: direkt nach W4 oder gebündelt mit W5 (Protokoll)? Entscheidet 🧑 beim Planen.

## Notizen

Entstanden bei der Vorbereitung von W2 bis W4 (Ticket-Vorschläge in W2.1, W4.2 und W4.3 (jetzt W4.3a/W4.3b) › Kontext „Spielstand“).
