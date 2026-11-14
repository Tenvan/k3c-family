# B-783 · plan_waves berechnet Wellen aus Abhängigkeit, Bereich und Sperre, plan_queue zeigt die Ralf-Queue

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ44
- **Projekt:** WZG
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

`plan_next` wählt eine Session nach Rang und schlägt höchstens drei weitere vor; kollidiert wird nur über Domäne und `Lane` (`next_lanes.go`). Dateimengen, Sperren und Abhängigkeiten von Tickets und Sprints liest das Werkzeug nicht. Am 2026-11-11 meldete es bei 21 aktiven Sprints „Parallel möglich: keine“. Was 🧑 freigeben, entscheiden oder abnehmen muss, steht verstreut in 20 Mensch-Sessions und den Entwurfs-Specs.

## Ziel

Ein Aufruf liefert die Welle, die jetzt parallel laufen kann, und ein zweiter die Ralf-Queue nach Gerät. Regeln: `docs/arbeitsweise.md` › Parallelität und Wellen.

## Beteiligte und Zielgruppen

Agenten und 🧑 beim Start einer Welle.

## Anforderungen

- Parser für die Zeilen `Bereich`, `Abhängig von`, `Braucht 🧑`, `Größe`, `Sperre` im Abschnitt „Regeln und Einschränkungen“ von Ticket und Sprint-README; Sessions liefern die Dateimenge aus „Erlaubte Dateien“.
- `plan_waves {slots?, project?, domain?}`: Ebene aus Abhängigkeiten, Kollision aus überlappenden Bereichen (Präfixvergleich, Sammeldateien ausgenommen) und gleicher Sperre, Rang entscheidet bei Kollision, höchstens vier Slots; Sessions in Arbeit belegen Slots. Die Antwort nennt je zurückgestelltem Knoten den Grund.
- `plan_queue {device?}`: Freigaben, Entscheidungen, Abnahmen je Gerät, Workshops, Veröffentlichungen.
- `plan_next` nutzt dieselbe Kollisionsprüfung für „Parallel möglich“.
- Unbekannte ID in `Abhängig von` wird als Fehler genannt, nicht still als blockiert gewertet.

## Nicht-Ziele

Sessions starten oder Worktrees anlegen; Feldprüfung (B-784).

## Regeln und Einschränkungen

Domäne DEV; nur lesende Tools; Tool-Beschreibungen enden mit „Nutze es bei: … Statt: …“.

- **Bereich:** `tools/k3c-workbench/internal/planning/**`, `tools/k3c-workbench/internal/mcpsrv/tools_planning*.go`, `tools/k3c-workbench/internal/mcpsrv/instructions.md`, `docs/arbeitsweise.md`
- **Abhängig von:** B-782
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** exe

## Beispiele

Zwei DEV-Tickets mit `…/assets/style/**` und `…/assets/sound/**` stehen in derselben Welle; zwei Tickets mit `tools/paper/cli/gen.ts` nicht.

## Ausnahme- und Fehlerfälle

Ticket ohne `Bereich` → nicht in einer Welle, mit Grund „Bereich fehlt“.

## Akzeptanzkriterien

- **AC-01** `plan_waves` liefert für ein Test-Repo mit Abhängigkeit, überlappendem Bereich und gleicher Sperre die erwarteten Wellen (Go-Test).
- **AC-02** `plan_queue` gruppiert offene Mensch-Sessions nach Gerät und nennt Specs im Entwurf mit `Braucht 🧑: Freigabe` (Go-Test).
- **AC-03** `plan_next` schlägt zwei Sessions derselben Domäne mit disjunkten Erlaubten Dateien als parallel vor (Go-Test).
- **AC-04** Die MCP-Instruktionen der Workbench beschreiben beide Tools.

## Offene Fragen

Woher kommt die Geräteklasse einer Mensch-Session? Default (Claude, 2026-11-12, Meldung im Nachgang): Zeile `Gerät:` im Abschnitt Kontext, sonst „unbekannt“.

## Notizen

Entwurf: Session k3c-03, 2026-11-11.
