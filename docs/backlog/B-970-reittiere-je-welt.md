# B-970 · Jede Welt hat ein eigenes Reittier mit kleiner Eigenheit

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** mittel
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

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Heute reitet jeder Monarch immer das Standard-Pferd (`horse`, Tempo- und Sprintfaktor 1,0; `docs/rules/monarch.md` § 7, Glossar „Reittier“).

## Ziel

Jede Welt hat ein eigenes Reittier mit genau einer Stärke, die in dieser Welt zählt. Es steht auf der Insel an einem Gatter und wird mit Münzen gekauft.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **Eigenheit (🧑 2026-11-15): eine Stärke, passend zur Welt.** Kein Nachteil. Kamel (Nil) sprintet länger, Rentier (Polar) wird im Schnee nicht gebremst, das Sengoku-Pferd sprintet schneller.
- **Freischalten (🧑 2026-11-15): auf der Insel finden und bezahlen.** Das Tier steht irgendwo auf der Insel an einem Gatter; wer Münzen einwirft, steigt um. Jeder Monarch zahlt für sein eigenes.
- **Inselwechsel (🧑 2026-11-15): fährt mit, gilt für die Welt.** Ein gekauftes Reittier bleibt dem Monarchen bis zum Ende der Welt und belegt keinen Frachtplatz (B-967).
- Ohne Kauf reitet der Monarch weiter das Standard-Pferd (`horse`, Faktoren 1,0).
- Funktioniert mit 2 Spielern; die Monarchen können verschiedene Reittiere haben.

## Nicht-Ziele

Nachteile je Reittier, Reittier als reine Grafik, Reittier als Fracht, Reittier als Belohnung nach dem Endgegner.

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/monarch.md`, `engine/sim/**`, `engine/level/**`, `data/**`, `data/worlds/**`, `public/grafik/**`, `native/k3c-game/**` (vorläufig)
- **Abhängig von:** –
- **Braucht 🧑:** TV-Abnahme
- **Größe:** L (vermutet)
- **Sperre:** spielstand (vermutet)
- Berührt Spielregeln (`monarch.md` § 7), Spielstand und das Rig des Monarchen; Preis und Stärke später über Balancing (derzeit gesperrt). Sim-Code im eigenen Worktree (B-925). Assets über `assets_*`.

## Beispiele

Nil, Insel 1: Monarch 1 findet das Kamel am Gatter, wirft Münzen ein und steigt um; sein Sprint hält länger. Monarch 2 bleibt auf dem Standard-Pferd. Auf Insel 2 reitet Monarch 1 weiter das Kamel.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

- Preis und Stärke je Reittier?
- Wo steht das Gatter (Stufe, Abstand zum Hub), auf jeder Insel oder nur auf der ersten?
- Die Schnee-Bremse für das Rentier gibt es heute nicht (ungeprüft); kommt sie mit dem Wetter (B-969)?
- Neue Welten aus dem Editor: Reittier-Katalog wie beim Wetter?

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt; Akzeptanzkriterien fehlen noch, Spec bleibt Entwurf.
