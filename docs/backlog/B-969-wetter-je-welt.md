# B-969 · Jede Welt hat ein eigenes Wetter-Ereignis mit Spielwirkung

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

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Welten unterscheiden sich bisher über Biome und Grafik, nicht über Ereignisse.

## Ziel

Jede Welt hat ein eigenes Wetter-Ereignis mit Spielwirkung: Sengoku Taifun, Nil Flut, Polar Schneesturm. Kurz, angekündigt, verändert für eine Nacht Sicht, Tempo oder Wege.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **Wirkung (🧑 2026-11-15): je Welt eine eigene.** Taifun (Sengoku) bremst alle und drückt Pfeile zur Seite; Flut (Nil) setzt tiefe Abschnitte unter Wasser und sperrt Wege; Schneesturm (Polar) nimmt die Sicht. Das Wetter trifft Spieler und Gegner gleich.
- **Zeitpunkt (🧑 2026-11-15): selten, einen Tag vorher angekündigt.** Etwa alle paar Tage, deterministisch aus dem Seed (`engine/rng`); am Vortag zeigt der Himmel Vorboten (Wolken, steigendes Wasser). Dauer eine Nacht.
- **Ablage (🧑 2026-11-15): Wetter-Katalog, die Welt wählt eins.** Ein kleiner Katalog fester Wetterarten mit Regel und Grafik; die Welt-Datei nennt nur die Art oder „keins“. Neue Welten wählen im Welten-Editor aus der Liste.
- Funktioniert mit 2 Spielern gleichzeitig, auch wenn sie auf verschiedenen Stufen sind.

## Nicht-Ziele

Wetter je Biom, frei einstellbare Wirkung je Welt, Belohnungen durch Wetter (angespülte Truhen, umgeworfene Bäume), Wetter als reine Kulisse.

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/**`, `engine/sim/**`, `data/**`, `data/worlds/**`, `native/k3c-game/**`, `tools/k3c-workbench/**` (vorläufig)
- **Abhängig von:** –
- **Braucht 🧑:** TV-Abnahme
- **Größe:** L (vermutet)
- **Sperre:** spielstand (vermutet)
- Berührt Spielregeln, Sim und das Format der Welt-Datei; Stärke und Häufigkeit später über Balancing (derzeit gesperrt). Sim-Code im eigenen Worktree (B-925).

## Beispiele

Sengoku, Tag 6: Am Abend ziehen dunkle Wolken auf. Nacht 7: Taifun, Monarchen, Truppen und Gegner sind langsamer, Pfeile treiben ab. Am Morgen ist er vorbei.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

- Verhältnis zum vorhandenen Event-System (`events.json`): eigenes System oder Event-Art (ungeprüft)?
- Gilt das Wetter für die ganze Insel oder je Stufe?
- Stärke der Wirkungen und Abstand in Tagen?
- Wie fällt die Flut auf Inseln ohne tiefe Abschnitte aus?

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt; Akzeptanzkriterien fehlen noch, Spec bleibt Entwurf. Vermutlich in mehrere Tickets zu teilen (Regel und Sim, Grafik, Editor).
