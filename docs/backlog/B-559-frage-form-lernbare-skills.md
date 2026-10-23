# B-559 · Ralf bestätigt die Form der lernbaren Skills im Protokoll

- **Domäne:** SRV
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** KR5
- **Projekt:** KRT
- **Erstellt:** 2026-10-23
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

RM1.2 (B-285) hat autonom entschieden: Die lernbaren Skills stehen als Feld `skills` im Eintrag `{ "action": "learn", "skills": [...] }` der Aktionsliste, ohne neue Protokoll-Version (Feld zusätzlich, älterer Server ohne Feld → Client lernt pauschal). `learn` fehlt, wenn nichts lernbar ist (auch bei Punkten, z. B. Tier-Gating).

## Ziel

🧑 bestätigt die Form oder wählt eine andere.

## Beteiligte und Zielgruppen

🧑 entscheidet; Agent zieht ggf. Server und Client nach.

## Anforderungen

- Optionen: (a) Feld `skills` in `learn` (umgesetzt, Default); (b) eigenes Feld je Spieler (`learnable`), `learn` bleibt pauschal; (c) wie (a), aber `learn` auch ohne lernbaren Skill bei Punkten.
- Entscheidet 🧑 anders: eine SRV-Session für beide Enden (`engine/net/actions.go`, `src/scenes/hud/skillMenuLogic.ts`, `docs/protocol.md`, `testdata/protocol/`), etwa 30 Zeilen.

## Nicht-Ziele

Lern-Regeln.

## Regeln und Einschränkungen

Protokoll-Änderung als eigene Session für beide Enden.

## Beispiele

nicht relevant: Entscheidungsfrage.

## Ausnahme- und Fehlerfälle

nicht relevant: Entscheidungsfrage.

## Akzeptanzkriterien

- **AC-01** Entscheidung von 🧑 steht in diesem Ticket; bei Abweichung Folge-Ticket.

## Offene Fragen

Form (a), (b) oder (c)? Default (a), umgesetzt.

## Notizen

Sprint KR5, Session KR5.1.
