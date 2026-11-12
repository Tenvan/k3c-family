# B-740 · gen:paper zeigt Fortschritt je Schritt in Konsole und Log

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** PT1
- **Projekt:** PAP
- **Erstellt:** 2026-11-08
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-12

## Ausgangslage

`bun tools/paper/bin/gen.ts` (Task `gen:paper`) rendert alle Gruppen (`tools/paper/cli/gen.ts`, `work.ts`) und gibt erst am Ende ein JSON aus. Währenddessen ist nichts zu sehen; ein Lauf dauert lange.

## Ziel

Man sieht in Konsole und Log, was gerade passiert: Schritt X/Y, Gruppe und Asset, Phase, Dauer je Schritt.

## Beteiligte und Zielgruppen

Entwickler (🧑) und Claude-Sessions, die `task gen:paper` laufen lassen.

## Anforderungen

- Eine Zeile je Schritt (Phasen Spiegeln, je Asset Prüfen/Rendern/Schreiben, Golden, Übertragen) auf stderr, mit Emoji-Präfix (CLAUDE.md › Logging) und Dauer in ms/s.
- stdout bleibt genau das eine JSON-Ergebnis (Verträge der Render-CLI).
- Gleiche Zeilen im Log: `.work/grafik/gen.log` (angehängt je Lauf).

## Nicht-Ziele

Kein Fortschrittsbalken, keine Parallelisierung.

## Regeln und Einschränkungen

Emoji-Log-Konvention; Datei ≤ 400 Zeilen; Ausgabe-Format des Programms nicht brechen (`tools/paper/cli/gen.test.ts`). Umgesetzt mit #518, Beleg in PT1.1.

- **Bereich:** `tools/paper/cli/gen*.ts`, `tools/paper/cli/fortschritt*.ts`, `tools/paper/bin/gen.ts`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`🚀 gen:paper: 6 Gruppen, 112 Assets` · `🔨 [14/112] figuren/goblin: rendern … ✅ 2,3 s` · `✅ gen:paper fertig in 4 min 12 s`.

## Ausnahme- und Fehlerfälle

Abgelehnte Spec: Zeile `🚫`/`❌` mit Asset, dann wie bisher JSON mit `fehler`, Exit 1.

## Akzeptanzkriterien

- **AC-01** `task gen:paper` schreibt je Asset genau eine Fortschrittszeile mit Index X/Y, Name und Dauer auf stderr; stdout bleibt ein JSON-Objekt.
- **AC-02** Dieselben Zeilen stehen in `.work/grafik/gen.log`.
- **AC-03** Ein Test prüft Format und Zählung der Zeilen (`tools/paper/cli/gen.test.ts`).

## Offene Fragen

keine

## Notizen

Umgesetzt zusammen mit B-741 auf einem Branch (Auftrag aus der Schaltzentrale).
