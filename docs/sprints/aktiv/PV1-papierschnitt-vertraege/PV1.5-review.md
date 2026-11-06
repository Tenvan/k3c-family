# PV1.5 · Review PV1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** DEV
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** pv1/5-review
- **Abhängig von:** PV1.8
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02, AC-03, AC-04, AC-05, AC-06, AC-07

## Ziel

Sprint PV1 ist geprüft und abgenommen; die Verträge sind für RD1, MC1, WB1, RV1 freigegeben.

## Kontext

Review nach `docs/arbeitsweise.md` › Review, gegen Entscheidung 005 und die Sprint-README PV1, nur den Diff des Sprints.

## Erlaubte Dateien

- Dateien der Sessions PV1.1–PV1.8 (nur Befund-Fixes)
- `docs/decisions/005-papierschnitt-assets-mcp.md` (nur Pfad-Korrekturen)
- `docs/sprints/` (Abnahme, Status), `docs/backlog/` (neue Tickets)

## Nicht-Ziele

Keine neuen Funktionen, keine Vertragsänderung ohne schweren Befund.

## Schritte

1. `task check`, `task check:go`, `task check:race` nacheinander.
2. CI auf Linux für den Sprint-Branch von Hand starten (`gh workflow run ci.yml --ref sprint/pv1`) und `probe.test.ts` auf Linux prüfen (Golden-Regel aus PV1.3).
3. Verträge konsistent: jedes Feld aus `paper-spec.md` im Schema; jede CLI-Antwort von der Attrappe geliefert; jede RPC-Methode aus `paper-mcp.md` in `internal/assetsui`, `uiMethods` und `wails.ts`; Prompt-Platzhalter stimmen.
4. Erlaubte Dateien der Folge-Lanes (RD1, MC1, WB1, RV1, MG*) disjunkt gegen den Ist-Stand; Abweichungen in 005 korrigieren.
5. Abnahme in der Sprint-README, Sprint auf erledigt.

## Fertig, wenn

- [ ] AC-01 bis AC-07 belegt.
- [ ] Linux-Lauf der Probe grün.
- [ ] Lane-Pfade disjunkt.

## Prüfen

```bash
task check
task check:go
task check:race
```

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
