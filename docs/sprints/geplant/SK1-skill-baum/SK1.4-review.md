# SK1.4 · Review und Abnahme des Sprints SK1

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** SIM
- **Umgebung:** offline
- **Branch:** sk1/4-review
- **Abhängig von:** SK1.2, SK1.3
- **Tickets:** B-007, B-270, B-321
- **Kriterien:** alle

## Ziel

Der Sprint SK1 ist nach `docs/arbeitsweise.md` › Review-Session geprüft, der PR des Sprints ist geöffnet und der Sprint liegt in `docs/sprints/erledigt/`; B-270 ist archiviert, B-007 nur, wenn S3.4 abgenommen ist.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt nur `engine/sim/` (Tests und `monarch.go`) und Planungs-Dateien. Besonders prüfen: `CanLearn`/`CanRespec` enthalten die einzige Kopie der Regeln, `LearnSkill`/`Respec` rufen sie auf; Fehlertexte unverändert; Golden-Daten unverändert; Respec ohne gelernte Skills erlaubt (Beschluss 🧑 2026-10-06); keine Zahl im Code, die nach `data/` gehört; keine Datei über 400 Zeilen, keine Funktion über 60. B-007/AC-02 hängt an S3.4 (Gerät): ist S3.4 offen, bleibt B-007 offen und wird im Ergebnis so geführt. Das Nachziehen von `respec` in `engine/net/actions.go` ist SRV und braucht ein Ticket, falls es keins gibt.

## Erlaubte Dateien

- `engine/sim/` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, Optimierung, Umbau von Produktionscode, Client oder Protokoll.

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/sk1` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-01 bis AC-04 aus den Ergebnissen von SK1.1 bis SK1.3 prüfen.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag (`Version: v… vorgeschlagen (Grund)`).
5. B-270 und B-321 auf `erledigt` setzen und nach `docs/backlog/archiv/` verschieben (Index-Zeile in „Archiv“); B-007 ebenso, falls S3.4 abgenommen ist.
6. Sprint-Ordner nach `docs/sprints/erledigt/` verschieben, `Status: erledigt`, Fahrplan in `docs/sprints/README.md` anpassen, committen, `git merge origin/develop`, pushen und den einen PR des Sprints öffnen (Branch `sprint/sk1`, Ziel `develop`).

## Fertig, wenn

- [ ] AC-01 bis AC-04 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt.
- [ ] `task check` und `task check:go` grün; Sprint liegt unter `docs/sprints/erledigt/`.

## Prüfen

```bash
task check
task check:go
```

## Ergebnis

–
