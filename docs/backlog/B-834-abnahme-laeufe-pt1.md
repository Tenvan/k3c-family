# B-834 · Abnahme-Läufe PT1

- **Domäne:** PAP
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** offen
- **Sprint:** PT1
- **Projekt:** PAP
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)

## Ausgangslage

Sprint PT1 (gen:paper schneller) ist abgenommen (PT1.3). Gen-Lauf und Grafik-Golden liefen am Feature in PT1.2; das Review hat danach `cli/pool.ts` in den Cache-Hash `codeStand` aufgenommen (`tools/paper/cli/genCache.ts`), dadurch rendert der nächste Lauf einmal alles neu.

## Ziel

Die Gen-Läufe nach B-742 bestätigen PT1 auf dem gemergten Stand.

## Beteiligte und Zielgruppen

Claude (autonom, nachts).

## Anforderungen

Läufe laut `docs/arbeitsweise.md` › Sim, Balance und Gen.

## Nicht-Ziele

Nachbessern; Rot wird ein neues Ticket (Typ Problem) im Projekt PAP.

## Regeln und Einschränkungen

Sim und Balancing gesperrt (Beschluss 2026-11-11).

- **Bereich:** `tools/paper/**` (nur lesend)
- **Abhängig von:** PT1
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

–

## Ausnahme- und Fehlerfälle

Golden rot ohne gewollte Änderung → Ticket Typ Problem.

## Akzeptanzkriterien

- **AC-01** Sim: verschoben (Sperre 2026-11-11); PT1 berührt die Simulation nicht.
- **AC-02** Balance: verschoben (Sperre 2026-11-11); nicht relevant.
- **AC-03** Gen: `check_run task:test` (ruft `gen:paper` auf, Grafik-Golden `tests/paperAssets.test.ts`, `tools/paper/golden`) grün; der zweite `task gen:paper` direkt danach rendert 0 Assets.
- **AC-04** Race: nicht relevant (kein Go-Code im Sprint).

## Offene Fragen

keine

## Notizen

–
