# B-964 · Die Seekarte im Spiel ist eine gestaltete Papierschnitt-Karte statt gezeichneter Formen

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WE4
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

🧑 2026-11-15: Im Welten-Editor entsteht eine Seekarte zur Bearbeitung der Inseln (B-963). Das Spiel bekommt mit B-962 eine Seekarte am Hafen, vorerst aus gezeichneten Formen (dort Nicht-Ziel: Grafik-Assets).

## Ziel

Sobald das Schiff fertig ist, zeigt das Spiel die Seekarte als grafisch schöne Karte im Papierschnitt-Stil: gleiche Inhalte wie B-962 (Etappen, Inseln, abgeschlossen, wählbar, gesperrt), aber gestaltet.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Gestaltete Karte statt gezeichneter Formen; Inhalte und Bedienung bleiben wie in B-962.
- **Stil (🧑 2026-11-15): Diorama mit Weltbildern.** Das Meer besteht aus Papierschichten, jede Insel erscheint als Miniatur aus den Weltbildern ihrer Welt (Sengoku, Nil, Polar).
- **Schiff (🧑 2026-11-15): fährt die Route.** Das Schiff liegt an der aktuellen Insel; nach der Wahl mit A fährt es als kurze Animation über die Papierwellen zur Zielinsel, danach lädt die Insel (dient als Übergang).
- **Miniaturen (🧑 2026-11-15): ein Bild je Welt mit Varianten.** Je Welt ein Miniatur-Satz aus dem Weltbild; Inseln unterscheiden sich über Größe, Tiefe (Stufenzahl) und kleine Aufsätze. Neue Inseln im Editor brauchen kein neues Asset.
- **Anzeige (🧑 2026-11-15): Overlay, Fahrt im Vollbild.** Die Wahl läuft als Overlay in der Hälfte des Spielers am Steg (wie B-962), der andere spielt weiter; erst die Schiffsfahrt nach A übernimmt den ganzen Bildschirm.
- Assets nur über MCP `assets_*` (Entscheidung 005).
- Funktioniert mit 2 Spielern im Split-Screen.

## Nicht-Ziele

Regeln der Zielwahl (B-961), Bedienung der Karte (B-962), Editor-Seekarte (B-963).

## Regeln und Einschränkungen

- **Bereich:** `native/k3c-game/hud/**`, `native/k3c-game/view/**`, `public/grafik/**`, `data/paper.json`
- **Abhängig von:** B-962, Schiff fertig
- **Braucht 🧑:** TV-Abnahme
- **Größe:** ?
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

keine

## Notizen

Idee aus der Ideenrunde k3c-07 (2026-11-15), Grundzüge mit 🧑 geklärt; Akzeptanzkriterien und Größe fehlen noch, Spec bleibt Entwurf.
