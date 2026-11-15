# B-971 · Am Strand treibt eine Flaschenpost mit Geschichtszeile und Hinweis an

- **Domäne:** REG
- **Typ:** Idee
- **Prio:** niedrig
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** GS1
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ideenrunde k3c-07 (2026-11-15), Vorschlag von Claude, von 🧑 ausgewählt. Die Geschichte (B-965) blendet nur an Wendepunkten ein; Erkunden wird davon nicht belohnt.

## Ziel

Alle paar Tage liegt morgens am Strand eine Flaschenpost. Wer sie aufhebt, sieht eine Zeile Kunstsprache und bekommt eine Markierung, die zu einem lohnenden Ort der Insel führt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- **Hinweis (🧑 2026-11-15): Markierung in der Welt.** Die Flasche zeigt einen Ort: versteckte Truhe, Einsiedler (B-972) oder Reittier-Gatter (B-970). Ohne Text verständlich.
- **Erscheinen (🧑 2026-11-15): morgens am Strand, selten.** Alle paar Tage liegt bei Tagesanbruch (`dawn`) eine Flasche an einem der beiden Inselenden auf Stufe 1, deterministisch aus dem Seed (`engine/rng`). Sie glitzert, der Monarch hebt sie im Vorbeireiten auf; bleibt sie liegen, holt die Flut sie am Abend.
- **Markierung (🧑 2026-11-15): Leuchtspur für beide, bis gefunden.** Nach dem Aufheben blendet die Geschichtszeile kurz ein (B-965), danach zeigt ein dezenter Pfeil am Bildrand beider Spielerhälften die Richtung, am Ziel leuchtet der Ort. Die Markierung bleibt, bis ein Monarch dort war; immer nur eine zur Zeit.
- Funktioniert mit 2 Spielern gleichzeitig.

## Nicht-Ziele

Schwäche des Endgegners als Hinweis, feste sammelbare Flaschen je Insel, Flaschen als Folge anderer Ereignisse, Hinweis nur auf der Seekarte.

## Regeln und Einschränkungen

- **Bereich:** `docs/rules/**`, `engine/sim/**`, `data/**`, `public/grafik/**`, `native/k3c-game/hud/**`, `native/k3c-game/view/**` (vorläufig)
- **Abhängig von:** B-965; Ziele Einsiedler und Gatter erst mit B-972 und B-970
- **Braucht 🧑:** TV-Abnahme
- **Größe:** M (vermutet)
- **Sperre:** spielstand (vermutet)
- Berührt Spielregeln und Sim; Häufigkeit später über Balancing (derzeit gesperrt). Sim-Code im eigenen Worktree (B-925). Asset über `assets_*`.

## Beispiele

Sengoku, Tag 5, Tagesanbruch: Am linken Inselende glitzert eine Flasche. Monarch 1 reitet darüber, eine Zeile Kunstsprache blendet ein, in beiden Hälften zeigt ein Pfeil nach rechts unten. Monarch 2 folgt ihm in Stufe 2 und findet die leuchtende Truhe; die Markierung erlischt.

## Ausnahme- und Fehlerfälle

Kein lohnender Ort mehr auf der Insel → offen (keine Flasche oder nur Münzen). Neue Flasche, während eine Markierung läuft → es bleibt bei einer Markierung, Verhalten der neuen Flasche offen.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

Abnahme-Ticket eines Sprints oder Projekts (B-742): Kriterien AC-01 Sim (`sim_test`), AC-02 Balance, AC-03 Gen
(`check_run task:test` samt `gen:paper` und Golden), AC-04 Race (`check_run task:check:race`), Einzelheiten in
`docs/arbeitsweise.md` › Sim, Balance und Gen.

## Offene Fragen

- Abstand in Tagen?
- Gibt es heute versteckte Truhen als Ziel, oder braucht es eine eigene Flaschenpost-Truhe (ungeprüft)?
- Was enthält die Flasche, wenn kein Ziel mehr übrig ist?
- Welche Zeilen der Geschichte: eigene Flaschenpost-Zeilen je Welt?

## Notizen

Grundzüge in der Ideenrunde k3c-07 (2026-11-15) mit 🧑 geklärt; Akzeptanzkriterien fehlen noch, Spec bleibt Entwurf.
