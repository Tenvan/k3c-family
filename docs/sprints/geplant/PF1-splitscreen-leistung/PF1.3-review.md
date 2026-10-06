# PF1.3 · Review

- **Status:** offen
- **Typ:** Review
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** pf1/3-review
- **Abhängig von:** PF1.2
- **Tickets:** B-194
- **Kriterien:** alle

## Ziel

Der Sprint ist leicht geprüft, schwere Befunde sind behoben oder als Ticket angelegt, und der PR des Sprints ist offen. Die Xbox-Messung (PF1.4) folgt danach am Gerät.

## Kontext

Leichter Review (nur schwere Befunde im Diff, günstiges Modell). Der Diff berührt `src/scenes/` (Kameras, Nacht-Filter, Debug-Overlay) und `docs/backlog/B-194-*.md`. Besonders prüfen: Split-Screen funktioniert für beide Spieler unverändert (eigene Stufe, Kamera-Folge, Nacht-Abdunklung); keine Spiel-Logik im Client (`src/scenes/noSim.test.ts`); B nicht belegt, View + Menu nicht anders belegt; `src/scenes/GameScene.ts` nicht gewachsen (vorher 403 Zeilen); vorher/nachher-Werte aus PF1.2 sind plausibel und in B-194 eingetragen; keine Datei über 400 Zeilen, keine Funktion über 60; Befunde haben Tickets. AC-01 wird erst in PF1.4 an der Xbox geprüft und bleibt hier offen.

## Erlaubte Dateien

- `src/scenes/` nur für schwere Befunde
- `docs/sprints/`, `docs/backlog/`, `docs/roadmap.md`

## Nicht-Ziele

Stil, weitere Optimierung, Messung an der Xbox (PF1.4).

## Schritte

1. Branch anlegen, `Status: in Arbeit`. `task check` grün.
2. `git fetch && git diff origin/develop...origin/sprint/pf1` lesen (nur den Diff), Befunde nach `docs/arbeitsweise.md` behandeln.
3. Nachweis je Kriterium AC-02 bis AC-04 aus den Ergebnissen von PF1.1 und PF1.2 prüfen; AC-01 als „offen bis PF1.4“ vermerken.
4. Abnahme (höchstens fünf Zeilen) in die Sprint-README schreiben, mit Versionsvorschlag (`Version: v… vorgeschlagen (Grund)`).
5. Sprint bleibt aktiv, solange PF1.4 offen ist (B-194 bleibt `eingeplant`); committen, `git merge origin/develop`, pushen und den einen PR des Sprints öffnen (Branch `sprint/pf1`, Ziel `develop`).

## Fertig, wenn

- [ ] AC-02 bis AC-04 haben einen Nachweis im Ergebnis der jeweiligen Session oder sind mit Grund und Ticket verschoben; AC-01 ist als offen bis PF1.4 vermerkt.
- [ ] Schwere Befunde sind behoben oder als Ticket angelegt.
- [ ] `task check` grün; PR des Sprints ist offen.

## Prüfen

```bash
task check
```

## Ergebnis

–
