# DV1 · INF, SRV, DEV · Domäne DEV für k3c-dev, Sprint ohne Prio und Einschiebbar

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** INF, SRV, DEV
- **Reife:** bereit
- **Tickets:** B-365, B-361, B-362
- **Start-Commit:** e9869ba
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-08, 🧑 im Chat (Revision 1 mit B-362)

## Ausgangslage

k3c-dev (`tools/k3c-dev/`) gehört heute zur Domäne SRV. Sprints am Werkzeug sperren deshalb Server-Arbeit, obwohl sie keine Dateien mit ihr teilen (B-365). Außerdem tragen Sprints noch `Prio` und `Einschiebbar`, obwohl die Reihenfolge seit PJ2 aus dem Projekt-Rang kommt. Vorlage, Planungstest und plan-Tools verlangen bzw. schreiben beide Felder weiter (B-361). Die Server-Tools von k3c-dev scheitern mit 401 am selbst gestarteten Spielserver, weil sie dessen Token nicht kennen (B-362).

## Ziel

Arbeit am Entwickler-Werkzeug läuft in einer eigenen Domäne `DEV` und sperrt SRV nicht mehr, Sprints planen sich nur noch über den Rang ohne `Prio`/`Einschiebbar`, und die Server-Tools von k3c-dev erreichen den selbst gestarteten Spielserver mit dessen Token. Am Ende sichtbar: `plan_list domain: DEV` listet die offenen Werkzeug-Tickets, die Planungsseite hat einen Filter-Chip `DEV`, ein neuer Sprint aus `plan_create` hat weder `Prio` noch `Einschiebbar`, `server_status` antwortet ohne gesetztes `K3C_STATUS_TOKEN`, und `task check` ist grün.

## Beteiligte und Zielgruppen

Agenten planen und arbeiten parallel an Werkzeug und Server. 🧑 entscheidet die offenen Fragen aus B-365 (Kürzel, Umfang) und B-361 (Fahrplan-Spalte) und gibt die Spec frei.

## Anforderungen

B-365 › Anforderungen; B-361 › Anforderungen; B-362 › Anforderungen. Sprint-eigen: B-365 und B-361 ändern dieselben Stellen (Vorlagen, `tests/planning.test.ts`/`tests/planningDocs.ts`, `tools/k3c-dev/internal/planning/`) und werden deshalb in einem Durchgang geändert: zuerst Regeln, Vorlagen und Test (INF), danach die plan-Tools. B-362 ist unabhängig davon und läuft als eigene Session nach der Umstellung, schon in der Domäne `DEV`.

## Nicht-Ziele

Erledigte Tickets und Sprints auf `DEV` umschreiben (bleiben bei ihrer alten Domäne). Ticket-Prio (bleibt). Weitere Funktionen von k3c-dev (B-360, B-363, B-366). Fachliche Änderungen an `cmd/k3c-load`, `cmd/k3c-tui` oder `src/tools/`; sie wechseln nur die Domäne.

## Regeln und Einschränkungen

- PJ3 (B-359) ist erledigt: alle offenen Sprints haben ein Projekt.
- Domäne je Session: INF für Regeln, Glossar, Vorlagen und Planungstest (DV1.1), SRV für `tools/k3c-dev/` (DV1.2; die Domäne `DEV` gibt es erst, wenn DV1.2 die plan-Tools umgestellt hat). Neue Begriffe zuerst ins Glossar.
- Zwischen DV1.1 und DV1.2 schreiben die alten plan-Tools weiter `Prio`/`Einschiebbar`, wenn die Datei das Feld hat; da DV1.1 die Felder aus allen Sprint-Dateien entfernt, übergehen sie es still (B-360). Kein `plan_create {kind: sprint}` in dieser Zeit.
- Planung nur über die plan-Tools. Das Entfernen der beiden Kopf-Felder aus allen Sprint-READMEs ist eine Formatumstellung ohne Tool und läuft in DV1.1 von Hand (Planungs-Dateien).
- Datei ≤ 400, Funktion ≤ 60 Zeilen, keine neue Abhängigkeit. Go-Tests für die plan-Tools, Vitest für den Planungstest.

## Beispiele

- `plan_create {kind: ticket, fields: {Domäne: DEV}}` → Ticket angelegt, `task test -- planning` grün.
- Sprint `X` in Domäne SRV aktiv, Session in `DEV` offen → beide laufen parallel, ohne dass sich die Domänen sperren.
- `plan_create {kind: sprint}` → README ohne `Prio` und `Einschiebbar`.
- B-361 › Beispiele.

## Ausnahme- und Fehlerfälle

- Unbekannte Domäne (z. B. `TOOL`) → `plan_create`/`plan_set` lehnen mit der Liste der gültigen Domänen ab, der Planungstest meldet die Datei.
- Ein `plan_set` mit `Prio` an einem Sprint → wird abgelehnt (B-360 regelt das allgemein, hier nur kein stilles Schreiben).
- B-361 › Ausnahme- und Fehlerfälle.

## Akzeptanzkriterien

- **AC-01** Die Domänen-Tabelle in `docs/arbeitsweise.md` nennt `DEV` mit `tools/k3c-dev/`, SRV nennt k3c-dev nicht mehr, das Glossar hat den Begriff (`B-365/AC-01`).
- **AC-02** `docs/vorlagen/sprint.md` hat weder `Prio` noch `Einschiebbar`, und `task test -- planning` ist grün (`B-361/AC-01`).
- **AC-03** `plan_create`/`plan_set` akzeptieren `Domäne: DEV`, und `task test -- planning` ist mit einem Ticket in `DEV` grün (`B-365/AC-02`, Go-Test).
- **AC-04** `plan_create {kind: sprint}` und `plan_set` schreiben weder `Prio` noch `Einschiebbar` (`B-361/AC-02`, Go-Test).
- **AC-05** Offene Tickets am Werkzeug tragen `DEV` (`B-365/AC-03`).
- **AC-06** Ohne `K3C_STATUS_TOKEN` in der Umgebung von k3c-dev antworten `server_status` und `sim_test start mode=online` gegen den per `svc_start` gestarteten Spielserver ohne 401 (`B-362/AC-01`, Go-Test mit Fake-Server).
- **AC-07** Ein gesetztes `K3C_STATUS_TOKEN` hat weiter Vorrang (`B-362/AC-02`, Go-Test).

## Offene Fragen

keine. Entschieden von 🧑 am 2026-10-08: Kürzel `DEV`; zu `DEV` gehören `tools/k3c-dev/`, `cmd/k3c-load/`, `cmd/k3c-tui/` und `src/tools/`. Die Fahrplan-Spalte `Prio` ist schon mit PJ3.3 entfallen.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| DV1.1 | `DV1.1-domaene-dev-regeln.md` | Umsetzung | autonom | fertig |
| DV1.2 | `DV1.2-plan-tools-dev.md` | Umsetzung | autonom | fertig |
| DV1.3 | `DV1.3-token-dienst.md` | Umsetzung | autonom | fertig |
| DV1.4 | `DV1.4-review.md` | Review | autonom | fertig |

## Abnahme

Review 2026-10-09 (DV1.4, eigener Review-Agent): keine schweren Befunde, `task check` und `task check:dev` grün.
- AC-01, AC-02 geprüft (DV1.1); AC-03–AC-05 geprüft (DV1.2: `TestDomaeneDEV`, `TestSprintOhnePrioUndEinschiebbar`, `plan_list domain: DEV`); AC-06, AC-07 geprüft (DV1.3: `TestServerClientNimmtDienstToken`, `TestServerClientUmgebungHatVorrang`).
- B-365, B-361, B-362 erledigt. Neue Tickets: keine.
- Version: Minor vorgeschlagen (Domäne DEV, Server-Tools ohne eigenes Token); nicht gesetzt (wartet auf Bestätigung 🧑).
