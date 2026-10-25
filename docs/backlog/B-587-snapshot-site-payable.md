# B-587 · Der Server meldet je Bauplatz, ob er jetzt Münzen annimmt

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GEB16
- **Projekt:** GEB
- **Erstellt:** 2026-10-24
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-10-25

## Ausgangslage

Der Client rät aus Hub-Stufe und Mauerlinien, ob ein Bauplatz Münzen annimmt (`buildableRules.ts`, B-584). Der Server entscheidet das in `sitePayable` (`engine/sim/economy.go`). Ralf (2026-10-24): genauer, der Client soll nur anzeigen, was der Server meint.

## Ziel

Der Snapshot trägt je Bauplatz `payable`; der Client zeigt das Bauschild nur dort und löscht seine eigene Regel.

## Beteiligte und Zielgruppen

Spielende; Agent SRV (SIM) und CLI; 🧑 nimmt ab.

## Anforderungen

- Feld `payable` je Site im Snapshot und im Delta, vom Server aus `sitePayable` berechnet (nur unbezahlte Plätze relevant).
- Protokoll in `docs/protocol.md` beschrieben, Client-Typ `Site.payable`.
- Der Client entfernt `buildableRules.ts` und nutzt das Feld.

## Nicht-Ziele

Neue Bauregeln.

## Regeln und Einschränkungen

Protokolländerung braucht eigene SRV-Session vor der CLI-Session; Spielstand-Format unberührt (Feld ist abgeleitet); Golden mit Begründung neu schreiben; deterministisch.

## Beispiele

Mauer 2 bei nicht gebauter Mauer 1 → `payable: false`, kein Schild.

## Ausnahme- und Fehlerfälle

Alter Server ohne Feld → Client zeigt kein Schild (nie falsch).

## Akzeptanzkriterien

- **AC-01** Der Snapshot enthält `payable` je Bauplatz, gleich `sitePayable` (Go-Test).
- **AC-02** Der Client zeigt das Schild nur bei `payable`; `buildableRules.ts` ist gelöscht (Vitest).

## Offene Fragen

keine

## Notizen

Aus B-584, Entscheidung Ralf 2026-10-24. Spec: Entwurf.
