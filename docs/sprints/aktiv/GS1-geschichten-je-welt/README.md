# GS1 · REG, SIM, CLI, DEV · Geschichten je Welt und Insel in Kunstsprache

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** REG, SIM, CLI, DEV
- **Reife:** bereit
- **Tickets:** B-965, B-974, B-975, B-976
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15

## Ausgangslage

🧑 2026-11-15: Welten und Inseln sollen kurze Geschichten in Kunstsprache bekommen (B-965). Etappen und Seekarte sind umgesetzt (B-961, B-962, in `main`). Die Welt-Datei kennt keine Geschichte, das Spiel keine Einblendung, die Welten-Werkzeuge kein Feld dafür. B-965 war zu groß (L) und ist zerlegt: Regel, Format und Engine bleiben in B-965, Anzeige B-974, Werkzeuge B-975, Texte B-976.

## Ziel

Jede der vier vorhandenen Welten erzählt an den Wendepunkten (Start, Ankunft, Endgegner, Abschluss, Etappe) ein bis zwei Zeilen in ihrer Kunstsprache; die Zeilen stehen in der Welt-Datei, entstehen über die Welten-Werkzeuge und blenden im Spiel je Spielerhälfte ein.

## Beteiligte und Zielgruppen

Spielende am TV (1–2 Monarchen). Claude schreibt Regel, Code und Texte; 🧑 gibt die Specs frei, baut die Workbench neu, nimmt die Texte ab und prüft die Einblendung am TV.

## Anforderungen

- Regel, Format der Welt-Datei, Prüfung und Wendepunkt-Funktionen der Engine: siehe `B-965 › Anforderungen`.
- Einblendung im Spiel: siehe `B-974 › Anforderungen`.
- Welten-Werkzeuge (`world_story_set`, `world_get`, `world_check`, `world_create`, Editor): siehe `B-975 › Anforderungen`.
- Texte für `standard`, `sengoku`, `nil`, `britannien`: siehe `B-976 › Anforderungen`.

## Nicht-Ziele

Ausformulierte Texte, Dialoge, Vertonung. Seekarte als Diorama (GS2, B-964), Andenken (GS3, B-966), Mechanik der Flaschenpost (GS4, B-971). Welt „Polar“.

## Regeln und Einschränkungen

- Reihenfolge REG → SIM → CLI → DEV → REG → Review; ein Commit je Session auf `sprint/gs1`, ein PR für den Sprint.
- GS1.2 (Sim-Code) läuft im eigenen Worktree mit Dev-Workbench (B-925, Agent `k3c-sim-dev`). Keine Simulation, kein Balancing.
- GS1.5 braucht die neu gebaute Workbench mit `world_story_set` (Neubau und Neustart durch 🧑, Sperre `exe`) und läuft im Hauptcheckout, weil die lokalen Welten in Worktrees fehlen.
- Golden und Spielstand bleiben unverändert (B-965/R7).

- **Bereich:** `docs/rules/welten.md`, `engine/sim/**`, `data/story.json`, `native/k3c-game/hud/**`, `native/k3c-game/view/**`, `native/k3c-game/game/**`, `native/k3c-game/local/**`, `tools/k3c-workbench/internal/world/**`, `tools/k3c-workbench/internal/mcpsrv/**`, `tools/k3c-workbench/frontend/src/world/**`, `tools/k3c-workbench/frontend/src/api/typesWorld.ts`, `docs/standards/workbench-seiten.md`, `data/worlds/**`
- **Abhängig von:** WE4.2
- **Braucht 🧑:** Abnahme
- **Sperre:** exe

## Beispiele

Sengoku zu zweit neu gestartet: In beiden Hälften blendet „Oru kamae KRONE no sen. Shiro nu BRENNT.“ ein und aus, danach die Ankunftszeile der Reisfeld-Ebene. Beim Endboss und nach dem Sieg folgen die Zeilen der Insel. `world_check sengoku` meldet nichts zur Geschichte.

## Ausnahme- und Fehlerfälle

- Welt ohne Geschichte → keine Einblendung, `world_check` gibt einen Hinweis.
- Ungültige Zeile → Welt wird nicht geladen bzw. nicht geschrieben, Meldung nennt Feld und Zeile.
- Workbench nicht neu gebaut → GS1.5 startet nicht (Tool fehlt), der Sprint bleibt bis dahin bei GS1.4 stehen.
- Einzelheiten je Ticket: `B-965`, `B-974`, `B-975`, `B-976 › Ausnahme- und Fehlerfälle`.

## Akzeptanzkriterien

- **AC-01** `docs/rules/welten.md` beschreibt Format, Prüfung und Wendepunkte der Geschichte; Glossar ergänzt (B-965/AC-01).
- **AC-02** Welt-Dateien mit `story` laden, jede Prüfung lehnt mit Welt, Feld und Zeile ab, Obergrenzen stehen in `data/story.json` (B-965/AC-02, AC-03, AC-06).
- **AC-03** Die Wendepunkt-Funktionen liefern die richtigen Zeilen, deterministisch und ohne Zustand; Golden und Spielstand unverändert (B-965/AC-04, AC-05).
- **AC-04** Das Spiel blendet die Zeilen an den Wendepunkten in jeder Spielerhälfte ein, ohne Eingabe zu verbrauchen, mit 2 Spielern gleichzeitig; alle Zeilen sind darstellbar (B-974/AC-01 bis AC-04).
- **AC-05** `world_story_set`, `world_get`, `world_check`, `world_create` und der Editor kennen die Geschichte; `check_run task:check:dev` grün (B-975/AC-01 bis AC-05).
- **AC-06** Die vier Welten haben vollständige Geschichten, `world_check` ohne Fehler und Hinweise, Golden unverändert (B-976/AC-01 bis AC-03).
- **AC-07** 🧑: Texte abgenommen (B-976/AC-04) und Einblendung am TV geprüft (B-974/AC-05).

## Offene Fragen

Alle mit Default entschieden, 🧑 bestätigt bei der Freigabe (Einzelheiten in den Tickets):

- Einblendung je Spielerhälfte statt ganzer Breite (B-965, B-974).
- „Etappe“ = Etappe der Welt; „vor dem Endgegner“ = Endboss ausgelöst (B-965).
- Engine nennt Zeilen über Funktionen, keine neuen Ereignisse (B-965).
- `world_create` nimmt die Geschichte als Argument, der Editor bietet einen Prompt-Button (B-975).
- Klang Britannien keltisch-altenglisch, Standard märchenhaft; Standard-Welt bekommt eine Geschichte (B-976).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| GS1.1 | `GS1.1-regeltext-geschichte.md` | Umsetzung | autonom | offen |
| GS1.2 | `GS1.2-story-format-engine.md` | Umsetzung | autonom | offen |
| GS1.3 | `GS1.3-einblendung-spiel.md` | Umsetzung | autonom | offen |
| GS1.4 | `GS1.4-welten-werkzeuge-story.md` | Umsetzung | autonom | offen |
| GS1.5 | `GS1.5-geschichten-schreiben.md` | Umsetzung | autonom | offen |
| GS1.6 | `GS1.6-review.md` | Review | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
