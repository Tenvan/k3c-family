# B-051 · Oxlint meldet im Bestand keine Warnungen mehr

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-09-30
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit SP01.1 läuft Oxlint mit seinen Standard-Regeln. Neben dem Budget (Fehler) meldet es im Bestand 9 Warnungen,
die den Lauf nicht scheitern lassen: `no-unused-expressions` 5× in `src/world/sim/economy.ts` (Zeilen 100, 101, 105,
108, 151), `no-extra-boolean-cast` 2× in `src/tools/gamepadTest.ts:47`, `unicorn/no-new-array` in
`src/world/levelGenerator.ts:51`, `unicorn/prefer-string-starts-ends-with` in `tests/projectRules.test.ts:14`.

## Ziel

Oxlint meldet im Bestand keine Warnungen mehr. Nutzen: Neue Warnungen fallen auf, statt im Rauschen unterzugehen.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die `npm run lint` lesen; Review-Session.

## Anforderungen

- Jede der 9 Warnungen ist behoben oder die Regel ist mit Begründung in `.oxlintrc.json` abgeschaltet.
- Verhalten bleibt gleich (Tests grün, Golden-Daten ab SP04 unverändert).

## Nicht-Ziele

Neue Regeln oder Kategorien einschalten; Budget-Ausnahmen (Ratsche) abbauen (B-018, B-034).

## Regeln und Einschränkungen

Prozess nur in `docs/arbeitsweise.md`; keine neue Abhängigkeit ohne Ticket und Zustimmung im Review; Komplexitäts-Budget.
`src/world/` hat Feature-Stopp (Entscheidung 001): dort nur Umformungen ohne Verhaltensänderung, sonst bis SP09 warten,
dann entfallen `economy.ts` und `levelGenerator.ts` ohnehin. Mehrere Domänen betroffen (SIM, PLAT, INF): Aufteilen
oder als Grenzfall planen.

## Beispiele

`cond && doSomething();` in `economy.ts` → `if (cond) doSomething();`, Test bleibt grün, Warnung weg.

## Ausnahme- und Fehlerfälle

Eine Warnung ist in diesem Projekt kein Problem → Regel abschalten, Grund als Kommentar in `.oxlintrc.json`.

## Akzeptanzkriterien

- **AC-01** `npm run lint` meldet 0 Warnungen.
- **AC-02** `npm run check` ist grün.

## Offene Fragen

Sollen Warnungen danach die CI scheitern lassen (`oxlint --deny-warnings`)? (🧑) Lohnt es sich vor SP09 überhaupt,
da 6 der 9 Warnungen in `src/world/` liegen, das dann gelöscht wird? (🧑)

## Notizen

Gefunden in SP01.1 (Messung mit Oxlint 1.86.0).
