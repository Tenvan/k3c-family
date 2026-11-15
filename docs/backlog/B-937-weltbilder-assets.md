# B-937 · Jede Welt hat ein großes Titelbild mit Text und ein kleines Bild als Papierschnitt-Asset

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** WG1
- **Projekt:** PAP
- **Erstellt:** 2026-11-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Hinweis 🧑 2026-11-15: Welten (`data/worlds/*.json`) haben keine eigenen Bilder für Spielauswahl und Spielstände.

## Ziel

Je Welt zwei Papierschnitt-Assets: ein großes Titelbild mit Weltnamen als Text (Weltauswahl) und ein kleines Bild (Spielstand, Pausenmenü). Zuordnung in der Welt-Datei oder `data/paper.json`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `public/grafik/**`, `data/paper.json`, `data/worlds/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** S
- **Sperre:** golden

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Assets für die Welt `standard` angelegt (nur über `assets_*`, Entscheidung 005), Review in der Workbench.
- **AC-02** Zuordnung Welt → Bilder in den Daten, Test dafür.

## Offene Fragen

Text im Bild gerendert oder vom Spiel in Schrift darübergelegt? Default: vom Spiel darüber (übersetzbar, schärfer). (🧑)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
