# GR1.4 · Review und Abnahme des Sprints GR1

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** gr1/4-review
- **Abhängig von:** GR1.3
- **Tickets:** B-161
- **Kriterien:** alle

## Ziel

Der Sprint GR1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft und liegt in `docs/sprints/erledigt/`; B-161 ist archiviert.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `docs/assets/`, einen Test unter `src/tools/` und Tickets. Besonders prüfen: kein Asset ohne Credit zugeordnet; nur CC0 oder CC-BY; Packs, die laut GR1.1 nicht passen, sind Lücken und nicht zugeordnet; der Test liest die echten Daten-Dateien (keine abgeschriebene ID-Liste) und wurde nicht gelockert; der Kopf nennt Q13 und die Bestätigung durch 🧑.

## Erlaubte Dateien

- `docs/assets/`, Test unter `src/tools/` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Formulierungen, neue Assets.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/gr1` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-06 aus den Ergebnissen von GR1.1 bis GR1.3 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag.
5. B-161 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“).
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, PR öffnen.

## Fertig, wenn

- [x] AC-01 bis AC-05 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [x] AC-06: `task check` grün.
- [x] Schwere Befunde sind behoben oder als Ticket angelegt; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
```

## Ergebnis

Leichtes Review (Agent, Sonnet) über `git diff e317292...HEAD`: keine schweren Befunde. Alle 67 zugeordneten Dateipfade existieren unter `public/`, alle Lizenzen CC0 oder CC-BY und in den CREDITS; der Test liest die echten Daten-Dateien und erkennt leere Tabellen. Hinweis ohne Ticket: Die Lizenzspalte wird nicht gegen „nur CC0/CC-BY“ geprüft.

- **AC-01 bis AC-05:** geprüft – Nachweise in GR1.1 bis GR1.3, Test `src/tools/zuordnung.test.ts` grün.
- **AC-06:** geprüft – `task check` grün (1046 Tests).
