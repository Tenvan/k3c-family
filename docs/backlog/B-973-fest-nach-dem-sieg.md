# B-973 · Nach dem Sieg über den Endgegner feiert das Volk ein Fest

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** SI1
- **Projekt:** KMP
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Der Sieg über den Endgegner öffnet den Inselwechsel, wird aber nicht gefeiert (ungeprüft, was das Spiel heute zeigt).

## Ziel

Mit dem Sieg über den Endgegner beginnt im Hub ein Fest im Stil der Welt; es dauert, bis das Schiff ablegt. Der Sieg fühlt sich endgültig an.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **Gestalt (🧑 2026-11-15): je Welt eigenes Fest.** Sengoku lässt Laternen steigen, am Nil brennen Fackeln und Blüten treiben auf dem Wasser, im Polar leuchtet Nordlicht über Lagerfeuern. Die Bürger jubeln und tanzen in allen Welten gleich; nur der Himmels-Effekt ist je Welt eigen.
- **Ablauf (🧑 2026-11-15): bis zur Abfahrt.** Das Fest beginnt mit dem Sieg im Hub und dauert, bis das Schiff ablegt; die Spieler bauen und beladen das Schiff in Ruhe (B-967).
- **Friedliche Insel (🧑 2026-11-15): Mit dem Sieg über den Endgegner enden alle Angriffe auf der Insel.** Neue Regel in `docs/rules/`; gilt bis zur Abfahrt.
- Dazu eine kurze Festmusik je Welt.
- Funktioniert mit 2 Spielern; beide Hälften zeigen das Fest.

## Nicht-Ziele

Ein Fest für alle Welten, Fest nur für eine Nacht, kurzer Jubel-Moment ohne Zustand, Belohnungen durch das Fest.

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/**`, `engine/sim/**`, `native/k3c-game/**`, `public/grafik/**`, `public/audio/**`, `data/sounds.json` (vorläufig)
- **Abhängig von:** –
- **Braucht 🧑:** TV-Abnahme
- **Größe:** L (vermutet)
- **Sperre:** –
- Regeländerung: In `docs/rules/` steht bisher keine Regel, dass Angriffe mit dem Sieg enden (gesucht 2026-11-15, Code ungeprüft). Berührt die Sim (eigener Worktree, B-925), Golden und den Korridor „Schiff fertig“ (`zielkorridore.md` § 6: bis 1 Tag nach dem Sieg). Assets über `assets_*`. Vermutlich zu teilen in Regel und Sim sowie Darstellung.

## Beispiele

Sengoku, Insel 1: Der Endgegner fällt. Im Hub steigen Laternen auf, die Bürger tanzen, Festmusik spielt. Die Monarchen bauen das Segel fertig und beladen das Schiff; mit dem Ablegen endet das Fest.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

- Was passiert mit Gegnern, die beim Sieg noch auf der Insel sind (fliehen, verschwinden), und mit offenen Portalen?
- Gilt der Frieden für alle Stufen der Insel?
- Ohne Zeitdruck nach dem Sieg: Bleibt der Korridor „Schiff fertig“ sinnvoll, laufen Wetter (B-969) und Events weiter?
- Arbeiten Bauern und Handwerker während des Fests weiter (Schiffsbau)?

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt; Akzeptanzkriterien fehlen noch, Spec bleibt Entwurf. Die ursprüngliche Annahme „ohne Regelwirkung“ ist überholt: 🧑 hat die friedliche Insel als Regel entschieden.
