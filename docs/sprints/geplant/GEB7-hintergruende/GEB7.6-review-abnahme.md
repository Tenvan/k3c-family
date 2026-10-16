# GEB7.6 · Review und Abnahme

- **Status:** blockiert
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb7/6-review
- **Abhängig von:** GEB7.5
- **Tickets:** B-435
- **Kriterien:** AC-08

## Ziel

Der Diff von GEB7 ist geprüft, der eine PR des Sprints ist offen; die Abnahme der Hintergründe (AC-08) liegt bei 🧑.

## Kontext

- Leichter Review nach `docs/arbeitsweise.md`: nur schwere Befunde im Diff, getrennter Reviewer; besonders: ältere PNGs unverändert (nur `A` für `hg-*`), gelöschte Grafik nirgends mehr genutzt (auch Fixtures und Mocks unter `tools/k3c-dev`, `site/`), `BACKGROUNDS` deckt alle fünf Biome, Dateien ≤ 400 Zeilen, Funktionen ≤ 60 Zeilen, kein `Math.random()`.
- **Sichtprüfung (AC-08):** gehört 🧑. Ein Agent prüft im Browser nur, wenn die Session sie nennt und 🧑 sie für diesen Lauf freigegeben hat; hier gilt: Claude nimmt **nicht** selbst ab. Vorschau `assetvorschau.html?szene=gebaeude` (Streifen „Hintergründe“), im Spiel je Biom Tiefe 0 bis 4 und der Fluss unter dem Hub; Vite und Spielserver über `svc_start`, Adressen über `get_urls`.
- **Abweichung vom Plan (autonomer Lauf, Spec Revision 2):** Das Wald-Gate (🧑 prüft den Wald vor GEB7.2/7.3) ist entfallen; 🧑 prüft **alle fünf Biome** gemeinsam und gibt Motive, Farben und Dichte frei oder lehnt einzelne Schichten ab (Nachbesserung = Farbtausch in der JSON, kein Code). Zusätzlich offen für 🧑 nach Abschluss: Antworten auf die Fragen 1 bis 9 der Spec (Annahmen), besonders Frage 5 (Pages-Seite umgestellt statt unverändert) und Frage 7 (nur Rechtecke genügen?).

## Erlaubte Dateien

- Dateien des Sprint-Diffs, nur für schwere Befunde oder von 🧑 verlangte Nachbesserungen
- `docs/sprints/`, `docs/backlog/`, `docs/projekte/`, `docs/roadmap.md`

## Nicht-Ziele

Stil-Nacharbeit ohne Auftrag von 🧑, neue Biome, Animationen, Skalierungsfaktor am TV (GR7.4), Paletten-Angleich der Biome (eigenes REG-Ticket).

## Schritte

1. `Status: in Arbeit`; `task check` und `task check:go` grün (`check_run`, mit `checkout`).
2. `git fetch && git diff origin/develop...HEAD` lesen (getrennter Reviewer), Befunde in den Domänen des Sprints beheben, sonst Ticket.
3. Nachweis je Kriterium AC-01 bis AC-07 aus den Session-Ergebnissen GEB7.1–GEB7.5; AC-08 bleibt `offen für 🧑` (neues Ticket zur Abnahme anlegen, wie B-449 bei GEB5).
4. Abschnitt „Abnahme“ der Sprint-README (≤ 5 Zeilen) und `Version: v… vorgeschlagen (Grund)` (Minor: Wirkung im Spiel).
5. Fahrplan; ist nur AC-08 offen, bleibt der Sprint `aktiv` (Hardware entkoppelt: Abnahme durch 🧑), sonst nach `erledigt/`. `git fetch`, `git merge origin/develop`, push, **PR** `sprint/geb7` → `develop` öffnen (Vorlage, eine Zeile je Session), Auto-Merge nicht von Hand erzwingen, wenn die Abnahme noch offen ist; kein Merge durch den Agenten ohne Auftrag.

## Fertig, wenn

- [ ] AC-08: Abnahme durch 🧑 steht im Ergebnis oder ist als offen für 🧑 geführt (Ticket).
- [ ] AC-01 bis AC-07: Nachweise aus GEB7.1–GEB7.5 geprüft; alle Kriterien mit Nachweis im Ergebnis (`umgesetzt`, `geprüft`, `verschoben` mit Ticket); AC-08 als offen für 🧑 geführt.
- [ ] `task check` und `task check:go` grün; PR offen.

## Prüfen

```bash
task check
task check:go
```

Manuelle Prüfung im Browser-Pane nur mit Freigabe von 🧑 für diesen Lauf.

## Ergebnis

- **Prüfen:** `check_run` `task:check`, `task:check:go`, `dev:test` und `task build` grün (nach `git merge origin/develop` mit GEB6; Worktree brauchte `npm ci` und `npm run build` im k3c-dev-Frontend).
- **Review:** `git diff origin/develop...HEAD` von einem vom Schreiber getrennten Reviewer gelesen: keine schweren Befunde (ältere PNGs unverändert, nur `A` für 15 × `hg-*` in 16 und 32 px; gelöschte Grafik nirgends mehr genutzt; kein `Math.random`; `BACKGROUNDS` deckt alle fünf Biome).
- **AC-01 bis AC-07 geprüft:** Nachweise aus GEB7.1–GEB7.5 (Session-Ergebnisse).
- **AC-08 blockiert (🧑):** Abnahme von Vorschau, Spiel und Pages-Seite steht aus; niemand außer Agenten hat Bilder gesehen. Claude hat die Vorschau im Browser nur technisch abgefragt (Streifen „Hintergründe“, zehn Figuren) und den Wald im Screenshot gesehen, nicht abgenommen. Ticket B-450; Session `blockiert`, PR als Entwurf ohne Auto-Merge (Arbeitsweise › Wenn etwas nicht passt).
- **Abweichungen:** GEB7 liegt wieder in `geplant/` (GEB6 noch `aktiv`, ein aktiver Sprint je Projekt); PR #319 (GEB6) wurde auf Anweisung von 🧑 aus dem Entwurf gemergt.
- Neue Tickets: B-450.
