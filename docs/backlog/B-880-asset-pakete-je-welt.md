# B-880 · Assets lassen sich zusammen mit einer Welt zu Paketen bündeln

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** PT6
- **Projekt:** PAP
- **Erstellt:** 2026-11-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Ralf, 2026-11-14: „Assets möchte ich zu Paketen mit einer Welt verpacken, usw.“ Heute liegen Papierschnitt-Assets je Gruppe unter `public/grafik/<gruppe>/` (Index `index.paper.json`, Zuordnung `data/paper.json`), Welten als Dateien unter `data/` (B-680). Eine Zusammenstellung „Welt + ihre Assets“ als Einheit gibt es nicht.

## Ziel

Ein Plan liegt vor, wie eine Welt mit ihren Assets (Grafik, Ton, Werte) als Paket gebündelt, ausgeliefert und vom nativen Spiel geladen wird; danach Tickets und Sprints zur Umsetzung.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Claude bereitet autonom vor: Bestandsaufnahme (welche Assets braucht eine Welt, wo steht die Zuordnung), 2–3 Varianten für Paketformat und Ablage mit Vor- und Nachteilen, Empfehlung, Auswirkungen auf `gen:paper`, `game:dist` (B-827) und den Welten-Editor (B-879).
- Ergebnis als Entscheidungsvorlage unter `docs/decisions/` (Entwurf) und als Spec dieses Tickets; Freigabe durch 🧑.

## Nicht-Ziele

Umsetzung vor der Freigabe; Veröffentlichung (PB1).

## Regeln und Einschränkungen

Entscheidung 005 (Assets nur über MCP) und 008 (natives Spiel) gelten.

- **Bereich:** `docs/decisions/**`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** M
- **Sperre:** –

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Entscheidungsvorlage mit Varianten und Empfehlung liegt vor.
- **AC-02** 🧑 hat entschieden; Folge-Tickets sind angelegt.

## Offene Fragen

Was gehört außer Grafik zum Paket (Ton, Werte, Level-Generator-Einstellungen)? Klärt die Vorlage.

## Notizen

Auftrag Ralf im Chat (2026-11-14): erst als Plan aufnehmen, dann autonom vorbereiten.

2026-11-14 PT6.1: Entscheidungsvorlage `docs/decisions/010-asset-pakete-je-welt.md` (Entwurf) liegt vor, AC-01 erfüllt. Empfehlung Variante A (Welt-Manifest, gemeinsamer Pool), C als Ausbau. Ticket bleibt offen, wartet auf Entscheidung von Ralf (AC-02, Fragen in der Vorlage).
