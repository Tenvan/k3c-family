# B-441 · Gebäude-JSONs tragen ihre Gebäude-Vorlage (`vorlage`)

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** GEB8
- **Projekt:** GEB
- **Erstellt:** 2026-10-16
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-16, Chat (Ralf), mit Sprint GEB8 Revision 1

## Ausgangslage

Gebäude entstehen in `public/grafik/gebaeude/<id>.json` als exakte Rechteck-Liste; `tools/gebaeude` rendert daraus deterministisch die PNGs. Die Motiv-Beschreibung (was das Gebäude zeigt, welche Teile, welche Stimmung) steht aber nur verstreut in Sprint-READMEs und Session-Kontexten (z. B. GEB2 › Anforderungen). Ändert eine spätere KI-Session ein Gebäude oder baut eine Variante, fehlt ihr die Absicht neben den Koordinaten (Frage Ralf, 2026-10-16, Chat).

## Ziel

Jede Gebäude-JSON trägt eine **Gebäude-Vorlage** (Feld `vorlage`, Glossar), aus der eine spätere Session das Gebäude annähernd gleich neu erzeugen oder ändern kann. `tools/gebaeude` verlangt sie, übernimmt sie ins `index.json`, und die Asset-Vorschau zeigt sie. Alle vorhandenen Gebäude (GEB1: Werkstatt, Tanne, Pappel, Birke; GEB2: Taverne, Heilplatz, Schmiede, Rüstkammer) bekommen ihre Vorlage nachgetragen.

Entschieden 2026-10-16 (Chat, Ralf): strukturierte Definition statt Freitext-Prompt, Name `vorlage` (nicht „Spec“, das ist im Projekt die SDD-Spec); eigener Sprint im Projekt GEB vor GEB3, dringend.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Format (Vorschlag, endgültig in der Regeldatei § 5): `"vorlage": { "motiv": "<ein Satz>", "teile": ["<Teil, Lage, Größe in Kacheln>", …], "materialien": ["<Material aus § 3>", …], "stimmung": "<Licht, Farbwirkung>", "fein": "<was bei 16 px entfällt>" }`; Teile von links nach rechts und hinten nach vorn.
- Pflichtfeld: Fehlt `vorlage` oder ist `motiv`/`teile` leer, bricht `task gebaeude` mit Meldung (Datei, Feld) ab.
- `index.json` enthält die Vorlage je Gebäude; die Vorschau zeigt sie unter dem Gebäude (Motiv, Teile).
- Die Vorlagen beschreiben den heutigen Stand der PNGs, abgeleitet aus Bausteinen und Sprint-Doku (GEB1, GEB2), ohne die Grafik zu ändern: gleiche PNGs vorher/nachher.

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
