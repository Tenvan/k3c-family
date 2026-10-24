# B-604 · Die Welt steht groß im Pausenmenü und bei jedem Spielstand in der Lobby

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** BD9
- **Projekt:** BED
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Bei der Abnahme BD5.4 (2026-10-24, PC) fiel auf: Die Welt-Anzeige im Pausenmenü ist klein, und in der Lobby steht die Welt nicht bei jedem Eintrag. Code (2026-10-24): Das Pausenmenü (`src/scenes/OptionsScene.ts:98`, `worldLine` in `optionsLogic.ts`) zeichnet die Welt mit 28 px, die Optionszeilen mit 40 px. In der Lobby (`src/scenes/lobbyLogic.ts`, `entryLabel`) zeigen Spielstände ihre Welt schon (`SaveInfo.world`), offene Räume nicht: `rooms[]` (`room.Info`, `engine/room/actions.go`, `docs/protocol.md`) hat kein Feld für die Welt.

## Ziel

Die Welt ist im Pausenmenü gut lesbar und in der Lobby bei jedem Spielstand sichtbar.

## Beteiligte und Zielgruppen

Spieler; 🧑 nimmt ab.

## Anforderungen

- Weltzeile im Pausenmenü in 40 px wie die Optionszeilen (`FONT_PX`), weiter oberhalb der Optionen.
- `rooms[]` im Protokoll bekommt `world` (ID der Welt, aus `isl.WorldID()`), Protokollversion + 1, `docs/protocol.md` und Client-Typ `RoomInfo` nachziehen.
- Jeder Eintrag der Lobby (offener Raum und Spielstand) nennt seine Welt; unbekannt → „–“.

## Nicht-Ziele

Neue Welt-Auswahl.

## Regeln und Einschränkungen

Nur zeichnen im Client (ADR 001); Protokoll-Änderung mit Version und Doku (`docs/protocol.md`, `src/online/clientProtocol.ts`); Texte über `t()`; Mindest-Schrift laut `docs/rules/bedienung.md` § 2; Datei ≤ 400 Zeilen.

## Beispiele

Lobby mit drei Spielständen → jede Zeile nennt ihre Welt.

## Ausnahme- und Fehlerfälle

Spielstand ohne bekannte Welt → Platzhalter „–“.

## Akzeptanzkriterien

- **AC-01** Pausenmenü zeigt die Welt in 40 px (Vitest der Layout-Werte, Beobachtung am PC).
- **AC-02** Jeder Eintrag der Lobby nennt seine Welt, auch offene Räume (Go-Test von `rooms[].world`, Vitest von `entryLabel`, Beobachtung am PC).

## Offene Fragen

keine

## Notizen

Aus BD5.4 (Ralf, 2026-10-24). Spec: Entwurf.
