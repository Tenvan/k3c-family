# PJ3.1 · Projekte anlegen, Sprints und Tickets zuordnen

- **Status:** in Arbeit
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** INF
- **Umgebung:** live
- **Branch:** pj3/1-projekte-anlegen
- **Abhängig von:** M11.2, TR3.2
- **Tickets:** B-359
- **Kriterien:** AC-01, AC-04

## Ziel

Die Projekte aus B-359 › Anforderungen existieren mit Rang und Sprint-Reihenfolge, jedes offene Ticket trägt ein Projekt, B-011 und B-099 sind Ziel-Tickets, und die veralteten Tickets sind bereinigt. Die Planungsseite der Workbench zeigt die Projekte.

## Kontext

- Spec: B-359 (Tabelle der Projekte mit Rang, Sprints und Tickets ohne Sprint; Stand 2026-10-07, von 🧑 entschieden). Regeln: `docs/arbeitsweise.md` › Projekte und Rang. Vorlage `docs/vorlagen/projekt.md`, Übersicht `docs/projekte/README.md` (heute leer).
- Werkzeug (PJ2): `plan_create kind=projekt` (id = Kürzel, legt Datei und Übersichtszeile an, Rang = letzter + 1), `plan_set` am Projekt (`Status`, `Rang`, `Sprints` als kommagetrennte Reihenfolge, `Ziel-Tickets`), `plan_set {Projekt: XXX}` an Sprint oder Ticket (trägt in die Sprint-Tabelle des Projekts ein), `plan_section` (Ziel, Nicht-Ziele, Notizen), `plan_list kind=projekt` zum Prüfen. `ABN` hat keinen Rang (Sonderfall seit PJ2).
- **Reihenfolge der Anlage = Rang:** LST, GRA, SND, BED, WRT, SKL, KMP, WZ, REL der Reihe nach anlegen; dann `PRZ` (Arbeitsweise, Sprints PJ1 → PJ2 → PJ3) als aktiv mit Rang 10, weil PJ3 noch läuft; PJ3.4 setzt PRZ auf `erledigt`. `BAL` anlegen und auf `ruht` setzen (Notiz: Grund „zurückgestellt 2026-10-07, Spieleabende später“), `ABN` mit HW1.
- Sprints, die laut B-359 in PJ3.2 zusammengelegt werden (RG3, BAL5, SO5, DBG4), bekommen hier schon das Projekt ihres Ziels (BAL, BAL, SND, GRA), damit die Prüfung grün bleibt; die Reihenfolge in der Projekt-Tabelle setzt sie direkt hinter ihr Ziel. Die sieben Sprints, die PJ3.2 schließt (DBG3, LP1, LT1, MON2, RL1, SO1, SO3), bekommen ein Projekt nach Thema: DBG3, LT1 und MON2 → WZ, LP1 → BED, RL1 → REL, SO1 und SO3 → SND.
- **Zwei aktive Sprints in WZ:** M11 und TR3 sind beide aktiv mit offenem Review; der Planungstest erlaubt je Projekt höchstens einen aktiven Sprint. Deshalb hängt diese Session von M11.2 und TR3.2 ab (beide autonom, Domänen SRV und CLI, laufen parallel). DBG3, LT1, MON2 zählen nicht, weil dort nur Mensch-Sessions offen sind.
- **Tickets:** 129 offene oder eingeplante Tickets (`plan_list kind=ticket`). Ein Ticket mit Sprint bekommt das Projekt seines Sprints; eines ohne Sprint nach B-359 (B-329, B-331 → GRA; B-339, B-333, B-214 → BED; B-342 → WRT; B-327, B-328, B-343, B-324 → KMP; B-352, B-354, B-341 → WZ), sonst nach Thema und Domäne. Passt kein Projekt: Frage-Ticket mit der Liste, das Ticket bleibt `Projekt: –`, Ergebnis nennt es.
- **Bereinigen:** B-090 (U1), B-092 (U3), B-208 (W5) hängen an erledigten Sprints: gegen deren Ergebnis prüfen, dann `Status: erledigt` (archiviert das Tool) oder Sprint `–` und ein Projekt. Ziel-Tickets: SND `B-011`, BAL `B-099`.
- k3c-dev bedient die Repo-Wurzel; ohne Header `X-K3C-Root` landen Änderungen aus einem Worktree dort (B-275). Darum im Checkout der Repo-Wurzel arbeiten und jede Antwortzeile `Checkout:` prüfen.

## Erlaubte Dateien

- `docs/projekte/` (neu: Projekt-Dateien, Übersicht)
- `docs/sprints/` (Feld `Projekt`, Fahrplan, Status und Ergebnis von PJ3)
- `docs/backlog/` (Feld `Projekt`, Bereinigung, neue Tickets)

## Nicht-Ziele

Sprints schließen oder zusammenlegen (PJ3.2), Fahrplan umgliedern und strenge Prüfung (PJ3.3), Inhalte von Sprints oder Tickets ändern, Sprints bereit machen oder freigeben, Reviews anderer Sprints durchführen.

## Schritte

1. `plan_list kind=sprint` gegen B-359 abgleichen: Sprint inzwischen erledigt → nur zuordnen; neuer Sprint seit 2026-10-07 → nach Thema zuordnen, im Ergebnis nennen.
2. Projekte LST, GRA, SND, BED, WRT, SKL, KMP, WZ, REL in dieser Reihenfolge mit `plan_create kind=projekt` anlegen, Abschnitt `Ziel` aus B-359 bzw. dem Thema füllen. Danach PRZ (aktiv, Rang 10), BAL (`Status: ruht`), ABN.
3. Jeden offenen Sprint per `plan_set {Projekt}` zuordnen (Kontext), dann je Projekt `plan_set {Sprints}` in der Reihenfolge aus B-359.
4. Jedes offene Ticket per `plan_set {Projekt}` zuordnen (Kontext); nicht zuordenbare als Frage-Ticket sammeln.
5. B-090, B-092, B-208 bereinigen; `Ziel-Tickets` von SND und BAL setzen.
6. `plan_list kind=projekt` prüfen, `check_run task:test pattern=planning` grün, committen.

## Fertig, wenn

- [ ] AC-01: `plan_list kind=projekt` zeigt LST … REL mit Rang 1–9, PRZ mit Rang 10, BAL ruhend, ABN ohne Rang, Sprints in der Reihenfolge aus B-359.
- [ ] AC-04: B-090, B-092, B-208 archiviert oder einem Projekt zugeordnet, B-327 und B-328 bei KMP, B-011 und B-099 Ziel-Tickets.
- [ ] Jedes offene Ticket hat ein Projekt oder steht im Frage-Ticket.
- [ ] `task test -- planning` grün.

## Prüfen

```bash
task test -- planning
```

`check_run task:test pattern=planning` über k3c-dev. Keine manuellen Prüfungen.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
