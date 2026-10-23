# SK3.3 · Studio zeigt den Skill-Look des Spiels

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** sk3/3-studio-skill-ring
- **Abhängig von:** –
- **Tickets:** B-562
- **Kriterien:** AC-03

## Ziel

Die Monarch-Seite des Studios zeigt den Skill-Look des Spiels statt der Lücke.

## Kontext

B-562; `tools/k3c-studio/frontend/src/monarch/`.

## Erlaubte Dateien

- `tools/k3c-studio/frontend/src/monarch/`, `src/styles/world.css`

## Nicht-Ziele

Was in dieser Session ausdrücklich **nicht** passiert.

## Schritte

1. `skillRows` mit Look. 2. Farbfeld in der Liste.

## Fertig, wenn

- [x] AC-03.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go`, `task check:race` (nacheinander).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

AC-03 umgesetzt: `skillRows` liefert den Look aus dem Spiel, Liste zeigt Farbfeld und Bewegung; Studio-Vitest (248) und `go test` grün.
