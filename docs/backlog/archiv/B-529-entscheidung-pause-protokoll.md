# B-529 · Entscheidung: Protokoll v8 für pause/resume ohne Rückmeldung im Zustand

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** KR1
- **Projekt:** BED
- **Erstellt:** 2026-10-22
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

BD3.1 führt `pause`/`resume` als Geräte-Nachricht ein (Beschluss 🧑 2026-10-13). Die Version steigt auf 8 (`engine/net/protocol.go`), die Client-Konstante `PROTOCOL_VERSION` in `src/online/clientProtocol.ts` zieht mit, sonst lehnt der Server jeden Client ab.

## Ziel

🧑 bestätigt die Form oder wählt die Alternative, bevor der CLI-Sprint den Client anbindet.

## Beteiligte und Zielgruppen

🧑 entscheidet; CLI-Sprint baut darauf auf.

## Anforderungen

nicht relevant (Frage).

## Nicht-Ziele

nicht relevant (Frage).

## Regeln und Einschränkungen

`docs/protocol.md` › Nachrichten.

## Beispiele

`testdata/protocol/c2s-pause.json`, `c2s-resume.json`.

## Ausnahme- und Fehlerfälle

nicht relevant (Frage).

## Akzeptanzkriterien

- **AC-01** 🧑 hat beide Punkte unter Notizen beantwortet.

## Offene Fragen

Siehe Notizen.

## Notizen

**Entscheidung 🧑 (KR1.1, 2026-10-23):** beide Defaults bestätigt.

1. **Form:** zwei Nachrichten ohne Felder, `{"t":"pause"}` und `{"t":"resume"}`.
2. **Rückmeldung:** keine; der Client zeigt Stillstand bzw. Ende des Schutzes aus eigener Uhr.

Alternativen (verworfen): eine Nachricht `{"t":"pause","on":bool}`; `paused`/`protected` in `snap`/`delta` (additiv, bleibt später ohne neue Version möglich).
