# B-033 · Tests werden typgeprüft

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Status:** erledigt
- **Sprint:** SP01
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-09-30 🧑 Chat-Freigabe durch Ralf (Revision 1)

## Ausgangslage

`tsconfig.json` schließt mit `include: ["src"]` den Ordner `tests/` aus.

## Ziel

Tests werden typgeprüft. Nutzen: Typfehler in Tests fallen erst zur Laufzeit auf.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die Sessions autonom abarbeiten; Review-Session.

## Anforderungen

- `npm run typecheck` prüft auch `tests/`.

## Nicht-Ziele

Verhalten der Tests ändern.

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget.

## Beispiele

Typfehler in `tests/planning.test.ts` → `npm run typecheck` meldet ihn.

## Ausnahme- und Fehlerfälle

nicht relevant – reine Prüfung.

## Akzeptanzkriterien

- **AC-01** Ein absichtlicher Typfehler in einer Test-Datei lässt `npm run typecheck` scheitern.

## Offene Fragen

keine

## Notizen

–
