# GEB6.6 · Review und Abnahme

- **Status:** blockiert
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** CLI
- **Umgebung:** live
- **Branch:** geb6/6-review-abnahme
- **Abhängig von:** GEB6.5
- **Tickets:** B-435, B-438
- **Kriterien:** AC-03

## Ziel

Der Diff von GEB6 ist geprüft, 🧑 hat Natur und Objekte in der Vorschau je Biom und im Spiel abgenommen, der eine PR des Sprints ist offen.

## Kontext

- Leichter Review: nur schwere Befunde im Diff; besonders ältere PNGs unverändert (`tanne-32.png`, `pappel-32.png`, `birke-32.png` und alle GEB1–GEB5-Gebäude), gelöschte Grafik nirgends mehr genutzt (Erz-Blatt und `icons/8.png` bleiben), Baumwahl deterministisch ohne `Math.random()`, Plantage-Pappel unverändert, Adern zeigen `vein:*`, Datei ≤ 400, Funktion ≤ 60 Zeilen.
- **Sichtprüfung (AC-03):** nur mit Freigabe von 🧑 für diesen Lauf. Vite und Spielserver über `svc_start`, Adressen über `get_urls`; Vorschau `assetvorschau.html?szene=gebaeude` (Bäume, Fels, Adern-Stufenreihe, Truhe, Portal, Ausgang, Lager), Spiel nur, wenn 🧑 auch das freigibt, sonst prüft 🧑 selbst (Wald, Höhle, Mine).
- **Basis:** GEB5 ist vorher auf `develop` gemergt; das Review liest `git diff origin/develop...origin/sprint/geb6`.

## Erlaubte Dateien

- Dateien des Sprint-Diffs, nur für schwere Befunde oder von 🧑 verlangte Nachbesserungen
- `docs/sprints/`, `docs/backlog/`, `docs/projekte/`, `docs/roadmap.md`

## Nicht-Ziele

Stil-Nacharbeit ohne Auftrag von 🧑, GEB7 (Hintergründe), Animationen, TV-Test.

## Schritte

1. `Status: in Arbeit`; `task check` und `task check:go` grün.
2. `git fetch && git diff origin/develop...origin/sprint/geb6` lesen, Befunde behandeln.
3. Mit Freigabe von 🧑: Vorschau (und ggf. Spiel) zeigen, Abnahme im Ergebnis festhalten.
4. Nachweis je Kriterium AC-01 bis AC-06; Abschnitt „Abnahme“ der Sprint-README (höchstens fünf Zeilen, Versionsvorschlag).
5. Fehlt die Abnahme von 🧑: `Status: blockiert` mit Grund im Ergebnis (Arbeitsweise › Wenn etwas nicht passt) und aufhören.
6. Sprint nach `erledigt/`, `Status: erledigt` (nur wenn alle Sessions fertig); PR `sprint/geb6` → `develop` öffnen, Auto-Merge einschalten (Arbeitsweise › Review-Session, Schritt 5).

## Fertig, wenn

- [ ] AC-03: Abnahme durch 🧑 steht im Ergebnis (Vorschau je Biom und Spiel).
- [ ] AC-01, AC-02, AC-04–AC-06: Nachweise aus GEB6.1–GEB6.5 geprüft.
- [ ] `task check` und `task check:go` grün; PR offen, Auto-Merge eingeschaltet.

## Prüfen

```bash
task check
task check:go
```

Manuelle Prüfung im Browser-Pane nur mit Freigabe von 🧑 für diesen Lauf.

## Ergebnis

- **AC-03 blockiert (🧑):** Abnahme der Vorschau je Biom und des Spiels steht aus; niemand außer Agenten hat Bilder gesehen. Nach Arbeitsweise › Wenn etwas nicht passt bleibt die Session `blockiert`, der PR ist ein Entwurfs-PR ohne Auto-Merge.
- **AC-01, AC-02, AC-04, AC-05, AC-06 geprüft:** Ergebnisse GEB6.1–GEB6.5; `task check` und `task check:go` grün (pro Session), Sprint-Diff `origin/develop...HEAD` von einem getrennten Agenten gelesen (Domänen DEV, CLI): keine schweren Befunde, alte Gebäude-PNGs unverändert (nur `index.json` geändert), Baumwahl ohne `Math.random()`, Plantage-Pappel unverändert, Datei ≤ 400 und Funktion ≤ 60 Zeilen.
- **Hinweise:** GEB6 wurde erst nach dem Merge von GEB5 aktiviert (Planungstest: ein aktiver Sprint je Projekt); GEB6.1–6.4 liefen davor in `geplant/` auf `sprint/geb6`. Offene Frage 4 bleibt offen für 🧑 (Annahme umgesetzt, Änderungsweg in der Sprint-README).
