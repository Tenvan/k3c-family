# B-436 · plan_create scheitert an Platzhalter-Zeilen „– (Entwurf)“ in der Session-Tabelle

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** WZ12
- **Projekt:** WZG
- **Erstellt:** 2026-10-16
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-17, Ralf im Chat (Revision 1)

## Ausgangslage

Ein Sprint-Entwurf (GEB1) führte seine Sessions in der Tabelle mit Datei `– (Entwurf)`. `plan_create` für GEB1.1 brach ab mit `open …\GEB1-grundregeln-werkstatt\– (Entwurf): The system cannot find the file specified.` Das Tool hält die Platzhalter-Zeile für eine bestehende Session und will die Datei öffnen. Workaround 2026-10-16: Zeilen von Hand entfernt, dann lief `plan_create`.

## Ziel

`plan_create` ersetzt eine Platzhalter-Zeile ohne Datei (`–`, `– (Entwurf)`) durch die neue Session-Zeile, statt abzubrechen.

## Beteiligte und Zielgruppen

🧑 (Ralf) plant mit `plan_create` und weiteren Planungs-Tools und stößt auf den Fehler; Agenten legen Sessions an; Reviewer prüfen den Diff.

## Anforderungen

- `plan_create` für eine Session erkennt in der Session-Tabelle des Sprints Platzhalter-Zeilen: Zeilen, deren Datei-Zelle `–` oder `– (…)` lautet (keine Datei).
- Hat eine Platzhalter-Zeile die Nummer der neuen Session, ersetzt die neue Zeile sie; andere Platzhalter-Zeilen bleiben stehen (Annahme der Planung, ungeprüft; im Test festhalten).
- Eine Zeile mit Datei, die nicht existiert, bleibt ein Fehler wie heute.
- Domäne DEV (`tools/k3c-dev/internal/planning/`, `tables.go`, `create.go`); Tests nur auf Kopien (`t.TempDir()`); Funktion ≤ 60, Datei ≤ 400 Zeilen.

## Nicht-Ziele

Änderungen am Worktree-Vergleich (`wtdiff.go`, B-465); Format der Session-Tabelle ändern; andere Planungs-Tools; echte Sprint-Dateien in Tests anfassen.

## Regeln und Einschränkungen

Planung nur über die Tools (`docs/arbeitsweise.md`); Domäne DEV; Komplexitäts-Budget (Datei ≤ 400, Funktion ≤ 60 Zeilen); kein Datenverlust in `docs/` (keine Zeile geht beim Ersetzen verloren, die nicht Platzhalter ist).

## Beispiele

Sprint-Entwurf mit der Zeile `| GEB1.1 | – (Entwurf) | … |` → `plan_create` für GEB1.1 legt die Datei an und ersetzt die Zeile durch `| GEB1.1 | `GEB1.1-….md` | … |`. Eine Platzhalter-Zeile `GEB1.2` bleibt dabei stehen.

## Ausnahme- und Fehlerfälle

Zeile mit Datei-Name, die auf der Platte fehlt → weiterhin Fehler (nur Zeilen ohne Datei sind Platzhalter). Keine passende Platzhalter-Zeile → neue Zeile wird angehängt wie heute.

## Akzeptanzkriterien

- **AC-01** `plan_create` für eine Session ersetzt Platzhalter-Zeilen ohne Datei (`–`, `– (Entwurf)`) mit der Nummer der neuen Session durch die neue Zeile statt abzubrechen; Go-Test auf einer Kopie.
- **AC-02** Eine Platzhalter-Zeile mit anderer Nummer bleibt stehen; eine Zeile mit fehlender Datei bleibt ein Fehler; je ein Go-Test.

## Offene Fragen

Nicht blockierend, Vorschlag von Claude (ungeprüft): Platzhalter-Zeilen mit anderer Nummer bleiben stehen (Annahme, siehe Anforderungen). Entscheidet 🧑 anders, ändert sich AC-02 vor der Freigabe.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
