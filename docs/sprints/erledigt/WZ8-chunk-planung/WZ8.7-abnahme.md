# WZ8.7 · Abnahme der Chunk-Planung durch Ralf

- **Status:** fertig
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

Die Abnahme-Liste schreibt WZ8.6 unten in `### Abnahme-Liste`. Start: `task k3c-studio:build`, dann `bin/k3c-studio.exe` oder der Verweis in der Workbench. **Umgebung live:** Claude bereitet vor und trägt das Urteil von 🧑 ein, urteilt aber nicht selbst. Fenster- oder Browser-Prüfungen durch Claude nur mit Freigabe von 🧑 für diesen Lauf. Hardware entkoppelt: WZ8.7 ist keine Abhängigkeit des Reviews.

### Abnahme-Liste

Ablauf am PC im Studio-Fenster, Seite „Welt-Betrachter“, Editor „Planung“ unter der Chunk-Vorschau (AC-10):

1. Insel 1, Stufe 1 (Wald) wählen. Beispiel der Spec: Chunk hinzufügen, Platz auf +6 setzen, Art `portal` → Chunk- und Live-Vorschau zeigen das feste Portal sofort an derselben Stelle (Hub +325), das zweite Portal springt nach links.
2. Sondergegend planen (Chunk-Seed, feste Objekte, Truhe), mit zwei verschiedenen Seeds in der Chunk-Vorschau und in der Live-Vorschau prüfen: gleicher Ort und gleicher Inhalt in beiden Vorschauen.
3. Einen ungültigen Entwurf provozieren (z. B. Art `volcano`, Platz 0, Platz +9 im Wald): Der Fehler steht am Platz, Speichern ist gesperrt, das letzte gültige Bild bleibt stehen.
4. Speichern (nur auf einer Kopie von `data/islands.json` oder mit anschließendem Zurücksetzen, nichts einchecken): Der Hinweis „REG-Änderung: Commit mit Begründung, `task golden:update` nur bei gewollter Änderung“ erscheint; `git diff data/islands.json` zeigt nur den Wert von `plannedChunks`.
5. `data/islands.json` von außen ändern, dann Speichern: Es wird abgelehnt („data/islands.json geändert, bitte neu laden“), der Entwurf bleibt.
6. Entscheidung: Entwurf bleibt ungespeichert oder wird als REG-Ticket festgehalten (Sondergegend); offene Fragen zu den Annahmen in B-475 (Zählung fester Ereignis-Objekte, Minimum oder Maximum bei „kein Platz mehr“), WZ8.4 (Vorschau ohne Option = keine Planung) und der neuen Abhängigkeit in `tools/k3c-studio/go.mod` (aus WZ7, `replace k3c`, `coder/websocket`).

Hinweis des Reviews (2026-10-17): keine schweren Befunde; Optik und Bedienung des Editors sind nur durch Tests, nicht im Fenster geprüft.

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

Urteil von 🧑 am PC im Studio-Fenster (EXE von 🧑 selbst gebaut), 2026-10-18, Quelle Chat; Claude hat nicht geurteilt.

- Punkt 1 (festes Portal Wald, Platz +6): ok.
- Punkt 2 (Sondergegend, zwei Seeds, Chunk- und Live-Vorschau): Verhalten ok. Bedienung des Chunk-Editors könnte komfortabler sein (Dropdown zu klein, Schnellfilter zeigt die übrigen Auswahlmöglichkeiten nicht, Icon zum Text wünschenswert) → B-485 (Spec freigegeben, Sprint WZ17).
- Punkt 3 (ungültige Entwürfe): ok.
- Punkt 4 (Speichern, REG-Hinweis, Diff nur `plannedChunks`): ok.
- Punkt 5 (Datei von außen geändert, dann Speichern): beim ersten Durchgang **Verhalten falsch** (nichts gespeichert, keine Meldung) → B-486. Wiederholt mit neuem Build (Korrektur in `usePlan.save`: Ablehnung des Aufrufs zeigt jetzt „Speichern fehlgeschlagen: <Grund>“): **passt** (Urteil von 🧑 im Chat, 2026-10-18; der angezeigte Text wurde nicht zitiert).
- Punkt 6: Sondergegend und Chunk-Anpassungen kommen später und werden gezielt geplant (bleibt Entwurf, kein REG-Ticket jetzt); B-475 erstmal ok; WZ8.4 wie umgesetzt bestätigt (Biom-Vorschau ohne Option = keine Planung, eingebettete Planung erst bei Insel + Stufe); Abhängigkeit in `tools/k3c-studio/go.mod` ok.

AC-10 abgenommen: Punkte 1–5 von 🧑 mit „ok“ beurteilt. Die Korrektur zu B-486 hat ein getrennter Reviewer geprüft (keine schweren Befunde, mittlere Befunde → B-487). B-461 und WZ8 sind `erledigt`.

Neue Tickets: B-485 (Bedienung des Chunk-Editors), B-486 (Speichern schweigt, behoben).
