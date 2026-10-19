# tools/k3c-studio/frontend/src/ui/

## Responsibility

Oberflächen-Bausteine nur des Asset-Studios; die gemeinsamen Radix-Bausteine liegen in `tools/webkit` (`@webkit/parts`).

## Design

- `Splitter.tsx`: ziehbare Teilung zweier Bereiche, Anteil 20–80 % gemerkt über `../lib/prefs` (`loadText`, `savePref`); reine Helfer `splitShare`, `clampSplit`, `parseSplit`. Gleiche Logik wie `tasks/TaskBars.tsx` in k3c-dev.

## Flow

1. Seite rendert `Splitter` mit Schlüssel und Vorgabe → gemerkter Anteil → Ziehen setzt und merkt den neuen Anteil.

## Integration

- Konsumenten: `assets/AssetsPage.tsx`, `assets/SlotDetail.tsx`.
