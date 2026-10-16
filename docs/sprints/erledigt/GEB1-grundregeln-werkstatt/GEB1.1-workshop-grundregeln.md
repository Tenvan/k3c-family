# GEB1.1 · Grundregeln und Skalierung klären

- **Status:** fertig
- **Typ:** Workshop
- **Agent:** Mensch
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** geb1/1-workshop
- **Abhängig von:** –
- **Tickets:** B-435
- **Kriterien:** AC-01

## Ziel

🧑 legt die offenen Grundsatzfragen für Gebäude-Grafik fest (Skalierung am TV, Pixelgröße relativ zu den Figuren, Ort des Generators), damit GEB1.2 und GEB1.3 ohne Rückfragen laufen.

## Kontext

- Fragen: Sprint-README › Offene Fragen (Skalierung 1–4). Die Figuren laufen heute mit Quell-Sprites ×1,25 bis ×3,25 (`data/sprites.json`), die heutigen Gebäude-Packs mit 16 px ×2 (`docs/assets/zuordnung-objekte.md`).
- GR7.4 (B-251, Agent: Mensch) beobachtet das Figuren-Flimmern am TV und legt den Faktor fest.
- Vorbild für einen deterministischen Grafik-Generator: `tools/atlas` (Go, `task atlas`).

## Erlaubte Dateien

- `docs/sprints/geplant/GEB1-grundregeln-werkstatt/`

## Nicht-Ziele

Keine Code-Änderung, kein TV-Test (gehört GR7.4), keine Regeldatei (schreibt GEB1.2).

## Schritte

1. Agent stellt 🧑 die vier Skalierungsfragen einzeln, mit Kontext zu Figur- und Gebäudedichte.
2. Agent klärt den Ort des Build-Werkzeugs (`scripts/` oder `tools/` nach Vorbild `tools/atlas`).
3. Antworten in Sprint-README › Offene Fragen, Anforderungen und Akzeptanzkriterien eintragen.
4. Session-Dateien GEB1.2 bis GEB1.4 schreiben, Spec zur Freigabe vorlegen.

## Fertig, wenn

- [x] AC-01 (Entscheidungen): Skalierungsfragen 1–4 und Generator-Ort stehen mit Datum unter Offene Fragen.
- [x] Sessions GEB1.2 bis GEB1.4 sind ohne Rückfragen ausführbar; `task test -- planning` grün.

## Prüfen

```bash
task test -- planning
```

## Ergebnis

Workshop 2026-10-16 im Chat (Ralf, Fragen von Claude).
- Flimmern: noch nicht geprüft → Faktor aus GR7.4 (B-251).
- Ganzzahlige Skalierung gilt für Figuren, Gebäude und Objekte.
- Master bleibt 32 px je Kachel; Spiel-Dichte (32 px ×1 oder 16 px ×2) entscheidet 🧑 in GEB1.4 am Vergleich.
- TV: 4K, Kamera-Zoom 1 (ungeprüft: Edge rendert vermutet mit 1080p-Viewport, `devicePixelRatio` 2).
- Generator: Go in `tools/gebaeude/` statt `scripts/`.

AC-01 (Entscheidungen) umgesetzt; die Regeldatei selbst schreibt GEB1.2. Abweichung: Werkzeug-Fehler in `plan_create` (Platzhalter-Zeilen `– (Entwurf)` in der Session-Tabelle), Zeilen von Hand entfernt.
