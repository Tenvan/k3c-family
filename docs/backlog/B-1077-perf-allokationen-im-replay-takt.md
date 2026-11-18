# B-1077 · PERF: Replay-Takt (step) allokiert bis zu 788 mal je Bild, Ziel 0

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** RU43
- **Projekt:** RSU
- **Erstellt:** 2026-11-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-18
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

`k3c-game/tests/alloc_je_bild.rs` (RU42, B-1075) misst in der Wiedergabe `nacht-300` nach 120 Aufwärmbildern im Maximum 788 Allokationen in `step` (Mittel des Bildes gesamt 478). Fundstellen vermutet: `ReplayApp::step` in `replay/app.rs` mit `blend.capture` (Welt kopieren), `next_world` und `install` (Zustand je Aufzeichnungs-Takt bauen).

## Ziel

`step` allokiert nach dem Aufwärmen nicht mehr; die Obergrenze im Test sinkt entsprechend.

## Beteiligte und Zielgruppen

Agenten setzen um; Messung gilt (Ralf 2026-11-18: Optimierung am Ende, nach Messung).

## Anforderungen

- Puffer in `blend.capture`, `next_world`, `install` wiederverwenden.
- `MAX_ALLOCS_JE_BILD` im Test absenken.

## Nicht-Ziele

Optimierung des Zeichenpfads (B-1078).

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/src/replay/**`
- **Abhängig von:** RU42
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –

## Beispiele

nicht relevant: Messaufgabe.

## Ausnahme- und Fehlerfälle

Ist ein Teil nur im Replay (nicht im echten Spiel) heiß, genügt die Begründung als unkritisch.

## Akzeptanzkriterien

- **AC-01** `alloc_je_bild` meldet `step` mit höchstens der neuen Obergrenze, Obergrenze gesenkt.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
