# F0 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Parallele Sprints je Domäne
- **Projekt:** –

## Ziel

Je Domäne darf ein Sprint aktiv sein, und Sessions werden per Branch auf `origin` beansprucht. Am Ende sichtbar: `task check` grün mit zwei aktiven Sprints verschiedener Domänen, `docs/arbeitsweise.md` mit der neuen Regel.

## Ergebnis

2026-10-03: AC-01 bis AC-03 geprüft in F0.1 (Durchsicht `docs/arbeitsweise.md`, `tests/planning.test.ts`, `task check` grün), in F0.2 bestätigt. AC-02 belegt mit Tests der reinen Funktion `crowdedDomains` statt mit temporären Ordnern (so von F0.1 vorgegeben), Verhalten wie B-174/AC-02. Review ohne schweren Befund, nichts behoben. Neue Tickets: keine.

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | `docs/arbeitsweise.md` enthält die Regel je Domäne, das Beanspruchen per Branch und das Re … |
| AC-02 | geprüft | Der Test prüft „höchstens ein aktiver Sprint je Domäne“ und besteht bzw. scheitert wie in … |
| AC-03 | geprüft | `task check` ist grün (B-174/AC-03). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-174 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft | Je Domäne darf ein Sprint aktiv sein, Sessions werden per Branch beansprucht |
