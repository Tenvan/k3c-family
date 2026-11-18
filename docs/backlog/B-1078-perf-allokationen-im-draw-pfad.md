# B-1078 · PERF: draw allokiert bis zu 29 mal je Bild, Ziel 0

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

`k3c-game/tests/alloc_je_bild.rs` (RU42, B-1075) misst im Maximum 29 Allokationen in `draw` je Bild. Fundstellen vermutet: `Scene::build` einmalig (nach Aufwärmen nicht), je Bild `Rc`-Klone in `k3c-rig` `Actor::draw_mount` (`mount.clone()`, `skin.clone()`) und kurzlebige `Vec` in den Zeichnern.

## Ziel

`draw` allokiert nach dem Aufwärmen nicht mehr.

## Beteiligte und Zielgruppen

Agenten setzen um; Messung gilt (Ralf 2026-11-18).

## Anforderungen

- Puffer in den Zeichnern wiederverwenden, `Rc`-Klone im Bildpfad vermeiden.
- Obergrenze im Test absenken.

## Nicht-Ziele

Optimierung des Takts (B-1077).

## Regeln und Einschränkungen

- **Bereich:** `rust/crates/k3c-game/src/**`, `rust/crates/k3c-rig/src/**`
- **Abhängig von:** RU42
- **Braucht 🧑:** –
- **Größe:** M
- **Sperre:** –

## Beispiele

nicht relevant: Messaufgabe.

## Ausnahme- und Fehlerfälle

Nicht vermeidbare Allokation: als unkritisch begründen.

## Akzeptanzkriterien

- **AC-01** `alloc_je_bild` meldet `draw` mit höchstens der neuen Obergrenze, Obergrenze gesenkt.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
