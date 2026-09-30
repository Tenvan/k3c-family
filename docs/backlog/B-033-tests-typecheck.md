# B-033 · Tests werden typgeprüft

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** SP01
- **Erstellt:** 2026-09-30

## Beschreibung

`tsconfig.json` schließt mit `include: ["src"]` den Ordner `tests/` aus.

## Warum

Typfehler in Tests fallen erst zur Laufzeit auf.

## Akzeptanz

`npm run typecheck` prüft auch `tests/`.

## Notizen

–
