# WZ44 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Planungs-Werkzeuge für Wellen: plan_waves, plan_queue, plan_check, Sim-Sperre im Werkzeug
- **Projekt:** WZG

## Ziel

Ein Aufruf liefert die Welle, die jetzt parallel laufen kann, ein zweiter die Ralf-Queue nach Gerät; Planungsangaben werden geprüft, die Sim-Sperre gilt im Werkzeug, und der Fahrplan ist aufgeräumt. Am Ende sichtbar: `plan_waves`, `plan_queue`, `plan_check` im MCP; `sim_test start` lehnt mit Hinweis auf den Beschluss ab; `docs/sprints/aktiv/` ohne Sprint ohne offene Session.

## Ergebnis

2026-11-13 (WZ44.5, Claude): AC-01–AC-04 erfüllt (Ergebnisse WZ44.1–WZ44.3), AC-05 teilweise (WZ44.4; „Offen am Gerät“, KB6-Thema und LG1-Zeile verschoben nach B-842). `task check`, `task check:go` grün; `dev:test` im Worktree ohne Frontend-Build nicht lauffähig (B-831), Planungs-Tests per `go test` grün. Behoben im Review: `allowedFiles` liest alle Pfade je Zeile (Kollisionen in `plan_waves`/`plan_next` wurden übersehen). Neue Tickets: B-842, B-843, B-838 (Regel Sessions eines Sprints, Entscheidung 🧑), B-839; Abnahme-Ticket B-841 (B-842/B-843 umnummeriert von B-826/B-827, Kollision mit `sprin …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | erfüllt | `plan_next` leitet den Sprint-Branch richtig ab, auch mit führendem `sprint/` (`B-782/AC-0 … |
| AC-02 | erfüllt | `check_run` nimmt die drei Teilaufgaben, Sim-Sperre im Werkzeug mit Schalter (`B-788/AC-01 … |
| AC-03 | erfüllt | `plan_waves`, `plan_queue`, Kollision in `plan_next`, MCP-Instruktionen (`B-783/AC-01` bis … |
| AC-04 | erfüllt | `plan_check` und Regel-Test (`B-784/AC-01`, `B-784/AC-02`). |
| AC-05 | teilweise | Fahrplan aufgeräumt (`B-785/AC-01` bis `B-785/AC-03`). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-782 | erledigt | AC-01 erfüllt | plan_next nennt den Sprint-Branch richtig, auch wenn das Feld Branch schon mit s … |
| B-788 | erledigt | AC-02 erfüllt | check_run kennt die Teilaufgaben von check:diff, sim_test und sim_run lehnen wäh … |
| B-783 | erledigt | AC-03 erfüllt | plan_waves berechnet Wellen aus Abhängigkeit, Bereich und Sperre, plan_queue zei … |
| B-784 | erledigt | AC-04 erfüllt | Planungsangaben für Wellen werden geprüft: plan_check und Regel-Test für freigeg … |
| B-785 | erledigt | AC-05 teilweise | Fahrplan ist aufgeräumt: fertige Sprints erledigt, Abnahmen als Gerätetermine, a … |
