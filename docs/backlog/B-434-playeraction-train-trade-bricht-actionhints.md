# B-434 · PlayerAction mit train und trade bricht den Typcheck von actionHints.ts, das W11.3 nicht anfassen darf

- **Domäne:** CLI
- **Typ:** Frage
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** W11
- **Projekt:** WRT
- **Erstellt:** 2026-10-15
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

W11.3 (B-410/AC-03) verlangt `train` und `trade` in `PlayerAction` (`src/model/types.ts`). `src/scenes/actionHints.ts` (Zeile 90, `playerHint`) gibt `p.actions` direkt an `hintView(h: Hint)`; `Hint` kennt `train`/`trade` nicht, `task check` scheitert mit TS2345. Die Session darf `actionHints.ts` nicht ändern (Erlaubte Dateien, Nicht-Ziel: W11.4). Die Annahme aus B-410 › Regeln „ein Client ohne W11.4 ignoriert unbekannte Einträge“ stimmt für `actionHints.ts` nicht: Zur Laufzeit bekäme ein `train`-Eintrag `WEIGHT` undefined (Sortierung mit NaN) und den Text `hint.train` ohne Eintrag in den Texten.

Stand: Server, Sicht `sim.PayActionOf`, Tests, `docs/protocol.md` und Beispiel sind auf `sprint/w11` (W11.3); die Änderung an `PlayerAction` liegt als Patch vor und ist nicht committet.

## Ziel

Welches Ergebnis, welcher Nutzen (oder welcher Schaden verschwindet)? Kein Umsetzungsschritt.

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

1. Darf W11.3 in `actionHints.ts › playerHint` unbekannte Einträge herausfiltern (eine Zeile, z. B. `p.actions.filter((h): h is Hint => h.action in WEIGHT)`), damit B-410 › Regeln stimmt? Vorschlag: ja, Erlaubte Dateien von W11.3 um diese Zeile erweitern.
2. Oder wandert die Änderung an `PlayerAction` (B-410/AC-03, Teil Typ) nach W11.4, das `actionHints.ts` ohnehin anfasst?

## Notizen

Vorbereitete Änderung an `src/model/types.ts › PlayerAction` (nicht committet):

```ts
  | { action: 'train'; kind: 'miner' | 'builder' | 'craftsman'; gold: number; paid: number }
  | { action: 'trade'; side: 'buy' | 'sell'; resource: string; material: number; gold: number };
```

Kommentar dazu: „`train` und `trade` (B-410): das Ziel, das A halten am Ort jetzt bezahlt – Beruf (`gold` Preis, `paid` schon bezahlt) oder Händler (`material` gegen `gold`).“
