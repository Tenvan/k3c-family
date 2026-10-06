# B-217 · Die Ereignisse `built` und `playerDown` tragen ihren Ort

- **Domäne:** SIM
- **Typ:** Schuld
- **Prio:** niedrig
- **Status:** eingeplant
- **Sprint:** LV1
- **Erstellt:** 2026-10-04
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Feedback-Ereignisse `built` und `playerDown` haben kein `x` (`src/model/types.ts` › `GameEvent`, `docs/protocol.md` › Ereignisse). Der Client (GR5.1, `src/scenes/effects.ts`) behilft sich: Für `built` merkt er sich das `x` des letzten `buildProgress` derselben Bauart, für `playerDown` liest er den Ort aus dem letzten Snapshot. Ohne vorheriges `buildProgress` (z. B. Bau in einem Tick fertig) gibt es keinen Bau-Effekt.

## Ziel

Beide Ereignisse tragen `x` in Units, der Client braucht keine Ersatzlogik.

## Beteiligte und Zielgruppen

Entwickler Simulation und Client; Spieler am TV (Effekte am richtigen Ort).

## Anforderungen

- `built` und `playerDown` enthalten `x`; Protokoll-Dokument und Typen sind angepasst, Protokollversion nach Regel in `docs/protocol.md`.
- Deterministisch, 2+ Spieler unverändert.

## Nicht-Ziele

Neue Ereignistypen.

## Regeln und Einschränkungen

Protokoll-Verträge (`docs/protocol.md`), `engine/sim/` bleibt deterministisch.

## Beispiele

Mauer wird fertig → `built` mit `x` der Mauer → Effekt dort. Spieler stirbt → `playerDown` mit seinem `x`.

## Ausnahme- und Fehlerfälle

Ältere Clients übergehen das zusätzliche Feld.

## Akzeptanzkriterien

- **AC-01** Go-Test: `built` und `playerDown` enthalten `x` des Bauplatzes bzw. Spielers.
- **AC-02** `task check` ist grün.

## Offene Fragen

keine

## Notizen

Gefunden in GR5.1.
