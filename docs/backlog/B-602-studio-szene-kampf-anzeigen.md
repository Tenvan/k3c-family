# B-602 · Das Studio zeigt Boss-Leiste, Phasen, Warnkreis und Event-Banner mit Mock-Zuständen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ29
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bei der Abnahme K5.4 (Ralf, 2026-10-24): Boss-Leiste, Phasen-Text, Warnkreis und Event-Banner lassen sich im Studio nicht testen und im Spiel nur schwer erreichen.

## Ziel

Eine Studio-Szene „Kampf-Anzeigen“ zeigt alle Zustände dieser Anzeigen aus Mock-Daten, ohne Spiel und Server.

## Beteiligte und Zielgruppen

Ralf (Abnahme und Gestaltung); Agent DEV.

## Anforderungen

- Szene auf der Grafik-Seite (WZ29, B-598), Steuerung: Boss wählen (`data/bosses.json`), Phase, Lebensanteil, Warnkreis (Radius, Restzeit), Event (Vollmond, Blutmond, Händler-Überfall), Mondanzeige, Spieleranzahl (1–2), Zellengröße.
- Zeichnet mit denselben Funktionen wie das Spiel (`src/scenes/hud`, `kampfView.ts` und die reinen Funktionen aus K5.1), keine zweite Darstellung.
- Zeigt die freizuhaltende Fläche (Home-Button, oben 70 px) als Hilfslinie und die Mindest-Schrift.
- Animationsablauf abspielbar: Boss-Auftritt, Phasenwechsel, Warnkreis läuft ab, Event-Banner erscheint und verschwindet.

## Nicht-Ziele

Neue Anzeigen; Spiel-Logik.

## Regeln und Einschränkungen

Frontend im Studio (`tools/k3c-studio/frontend/src/graphics`), Atomic Design; Mock-Zustände als reine Daten mit Test; läuft offline.

## Beispiele

Boss Phase 2, 40 % Leben, Warnkreis 1,5 s vor Schlag → Leiste, Phasen-Text und Kreis wie im Spiel.

## Ausnahme- und Fehlerfälle

Boss ohne Phasen → Leiste ohne Phasen-Text.

## Akzeptanzkriterien

- **AC-01** Die Szene zeigt Boss-Leiste, Phasen-Text, Warnkreis, Event-Banner und Mondanzeige aus Mock-Daten (Vitest der Mock-Zustände, Beobachtung).
- **AC-02** Die Darstellung nutzt die Spiel-Funktionen (kein Doppel, Test über den Import).
- **AC-03** Ralf nimmt die Anzeigen in der Szene ab und ersetzt damit die Spiel-Prüfung in K5.4.

## Offene Fragen

keine

## Notizen

Aus K5.4, Entscheidung 2026-10-24. Spec: Entwurf. Sprint WZ29 (an B-598 anhängen). Zusammenhang: B-132, B-603.
