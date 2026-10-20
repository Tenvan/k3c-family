# tools/k3c-studio/frontend/src/ui/

## Responsibility

Oberflächen-Bausteine nur des Asset-Studios nach Atomic Design (`molecules/`, `organisms/`, `templates/`). Atome und alles Generische liegen in `tools/webkit` (`@webkit/atoms` …); `tests/atomicDesign.test.ts` prüft die Ebenen.

## Design

- Derzeit leer: der frühere `Splitter.tsx` ist nach `@webkit/atoms` (`Splitter`) und `@webkit/lib/split` (`useSplit`, `splitShare`, `clampSplit`, `parseSplit`) gewandert.

## Flow

1. Ein Baustein, den nur das Studio braucht, kommt in die passende Ebene; er importiert nur nach unten, aus `api/`, `lib/` und webkit.

## Integration

- Konsumenten wären die Feature-Ordner (`assets/`, `world/` …); nie umgekehrt.
