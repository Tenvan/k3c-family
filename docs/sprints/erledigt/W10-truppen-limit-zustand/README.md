# W10 · SRV · Kämpfer-Zahl und Truppen-Limit im Zustand

- **Status:** erledigt
- **Projekt:** –
- **Domäne:** SRV
- **Reife:** bereit
- **Tickets:** B-332
- **Start-Commit:** f6ad680
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-06, Chat, durch 🧑, Revision 1; B-332 Variante A, Domänen-Ausnahme SIM economy_view

## Ausgangslage

W6.1 braucht für den Limit-Text „Kämpfer/Limit“ die Zahl der Kämpfer und das Truppen-Limit je Stufe. Der Zustand v5 nennt beides nicht: `fighters(w)` und `troopLimit(w)` (`engine/sim/barracks.go`) sind unexportiert, `sim.EconomyOf` (`engine/sim/economy_view.go`) liefert nur `stockMax`, `hubLevel`, `hubUpgrade`, `danger` (B-332). Entscheidung 🧑 2026-10-06: Variante A.

## Ziel

Der Server sendet Kämpfer-Zahl und Truppen-Limit je Stufe im Zustand, damit W6.1 den Limit-Text ohne Rechnung im Client bildet.

Am Ende sichtbar: `fighters` und `troopLimit` in `docs/protocol.md` › Wirtschaft und in `testdata/protocol/`, `task check` und `task check:go` grün.

## Beteiligte und Zielgruppen

Entwickler SRV (Umsetzung, mit Domänen-Ausnahme SIM für `engine/sim/economy_view*.go`), danach CLI (W6.1); 🧑 hat die Spec freigegeben.

## Anforderungen

B-332 › Anforderungen, Variante A.

## Nicht-Ziele

Limit-Text und Anzeige im HUD (W6.1, W6.3); Änderung der Limit-Regel (`docs/rules/buerger.md` § 3); neue Werte in `data/`; Protokollversion 6.

## Regeln und Einschränkungen

- Domäne SRV. **Domänen-Ausnahme SIM** (🧑 2026-10-06, mit der Freigabe): nur `engine/sim/economy_view*.go`, sonst nichts in `engine/sim/`.
- Die Regel bleibt an genau einer Stelle (`fighters`, `troopLimit` in `barracks.go`); `EconomyOf` liest sie nur ab, nichts wird gespeichert (Spielstand unverändert).
- `stateOf` (`engine/net/protocol.go`) übernimmt alle JSON-Felder von `EconomyOf` in `s`. Damit W10.1 das Protokoll nicht vorzeitig ändert, tragen die neuen Felder dort `json:"-"`; W10.2 gibt ihnen die JSON-Namen und passt das Protokoll an.
- Protokoll: Die Version bleibt 5. `docs/protocol.md` erlaubt Zusatzfelder ohne Versionssprung (Präzedenz `playerDown.cause`, Version blieb 3; *Wirtschaft*: ein älterer Server sendet die Felder nicht, der Client behandelt sie alle als optional). Beide Enden ignorieren unbekannte Felder im Zustand.
- Welt-JSON und Golden-Daten bleiben unverändert.

## Beispiele

12 Kämpfer, eine Kaserne gebaut → Zustand `fighters: 12, troopLimit: 20` (B-332 › Beispiele). Ohne Kaserne, keine Kämpfer → `fighters: 0, troopLimit: 10`.

## Ausnahme- und Fehlerfälle

Älterer Server ohne die Felder → Client-Typ hat sie optional, kein Fehler (B-332 › Ausnahme- und Fehlerfälle). Zerstörte Kaserne zählt nicht (Regel in `troopLimit`).

## Akzeptanzkriterien

- **AC-01** Variante A ist in B-332 mit Datum durch 🧑 festgehalten, W6 ist angepasst (W6.1 hängt von W10 ab) (B-332/AC-01).
- **AC-02** `sim.EconomyOf` liefert Kämpfer-Zahl und Truppen-Limit der Stufe, abgelesen aus `fighters(w)` und `troopLimit(w)`; ein Go-Test in `engine/sim/` belegt es; Welt-JSON und Golden unverändert (B-332 › Anforderungen, Variante A).
- **AC-03** Der Server sendet `fighters` und `troopLimit` oben in `s` (`snap`, `delta`); `docs/protocol.md` › Wirtschaft, `testdata/protocol/` und `src/model/types.ts` (optional) nennen sie; Version bleibt 5; ein Client-Test belegt den Zustand ohne die Felder (B-332 › Anforderungen, Ausnahme- und Fehlerfälle).

## Offene Fragen

keine

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| W10.1 | `W10.1-economy-kaempfer.md` | Umsetzung | autonom | fertig |
| W10.2 | `W10.2-protokoll.md` | Umsetzung | autonom | fertig |
| W10.3 | `W10.3-review.md` | Review | autonom | fertig |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben.

## Abnahme

- 2026-10-06 (W10.3, autonom): AC-01 (W10.1: B-332 Variante A, W6.1 abhängig von W10), AC-02 (W10.1: `TestEconomyKaempferUndLimit`, Golden und Welt-JSON unverändert), AC-03 (W10.2: `TestKaempferZustandUndDelta`, `TestFormWieBeispiele`, `TestWirtschaftBeispiel`, `clientWirtschaft.test.ts` mit und ohne die Felder) mit Nachweis.
- Review des Diffs: keine schweren Befunde, keine behoben; in `engine/sim/` nur `economy_view*.go`, Regel nur in `barracks.go`, Version bleibt 5.
- Neue Tickets: keine. B-332 erledigt und archiviert.
- Version: v0.14.1 vorgeschlagen (Patch: zwei zusätzliche optionale Zustandsfelder, Version bleibt 5, keine sichtbare Wirkung, bis W6 sie zeigt); gesetzt erst nach Bestätigung durch 🧑.
