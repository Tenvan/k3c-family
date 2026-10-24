# KB2.4 · Client-Typen für Gegner- und Wellendaten passen zu den JSON-Dateien

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** offline
- **Branch:** kb2/4-client-typen-gegner
- **Abhängig von:** KB2.3
- **Tickets:** B-324
- **Kriterien:** AC-04

## Ziel

`src/model/data.ts` beschreibt Gegner- und Wellendaten so, wie sie in `data/enemies.json` und `data/waves.json` stehen, ohne `as unknown as`.

## Kontext

`src/model/data.ts` (`EnemyData`, `ENEMIES`, `WAVES`), `data/enemies.json`, `data/waves.json`. JSON-Importe kennen keine Tupel (`Range`) und keine Literal-Typen (`tier`).

## Erlaubte Dateien

- `src/model/data.ts`

## Nicht-Ziele

Anzeige der Traits (K5); andere Datentabellen in `data.ts`.

## Schritte

1. `EnemyData` um `attacksPerSecond` und die optionalen Felder ergänzen.
2. `WAVES` ohne `attacksPerSecond`, mit `perExtraPlayer`.
3. Prüfung der Form per `JsonOf<T>` statt `as unknown as`.

## Fertig, wenn

- [x] AC-04: `task typecheck` ist grün. `WAVES.attacksPerSecond` wäre jetzt ein Typfehler.

## Prüfen

```bash
task check
```

Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-04: umgesetzt. Geprüft mit `task typecheck`, `task lint` und `task test -- model`, alle grün.
- Der neue Typ `JsonOf<T>` prüft beim Typecheck, dass die JSON-Datei die Felder des Typs hat. Danach ist der Cast nur noch die Verengung auf Tupel und Literale. Die übrigen `as unknown as` in `data.ts` (Gebäude, Truppen, Ökonomie) bleiben, sie gehören nicht zu diesem Kriterium.
