# B-322 · Die Werkzeug-Seiten holen ihre Texte aus den zentralen Textdateien

- **Domäne:** PLAT
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** PL2
- **Projekt:** WZG
- **Erstellt:** 2026-10-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Werkzeug-Seiten unter `src/tools/` (z. B. `leveltest.ts`, `soundtest.ts`, `monitor.ts`, `dm.ts`, `credits.ts`,
`grafiken.ts`) haben ihre Texte als deutsche Literale im Code. Beschluss 🧑 2026-10-06 zu B-215: Auch die
Werkzeug-Seiten werden übersetzt und folgen der gewählten Sprache. PL1 setzt B-215 nur für Touch-Overlay und Shell um.

## Ziel

Jede Werkzeug-Seite zeigt ihre Texte in der gewählten Sprache (de/en), mit Rückfall auf Deutsch.

## Beteiligte und Zielgruppen

🧑 und Mitspielende, die die Werkzeug-Seiten am PC, Handy oder TV öffnen.

## Anforderungen

- Texte der Seiten unter `src/tools/` über `t()` aus `texts.de.ts`/`texts.en.ts`.
- Rückfall auf Deutsch bei fehlendem Text oder unbekannter Sprache.

## Nicht-Ziele

Weitere Sprachen; Texte in `src/scenes`, `src/input`, `src/core` (S5.3, B-215).

## Regeln und Einschränkungen

Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; Seitenregeln aus `CLAUDE.md`. Je Session höchstens ~400 Code-Zeilen, also seitenweise aufteilen.

## Beispiele

Sprache Englisch gewählt → `soundtest.html` zeigt englische Beschriftungen.

## Ausnahme- und Fehlerfälle

Text fehlt in `texts.en.ts` → deutscher Text, kein leerer Knopf.

## Akzeptanzkriterien

- **AC-01** Kein Modul unter `src/tools/` enthält ein deutsches Text-Literal für die Oberfläche (Test wie `src/scenes/textRule.test.ts`).
- **AC-02** Nach Sprachwechsel zeigen die Werkzeug-Seiten die gewählte Sprache (🧑 am Gerät).

## Offene Fragen

keine

## Notizen

Abgespalten aus B-215 bei der Planung von PL1 (2026-10-06).
