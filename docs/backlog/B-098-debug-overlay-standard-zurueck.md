# B-098 · Das Debug-Overlay ist vor dem Release wieder nur mit ?dev=1 verfügbar

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Status:** eingeplant
- **Sprint:** K5
- **Erstellt:** 2026-10-02
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

In der Entwicklungsphase ist das Debug-Overlay (B-093, Revision 2) ohne Parameter verfügbar, nur `?dev=0` schaltet es ab (`debugEnabled` in `src/scenes/debugOverlay.ts`). Für Spielende (Familie, Xbox) soll es das später nicht sein.

## Ziel

Vor dem ersten Release oder Spieleabend ist das Overlay wieder nur mit `?dev=1` verfügbar.

## Beteiligte und Zielgruppen

Entwickler; 🧑 entscheidet, wann die Entwicklungsphase endet.

## Anforderungen

- `debugEnabled` liefert ohne `dev`-Parameter `false`, mit `?dev=1` `true`.
- Test in `debugOverlay.test.ts` entsprechend angepasst.

## Nicht-Ziele

Eine Einstellung im Spiel oder eine Umgebungsvariable.

## Regeln und Einschränkungen

Domäne CLI; Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

`game.html` → kein Overlay; `game.html?dev=1` → Overlay per Ö oder Stick-Klick umschaltbar.

## Ausnahme- und Fehlerfälle

nicht relevant: eine Ein-Zeilen-Änderung.

## Akzeptanzkriterien

- **AC-01** `debugEnabled('')` ist `false`, `debugEnabled('?dev=1')` ist `true`; `task check` grün.

## Offene Fragen

Zeitpunkt: vor dem nächsten Release-Tag `v0.<n>.0` oder Spieleabend (entscheidet 🧑).

## Notizen

Entstanden aus der Änderung von B-093 am 2026-10-02.
