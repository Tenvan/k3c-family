# B-476 · preloadSprites und createSpriteAnims laden beliebige Reittiere statt nur das Standard-Reittier

- **Domäne:** CLI
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ23
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`preloadSprites` und `createSpriteAnims` in `src/scenes/render/sprites.ts` laden nur Sheets mit Rolle (Atlas) und das Standard-Reittier `MONARCH.mount.sprite` als Einzel-PNG; `createRider` in `src/scenes/render/mountView.ts` ist an `Player` und das Standard-Reittier gebunden. Das Asset-Studio (WZ10.1) zeigt alle 13 Reittiere und baut dafür Laden und Animationen selbst nach (`abspiel/figureView.ts`, `loadFigureSheets`), weil `src/scenes/` nicht geändert werden darf. Das ist eine Abweichung von B-463/AC-02.

## Ziel

`src/scenes/render/sprites.ts` bietet einen Weg, beliebige Reittiere (Sheets aus `data/sprites.json` › `mounts`) zu laden und ihre Animationen anzulegen, ohne dass das Studio Code des Spiels nachbaut; `createRider` nimmt das Reittier als Parameter. Danach entfallen `loadFigureSheets` und `createSheetAnims` im Studio.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidet 🧑: Soll das Spiel selbst alle Reittiere laden (Kosten im Ladevorgang auf der Xbox) oder nur auf Anforderung (z. B. `preloadMount(scene, key)`)? Vorschlag von Claude (ungeprüft): auf Anforderung, damit das Spiel nicht mehr lädt.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
