# B-529 · Entscheidung: Protokoll v8 für pause/resume ohne Rückmeldung im Zustand

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
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

1. **Form:** Default: zwei Nachrichten ohne Felder, `{"t":"pause"}` und `{"t":"resume"}`. Alternative: eine Nachricht `{"t":"pause","on":true|false}`.
2. **Rückmeldung:** Default: keine; der Client weiß selbst, dass er pausiert hat, Stillstand bzw. Ende des Schutzes zeigt er aus eigener Uhr. Alternative: `snap`/`delta` nennen `paused` (Raum steht) und je Monarch `protected`, damit andere Geräte den geschützten Monarchen markieren können (additiv, ohne neue Version möglich).
