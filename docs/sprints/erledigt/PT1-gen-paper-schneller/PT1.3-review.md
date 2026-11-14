# PT1.3 · Review PT1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** pt1/3-review
- **Abhängig von:** PT1.2
- **Tickets:** B-740, B-748, B-749, B-750, B-751
- **Kriterien:** alle

## Ziel

Sprint PT1 ist abgenommen, der eine PR des Sprints ist offen mit Auto-Merge, der Sprint liegt in `erledigt/`.

## Kontext

Review nach `docs/arbeitsweise.md` › Review-Session, zusätzlich Agent `k3c-reviewer` (Diff in `tools/paper`). Prüfpunkte: bitgleiche Ausgabe, stdout bleibt ein JSON, Cache invalidiert bei Stil- und Renderer-Änderung, keine neue Abhängigkeit. Gen-Lauf und Grafik-Golden liefen am Feature (PT1.2); ein Abnahme-Ticket nur, falls dort etwas offen blieb.

## Erlaubte Dateien

- alle Erlaubten Dateien von PT1.1 und PT1.2
- Planung PT1 und ihre Tickets (über MCP)

## Nicht-Ziele

Neue Funktionen, Stil-Befunde.

## Schritte

1. `task check`, `task check:go` nacheinander.
2. Diff `origin/develop...sprint/pt1` lesen, `k3c-reviewer` parallel; schwere Befunde beheben.
3. Abnahme in der README, Sprint nach `erledigt/`, PR mit Auto-Merge.

## Fertig, wenn

- [ ] Alle Kriterien der README mit Nachweis in den Session-Ergebnissen.
- [ ] Review-Befunde behoben oder als Ticket.
- [ ] PR offen mit Auto-Merge.

## Prüfen

```bash
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).
Review-Session statt dessen: `task check`, `task check:go` (nacheinander) und das Abnahme-Ticket des Sprints anlegen.
Sim-, Balance- und Gen-Läufe (`sim_test`, Balancing, `gen:paper` samt Golden, `check:race`) nur, wenn diese Session das
betroffene Feature entwickelt; sonst laufen sie im Abnahme-Ticket (`docs/arbeitsweise.md` › Sim, Balance und Gen).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- **AC-01 bis AC-03 geprüft:** Nachweise in PT1.1 (`fortschritt.test.ts`, Läufe `gen:paper-sync --status`, `--fehlt`, 210 ↳-Zeilen beim Monarch).
- **AC-04 geprüft:** Nachweis in PT1.2 (zweiter Lauf 0 Assets, kalt 5 min 19 s → 54 s mit 8 Workern, `golden.json` unverändert, Bitgleichheit per `genParallel.test.ts`).
- **Prüfläufe:** `task check` grün (6 min 28 s), `task check:go` grün (3 min 43 s), `check:diff` grün vor dem Commit.
- **Review (k3c-reviewer, Diff `ebbf1d9fd..9b2ef1dcb` in `tools/paper`, `tools/Taskfile.yml`):** keine schweren Befunde; belegt: Reihenfolge der Ausgaben deterministisch, stdout bleibt ein JSON (Worker nur `postMessage`), keine neue Abhängigkeit, Worker-Absturz → Exit 2 mit Asset-Name, Grenzen 400/60 eingehalten. Behoben (niedrig): `cli/pool.ts` fehlte im Cache-Hash `codeStand` (`genCache.ts`, 1 Zeile).
- **Abweichung:** Kein PR und kein Auto-Merge aus dieser Session; der Koordinator der Welle 5b führt den Branch zusammen. Kein Push.
- **Neu:** B-834 Abnahme-Läufe PT1 (Gen nach dem Hash-Fix). Offen niedrig aus PT1.2: kein Timeout für hängendes resvg (nicht als Ticket, kein Befund nach Review-Regel).
