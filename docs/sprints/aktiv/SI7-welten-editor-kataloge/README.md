# SI7 · DEV · Welten-Editor wählt Wetter, Reittier und Figur je Welt

- **Status:** aktiv
- **Projekt:** KMP
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-1003
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

Folge der Sprints SI3, SI4 und SI5: Die Welt-Datei bekommt die optionalen Felder `weather`, `mount` und `hermit` mit IDs aus Katalogen der Engine. 🧑 hat am 2026-11-15 entschieden, dass neue Welten ihr Wetter im Welten-Editor aus der Liste wählen (B-969). Der Editor liegt in `tools/k3c-workbench/frontend/src/world/`, das Backend in `tools/k3c-workbench/internal/world/`.

## Ziel

Der Welten-Editor und die MCP-Tools `world_*` wählen je Welt Wetter, Reittier und Figur aus den Katalogen oder „keins“.

## Beteiligte und Zielgruppen

Nutzer: 🧑 und Agenten beim Anlegen und Pflegen von Welten. Umsetzung: Claude in autonomen Sessions, Fachprüfung durch den Agenten `k3c-workbench`. Freigabe: 🧑.

## Anforderungen

Siehe `B-1003 › Anforderungen`.

## Nicht-Ziele

Kataloge im Editor bearbeiten, Vorschau von Wetter, Reittier oder Bauwerk, Himmels-Effekt des Fests (B-1002). Neubau der Workbench-EXE (macht 🧑 nach dem Merge).

## Regeln und Einschränkungen

- **Bereich:** `tools/k3c-workbench/**`
- **Abhängig von:** SI3, SI4, SI5
- **Braucht 🧑:** Freigabe
- **Sperre:** –
- Kette DEV → Review auf `sprint/si7`. Kein Sim-Code; die Workbench liest Kataloge und Prüfung aus der eingebundenen Engine.
- Oberfläche nach Atomic Design (`docs/arbeitsweise.md` › Oberflächen der Wails-Programme), geprüft von `tests/atomicDesign.test.ts`.

## Beispiele

Welt `britannien` öffnen, Wetter `taifun` wählen, speichern → `data/worlds/britannien.json` hat `"weather": "taifun"`, `world_check` ist grün.

## Ausnahme- und Fehlerfälle

Siehe `B-1003 › Ausnahme- und Fehlerfälle` (ungültiger Bestandswert, leerer Katalog, unbekannte ID bei `world_set`).

## Akzeptanzkriterien

- **AC-01** Backend und MCP-Tools kennen die drei Felder und lehnen unbekannte IDs ab (B-1003/AC-01, AC-02).
- **AC-02** Die Übersicht der Welt zeigt die drei Auswahlfelder und speichert sie (B-1003/AC-03, AC-04, AC-05).
- **AC-03** Review: `check_run task:check` und `task:check:dev` grün, schwere Befunde behoben; ein Abnahme-Ticket entfällt (kein Sim-, Balance- oder Gen-Bezug), das hält die Review-Session im Ergebnis fest.

## Offene Fragen

Keine blockierenden. Default in `B-1003 › Offene Fragen` (vorläufig, 🧑 bestätigt): Der Editor wählt alle drei Kataloge, nicht nur das Wetter.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| SI7.1 | `SI7.1-editor-kataloge.md` | Umsetzung | autonom | offen |
| SI7.2 | `SI7.2-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

–
