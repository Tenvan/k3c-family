# B-993 · Das Spiel zeigt Bau und Endboss in jedem Biom, ohne eigenes Asset mit dem der Basisart

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WE6
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Das native Spiel (`native/k3c-game`) zeichnet den Bau und den Endboss nur für die Kristallhöhle (Asset `public/grafik/bosse/kristallherz-waechter`, Texte in `native/k3c-game/hud/hints.go`, `texts.go`). Mit B-981 liegt der Bau in der tiefsten Stufe jeder Insel, und es gibt Endbosse ohne eigenes Asset.

## Ziel

Bau und Endboss sind in jedem Biom sichtbar und bekämpfbar, auch wenn der Endboss noch keine eigene Grafik hat.

## Beteiligte und Zielgruppen

Spieler (2 Spieler, Split-Screen). 🧑 gibt frei und prüft am PC.

## Anforderungen

- Der Bau wird in jedem Biom gezeichnet, in dem die Engine ihn meldet (kein fester Bezug auf die Kristallhöhle).
- Ein Endboss ohne eigenes Asset wird mit dem Asset seiner Basisart (`base` in `data/bosses.json`) vergrößert gezeichnet; der Faktor steht in `data/paper.json` › `skalierung` (vorläufig).
- Name, Lebensbalken und Hinweise nutzen den Namen aus `data/bosses.json`, keine festen Texte je Boss.
- Fehlt ein Asset, loggt das Spiel einmal (`slog`, Emoji nach Regel) und läuft weiter.
- Beide Spieler sehen Bau und Boss in ihrer Bildhälfte; das Spiel zeichnet nur, keine Logik.

## Nicht-Ziele

Eigene Grafik für die neuen Endbosse, neue Animationen, Änderungen an der Engine.

## Regeln und Einschränkungen

Das Rust-Spiel zeichnet nur (CLAUDE.md › Regeln). Texturbudget beachten (Agent `k3c-grafik` prüft bei Bedarf).

Umgehängt auf Rust (B-1143, 2026-11-19): Das Spiel ist `rust/crates/k3c-game`; die Go-Vorlage steht nur im Tag `go-referenz-final` (Entscheidung 013).

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `rust/crates/k3c-game/src/**`, `data/paper.json`
- **Abhängig von:** B-981, RU37
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** –

## Beispiele

Insel endet im Eisenstollen mit dem neuen Endboss der Tiefe 3 → Bau am Ende des Levels, Boss als vergrößerter Lavaschleim mit eigenem Namen im Balken.

## Ausnahme- und Fehlerfälle

Asset der Basisart fehlt ebenfalls → Platzhalter wie bei anderen fehlenden Assets, ein Log-Eintrag, kein Absturz.

## Akzeptanzkriterien

- **AC-01** Test in `native/k3c-game`: Endboss ohne eigenes Asset löst auf das Asset der Basisart auf; Endboss mit Asset nutzt seines.
- **AC-02** Test: Bau wird für jede Tiefe 0–4 gezeichnet, wenn der Zustand ihn meldet.
- **AC-03** Sichtprüfung 🧑 am PC: Insel mit tiefster Stufe ungleich Kristallhöhle zeigt Bau und Boss in beiden Bildhälften.

## Offene Fragen

- Größe des Ersatz-Bosses → Default: 2,5× der Basisart (vorläufig, 🧑 bestätigt).

## Notizen

Zerlegt aus B-959 (Punkt Endboss je Insel-Thema, Anteil Spiel).
