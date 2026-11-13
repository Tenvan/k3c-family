# B-837 · Katalog-Vitests der Workbench bleiben unter Last grün

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ50
- **Projekt:** WZG
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

Im Review WZ42.4 war `task check:dev` in Lauf 2 und 3 rot, in Lauf 1 grün (gleicher Stand), jeweils nur in Katalog-Vitests von `tools/k3c-workbench/frontend/src/assets/catalog/`: Lauf 2 `CatalogCache.test.tsx` › „zeigt nur Assets im Diff“ und `CatalogRuhig.test.tsx:95` › „Frame-Player mit festen Breiten“ (`expect(before.every(Boolean)).toBe(true)`), Lauf 3 `CatalogRuhig.test.tsx` › „Befund 4: defekter Frame lässt das letzte Bild stehen“. Beide Dateien allein 5× grün. Vermutet (ungeprüft): Die Tests warten mit festen Zeiten (`wait(1200)`) bzw. prüfen vor dem ersten Render; unter Last (parallele Agenten, 81 Vitest-Worker) reicht das nicht.

## Ziel

`check:dev` wird durch die Katalog-Tests nicht mehr rot, wenn andere Agenten parallel prüfen.

## Beteiligte und Zielgruppen

Claude (Umsetzung); alle parallel arbeitenden Agenten.

## Anforderungen

- Warten auf Bedingung statt fester Zeit oder fester Runden (`flush()`); Aussage der Tests bleibt (Filter „nur geänderte“, feste Breiten, ein Laden je Frame, defekter Frame nur einmal).
- Gilt für alle Katalog-Tests im Bereich (`Catalog`, `CatalogCache`, `CatalogPrompts`, `CatalogRuhig`, `TeilSicht`), weil alle dieselbe Ursache haben.
- Kurze feste Zeiten nur noch für Gegenproben („nach dem Anhalten kommt nichts mehr“, höchstens 400 ms).

## Nicht-Ziele

Umbau des Katalogs (WZ43).

## Regeln und Einschränkungen

Domäne DEV, Klasse A (Problem, nur Tests und Test-Konfiguration, keine Spielregeln, kein Protokoll, keine neue Abhängigkeit).

- **Bereich:** `tools/k3c-workbench/frontend/src/assets/catalog/**`, `tools/k3c-workbench/frontend/vitest*.ts`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Beide Dateien 20× grün, während parallel `go test ./...` der Workbench läuft.

## Ausnahme- und Fehlerfälle

Ursache liegt im Katalog-Code (Breiten erst nach Daten) → Fix im Code mit Test.

## Akzeptanzkriterien

- **AC-01** `CatalogCache.test.tsx` und `CatalogRuhig.test.tsx` 20× grün unter paralleler Last, ohne längere feste Wartezeiten.

## Offene Fragen

keine

## Notizen

Gefunden im Review WZ42.4 (Log `check:dev` Lauf 2).
