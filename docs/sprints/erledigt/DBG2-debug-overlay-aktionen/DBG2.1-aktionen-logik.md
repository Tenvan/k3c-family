# DBG2.1 · Reine Funktionen: Auswahl zur Dev-Nachricht, Sichtbarkeit der Aktionsliste

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Umgebung:** live
- **Branch:** dbg2/1-aktionen-logik
- **Abhängig von:** DBG1 (Protokoll `dev` auf dem Server und in `docs/protocol.md`, Ticket B-178; erledigt, sonst fehlen Nachricht und Zustandsfeld)
- **Tickets:** B-179
- **Kriterien:** AC-01, AC-02, AC-04

## Ziel

Zwei getestete reine Funktionen liegen auf `develop`: eine macht aus der Overlay-Auswahl die zu sendende `dev`-Nachricht (bei unbekannter Auswahl nichts), die andere sagt, ob die Aktionsliste sichtbar ist (nur bei aktivem Overlay und Dev-Mode des Raums). Eine Oberfläche gibt es noch nicht.

## Kontext

- Ticket B-179 (`docs/backlog/B-179-debug-overlay-aktionen.md`) und B-178 (`docs/backlog/B-178-dev-aktionen-gold-material-zeitraffer.md`). DBG1 hat die Client-Nachricht `dev` eingeführt; die **tatsächlichen Feldnamen stehen in `docs/protocol.md` und `testdata/protocol/`**, nicht hier. Nach B-178 (Entwurf): `{ "t": "dev", "action": "gold", "slot", "amount" }` (Mengen 10, 50, 100), `{ …"action": "material", "resource", "amount" }` (`wood`, `stone`, `copper`, `iron`, `crystal`, je 50), `{ …"action": "timescale", "factor" }` (1, 2, 4, 8). Weicht DBG1 ab, gilt das Protokoll-Dokument.
- Ist-Stand im Client: `src/online/clientProtocol.ts` hat `ClientMessage` (Union der Nachrichten) und `ErrorCode`. `src/scenes/debugOverlay.ts` enthält nur reine Funktionen (`debugLines`, `debugEnabled`, Typen `DebugClient`, `DebugWorld`, `DebugInput`); `debugOverlay.test.ts` testet sie. `debugEnabled(search)` ist wahr, solange `?dev` nicht `0` ist.
- Dev-Mode des Raums erkennt der Client nur am Zustand: B-178 verlangt ein Feld im Snapshot (Zeitfaktor), **nur im Dev-Mode**. Vorschlag: Ist das Feld in `world` vorhanden (Name aus `docs/protocol.md`), läuft der Raum im Dev-Mode; fehlt es, ist die Aktionsliste verborgen (ohne Meldung). Falls DBG1 den Dev-Mode anders sichtbar macht (z. B. über den Grad `dev` in `rooms[]`), diese Quelle nehmen und in der Sprint-README unter Offene Fragen vermerken. Das Feld ist eine Voraussetzung aus DBG1, nicht hier zu erfinden.
- Regel „Der Client rechnet nichts“: `src/scenes/noSim.test.ts` verbietet Importe aus `world/`; die Funktionen bilden nur Auswahl auf Nachricht ab, ohne Spielregeln (keine Lager-Maxima, keine Münzrechnung).
- Belegung (für DBG2.2, hier nur beachten): B, View + Menu und X bleiben frei von Dev-Aktionen; der Overlay-Toggle Ö bzw. Klick auf den linken Stick bleibt.
- Regeln: Datei ≤ 400 Zeilen, Funktion ≤ 60 (`.oxlintrc.json`); die Komplexitäts-Ausnahmen dort dürfen nicht steigen.

## Erlaubte Dateien

- `src/scenes/debugActions.ts` (neu: Aktionsliste als Daten, `devMessage`, `actionsVisible`)
- `src/scenes/debugActions.test.ts` (neu)
- `src/online/clientProtocol.ts` (nur Typen: `dev` in `ClientMessage`, `forbidden` in `ErrorCode`, falls DBG1 sie dort noch nicht ergänzt hat)
- `docs/sprints/geplant/DBG2-debug-overlay-aktionen/` (nur Status und Ergebnis; nach Aktivierung `docs/sprints/aktiv/DBG2-debug-overlay-aktionen/`), `docs/backlog/` (nur Status und neue Tickets)

## Nicht-Ziele

Zeichnen, Eingabe, Senden über die Verbindung, Zeitfaktor-Anzeige (DBG2.2), Änderungen am Server oder Protokoll (DBG1), Wechsel des Schwierigkeitsgrads (B-107).

## Schritte

1. Branch anlegen, `Status: in Arbeit`; Start-Commit des Sprints eintragen, falls noch leer (`git rev-parse --short origin/develop`). `docs/protocol.md` und `testdata/protocol/c2s-dev*.json` lesen: Nachrichtenform und Name des Zustandsfelds für den Zeitfaktor notieren.
2. Falls `src/online/clientProtocol.ts` die Nachricht `dev` oder den Code `forbidden` noch nicht kennt, die Typen genau nach dem Protokoll ergänzen.
3. `debugActions.ts` anlegen: Die Aktionen als Daten (Gruppe, Beschriftung, Auswahl-Schlüssel): Gold 10/50/100, Material Holz/Stein/Kupfer/Eisen/Kristall je 50, Zeit 1×/2×/4×/8×. Reihenfolge und Beschriftungen wie im Ticket („Gold 50“, „Zeit 8×“).
4. `devMessage(selection, slot)` schreiben: liefert die `dev`-Nachricht (Typ aus `clientProtocol.ts`) für eine bekannte Auswahl und den lokalen Slot (für `gold`; bei `timescale` und `material` je nach Protokoll), sonst `null`. Keine Eingabeprüfung außer „Auswahl bekannt“.
5. `actionsVisible({ overlayOn, devMode })` schreiben (`overlayOn` aus Overlay-Zustand und `debugEnabled`, `devMode` aus dem Zustandsfeld aus Schritt 1): wahr nur, wenn beide wahr sind.
6. Tests in `debugActions.test.ts` mit eigenen erwarteten Werten (die Nachricht wörtlich aus den Beispielen in `testdata/protocol/` abgeschrieben, nicht aus dem Code berechnet): jede Aktion der Liste ergibt genau die erwartete Nachricht; unbekannte Auswahl und unbekannter Faktor ergeben `null`; `actionsVisible` für alle vier Kombinationen aus Overlay an/aus und Dev-Mode ja/nein.
7. `task check` ausführen, Ergebnis eintragen.

## Fertig, wenn

- [x] AC-01: `npx vitest run src/scenes/debugActions.test.ts` zeigt Tests, in denen jede Auswahl (Gold, Material, Zeitfaktor) die erwartete `dev`-Nachricht liefert und eine unbekannte Auswahl `null`.
- [x] AC-02: Derselbe Testlauf zeigt, dass `actionsVisible` nur bei aktivem Overlay und Dev-Mode wahr ist (die drei anderen Kombinationen falsch).
- [x] AC-04: `task check` grün, darin `src/scenes/noSim.test.ts` und `tests/projectRules.test.ts`.
- [x] `.oxlintrc.json` unverändert.

## Prüfen

```bash
task check
```

## Ergebnis

2026-10-03, Agent (Claude Opus 5.5), Branch `dbg2/1-aktionen-logik`. Sprint DBG2 aktiviert (Start-Commit `ae2ca20`).

- Protokoll (Schritt 1): Nachricht `dev` wie `testdata/protocol/c2s-dev-*.json` (`gold`: `slot`, `amount`; `material`: `slot`, `resource`, `amount`; `timescale`: `factor`); Dev-Mode erkennt der Client am Feld `devTimescale` im Zustand (nur im Dev-Mode, auch bei 1). `dev` und `forbidden` standen schon in `clientProtocol.ts` (DBG1).
- **AC-01 geprüft:** `npx vitest run src/scenes/debugActions.test.ts` (6 Tests grün): Gold und Zeitfaktor wörtlich wie die Beispiele, jede der zwölf Aktionen (Gold 10/50/100, Holz/Stein/Kupfer/Eisen/Kristall je 50, Zeit 1×/2×/4×/8×) ergibt die erwartete Nachricht, unbekannte Auswahl, Menge oder Faktor → `null`.
- **AC-02 geprüft:** derselbe Lauf: `actionsVisible` nur bei Overlay an und Dev-Mode wahr (vier Kombinationen); `roomDevMode` wahr bei `devTimescale` 4 und 1, falsch ohne Feld und ohne Zustand.
- **AC-04 geprüft:** `task check` grün (inkl. `noSim.test.ts`, `projectRules.test.ts`); `.oxlintrc.json` unverändert.
- Abweichung: Der Client-Typ `ResourceKind` kennt nur Holz, Stein, Kupfer. Für Eisen und Kristall hat `clientProtocol.ts` den Typ `DevResource` bekommen (nur Typ, erlaubte Datei); die Angleichung des Client-Typs ist **B-188**.
