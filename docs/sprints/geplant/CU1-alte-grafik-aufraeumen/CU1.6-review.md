# CU1.6 · Review und Abnahme CU1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** INF
- **Lane:** –
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** sprint/cu1
- **Abhängig von:** CU1.5
- **Tickets:** B-700
- **Kriterien:** alle

## Ziel

Der Sprint CU1 ist geprüft und die Abnahme der Sprint-README ausgefüllt; B-700 erfüllt, D-CU1-Zeilen in B-701 gemeldet. Damit ist das Projekt PAP technisch abgeschlossen.

## Kontext

- Kriterien AC-01–AC-05 der Sprint-README, B-700 AC-01–AC-03, D-US2-02 in B-701.
- Risiken: versteckte Laufzeit-Pfade (Strings wie `grafik/` + Variable), Docker-Build ohne Atlas-Stufe, Sound-Regression durch `data/sounds.json`, verwaiste Kacheln oder Seiten.
- Manuelle Sichtprüfung (Gerüste, Plantage, Sound im Spiel) ist Ralfs Sache; Claude listet sie als offene Abnahme.

## Erlaubte Dateien

- `docs/sprints/*/CU1-alte-grafik-aufraeumen/**`
- `docs/backlog/B-700-alte-pixel-grafik-aufraeumen.md`, `docs/backlog/B-701-pap-gesammelte-fragen.md`
- Fixes unter etwa 30 Zeilen in den Dateien der Sessions CU1.2–CU1.5; größere Befunde → Ticket

## Nicht-Ziele

Neue Funktionen; Tickets verwerfen (Ralf).

## Schritte

1. Diff des Sprints gegen den Start-Commit lesen (Opus-Review-Agent, getrennt vom Schreiben), Schwerpunkt Risiken aus dem Kontext.
2. Suchen aus AC-01 erneut ausführen, Ergebnis festhalten.
3. `task check`, `task check:go`, `task check:race` nacheinander (ohne gcc: übersprungen nennen), `task check:dev`, `task buildall`.
4. Lade-Probe headless: Spiel startet, Netzwerk-Log zeigt keine Anfragen auf `sprites/`, `atlas/`, `grafik-db32/` oder alte Packs.
5. Abnahme in der README (höchstens fünf Zeilen), B-700 auf erledigt, Sprint-Status über `plan_set`.

## Fertig, wenn

- [ ] Alle Kriterien AC-01–AC-05 mit Nachweis im Ergebnis.
- [ ] Lade-Probe ohne Anfragen auf alte Pfade.
- [ ] Schwere Review-Befunde behoben oder als Ticket erfasst.

## Prüfen

```bash
task check
task check:go
task check:race
task check:dev
task buildall
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
