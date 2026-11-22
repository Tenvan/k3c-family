# tools/k3c-workbench/frontend/src/assets/game/

## Responsibility

Knopf „Im Spiel ansehen“ im Katalog (Entscheidung 005 › 5, B-847): legt den Testspielstand „voll ausgebaut“ an und
startet das native Spiel mit ihm.

## Design

- `GameOpen.tsx`: `GAME_SEED = 'ansicht'` (fester Seed des Testspielstands, D-04); `GameOpen` zeigt einen
  `ActionButton` mit Zustand `busy`/`error`/`info`. Der Spielstand liegt außerhalb des Repos, deshalb gilt der Knopf als
  Ansehen und nicht als schreibendes Control.
- Fehlertexte über `errorText` (`@webkit/lib/errors`).

## Flow

1. Klick -> `backend.studioGameOpen(GAME_SEED)` -> Go `App.StudioGameOpen` (`tools/k3c-workbench/app_studio.go`,
   Logik in `internal/studioview/game.go`) legt den Testspielstand `voll-ansicht` an und startet das native Spiel.
2. Die Antwort (Infotext) erscheint neben dem Knopf; ein Fehler rot statt Abbruch.

## Integration

- Konsument: `../catalog/CatalogPage.tsx`.
- Abhängigkeiten: `backend` aus `src/api`, `@webkit/atoms` (`ActionButton`, `Flex`, `Text`).
