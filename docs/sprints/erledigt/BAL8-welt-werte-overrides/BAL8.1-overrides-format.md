# BAL8.1 · Regeln und Datenformat der Welt-Overrides, Auflösung und Prüfung

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** REG
- **Umgebung:** offline
- **Branch:** bal8/1-overrides-format
- **Abhängig von:** –
- **Tickets:** B-680
- **Kriterien:** AC-01

## Ziel

`docs/rules/welten.md` beschreibt `overrides` und `extends` einer Welt; ein Go-Paket löst eine Welt zu einem Wertesatz auf (Basis + Eltern-Kette + Overrides) und lehnt Ungültiges mit Welt, Datei und Pfad ab.

## Kontext

Welt-Dateien: `data/worlds/<id>.json` (Typ in `engine/sim/island_data.go`), Basis-Werte `data/*.json`, geladen in `engine/sim/data.go` (Paket-Variablen buildings, troops, economy, hub, monarch, enemyData, waves, goals). Entscheidung von Ralf (2026-10-31): Eine Welt darf alle Basis-Werte übersteuern (Ressourcen, Kosten, Material-Vorrat, Gegner, Truppen, Gebäude, Monarch, Coop-Multiplikatoren) und mit `extends` von einer anderen Welt erben (Kette Basis → A → B, Tiefe höchstens 4, Zyklus, unbekannte Eltern-Welt und Löschen/Umbenennen einer Welt mit Kindern werden abgelehnt). Merge: Feld ersetzt Basiswert, Listen je Schlüssel/ID, nicht als Ganzes. Fehlt `overrides`, ist alles byte-gleich zu heute. Coop2/4 nur über Multiplikator-Felder.

## Erlaubte Dateien

- `docs/rules/welten.md`
- `engine/sim/world_values*.go` (neu) und Tests
- `engine/sim/island_data.go` (nur Felder `overrides`, `extends` am Welt-Typ)
- `engine/store/` nur, wenn das Löschen/Umbenennen einer Welt dort geschieht (Prüfung auf Kinder)

## Nicht-Ziele

Umbau der Sim auf Werte je Raum (BAL8.2), Editor und Chat (WZ37), neue Balancing-Werte, Protokoll- oder Spielstand-Änderung.

## Schritte

1. `docs/rules/welten.md` § 1 auf Erben/Übersteuern umschreiben, Format, Merge, Prüfung und Fehlermeldungen beschreiben (AC-01).
2. Typen `overrides`/`extends` am Welt-Typ, strikte Prüfung (unbekannte Datei/Feld, Typ, Wertebereich, fail closed).
3. Auflösung Basis → Eltern → Welt zu einem Wertesatz (reine Funktion, ohne globalen Zustand).
4. Go-Tests: ohne Overrides identisch zur Basis; Override ändert nur das Feld; ungültig mit Pfad abgelehnt; Kette, Zyklus, unbekannter Elternteil, Tiefe, Löschen mit Kindern (B-680/AC-02, AC-03, AC-05); Golden der Standard-Welt unverändert (AC-04).

## Fertig, wenn

- [ ] AC-01: `docs/rules/welten.md` beschreibt Format, Merge und Prüfung (B-680/AC-01); Bestätigung der Regeln durch Ralf folgt im Review.
- [ ] B-680/AC-02 bis AC-05 durch Go-Tests belegt.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 / B-680/AC-01 erfüllt: `docs/rules/welten.md` § 1 neu (Erben, `extends`, Kind-Welten, Solo-Basis/Coop nur über Multiplikatoren), § 1.1 Format und Merge, § 1.2 Prüfung und Fehlermeldungen, § 2 Felder `extends`/`overrides`; Bestätigung durch Ralf im Review offen.
- B-680/AC-02 geprüft: `TestWorldValuesOhneOverrides` (DeepEqual und JSON byte-gleich zur Basis, auch über `extends`), `TestWorldValuesOverrideAendertNurFeld` (nur genannte Felder, Liste je Index und je `id`, Basis unverändert).
- B-680/AC-03 geprüft: `TestWorldValuesUngueltig` (16 Fälle: Datei, Feld, Eintrag, Typ, Listenlänge, Schreibweise, Traits, Biom-Pool, Wellen, Presets; Meldung mit Welt, Datei, Pfad), `TestParseWorldExtendsOverrides`.
- B-680/AC-04 geprüft: `go test ./engine/sim -run TestGolden` grün, kein `task go:golden`.
- B-680/AC-05 geprüft: `TestWorldValuesKette`, `TestWorldValuesKetteAbgelehnt` (Zyklus, unbekannt, 5 Welten), `TestWorldRemovable` (Kinder in der Meldung).
- Abweichung: `WorldDef` liegt in `engine/sim/island_worlds.go` (nicht `island_data.go`), dort Felder und Lesen von `extends`/`overrides` ergänzt. `economyData` dupliziert den anonymen Typ von `economy` (Test sichert Gleichheit), BAL8.2 stellt `economy` darauf um.
- Offen: `CheckWorldRemovable` ist noch nicht im Löschen/Umbenennen des Studios verdrahtet (liegt in `tools/k3c-studio`, nicht erlaubt) → B-682; Laden eines Raums ruft `ResolveWorldValues` erst in BAL8.2. Geprüft: gezielte `go test ./engine/sim`, `./engine/net`, `./engine/internal/golden`, `go:lint` grün; kein `check:diff`, kein Commit.
