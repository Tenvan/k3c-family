# B-034 · Große PLAT-Dateien liegen unter 300 Zeilen

- **Domäne:** PLAT
- **Typ:** Schuld
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-30

## Beschreibung

`src/tools/gamepadTest.ts` (332), `src/tools/spriteReference.ts` (330), `src/landing/landing.ts` (319).

## Warum

Komplexitäts-Budget: die Ausnahmeliste soll schrumpfen.

## Akzeptanz

Alle drei ≤ 300 Zeilen, Einträge aus der Ausnahmeliste entfernt.

## Notizen

–
