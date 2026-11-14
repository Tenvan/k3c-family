# B-750 · gen:paper meldet die Ausgaben innerhalb großer Assets

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** PT1
- **Projekt:** PAP
- **Erstellt:** 2026-11-08
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

`figuren/monarch` hat 195 Ausgaben und braucht gut zwei Minuten; auch mit Startzeile (B-749) bleibt die Konsole in der Zeit stumm.

## Ziel

Auch bei großen Assets sieht man den Fortschritt.

## Beteiligte und Zielgruppen

Entwickler (🧑).

## Anforderungen

- Assets mit mehr als 4 Ausgaben melden jede Ausgabe als eine Zeile `   ↳ n/N gruppe/id: schlüssel Dauer`; kleine Assets nur sich selbst.

## Nicht-Ziele

Kein Fortschritt innerhalb eines Assets, kein schnelleres Rendern.

## Regeln und Einschränkungen

Emoji-Log-Konvention; stdout bleibt das JSON. Umgesetzt mit #522, Beleg in PT1.1.

- **Bereich:** `tools/paper/cli/gen*.ts`, `tools/paper/cli/work*.ts`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

`⏳ [7/127] figuren/monarch rendert …` erscheint sofort nach den Bossen.

## Ausnahme- und Fehlerfälle

nicht relevant: Fehler wie bisher.

## Akzeptanzkriterien

- **AC-01** `task gen:paper` zeigt für `figuren/monarch` 195 Zeilen `↳ n/195 …` während des Renderns.

## Offene Fragen

keine

## Notizen

Messung (20 Ausgaben, 160 Frames, 3,5 MB SVG): TypeScript 0,23 s, resvg 14,3 s. Folge-Idee B-751.
