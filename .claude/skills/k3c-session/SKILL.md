---
name: k3c-session
description: Schlanker Ablauf für eine K3C-Session (Sprint-Session umsetzen, Review-Session, Planungs-Session) mit plan_next, parallelen Agenten, gezielten Tests, einem check:diff und Review nur bei Risiko. Use when Ralf "nächste Session", "weiter mit <Sprint>", "Session <ID> umsetzen", "Review-Session" oder "autonom weiterarbeiten" sagt. Nicht für reine Fragen oder Ticket-Anlage ohne Umsetzung.
---

# K3C-Session

Regelquelle ist `docs/arbeitsweise.md` (Autonomer Ablauf, Tempo im Lauf). Dieser Skill ist die Kurzfassung für den Lauf;
die ganze Datei nur bei Regelfragen lesen.

## 1. Session wählen (ein Aufruf)

1. `git fetch --prune`, dann `task ops:aufraeumen`.
2. `plan_next` (k3c-workbench) liefert Session, Pfad, Branch-Präfix, Kriterien und „Parallel möglich“.
   Ohne k3c-workbench: `docs/arbeitsweise.md` › Autonomer Ablauf, Schritt 1.
   **Achtung:** `plan_next` liest nur den ausgecheckten Stand. Liegt `origin/sprint/<präfix>` vor, den Status der
   gewählten Session dort prüfen (`git show origin/sprint/<präfix>:<Pfad>`); ist sie nicht `offen`, den Sprint-Branch
   auschecken und `plan_next` erneut fragen (B-519).
3. Sperre über `origin`: `git ls-remote --heads origin 'sprint/*'`; steht dort eine Session derselben Domäne auf
   `in Arbeit`, `plan_next` mit anderer `domain` fragen.
4. Steht unter „Parallel möglich“ eine `offline`-Session einer anderen Domäne, sie Ralf als eigene Session vorschlagen
   (Worktree), nicht nacheinander abarbeiten.

## 2. Beanspruchen

Sprint-Branch auschecken (fehlt er: von `origin/develop`), `plan_set {Status: in Arbeit}`, committen,
`git push -u origin sprint/<präfix>`.

## 3. Umsetzen in parallelen Spuren

- Unabhängige Teile gleichzeitig starten (eine Nachricht, mehrere Agenten): z. B. `Explore` für Kontext ∥
  `testing-red` für Tests ∥ `executor` für Code in anderen Dateien. Zwei Schreiber nie in derselben Datei.
- Modell: `sonnet` als Standard, `opus` nur für Protokoll, Spielstand, Nebenläufigkeit oder Architektur.
- Nur die **Erlaubten Dateien**; neue Ideen als Ticket (`plan_create` + `plan_sections`), nicht nebenbei.
- Während der Arbeit nur gezielte Tests: `check_run go:test` bzw. `task:test` mit `pattern`, `go test ./pkg -run X`.

## 4. Prüfen und Review (einmal, parallel)

1. `check_run task:check:diff` wartet bis zum Ende und liefert das Ergebnis (B-520); über 2 min macht Claude Code den
   Aufruf selbst zum Hintergrund-Task. Kommt doch „läuft noch“ mit Lauf-ID, `check_run {run}` einmal aufrufen (wartet
   weiter), keine Schleife. Ohne k3c-workbench: `task check:diff` per Bash mit `run_in_background`.
2. **Gleichzeitig** im Hintergrund ein Reviewer, aber nur bei Domäne SIM oder SRV, Protokoll- oder Spielstand-Änderung,
   Shell-, Pfad- oder Eingabe-Code oder ab etwa 150 geänderten Code-Zeilen. Auftrag: nur schwere Befunde (falsches Verhalten, Datenverlust,
   Sicherheit, Determinismus, 2 Spieler, CLAUDE.md-Regeln, Kriterium ohne Nachweis).
3. Schwere Befunde beheben. Fix unter etwa 30 Zeilen: kein zweites Review, nur die betroffenen Tests und
   `check:diff` einmal neu. Roter Lauf: Ursache beheben, einmal wiederholen, nie mehrfach „zur Sicherheit“.

## 5. Abschließen

`plan_sections` mit `Ergebnis` (je Kriterium höchstens zwei Zeilen: `umgesetzt`, `geprüft (womit)`, `verschoben (Ticket)`),
dann `plan_set {Status: fertig}`. Ein Commit mit Session im Titel (`feat(sim): … (WZ14.3)`), `git fetch`,
`git merge origin/develop`, push. Kein PR außer in der letzten Session des Sprints.

## Review-Session

Nur auf dem Sprint-Branch, nie im selben Lauf wie eine Umsetzung: `task check`, `task check:go`, `task check:race`
**nacheinander** (Go-Wanduhr-Tests flackern unter Last, B-284), währenddessen den Diff `origin/develop...origin/sprint/<präfix>` lesen; Abnahme in höchstens
fünf Zeilen, PR mit Auto-Merge (`docs/arbeitsweise.md` › Review-Session).

## Feste Regeln

- Eine Session pro Lauf; **Nicht-Ziele** der Session einhalten.
- Testläufe (Balancing, Performance, Stabilität) nur über `sim_test`, nie in der Shell.
- **Wenn etwas nicht passt** (Schritt unklar, Befehl scheitert unerklärlich, Datei nicht erlaubt, Kriterium widerspricht
  Code oder Regel): `Status: blockiert`, Grund im Ergebnis, Ticket vom Typ `Frage`, Stand pushen, Entwurfs-PR des
  Sprints (falls keiner offen), **aufhören**.

## Verboten im Lauf

- `sleep`-Schleifen und Polling von Hintergrundläufen; Messreihen über drei Läufe.
- Den vollen Prüfsatz mehrfach je Session.
- Review-Ketten (Review → Fix → Re-Review → …) für kleine Fixes.
- `plan_section` je Abschnitt, wenn mehrere Abschnitte zu füllen sind.
