# B-051 · Oxlint meldet im Bestand keine Warnungen mehr

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** I1
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-01 🧑 Chat („baue aus den offenen Punkten den nächsten Sprint und aktiviere ihn“, Sprint I1 Revision 1)

## Ausgangslage

Oxlint meldet im Bestand noch 2 Warnungen (`no-extra-boolean-cast` in `src/tools/gamepadTest.ts:47`); die übrigen sieben sind mit dem Löschen von `src/world/` (SP09) verschwunden.

## Ziel

Oxlint meldet im Bestand keine Warnungen mehr. Nutzen: Neue Warnungen fallen auf, statt im Rauschen unterzugehen.

## Beteiligte und Zielgruppen

Entwickler und Cloud-Agenten, die `npm run lint` lesen; Review-Session.

## Anforderungen

- Beide Warnungen sind behoben (Verhalten gleich, Tests grün).

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

- **AC-01** `task lint` meldet 0 Warnungen.
- **AC-02** `task check` ist grün.

## Offene Fragen

keine (Revision 2: Warnungen lassen die CI nicht scheitern, `--deny-warnings` ist kein Ziel dieses Tickets).

## Notizen

Gefunden in SP01.1 (Messung mit Oxlint 1.86.0).
