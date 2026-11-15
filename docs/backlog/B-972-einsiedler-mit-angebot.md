# B-972 · Auf jeder Insel wohnt eine besondere Figur der Welt mit einem einmaligen Angebot

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

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Es gibt bereits den Händler als Besucher im Hub (tauscht Material gegen Gold, `docs/rules/buerger.md` § 1); eine Figur mit Eigenart der Welt gibt es nicht.

## Ziel

Auf jeder Insel wohnt abseits in einer tieferen Stufe eine besondere Figur der Welt (Schwertschmied, Priesterin, Walfänger). Gegen Münzen zieht sie in den Hub und errichtet dort ein einmaliges Bauwerk der Welt mit einem Dauerbonus.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **Angebot (🧑 2026-11-15): einmaliges Bauwerk.** Die Figur errichtet ein besonderes Gebäude der Welt (Sengoku Dojo, Nil Schrein, Polar Tranhütte). Klar abgegrenzt vom Händler (Material gegen Gold).
- **Ort (🧑 2026-11-15): abseits, in einer tieferen Stufe.** Die Figur lebt in einer eigenen kleinen Hütte weit weg vom Hub, erst ab Stufe 2; man muss sie suchen und den Weg sichern.
- **Bonus (🧑 2026-11-15): ein Dauerbonus je Welt.** Genau ein dauerhafter Vorteil auf dieser Insel: Dojo macht Kämpfer stärker, Schrein heilt Mauern über Nacht, Tranhütte lässt Fackeln und Sicht weiter reichen.
- **Bauplatz (🧑 2026-11-15): im Hub der Oberwelt.** Der Monarch zahlt beim Einsiedler, die Figur zieht in den Hub und baut dort auf einem festen Hub-Platz (`data/hub.json`).
- Einmalig je Insel; funktioniert mit 2 Spielern (wer zahlt, ist egal, der Bonus gilt für die Insel).

## Nicht-Ziele

Segen für eine Nacht, besondere Truppenart, Ausbau des Bauwerks in Stufen, frei gewählter Bauplatz, Besucher im Hub wie der Händler.

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/**`, `engine/sim/**`, `engine/level/**`, `data/hub.json`, `data/buildings.json`, `data/worlds/**`, `public/grafik/**`, `native/k3c-game/**` (vorläufig)
- **Abhängig von:** –
- **Braucht 🧑:** TV-Abnahme
- **Größe:** L (vermutet)
- **Sperre:** spielstand (vermutet)
- Berührt Spielregeln, Level-Generator, Hub-Plätze und Spielstand; Preis und Stärke des Bonus später über Balancing (derzeit gesperrt). Sim-Code im eigenen Worktree (B-925). Assets über `assets_*`.

## Beispiele

Sengoku, Insel 1: In Stufe 2 steht abseits die Hütte des Schwertschmieds. Monarch 2 wirft Münzen ein, der Schmied wandert in den Hub und baut dort den Dojo; ab dann sind die Kämpfer dieser Insel stärker.

## Ausnahme- und Fehlerfälle

Einsiedler wird auf dem Weg in den Hub angegriffen → offen (Bürger sterben nicht). Insel mit nur einer Stufe → offen.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

- Preis und Stärke je Bonus?
- Welcher Hub-Platz, ist dort noch einer frei (`data/hub.json`, ungeprüft)?
- Wo wohnt die Figur auf Inseln mit nur einer Stufe?
- Neue Welten aus dem Editor: Katalog der Figuren und Bauwerke wie beim Wetter?
- Tranhütte: „Sicht weiter“ setzt eine Sichtregel voraus (ungeprüft, hängt an B-969 Schneesturm).

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt; Akzeptanzkriterien fehlen noch, Spec bleibt Entwurf.
