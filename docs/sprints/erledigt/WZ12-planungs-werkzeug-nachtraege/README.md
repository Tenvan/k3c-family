# WZ12 · DEV · Planungs-Werkzeug: Platzhalter-Zeilen und Worktree-Pills

- **Status:** erledigt
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-436, B-467
- **Start-Commit:** f6912198
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-17, Ralf im Chat (Revision 1)

## Ausgangslage

`B-436 › Ausgangslage`: `plan_create` bricht ab, wenn die Session-Tabelle eines Sprint-Entwurfs Platzhalter-Zeilen ohne Datei (`– (Entwurf)`) führt. `B-467 › Ausgangslage`: Die Planungsseite zeigt Änderungen aus Worktrees als lange Leiste (`WorktreeBar`) oben statt am betroffenen Eintrag.

## Ziel

`plan_create` ersetzt Platzhalter-Zeilen ohne Datei durch die neue Session-Zeile, und die Planungsseite zeigt Worktree-Änderungen als Pill am Projekt, Sprint, an der Session oder am Ticket statt als Leiste. Am Ende sichtbar: `task check:dev` grün, die Planungsseite ohne Worktree-Leiste, Pills mit Tooltip (Branch und Änderung) im Mock; von 🧑 am PC abgenommen.

## Beteiligte und Zielgruppen

🧑 (Ralf) plant mit mehreren parallelen Worktrees und gibt die Spec frei; Agenten setzen um (DEV); ein getrennter Reviewer prüft.

## Anforderungen

`B-436 › Ziel` und `B-467 › Anforderungen`. Sprint-eigen: Beide Tickets sind Spec-Entwürfe (Abschnitte Anforderungen, Beispiele und Fehlerfälle in B-436 noch leer); sie werden vor der Freigabe vervollständigt. Domäne DEV, Daten für die Pills aus dem bestehenden Workflow-Check (`WorktreeCheck`, `internal/planning/wtdiff.go`).

## Nicht-Ziele

Änderungen am Vergleich selbst (Merge-Base, `wtdiff`) und Lint-Befunde dort (B-465); andere Planungs-Tools.

## Regeln und Einschränkungen

Domäne DEV (`tools/k3c-dev/`); Funktion ≤ 60, Datei ≤ 400 Zeilen; keine neue Abhängigkeit; Planung nur über die Tools, Tests auf Kopien.

## Beispiele

Sprint-Entwurf mit Zeile `– (Entwurf)` → `plan_create` für GEB1.1 legt die Session an und ersetzt die Zeile. Worktree `sprint/wz6` setzt WZ6.2 auf „in Arbeit“ → an der Session WZ6.2 steht eine Pill, Tooltip „sprint/wz6: Status offen → in Arbeit“.

## Ausnahme- und Fehlerfälle

Workflow-Check scheitert → keine Pills, Hinweis wie heute. Platzhalter-Zeile mit Datei, die fehlt → weiterhin Fehler (nur Zeilen ohne Datei sind Platzhalter).

## Akzeptanzkriterien

- **AC-01** `plan_create` ersetzt Platzhalter-Zeilen ohne Datei (`–`, `– (Entwurf)`) durch die neue Session-Zeile statt abzubrechen; Go-Test auf einer Kopie (`B-436/AC-01`); eine Platzhalter-Zeile mit anderer Nummer bleibt stehen, eine Zeile mit fehlender Datei bleibt Fehler (`B-436/AC-02`).
- **AC-02** Die Planungsseite zeigt keine Worktree-Leiste mit Änderungs-Chips mehr (`B-467/AC-01`).
- **AC-03** Projekt, Sprint, Session und Ticket mit Änderung in einem Worktree tragen eine Worktree-Pill, Tooltip mit Branch und Änderung(en) (`B-467/AC-02`).
- **AC-04** Neu im Worktree angelegte Einträge bleiben sichtbar (`B-467/AC-03`).
- **AC-05** Komponententest in `task test`, `task check:dev` grün (`B-467/AC-04`).
- **AC-06** 🧑 nimmt die Planungsseite mit Pills am PC ab.

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat): Einträge, die nur im Worktree existieren (neues Ticket, neue Session), stehen in der normalen Liste an ihrem üblichen Ort (Ticket in der Ticket-Liste, Session unter ihrem Sprint) mit einer Pill „neu“ (`B-467 › Offene Fragen`, AC-04).

Vor der Freigabe offen: nichts. `B-436`: Platzhalter-Zeilen mit anderer Nummer bleiben stehen (Annahme der Planung, nicht blockierend, im Ticket festgehalten).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ12.1 | `WZ12.1-platzhalter-zeilen.md` | Umsetzung | autonom | fertig |
| WZ12.2 | `WZ12.2-worktree-pills.md` | Umsetzung | autonom | fertig |
| WZ12.3 | `WZ12.3-review.md` | Review | autonom | fertig |
| WZ12.4 | `WZ12.4-abnahme.md` | Review | Mensch | fertig |

- WZ12.1 (DEV) Platzhalter-Zeilen ohne Datei in `plan_create` und Session-Tabelle behandeln, Go-Test (AC-01).
- WZ12.2 (DEV) Worktree-Pills statt Leiste: Daten aus `WorktreeCheck`, Pill-Baustein, Komponententest (AC-02, AC-03, AC-04, AC-05).
- WZ12.3 (DEV) Review: `task check:dev`, Diff, nur schwere Befunde, PR des Sprints (alle).
- WZ12.4 (DEV) Abnahme durch 🧑 am PC, Agent Mensch (AC-06).

## Abnahme

2026-10-18, Review WZ12.3 (autonom, getrennte Reviewer): AC-01 bis AC-05 geprüft (`task:check`, `dev:test`, `go:lint` grün); AC-06 offen (WZ12.4).
Behoben: Pills veraltet/überholt (Fokus-Neuladen, Antwort-Zähler), Auswahl vor erstem Check, Verdrahtungstests. Tickets: B-483, B-484.
Version: v– vorgeschlagen (reines Entwickler-Werkzeug).
