# F2 · INF · Golden-Ablauf, Spielstand-Migration und Determinismus

- **Status:** geplant
- **Domäne:** INF
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-137, B-138, B-071
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Golden-Dateien in `testdata/golden/` haben keinen Task zum Aktualisieren (der Erzeuger `tests/golden.test.ts` existiert nicht mehr). Der Spielstand des Raums hat `IslandSaveVersion = 2` (Version 1 wird überführt), aber keine Fixtures und keine Migrationsregel, obwohl R2 bis R4 neue Felder bringen. Es gibt keine Prüfung gegen Map-Iteration in `engine/sim` und keinen Test für einen langsamen Raum-Takt; arm64 (Pi) wird in der CI nur gebaut, nicht getestet.

## Ziel

Drift der Golden-Daten, Datenverlust bei Spielständen und Abweichungen auf arm64 machen die CI rot. Am Ende sichtbar: `task golden:update` in `task --list`, Abschnitte in `docs/arbeitsweise.md`, ein grüner arm64-Job in der CI.

## Beteiligte und Zielgruppen

Entwickler und Agenten; 🧑 bestätigt künftig jedes Golden-Update (Q09) und entscheidet den arm64-Runner (B-071).

## Anforderungen

B-137, B-138 und B-071 › Anforderungen.

## Nicht-Ziele

Neue Spielstand-Felder (B-022), Replay-Format (B-159), Ereignisse (F3), Anpassung von Werten.

## Regeln und Einschränkungen

Domäne INF. **Domänen-Ausnahme (Freigabe dieser Spec erlaubt sie):** F2.2 und F2.3 dürfen Go-Testdateien in `engine/sim/` und `engine/room/` sowie Fixtures unter `testdata/saves/` anlegen; F2.1 darf Go-Testdateien in `engine/sim/`, `engine/level/` und `engine/internal/golden/` ändern. Produktionscode in `engine/` bleibt unverändert; was ein Test als Fehler aufdeckt, wird ein Ticket. Keine neue Abhängigkeit ohne Zustimmung von 🧑. Von diesem Rechner aus läuft kein arm64-Test.

## Beispiele

Wertänderung in `data/` ändert den Zustand eines Golden-Laufs → `task go:test` ist rot → `task golden:update` mit Begründung im Commit → Diff liegt vor. Spielstand von morgen (`version: 2`) auf altem Server → klare Meldung mit beiden Zahlen, Datei unverändert.

## Ausnahme- und Fehlerfälle

`rng.json` ist ein Referenzwert der Zufallsfolge und wird von `golden:update` nicht angefasst. Lässt sich eine der übrigen Dateien nicht aus dem Go-Lauf neu erzeugen, bleibt der Test rot und der Agent legt ein Ticket an (kein Umgehen).

## Akzeptanzkriterien

- **AC-01** `task --list` zeigt `golden:update`; der Lauf erzeugt `testdata/golden/` neu, danach ist `task go:test` grün (B-137/AC-01).
- **AC-02** `docs/arbeitsweise.md` enthält die Abschnitte „Golden aktualisieren“ und „Spielstand-Format ändern“ (B-137/AC-02).
- **AC-03** Ein Go-Test lädt je ein Fixture aus `testdata/saves/v1/` und `v2/` über `ParseIslandSave` (B-137/AC-03).
- **AC-04** Ein Spielstand mit neuerer Version ergibt eine Meldung mit beiden Zahlen und lässt die Quelldatei unverändert (B-137/AC-04).
- **AC-05** Ein Test verlangt zu jeder Version von 1 bis `IslandSaveVersion` ein Fixture-Verzeichnis (B-137/AC-05).
- **AC-06** Ein Check findet `range` über Maps in `engine/sim` und `engine/level`, Ausnahmen stehen mit Begründung in der Liste (B-138/AC-01, B-138/AC-03).
- **AC-07** Ein Test belegt gleichen Weltzustand nach 600 Ticks mit und ohne verlangsamten Takt (B-138/AC-02).
- **AC-08** Ein CI-Job führt die Go-Golden-Tests für arm64 aus und ist grün (B-071/AC-01).

## Offene Fragen

Runner für arm64: nativer arm64-Runner von GitHub oder Emulation (B-071, 🧑). Wer bestätigt Golden-Updates: `docs/fragenkatalog.md` Q09 (🧑). Beides blockiert die Freigabe.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| F2.1 | `F2.1-golden-update-arm64.md` | Umsetzung | autonom | offen |
| F2.2 | `F2.2-spielstand-migration.md` | Umsetzung | autonom | offen |
| F2.3 | `F2.3-determinismus.md` | Umsetzung | autonom | offen |
| F2.4 | `F2.4-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

–
