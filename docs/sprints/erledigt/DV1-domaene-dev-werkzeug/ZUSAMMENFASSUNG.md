# DV1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Domäne DEV für k3c-dev, Sprint ohne Prio und Einschiebbar
- **Projekt:** WZG

## Ziel

Arbeit am Entwickler-Werkzeug läuft in einer eigenen Domäne `DEV` und sperrt SRV nicht mehr, Sprints planen sich nur noch über den Rang ohne `Prio`/`Einschiebbar`, und die Server-Tools von k3c-dev erreichen den selbst gestarteten Spielserver mit dessen Token. Am Ende sichtbar: `plan_list domain: DEV` listet die offenen Werkzeug-Tickets, die Planungsseite hat einen Filter-Chip `DEV`, ein neuer Spri …

## Ergebnis

Review 2026-10-09 (DV1.4, eigener Review-Agent): keine schweren Befunde, `task check` und `task check:dev` grün. - AC-01, AC-02 geprüft (DV1.1); AC-03–AC-05 geprüft (DV1.2: `TestDomaeneDEV`, `TestSprintOhnePrioUndEinschiebbar`, `plan_list domain: DEV`); AC-06, AC-07 geprüft (DV1.3: `TestServerClientNimmtDienstToken`, `TestServerClientUmgebungHatVorrang`). - B-365, B-361, B-362 erledigt. Neue Tickets: keine. - Version: Minor vorgeschlagen (Domäne DEV, Server-Tools ohne eigenes Token); nicht gesetzt (wartet auf Bestätigung 🧑).

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Die Domänen-Tabelle in `docs/arbeitsweise.md` nennt `DEV` mit `tools/k3c-dev/`, SRV nennt … |
| AC-02 | geprüft | `docs/vorlagen/sprint.md` hat weder `Prio` noch `Einschiebbar`, und `task test -- planning … |
| AC-03 | geprüft | `plan_create`/`plan_set` akzeptieren `Domäne: DEV`, und `task test -- planning` ist mit ei … |
| AC-04 | geprüft | `plan_create {kind: sprint}` und `plan_set` schreiben weder `Prio` noch `Einschiebbar` (`B … |
| AC-05 | geprüft | Offene Tickets am Werkzeug tragen `DEV` (`B-365/AC-03`). |
| AC-06 | geprüft | Ohne `K3C_STATUS_TOKEN` in der Umgebung von k3c-dev antworten `server_status` und `sim_tes … |
| AC-07 | geprüft | Ein gesetztes `K3C_STATUS_TOKEN` hat weiter Vorrang (`B-362/AC-02`, Go-Test). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-365 | erledigt | AC-01 geprüft, AC-03 geprüft, AC-05 geprüft | Das Entwickler-Werkzeug k3c-dev hat eine eigene Domäne statt SRV |
| B-361 | erledigt | AC-02 geprüft, AC-04 geprüft | Die Sprint-Felder Prio und Einschiebbar entfallen in Vorlage, Planungstest und p … |
| B-362 | erledigt | AC-06 geprüft, AC-07 geprüft | k3c-dev fragt den selbst gestarteten Spielserver mit dessen Token an |
