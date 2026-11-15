# B-968 · Zwei Monarchen am selben Ort lösen gemeinsame Koop-Aktionen aus

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

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Zusammenspiel der beiden Monarchen wird bisher nicht eigens belohnt.

## Ziel

Dinge, die nur gemeinsam gehen, belohnen das Zusammenspiel auf der Couch. Erste Koop-Aktion ist die **Doppel-Truhe**: Sie öffnet sich nur, wenn beide Monarchen gleichzeitig davorstehen, und gibt deutlich mehr Gold.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **Erste Aktion (🧑 2026-11-15): Doppel-Truhe.** Besondere Truhe mit zwei Schlössern; sie öffnet sich nur, wenn beide Monarchen gleichzeitig davorstehen, und gibt deutlich mehr Gold als eine normale Truhe.
- **Solo (🧑 2026-11-15): normale Truhe.** Mit einem Spieler erzeugt der Level-Generator an der Stelle eine gewöhnliche Truhe; Solo verliert nichts, der Koop-Bonus läuft über Multiplikator (Basiswerte = Solo).
- **Beute (🧑 2026-11-15): Fontäne für beide.** Die Truhe wirft ihre Münzen als Fontäne aus (wie B-953), beide sammeln, was sie erwischen.
- Steht nur ein Monarch davor, bleibt die Truhe zu und zeigt, dass der zweite fehlt.
- Weitere Koop-Aktionen (gemeinsam bezahlen und bauen, gemeinsamer Angriff) sind spätere Kandidaten, nicht Teil dieses Tickets.

## Nicht-Ziele

Gemeinsames Bauen, gemeinsamer Angriff, besondere Beute (Ausrüstung, Material), Truppe als zweiter Schlüssel.

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/**`, `engine/level/**`, `engine/sim/**`, `data/**`, `public/grafik/**`, `native/k3c-game/**` (vorläufig)
- **Abhängig von:** –
- **Braucht 🧑:** TV-Abnahme
- **Größe:** M (vermutet)
- **Sperre:** –
- Berührt Spielregeln und Level-Generator; Goldmenge und Häufigkeit später über Balancing (derzeit gesperrt). Sim-Code im eigenen Worktree (B-925). Asset über `assets_*`.

## Beispiele

Koop: Monarch 1 steht an der Doppel-Truhe, ein Schloss leuchtet; Monarch 2 kommt dazu → Truhe springt auf, Fontäne, beide sammeln. Solo: an derselben Stelle steht eine normale Truhe.

## Ausnahme- und Fehlerfälle

Ein Spieler verlässt das Spiel, während Doppel-Truhen ungeöffnet sind → Verhalten offen (siehe Offene Fragen). Monarch ohne Krone → offen.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

- Wie viel Gold, wie viele Doppel-Truhen je Insel oder Stufe?
- Was wird aus ungeöffneten Doppel-Truhen, wenn ein Spieler geht oder dazukommt?
- Mehr als 2 Spieler: reichen zwei Monarchen?

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt; Akzeptanzkriterien fehlen noch, Spec bleibt Entwurf.
