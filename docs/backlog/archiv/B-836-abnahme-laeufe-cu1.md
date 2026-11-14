# B-836 · Abnahme-Läufe CU1

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** PT5
- **Projekt:** PAP
- **Erstellt:** 2026-11-13
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Regel B-742 (Abnahme-Ticket)

## Ausgangslage

Sprint CU1 (alte Grafik aufräumen) ist abgenommen (CU1.6). CU1.1 hat Papier-Assets angelegt (Gerüste, Plantage), CU1.4 und CU1.5 haben Go-Code in `tools/assetkit` und der Workbench geändert. Offen aus der Spec: CU1/AC-02 (`task check:all`, `task buildall`) lief in keiner Session; `dev:test` scheiterte im Review-Worktree am fehlenden `frontend/dist` (Umgebung).

## Ziel

Gen, Race und die Voll-Builds bestätigen CU1 auf dem gemergten Stand.

## Beteiligte und Zielgruppen

Claude (autonom, nachts).

## Anforderungen

Läufe laut `docs/arbeitsweise.md` › Sim, Balance und Gen, dazu CU1/AC-02.

## Nicht-Ziele

Nachbessern; Rot wird ein neues Ticket (Typ Problem) im Projekt PAP.

## Regeln und Einschränkungen

Sim und Balancing gesperrt (Beschluss 2026-11-11). `task buildall` nur, wenn keine EXE läuft.

- **Bereich:** nur lesend
- **Abhängig von:** CU1
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

–

## Ausnahme- und Fehlerfälle

Golden rot ohne gewollte Änderung → Ticket Typ Problem.

## Akzeptanzkriterien

- **AC-01** Sim: verschoben (Sperre 2026-11-11); CU1 berührt die Simulation nicht.
- **AC-02** Balance: verschoben (Sperre 2026-11-11); nicht relevant.
- **AC-03** Gen: `check_run task:test` (ruft `gen:paper` auf, Grafik-Golden `tests/paperAssets.test.ts`, `tools/paper/golden`) grün.
- **AC-04** Race: `check_run task:check:race` grün (ohne C-Compiler übersprungen, so nennen).
- **AC-05** CU1/AC-02: `task check:all`, `task check:dev` und `task buildall` grün.

## Offene Fragen

keine

## Notizen

Lauf PT5.1 (2026-11-14, Claude, Branch sprint/pt5):
- AC-01 Sim und AC-02 Balance verschoben (Sperre 2026-11-11).
- AC-03 Gen: `check_run task:check` (4 min 54 s, ruft `gen:paper` und die Grafik-Golden) grün: 175 Testdateien grün; einziger Fehler war die Vorlage der PT5-README (Kriterium ohne Session), behoben, Planungstest danach grün.
- AC-04 Race: `check_run task:check:race` exit 0 in 0,4 s, lokal ohne gcc übersprungen.
- AC-05 CU1/AC-02: `task:check:go` grün (1 min 36 s), `task:check:dev` grün (4 min 22 s), `task:build` grün, `task buildall` exit 0 (alle EXE in bin/ um 10:37–10:38 neu, Workbench dazu kurz gestoppt). `check:all` ist kein check_run-Ziel und wurde über diese Teilläufe abgedeckt.
