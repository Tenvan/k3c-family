# GR2.3 · Gewählte Assets einbinden, Credits, Zuordnung aktualisieren

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** gr2/3-assets-einbinden
- **Abhängig von:** GR2.2
- **Tickets:** B-162
- **Kriterien:** AC-03, AC-04, AC-05, AC-06

## Ziel

Die gewählten Packs liegen gefiltert unter `public/grafik/<id>/` mit Lizenzdatei, Index- und Credit-Eintrag; die Zuordnungstabelle zeigt für jede entschiedene Lücke „zugeordnet“ oder „kein Treffer, Platzhalter“; alle Tests grün.

## Kontext

- Regeln für Packs: `public/grafik/CREDITS.md` (Kopf: nur PNG der Umgebung und `LICENSE.txt` je Ordner, bzw. Hinweis auf die Lizenzangabe der Quellseite; keine Musik, kein Demo-Code, keine PSD/Aseprite/GIF). Bildliste `public/grafik/index.json` (Felder `pack`, `group`, `file`, `w`, `h`).
- **Tests:** `src/tools/grafikPacks.test.ts` prüft Packs aus `src/tools/grafikPacks.ts` gegen Dateien, Index, `CREDITS.md` und `lizenzen.html` und erwartet heute **genau zwölf** Packs — die Zahl wächst mit den neuen Packs (Test anpassen, nicht lockern). Hat GR6 die Credits auf eine gemeinsame Datenquelle umgestellt, dort eintragen (Ergebnis von GR6.1 lesen).
- Zuordnung: `docs/assets/zuordnung.md` (GR1); Test aus GR1.2 prüft Credit je zugeordnetem Asset und Ticket je Lücke.
- Nachbearbeitung nur Skalieren und Palette (Q13); Dateien sonst unverändert.
- Herunterladen: nur von den in GR2.1 verlinkten Quellen der gewählten Kandidaten; Lizenzdatei mitnehmen.

## Erlaubte Dateien

- `public/grafik/<neue-id>/`, `public/grafik/index.json`, `public/grafik/CREDITS.md`
- `src/tools/grafikPacks.ts`, `src/tools/grafikPacks.test.ts` (Zahl der Packs), ggf. Credits-Datenquelle aus GR6
- `lizenzen.html` (falls noch handgepflegt)
- `docs/assets/zuordnung.md`, Referenzseite aus GR2.1 (Vermerk „eingebunden“)
- `docs/sprints/` (nur Status dieser Session), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Einbau in den Renderer (GR3), Atlas (GR4), neue Suche.

## Schritte

1. Branch anlegen, `Status: in Arbeit`.
2. Je gewähltem Kandidaten: Pack laden, gefiltert ablegen, Lizenzdatei, Index-Einträge, Credits (CC-BY mit Urheber und Quelle).
3. Zuordnungstabelle: entschiedene Lücken auf „zugeordnet“ (Pack, Datei, Stil, Lizenz) oder „kein Treffer, Platzhalter“.
4. Tests anpassen (Zahl der Packs), `task test` und `task check`.
5. Neue Packs auf `grafiken.html` im Browser-Pane ansehen (Screenshot im PR). Ergebnis, `Status: fertig`, Tabelle der Sprint-README.

## Fertig, wenn

- [x] AC-03: Gewählte Assets mit Lizenzdatei, Index- und Credit-Eintrag; `task test` grün.
- [x] AC-04: Zuordnungstabelle zeigt für jede entschiedene Lücke den neuen Status.
- [x] AC-05: `task check` grün.
- [x] AC-06: Nicht gewählte CC0-/CC-BY-Kandidaten als Gruppe `kandidaten` im Bestand, mit Lizenzdatei und Credit, nicht zugeordnet (Erweiterung 🧑 2026-10-04).

## Prüfen

```bash
task test
task check
```

## Ergebnis

Teil 1 (gewählte Packs, AC-03/AC-04) kam schon mit #126 auf `develop`; Teil 2 (AC-06) am 2026-10-04 auf `sprint/gr2`.

- **AC-03:** umgesetzt – 9 gewählte Packs (inkl. `k3c-paletten`) mit `LICENSE.txt`, Index- und Credit-Eintrag; geprüft mit `src/tools/grafikPacks.test.ts`.
- **AC-04:** umgesetzt – `docs/assets/zuordnung-welt.md` und `zuordnung-objekte.md` zeigen je entschiedener Lücke „zugeordnet“ oder „kein Treffer, Platzhalter“ (Plantage, Holz-Stufe, Zauberturm); geprüft mit `src/tools/zuordnung.test.ts`.
- **AC-05:** geprüft – `task check` grün (1130 Tests).
- **AC-06:** umgesetzt – 20 nicht gewählte Kandidaten (17 × CC0, `materials-pack` CC BY 3.0, `plants-and-flowers-pixel-art` und `kyrises-free-16x16-rpg-icon-pack` CC BY 4.0) mit 93 Bildern in der Gruppe `kandidaten`, je mit `LICENSE.txt`, Credit-Zeile und Lizenzhinweis; neuer Test: Kandidaten-Packs nur in Gruppe `kandidaten` und in keiner Zuordnungsdatei. CC-BY-SA (LPC Fruit Trees, 16px Items) bleibt draußen. Pack-Zahl im Test 21 → 41.
- Lizenz `materials-pack`: Ein alter Kommentar auf der Quellseite nennt CC-BY-SA 1.0 in der readme; die geladene readme nennt CC BY 3.0 wie die Seite (Vermerk in `CREDITS.md`).
- Schritt 5 (Ansicht `grafiken.html` im Browser-Pane) nicht gemacht: Browser-Prüfung ohne Freigabe 🧑 für diesen Lauf. Offen für 🧑.
- Hinweis: k3c-dev (`plan_set`, `check_run`) arbeitete aus diesem Worktree auf der Repo-Wurzel; Status und Prüfung deshalb von Hand bzw. per `task check`.
