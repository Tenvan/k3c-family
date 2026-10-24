# KB3.2 · Kachel Spiel (Dev) mit ?dev=1 auf der Entwicklerseite

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** PLAT
- **Umgebung:** offline
- **Branch:** kb3/2-dev-kachel
- **Abhängig von:** –
- **Tickets:** B-564
- **Kriterien:** AC-02

## Ziel

Die Entwicklerseite öffnet das Spiel mit `?dev=1`.

## Kontext

B-564; `src/tools/devTiles.ts`.

## Erlaubte Dateien

- `src/tools/`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. Kachel. 2. Test.

## Fertig, wenn

- [x] AC-02.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-02 umgesetzt und geprüft: Kachel „Spiel (Dev)“ über `openPage()`, `devTiles.test.ts`, `tests/projectRules` grün.
