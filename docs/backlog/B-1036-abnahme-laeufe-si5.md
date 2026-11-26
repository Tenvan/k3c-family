# B-1036 · Abnahme-Läufe SI5: Sim, Balance, Gen samt Golden und Race sind grün

- **Domäne:** INF
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** KN1
- **Projekt:** NAC
- **Erstellt:** 2026-11-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Review SI5.5 (2026-11-17): `task:check:go` (go test) grün, `task:check` bis auf einen Windows-Temp-Fehler in `tools/paper/render/render.test.ts` (EPERM beim Umbenennen im Temp-Ordner, Datei im Sprint nicht berührt) grün. Sim, Balance, Gen und Race laufen nach B-742 nur hier. SI5.2 (Engine) und SI5.4 (Anzeige) sind verworfen; die Läufe decken daher nur die Assets ab.

## Ziel

Die Abnahme-Läufe des Sprints SI5 sind gelaufen, Abweichungen als Tickets erfasst.

## Beteiligte und Zielgruppen

Umsetzung: Claude autonom. Keine Freigabe durch 🧑 nötig.

## Anforderungen

- **AC-01** `gen:paper` rendert `einsiedler-huette`, `dojo`, `schrein`, `schwertschmied` und `priesterin` ohne Fehler, Grafik-Golden stimmt.
- **AC-02** `check_run task:check:race` grün (ohne gcc: als übersprungen nennen).
- **AC-03** Sim und Balance: entfällt bis zur Aufhebung der Sperre (Beschluss 🧑 2026-11-11); danach `sim_test`-Lauf ohne Abweichung zur Baseline.
- **AC-04** `tools/paper/render/render.test.ts` läuft grün (EPERM im Temp-Ordner aus der Review SI5.5 einmal nachprüfen; bleibt er rot, eigenes Ticket).

## Nicht-Ziele

Engine und Anzeige von Einsiedler und Bauwerk (SI5.2/SI5.4 verworfen).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `pfad/**`, `pfad/datei.go` (alle Dateien, die das Ticket anfasst; Sammeldateien wie `docs/backlog/README.md` nicht nennen)
- **Abhängig von:** – (nur IDs: B-000, SP00, SP00.1; der andere Stand muss erledigt und gemergt sein)
- **Braucht 🧑:** – (oder Freigabe | Entscheidung | Abnahme | Gerät; `–` heißt vollständig autonom)
- **Größe:** S (S = eine Session, M = zwei bis drei, L = vor der Freigabe in Tickets je Teil zerlegen)
- **Sperre:** – (oder golden | protokoll | spielstand | deps | taskfile | exe)

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `check_run task:test` samt `gen:paper`: die fünf SI5-Assets rendern, Golden unverändert oder Abweichung begründet.
- **AC-02** `check_run task:check:race` grün oder ohne gcc als übersprungen genannt.
- **AC-03** Sim/Balance: entfällt, solange die Sperre gilt.
- **AC-04** `render.test.ts` grün oder Ticket angelegt.

## Offene Fragen

keine

## Notizen

–
