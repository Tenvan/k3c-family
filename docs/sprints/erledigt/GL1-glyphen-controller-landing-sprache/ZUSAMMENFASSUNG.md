# GL1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Landingpage in der gewählten Sprache
- **Projekt:** BED

## Ziel

Die Landingpage folgt der gewählten Sprache, mit Rückfall Deutsch. Am Ende sichtbar: Nach dem Sprachwechsel auf English in den Optionen zeigt die Landingpage nach dem Neuladen englische Kacheln, Statuszeile, den Vollbild-Knopf und die Fußleiste.

## Ergebnis

- **2026-10-11, Review (GL1.2):** Kriterien: AC-02 `geprüft` (`platformTextRule.test.ts`, `task check` grün); AC-06 `geprüft` am PC (GL1.3, 🧑 „geprüft, alles ok“); AC-01, AC-03, AC-04, AC-05, AC-07 `entfällt (B-339 verworfen)`. - Befunde: keine schweren. Neue Tickets: keine. - Version: v0.15.0 von 🧑 bestätigt. Gesetzt wird der Tag erst, wenn PR #267 in `develop` gemergt und `main` per Fast-Forward nachgezogen ist (`arbeitsweise.md` › Branches, Release).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | – | Entfällt (war `B-339/AC-01`, B-339 verworfen). |
| AC-02 | geprüft | `src/landing/*.ts` und `index.html` enthalten kein deutsches Text-Literal mehr, die Texte … |
| AC-03 | – | Entfällt (war `B-339/AC-04`, B-339 verworfen). |
| AC-04 | – | Entfällt (war `B-339/AC-02`, B-339 verworfen). |
| AC-05 | – | Entfällt (war `B-339/AC-03`, B-339 verworfen). |
| AC-06 | geprüft | Nach Sprachwechsel in den Optionen zeigt die Landingpage nach dem Neuladen die gewählte Sp … |
| AC-07 | – | Entfällt (war `B-339/AC-05`, B-339 verworfen). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-369 | verworfen | AC-02 geprüft, AC-06 geprüft | Die Landingpage zeigt ihre Texte in der gewählten Sprache |
