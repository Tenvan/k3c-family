# KB6.1 · Schutzplatz stabil bei Truppenwechsel

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** kb6/1-schutzplatz-stabil
- **Abhängig von:** –
- **Tickets:** B-605
- **Kriterien:** AC-02

## Ziel

Eine Truppe behält ihren Schutzplatz während einer Gefahr, auch wenn andere Truppen sterben oder hinzukommen; der Golden-Nachweis aus B-260/AC-01 liegt als Test vor.

## Kontext

`engine/sim/unit_common.go` › `shelterX` ordnet den Schutzplatz über den Index in `w.Troops` zu (B-260). Tod oder Zugang verschiebt Indizes, Bauern wechseln mitten in der Gefahr den Platz. Golden-Läufe: `engine/sim/sim_test.go` (`replay`), Kandidaten-Hub über `Options.hub`.

## Erlaubte Dateien

- `engine/sim/unit_common.go`, `engine/sim/types.go`
- `engine/sim/sim_test.go`, `engine/sim/unit_shelter_test.go`
- `testdata/golden/` (nur falls nötig, mit Begründung)

## Nicht-Ziele

Neue Schutzplatz-Regeln, andere Anzahl Plätze, Protokoll- oder Spielstand-Änderung.

## Schritte

1. Schutzplatz beim ersten Aufruf je Gefahr an der Truppe merken (`Troop.shelter`), bei Tag zurücksetzen.
2. Go-Test mit sterbender und neuer Truppe.
3. Go-Test: alle Golden-Läufe mit und ohne zusätzlichen unbezahlten Hub-Platz, Vergleich ohne IDs und ohne den neuen Platz.
4. Golden-Läufe unverändert grün.

## Fertig, wenn

- [x] AC-02 (B-605/AC-01): `TestSchutzplatzStabilBeiTruppenwechsel` grün.
- [x] AC-02 (B-605/AC-02): `TestGoldenZusatzplatzNurIDs` grün.
- [x] `TestGoldenLaeufe` grün ohne neue Golden-Dateien.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-02 geprüft: `TestSchutzplatzStabilBeiTruppenwechsel` (Bauer 2 stirbt, ein Bauer kommt vorn und einer hinten hinzu → übrige behalten den Platz; nach der Gefahr neu nach Index) und `TestGoldenZusatzplatzNurIDs` (alle 13 Golden-Läufe mit zusätzlichem unbezahltem Platz `armory` bei −20, Hub-Stufe 5: je Snapshot gleich bis auf ID-Felder, ID-Verweise wie `payKey` `site:N`, Ereignis-Felder `from`/`to`/`site`/`boss` und den neuen Platz) grün. Lösung: `Troop.shelter` (unexportiert, nicht im Spielstand; nach Laden mitten in der Gefahr neu nach Index). Golden unverändert: Die Läufe bleiben ohne `task go:golden` grün. Einschränkung: Der Zusatzplatz wirkt nur in der Sim (`Options.hub`); `engine/level` liest hub.json selbst, der Generator ist nicht Teil des Nachweises. `replay` nutzt jetzt `goldenWorld`.
