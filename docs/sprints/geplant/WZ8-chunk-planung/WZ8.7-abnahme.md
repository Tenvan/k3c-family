# WZ8.7 · Abnahme der Chunk-Planung durch Ralf

- **Status:** offen
- **Typ:** Review
- **Agent:** Mensch
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** wz8/7-abnahme
- **Abhängig von:** WZ8.6
- **Tickets:** B-461
- **Kriterien:** AC-10

## Ziel

🧑 plant am PC im Asset-Studio eine Sondergegend als ungespeicherten Entwurf und nimmt sie in Chunk- und Live-Vorschau bei zwei Seeds ab (AC-10). Es wird keine Planung eingecheckt.

## Kontext

Die Abnahme-Liste schreibt WZ8.6 unten in `### Abnahme-Liste`. Start: `task k3c-studio:build`, dann `bin/k3c-studio.exe` oder der Verweis in der Workbench. **Umgebung live:** Claude bereitet vor und trägt das Urteil von 🧑 ein, urteilt aber nicht selbst. Fenster- oder Browser-Prüfungen durch Claude nur mit Freigabe von 🧑 für diesen Lauf.

Vorläufige Punkte (WZ8.6 überarbeitet sie): (1) Beispiel Wald Platz +6 (Hub +300 … +350) festes Portal setzen → beide Vorschauen zeigen es sofort bei Hub +325, das zweite Portal springt nach links; (2) Sondergegend planen, zwei Seeds, gleicher Ort und gleicher Inhalt in Chunk- und Live-Vorschau; (3) ungültigen Entwurf provozieren → Fehler am Platz, letztes gültiges Bild bleibt; (4) Speichern zeigt den REG-Hinweis (nur auf einer Kopie oder mit anschließendem Zurücksetzen, nichts einchecken); (5) `data/islands.json` extern ändern → Speichern wird abgelehnt, Entwurf bleibt. Hardware entkoppelt: WZ8.7 ist keine Abhängigkeit des Reviews.

## Erlaubte Dateien

- Planungsdateien von WZ8 und B-461, neue Tickets für Befunde und für die Sondergegend (REG-Ticket), `docs/sprints/README.md` (Fahrplan), alle über k3c-dev

## Nicht-Ziele

Code ändern oder Befunde beheben (→ Ticket); urteilen oder abhaken anstelle von 🧑; eine Planung in `data/islands.json` speichern oder einchecken; Golden-Daten ändern; Spec-Freigaben oder Kriterien umformulieren.

## Schritte

1. Claude bereitet vor: `workbench_status`, `svc_status`, `task k3c-studio:build`, Abnahme-Liste bereitstellen.
2. 🧑 geht die Liste am PC durch und plant die Sondergegend als Entwurf.
3. Claude trägt das Urteil von 🧑 je Punkt mit Datum und Quelle ins Ergebnis ein; Befunde werden Tickets.
4. 🧑 entscheidet, ob die Sondergegend Entwurf bleibt oder als REG-Ticket festgehalten wird; Claude legt das Ticket an.
5. Bei Abnahme: B-461 `erledigt`, WZ8 `erledigt` (Ordner nach `erledigt/`), Fahrplan anpassen. Commit `docs(dev): Abnahme WZ8 (WZ8.7)`, `git merge origin/develop`, push auf `sprint/wz8`; ist der Sprint-PR schon gemergt, bekommt dieser Commit einen eigenen PR gegen `develop`.

## Fertig, wenn

- [ ] AC-10: Urteil von 🧑 je Punkt der Abnahme-Liste im Ergebnis, mit Datum und Quelle; Sondergegend bleibt Entwurf oder ist als REG-Ticket festgehalten.
- [ ] Bei Abnahme: B-461 und WZ8 `erledigt`; `check_run task:test` mit `pattern: planning` grün.

## Prüfen

```bash
task test -- planning
```

Manuelle Prüfungen am PC (Asset-Studio-Fenster) macht 🧑. Claude prüft im Fenster oder Browser nur, wenn 🧑 das für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
