# GEB8 · DEV · Gebäude-Vorlagen in den Daten-JSONs

- **Status:** geplant
- **Projekt:** GEB
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-441
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Acht Gebäude (GEB1: Werkstatt, Tanne, Pappel, Birke; GEB2: Taverne, Heilplatz, Schmiede, Rüstkammer) liegen als exakte Rechteck-Listen in `public/grafik/gebaeude/*.json`. Ihre Absicht (Motiv, Teile, Materialien) steht nur verstreut in Sprint-Dateien. B-441 verlangt eine Gebäude-Vorlage in jeder JSON (Entscheidung 🧑 2026-10-16, dringend, vor GEB3).

## Ziel

Jede Gebäude-JSON trägt ihre Gebäude-Vorlage (`vorlage`); `tools/gebaeude` verlangt sie und schreibt sie ins `index.json`; die Asset-Vorschau zeigt sie. Am Ende sichtbar: Vorschau mit Motiv und Teilen unter jedem Gebäude, von 🧑 gelesen und abgenommen.

## Beteiligte und Zielgruppen

🧑 liest die Vorlagen und nimmt sie ab. Agenten künftiger GEB-Sprints nutzen sie, um Gebäude zu ändern oder neu zu erzeugen.

## Anforderungen

B-441 › Anforderungen (Format, Pflichtfeld, `index.json`, Vorschau, PNGs unverändert). Sprint-eigen: Die Vorlage jedes Gebäudes wird aus seinen Bausteinen und der Sprint-Doku abgeleitet (GEB1 › Ausgangslage/Beispiele, GEB2 › Anforderungen, Session-Kontexte GEB1.3, GEB2.1, GEB2.2) und beschreibt, was das PNG heute zeigt.

## Nicht-Ziele

Keine Änderung an Grafik oder Bausteinen; keine KI-Erzeugung aus der Vorlage im Asset-Manager (Ziel B-435, später); keine Vorlagen für Pack-Grafik.

## Regeln und Einschränkungen

Domäne DEV: `tools/gebaeude/`, `public/grafik/gebaeude/`, `src/tools/assetvorschau*.ts`, `docs/standards/gebaeude-grafik.md`. Determinismus: PNGs byte-gleich vor und nach dem Sprint. Keine neue Abhängigkeit; Datei ≤ 400, Funktion ≤ 60 Zeilen. Begriff „Gebäude-Vorlage“ laut Glossar.

## Beispiele

- `taverne.json` → `"vorlage": { "motiv": "Fachwerk-Taverne mit rotem Satteldach", "teile": ["Kamin aus Stein rechts hinter dem First", "Satteldach in Stufen, 5,5 × 1,4 Kacheln", "zwei Fenster mit warmem Licht", "Tür mittig mit Stufe", "Fass vor der Wand rechts der Tür", "Hängeschild mit Bierkrug rechts am Giebel"], "materialien": ["Putz", "Pfosten, Balken", "Dachziegel", "Fensterlicht", "Stein"], "stimmung": "einladend, warmes Licht", "fein": "Schindel-Fugen, Sprossen, Türfugen, Fassreifen, Kette" }`.
- `task gebaeude` → `index.json` enthält `vorlage` je Eintrag; PNG-md5 unverändert.

## Ausnahme- und Fehlerfälle

- JSON ohne `vorlage`, oder `motiv`/`teile` leer → `task gebaeude` bricht ab (Datei, Feld), kein PNG geschrieben.
- Unbekanntes Feld in `vorlage` → Abbruch mit Meldung (verhindert Tippfehler).

## Akzeptanzkriterien

- **AC-01** `tools/gebaeude` liest `vorlage` (Felder `motiv`, `teile`, `materialien`, `stimmung`, `fein`), verlangt `motiv` und `teile`, lehnt fehlende Vorlage und unbekannte Felder mit Meldung ab und schreibt die Vorlage ins `index.json`; `go test` deckt Pflicht, Ablehnung und Übernahme ab; Format in `docs/standards/gebaeude-grafik.md` § 5.
- **AC-02** Alle acht Gebäude-JSONs tragen eine Vorlage, die ihr heutiges PNG beschreibt; `task gebaeude` erzeugt byte-gleiche PNGs wie vor dem Sprint.
- **AC-03** Die Asset-Vorschau zeigt unter jedem Gebäude Motiv und Teile der Vorlage (Logik mit Vitest).
- **AC-04** 🧑 hat die Vorlagen in der Vorschau gelesen und abgenommen.

## Offene Fragen

- Entschieden 2026-10-16 (Chat, Ralf): strukturierte Definition statt Prompt, Name `vorlage`, eigener Sprint vor GEB3.
- Freigabe der Spec setzt 🧑.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GEB8.1 | `GEB8.1-vorlage-werkzeug-daten.md` | Umsetzung | autonom | offen |
| GEB8.2 | `GEB8.2-vorlage-vorschau.md` | Umsetzung | autonom | offen |
| GEB8.3 | `GEB8.3-review-abnahme.md` | Review | autonom | offen |

- GEB8.1 Werkzeug verlangt und übernimmt `vorlage`; Vorlagen für alle acht Gebäude (AC-01, AC-02).
- GEB8.2 Vorlage in der Asset-Vorschau (AC-03).
- GEB8.3 Review und Abnahme durch 🧑 (AC-04, alle).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
