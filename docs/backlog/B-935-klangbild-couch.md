# B-935 · Klangbild für Couch-Abende festlegen und konsolige Piep-Sounds ersetzen

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** offen
- **Sprint:** SR1
- **Projekt:** SND
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Live-Test 🧑 2026-11-15: Der Klang ist zu konsolenlastig, mit Pieptönen und zu schnell, und für gemütliche Couch-Abende unangenehm. Die Slots in `data/sounds.json` nutzen viele Kenney-Kandidaten aus `digital-audio` und `interface-sounds` (z. B. `sfx:warn`).

## Ziel

Ein schriftliches Klangbild (warm, akustisch, Holz, Stoff, Glocken, gedämpft, langsam) und alle Slots, die dagegen verstoßen, durch ruhigere Kandidaten ersetzt.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Klangbild als Abschnitt in `docs/rules/` oder bei den Sound-Regeln, mit Ausschlüssen (Synth-Piepen, 8-Bit, schrille Hochfrequenz).
- Liste der Slots, die verstoßen, und je Slot ein Ersatz-Kandidat mit Lizenz.
- Lautstärke und Ducking gedämpfter, Werte in `data/sounds.json`.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

- **Bereich:** `data/sounds.json`, `public/audio/**`, `tools/soundatlas/**`, `docs/rules/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Klangbild steht schriftlich.
- **AC-02** Kein Slot nutzt mehr einen Kandidaten aus den ausgeschlossenen Kategorien (Test über `data/sounds.json`).
- **AC-03** Abhören am TV durch 🧑.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
- Spieltest 2026-11-22-11, Befund f5: „Okay, der Sound für die Truhe. Die Fontäne, die ist ja auch ganz komisch. Da gehört auch was Besseres hin.“ (Insel 1 Stufe 0, Welt standard, Tag, Tag 1)
