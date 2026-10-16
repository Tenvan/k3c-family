# B-441 · Gebäude-JSONs tragen ihre Motiv-Beschreibung im Feld beschreibung

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** GEB
- **Erstellt:** 2026-10-16
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Gebäude entstehen in `public/grafik/gebaeude/<id>.json` als exakte Rechteck-Liste; `tools/gebaeude` rendert daraus deterministisch die PNGs. Die Motiv-Beschreibung (was das Gebäude zeigt, welche Teile, welche Stimmung) steht aber nur verstreut in Sprint-READMEs und Session-Kontexten (z. B. GEB2 › Anforderungen). Ändert eine spätere KI-Session ein Gebäude oder baut eine Variante, fehlt ihr die Absicht neben den Koordinaten (Frage Ralf, 2026-10-16, Chat).

## Ziel

Jede Gebäude-JSON trägt ein Feld `prompt` (Motiv, Teile von links nach rechts, Materialien, Stimmung, was `fein` ist), aus dem eine KI das Gebäude erneut annähernd gleich erzeugen kann. `tools/gebaeude` liest es mit und übernimmt es ins `index.json`; die Asset-Vorschau zeigt es unter dem Gebäude. Die Regeldatei (§ 5 Datenformat) verlangt es, der Begriff „Gebäude-Prompt“ kommt ins Glossar (nicht „Spec“, das ist im Projekt Ticket/Sprint-README). Die vorhandenen Gebäude (Werkstatt, Taverne, Heilplatz, Schmiede, Rüstkammer, drei Bäume) bekommen ihren Prompt nachgetragen.

Entschieden 2026-10-16 (Chat, Ralf): Name `prompt` statt `beschreibung`.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
