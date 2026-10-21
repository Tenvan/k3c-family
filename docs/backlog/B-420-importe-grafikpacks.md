# B-420 · Mit k3c-dev importierte Kandidaten unter public/grafik/kandidaten/ brechen grafikPacks.test nicht

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GR9
- **Projekt:** GRA
- **Erstellt:** 2026-10-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Seit M10.5 importiert k3c-dev Recherche-Treffer nach `public/grafik/kandidaten/` mit Credit-Zeile `kandidaten/<datei>` in `public/grafik/CREDITS.md` (Abschnitt „Importe (k3c-dev)“). `src/tools/credits.test.ts` kennt diese Zeilen, aber `src/tools/grafikPacks.test.ts` verlangt, dass jede Datei unter `public/grafik/` zu einem Pack aus `GRAFIK_PACKS` gehört: Der erste echte Import macht `task check` rot.

## Ziel

Ein Import nach `public/grafik/kandidaten/` hält `task check` grün, und `grafiken.html` bzw. `lizenzen.html` zeigen importierte Kandidaten mit ihrem Credit.

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

- **AC-01** Eine Datei unter `public/grafik/kandidaten/` mit Credit-Zeile `kandidaten/<datei>` lässt `src/tools/grafikPacks.test.ts` grün (Vitest mit Beispieldatei).
- **AC-02** `lizenzen.html` nennt importierte Kandidaten mit Urheber, Lizenz und Quelle (Beobachtung).

## Offene Fragen

Soll `kandidaten/` ein eigener Eintrag in `GRAFIK_PACKS` werden oder nimmt der Test den Ordner aus (Kandidaten sind im Spiel keinem Objekt zugeordnet)?

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
