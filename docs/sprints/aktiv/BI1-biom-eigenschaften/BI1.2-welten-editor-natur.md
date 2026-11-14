# BI1.2 · Welten-Editor zeigt und setzt Natur-Werte

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** –
- **Modell:** –
- **Umgebung:** offline
- **Branch:** bi1/2-welten-editor-natur
- **Abhängig von:** BI1.1
- **Tickets:** B-891
- **Kriterien:** AC-05

## Ziel

Der Welten-Editor der Workbench zeigt je Insel und Stufe die geltenden Natur-Werte (gewürfelt oder fest), setzt sie fest und setzt sie auf „aus Seed“ zurück; die Chunk-Vorschau zeigt das Ergebnis.

## Kontext

`k3c-sim world` liefert seit BI1.1 im Level-Bericht `nature`, `natureFixed`, `natureTraits`, `natureWeights` (`cmd/k3c-sim/world.go`). Welt-Datei-Felder `nature`/`natureByDepth` (docs/rules/welten.md § 2.1). Workbench: Go `tools/k3c-workbench/internal/world/` (Entwurf patchen, `patch.go`), Frontend `tools/k3c-workbench/frontend/src/world/` (WorldPage, ChunkInspector, levelView).

## Erlaubte Dateien

- `tools/k3c-workbench/internal/world/**`
- `tools/k3c-workbench/frontend/src/world/**`

## Nicht-Ziele

Darstellung im Spiel; neue Eigenschaften.

## Schritte

1. Level-Antwort im Frontend um die Natur-Felder erweitern.
2. Abschnitt „Natur“ je Stufe: Wert, Bereich, Quelle (Seed/fest), Schieber zum Festsetzen, Zurücksetzen.
3. Festgesetzte Werte in den Entwurf (`natureByDepth`) schreiben, Prüfung über `check`.
4. Tests (Vitest, go test).

## Fertig, wenn

- [ ] AC-05: Editor zeigt und setzt die Werte (Tests grün); Sichtprüfung durch 🧑 offen.
- [ ] `task check:diff` grün.

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

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
