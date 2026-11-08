# B-749 · gen:paper meldet den Start jedes Assets, lange Assets wirken sonst wie ein Stillstand

- **Domäne:** PAP
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-08
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`gen:paper` meldet ein Asset erst, wenn es fertig ist (B-740). `figuren/monarch` braucht über zwei Minuten; in der Zeit erscheint nichts, der Lauf wirkt nach den Bossen wie abgebrochen.

## Ziel

Man sieht jederzeit, welches Asset gerade gerendert wird.

## Beteiligte und Zielgruppen

Entwickler (🧑).

## Anforderungen

- Vor jedem Asset eine Zeile `⏳ [i/n] gruppe/id rendert …`; nach dem Asset wie bisher `🔨 … gerendert ✅ Dauer`.

## Nicht-Ziele

Kein Fortschritt innerhalb eines Assets, kein schnelleres Rendern.

## Regeln und Einschränkungen

Emoji-Log-Konvention; stdout bleibt das JSON.

## Beispiele

`⏳ [7/127] figuren/monarch rendert …` erscheint sofort nach den Bossen.

## Ausnahme- und Fehlerfälle

nicht relevant: Fehler wie bisher.

## Akzeptanzkriterien

- **AC-01** Test in `tools/paper/cli/gen.test.ts`: je Asset eine ⏳-Zeile vor der 🔨-Zeile.

## Offene Fragen

keine

## Notizen

Gefunden beim Test der Workbench (Lauf schien bei den Bossen zu enden).
