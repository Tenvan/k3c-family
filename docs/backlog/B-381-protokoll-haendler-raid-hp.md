# B-381 · Das Protokoll beschreibt Überfall und HP des Händlers

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** KMP
- **Erstellt:** 2026-10-09
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit K3.2 steht `merchant.raid` im Zustand (`engine/sim/merchant.go`, Feld `Raid`), `docs/protocol.md` und `src/model/types.ts` (`Merchant`) kennen es nicht. Händler-HP stehen nicht im Zustand (`json:"-"`), die Anzeige braucht sie (Review K3.3).

## Ziel

Client und Doku kennen Überfall und HP des Händlers.

## Beteiligte und Zielgruppen

Entwickler (Client K5).

## Anforderungen

- `docs/protocol.md` und `src/model/types.ts` nennen `raid`; `hp`/`maxHp` im Zustand mit Protokoll-Beispiel, Golden geprüft.

## Nicht-Ziele

Darstellung (K5).

## Regeln und Einschränkungen

Protokoll-Änderung als eigene Session mit beiden Enden (`docs/arbeitsweise.md` › Domänen); passt zu K4.

## Beispiele

Überfallener Händler mit 60 HP → `merchant: {resource, leaves, raid: true, hp: 60, maxHp: 100}`.

## Ausnahme- und Fehlerfälle

Kein Händler → Feld fehlt wie bisher.

## Akzeptanzkriterien

- **AC-01** Protokoll-Test: `merchant` enthält `raid`, `hp`, `maxHp`; Doku und Client-Typ nennen sie.

## Offene Fragen

keine

## Notizen

Gefunden im Review K3.3; passt in K4 (B-154).
