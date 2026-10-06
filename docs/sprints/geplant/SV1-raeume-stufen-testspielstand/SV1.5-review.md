# SV1.5 · Review und Abnahme des Sprints SV1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Branch:** sv1/5-review
- **Abhängig von:** SV1.4
- **Tickets:** B-315, B-199, B-290, B-204
- **Kriterien:** alle

## Ziel

Der Sprint SV1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft, der PR des Sprints ist geöffnet und der Sprint liegt in `docs/sprints/erledigt/`; erledigte Tickets sind archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff). Der Diff berührt `engine/room/`, `engine/net/`, ggf. `engine/store/`, `src/model/biome.ts`, `src/tools/levelApi.ts`, `src/tools/leveltest.ts`, `leveltest.html`. Besonders prüfen: Die Stufen eines neuen Raums kommen aus den Biomen, nicht aus einer festen Liste; geladene Stände behalten ihre Stufen; Test-Räume schließen nur ohne verbundenes oder wartendes Gerät sofort, normale Räume bleiben bei `EmptyFor`; der Voll-Ausbau-Stand ist deterministisch und nur im Dev-Mode erzeugbar; `engine/sim/` ist unverändert; die PLAT-Änderungen bleiben auf die Dateien aus SV1.4 beschränkt (Domänen-Ausnahme); Seiten-Regeln aus `CLAUDE.md` eingehalten (keine Taste B); keine Datei über 400 Zeilen, keine Funktion über 60; Befunde haben Tickets. Die Prüfung der Testseite aus SV1.1 (Schritt 6) steht im Ergebnis; ist B-204 dort offen geblieben, AC-04 als verschoben führen. B-315/AC-03 (Klick am PC) beobachtet 🧑 bei der Abnahme; bis dahin bleibt B-315 offen.

## Erlaubte Dateien

- `engine/room/`, `engine/net/`, `engine/store/`, `src/model/biome.ts`, `src/tools/levelApi.ts`, `src/tools/leveltest.ts`, `leveltest.html` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Umbau von Produktionscode, neue Biome.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/sv1` lesen (alles, was der Sprint ändert, nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-06 aus den Ergebnissen von SV1.1 bis SV1.4 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag (`Version: v… vorgeschlagen (Grund)`) und dem Hinweis auf die offene PC-Beobachtung durch 🧑 (B-315/AC-03).
5. B-199, B-290 und B-204 (falls AC-04 erfüllt) auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“); B-315 erst nach der Beobachtung durch 🧑.
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, committen, `git merge origin/develop`, pushen und den einen PR des Sprints öffnen (Branch `sprint/sv1`, Ziel `develop`).

## Fertig, wenn

- [ ] AC-01 bis AC-06 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt.
- [ ] `task check` und `task check:go` grün; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

–
