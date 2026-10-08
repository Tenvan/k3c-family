# B-059 · Nur gesteuerte Monarchen entscheiden über den Stufenwechsel

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** SP06
- **Projekt:** –
- **Erstellt:** 2026-09-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-01 🧑 Chat (SP06: campaign-abstieg, Abdeckung ≥ 90 %, B-059 in SP06.2, Golden ≤ 8 MB)

## Ausgangslage

`src/world/sim/travel.ts` (`stepTravel`) startet einen Stufenwechsel nur, wenn **alle lebenden** Monarchen am
Tiefen-Eingang bzw. an der Treppe stehen. Mit Protokoll v2 (`docs/protocol.md`) gibt es Monarchen, die niemand steuert
(frei oder wartend nach einem Abbruch). Sie stehen still und würden den Wechsel für immer blockieren (Review SP02, K1).

## Ziel

Ein Monarch, den niemand steuert, blockiert keinen Stufenwechsel. Nutzen: Geht ein Spieler, können die anderen
weiterspielen.

## Beteiligte und Zielgruppen

🧑 hat entschieden (2026-09-30, Chat, Review SP02); Umsetzung im Go-Port (SP05/SP06) durch Entwickler oder Agent.

## Anforderungen

- Für den Stufenwechsel zählen nur besetzte Monarchen; freie und wartende reisen automatisch mit.
- Die Simulation braucht dafür pro Monarch die Angabe „gesteuert ja/nein“ (vom Raum gesetzt).
- Ein gespeicherter Spielstand legt keine Monarchen an; sie entstehen beim Beitreten, Gold gilt pro Index (wie heute).

## Nicht-Ziele

Änderung an `src/world/` (Feature-Stopp, Entscheidung 001); Raum-Verwaltung (SP07).

## Regeln und Einschränkungen

Entscheidung 001 (neue Mechanik nur in Go), Protokoll v2 (`docs/protocol.md`), deterministisch; Golden-Tests aus
der TS-Simulation kennen den Fall nicht und brauchen einen eigenen Go-Test.

## Beispiele

Drei Monarchen, Monarch 1 ist frei und steht im Wald; Monarch 0 und 2 stehen am Tiefen-Eingang → Wechsel startet,
alle drei stehen danach an der Burg der Zielstufe.

## Ausnahme- und Fehlerfälle

Kein Monarch ist besetzt → kein Stufenwechsel (Raum ist ohnehin pausiert).

## Akzeptanzkriterien

- **AC-01** Go-Test: ein freier Monarch fern vom Ausgang blockiert den Wechsel nicht und reist mit.
- **AC-02** Go-Test: ohne besetzten Monarchen startet kein Wechsel.

## Offene Fragen

keine

## Notizen

Beim Planen von SP05/SP06 einplanen. Eingeplant in SP06.2 (🧑, 2026-10-01, Chat): Feld `free` am Monarchen, im JSON nur bei `true`. Erledigt in SP06.2: `stepTravel` zählt nur gesteuerte Monarchen, Go-Tests `TestFreierMonarchBlockiertNicht` (AC-01) und `TestOhneGesteuertenMonarchenKeinWechsel` (AC-02). Setzen durch den Raum: SP07.
