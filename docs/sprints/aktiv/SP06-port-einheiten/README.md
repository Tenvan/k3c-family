# SP06 · SIM · Port II – Gegner, Wellen, Reisen, Kampagne

- **Status:** aktiv
- **Domäne:** SIM
- **Reife:** bereit
- **Einschiebbar:** nein
- **Tickets:** B-043, B-074, B-059
- **Start-Commit:** 520e39d
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat (SP06: campaign-abstieg, Abdeckung ≥ 90 %, B-059 in SP06.2, Golden ≤ 8 MB)

## Ausgangslage

SP05 hat Welt, Tag/Nacht, eigene Truppen und Wirtschaft nach `engine/sim` portiert. Grün sind `sim-forest-tag` und
`sim-forest-ohne-spieler` vollständig, `sim-forest-nacht` und `sim-cave-aggression` bis zur ersten Welle. Es fehlen
Gegner, Wellen, Projektile, Burg-Fall, Reisen, Kampagne und Spielstand. Einige portierte Pfade deckt kein Golden-Lauf
ab (B-074). Golden-Läufe mit Stufenwechsel oder Spielstand gibt es noch nicht.

## Ziel

Die Go-Simulation kann alles, was die TS-Simulation kann, und jeder Pfad ist gegen TS geprüft. Am Ende sichtbar:
`go test -cover ./engine/sim/` ist grün, alle Golden-Läufe stimmen vollständig, auch Nacht, Stufenwechsel und
Spielstand, und die Abdeckung liegt bei mindestens 90 %.

## Beteiligte und Zielgruppen

Entwickler oder Agent; der Go-Server (SP07) baut darauf auf. 🧑 gibt die Spec frei.

## Anforderungen

B-043 › Anforderungen, B-074 › Anforderungen, B-059 › Anforderungen. Sprint-eigen:

- Gegner (`enemies.ts`), Wellen (`waves.ts`), `startWave`, Burg-Fall (`castleFallen`), `applyDamage` und
  `destroySite` (`common.ts`) in Go; `Step` und `stepCycle` sind danach vollständig.
- Reisen (`travel.ts`) und Kampagne mit Spielstand (`campaign.ts`: `createCampaign`, `joinPlayer`, `travel`,
  `toSave`, `fromSave`) in Go. Das Spielstand-Format ist ein Vertrag: `version` 1, dieselbe JSON-Form wie TS.
- Neuer Golden-Lauf `campaign-abstieg.json`: 2 Spieler steigen über die Tiefen-Eingänge von forest über cave nach
  mine ab. Unterwegs wird gespeichert und geladen, danach geht der Lauf auf der geladenen Kampagne weiter.
- Weitere Golden-Läufe, bis `go test -cover ./engine/sim/` mindestens 90 % erreicht (B-074).
- B-059: Ein Monarch mit `free: true` (vom Raum gesetzt, im JSON nur, wenn wahr) zählt nicht für den Stufenwechsel
  und reist mit. Das ist eine neue Go-Mechanik ohne TS-Gegenstück, sie wird mit eigenen Go-Tests geprüft.

## Nicht-Ziele

Räume, Netz, Protokoll (SP07); Setzen von `free` durch den Raum (SP07); Spielstand-Version 2; neue Mechaniken außer B-059.

## Regeln und Einschränkungen

- Wie SP05: `src/` wird nur gelesen; stabile Sortierung, `float64(…)` gegen FMA, keine `map`-Iteration mit Einfluss
  auf das Ergebnis, RNG-Reihenfolge exakt; Werte nur aus `data/`; keine neue Abhängigkeit; Schichtgrenzen.
- Ausnahme außerhalb der Domäne (INF): `tests/golden.test.ts` bekommt neue Läufe und den Kampagnen-Lauf. Bestehende
  Golden-Dateien ändern sich nicht.
- Golden-Daten insgesamt höchstens 8 MB. Für lange Läufe darf ein Lauf `snapshotEvery` 60 haben.

## Beispiele

- `sim-forest-nacht` → Go erzeugt alle 91 Snapshots identisch, inklusive Wellen, Pfeilen und toter Gegner.
- `campaign-abstieg` → bei jedem Stufenwechsel stimmen Tiefe, Welt und Spieler; der in Go geschriebene Spielstand
  ist derselbe wie der aus TS.
- Drei Monarchen, Nr. 1 ist frei und steht im Wald, 0 und 2 stehen am Tiefen-Eingang → Wechsel startet, alle drei
  stehen danach an der Burg der Zielstufe (B-059).

## Ausnahme- und Fehlerfälle

- Abweichung → der Test nennt Lauf, Tick und Feldpfad.
- Spielstand mit anderer `version` oder kaputtem Aufbau → `FromSave` liefert einen Fehler, keine halbe Kampagne.
- Kein Monarch gesteuert → kein Stufenwechsel (B-059/AC-02).

## Akzeptanzkriterien

- **AC-01** Gegner, Wellen, Kampf und Burg-Fall: `sim-forest-nacht` und `sim-cave-aggression` sind in allen Snapshots
  vollständig grün (B-043/AC-02 für diesen Teil).
- **AC-02** Reisen und Kampagne: `campaign-abstieg` ist in allen Snapshots grün, auch nach dem Laden (B-043/AC-02).
- **AC-03** Spielstand Version 1: Go liest den TS-Spielstand aus `campaign-abstieg.json` verlustfrei
  (`ToSave(FromSave(s))` = `s`), schreibt im Lauf denselben Spielstand wie TS und lehnt eine andere `version` mit
  Fehler ab.
- **AC-04** Freie Monarchen: B-059/AC-01 und B-059/AC-02 als Go-Tests grün; die Golden-Läufe bleiben unverändert.
- **AC-05** Abdeckung: `go test -cover ./engine/sim/` meldet mindestens 90 % der Anweisungen (B-074/AC-01).
  Jede Funktion unter 50 % ist im Ergebnis der Session begründet.

## Offene Fragen

keine. Geklärt von 🧑 (2026-10-01, Chat): B-059 gehört in SP06.2.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SP06.1 | `SP06.1-kampf.md` | Umsetzung | autonom | fertig |
| SP06.2 | `SP06.2-kampagne.md` | Umsetzung | autonom | fertig |
| SP06.3 | `SP06.3-abdeckung.md` | Umsetzung | autonom | fertig |
| SP06.4 | `SP06.4-review.md` | Review | autonom | offen |

## Abnahme

–
