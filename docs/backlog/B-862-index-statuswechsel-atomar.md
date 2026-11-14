# B-862 · Ein Statuswechsel schreibt Haupt- und Archiv-Index so, dass keine Zeile verloren geht

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ54
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14

## Ausgangslage

WZ47.4: `changeSet.apply` schreibt Haupt- und Archiv-Index nacheinander. Scheitert der zweite Schreibvorgang, fehlt die Zeile in beiden Dateien.

## Ziel

Ein Statuswechsel verliert nie eine Index-Zeile: erst eintragen, dann entfernen, oder Rückrollen bei Fehler.

## Beteiligte und Zielgruppen

Claude (Umsetzung, Klasse A).

## Anforderungen

- Feste Reihenfolge oder Rollback in `changeSet.apply`.
- Test mit erzwungenem Schreibfehler.

## Nicht-Ziele

Andere Schreibwege.

## Regeln und Einschränkungen

Klasse A (Schuld, DEV).

- **Bereich:** `tools/k3c-workbench/internal/planning/**`
- **Abhängig von:** WZ47
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

Zweites Schreiben scheitert → Zeile steht noch im alten Index.

## Ausnahme- und Fehlerfälle

Beide scheitern → Fehler, kein Zustand geändert.

## Akzeptanzkriterien

- **AC-01** Test: Schreibfehler beim zweiten Index lässt die Zeile in genau einem Index.

## Offene Fragen

keine

## Notizen

Befund WZ47.4 (2026-11-14).
