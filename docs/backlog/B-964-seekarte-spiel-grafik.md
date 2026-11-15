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
- Assets nur über MCP `assets_*` (Entscheidung 005).
- Funktioniert mit 2 Spielern im Split-Screen.
- Details offen, siehe Offene Fragen (Ideenrunde k3c-07).

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

- Darstellung der Karte (Stil, Vollbild oder Overlay)?
- Inseln als eigene Bilder je Welt oder generisch?
- Bewegt sich das Schiff auf der Karte?

## Notizen

Idee aus der Ideenrunde k3c-07, noch nicht ausgearbeitet.
