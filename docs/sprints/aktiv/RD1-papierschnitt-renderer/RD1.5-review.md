# RD1.5 · Review: Sprint RD1 abnehmen

- **Status:** fertig
- **Typ:** Review
- **Agent:** autonom
- **Domäne:** PAP
- **Lane:** renderer
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** rd1/5-review
- **Abhängig von:** RD1.4
- **Tickets:** B-699
- **Kriterien:** alle

## Ziel

Sprint RD1 ist geprüft und abgenommen: alle Kriterien AC-01 bis AC-05 mit Nachweis, Befunde behoben oder als Ticket
erfasst, Golden-Regel auf Linux (CI) bestätigt, Abnahme in der Sprint-README.

## Kontext

- Sessions RD1.1–RD1.4 mit ihren Ergebnis-Abschnitten; Kriterien in der Sprint-README.
- Verträge: `docs/standards/paper-spec.md` (CLI, Hash, Golden, Ausgabe), `docs/standards/paper-mcp.md` (MC1 ruft die
  CLI). Vorgehen: `docs/arbeitsweise.md` › Review-Session.
- Golden-Regel: Windows rendert byte-gleich; ob Linux gleich ist, zeigt der CI-Lauf (`task test` auf `ubuntu-latest`).
  `tools/paper/golden/regel.ts` › `GOLDEN_REGEL` muss zum Stand in `paper-spec.md` › 6 passen.
- Sprint-Festlegungen (README › Regeln) mit Vertrags-Nachträgen (Reit-Rahmen im Manifest, Regel `reittier-unbekannt`,
  `renderKey` ohne Reittier) gehen als Befund an die Haupt-Session (Ticket im Projekt PAP), nicht in den Vertrag.

## Erlaubte Dateien

- `tools/paper/` außer `schema/` und `fixtures/` (nur Fixes aus Befunden)
- `public/grafik/figuren/`, `public/grafik/reittiere/` (nur neu gerenderte Ausgabe nach einem Fix)
- `docs/sprints/` (Abnahme, Status)

## Nicht-Ziele

Keine neuen Funktionen, keine Vertragsänderung, keine Migration weiterer Assets.

## Schritte

1. Ergebnis-Abschnitte RD1.1–RD1.4 lesen; je Kriterium den Nachweis prüfen (Test vorhanden, grün, misst das Kriterium).
2. `superpowers:requesting-code-review` über den Sprint-Diff (`git diff origin/develop...HEAD -- tools/paper public/grafik tools/Taskfile.yml`):
   Determinismus (kein `Math.random`, keine Zeit, keine Map-Reihenfolge aus Dateisystem ohne `sort()`), atomares
   Schreiben, Exit-Codes, Dateigrößen ≤ 400 / Funktionen ≤ 60 Zeilen, keine Importe aus `fixtures/` im Renderer.
3. Schwere Befunde beheben (test-first), leichte als Ticket (`plan_create`, Projekt PAP).
4. `task gen:paper` zweimal → kein Diff; `bun tools/paper/bin/paper.ts render public/grafik/figuren` → Exit 0.
5. Sichtvergleich Monarch `galopp.png` und Pferd `galopp.png` gegen die Prototyp-Fixtures (Read-Tool), Urteil ins Ergebnis.
6. CI-Lauf des Sprint-Branches abwarten: Golden-Tests auf Linux grün? Falls nicht: Ergebnis mit Abweichung
   (Anteil Pixel) melden, `GOLDEN_REGEL` nur ändern, wenn `paper-spec.md` › 6 bereits `toleranz` festlegt; sonst Ticket.
7. Abnahme (höchstens fünf Zeilen) in die Sprint-README, Status der Session `fertig`.

## Fertig, wenn

- [ ] AC-01 bis AC-05: je Kriterium Nachweis im Ergebnis (Testname und Befehl).
- [ ] `task check`, `task check:go`, `task check:race` grün (nacheinander; `check:race` ohne gcc lokal = übersprungen, so nennen).
- [ ] CI des Sprint-Branches grün, Golden auf Linux bestätigt.
- [ ] Abnahme in `README.md` ausgefüllt.

## Prüfen

```bash
task check
task check:go
task check:race
```

Review-Session: die drei Befehle nacheinander, frisch vor dem Commit (`superpowers:verification-before-completion`).
Manuelle Prüfungen (Browser, Xbox, TV) nur, wenn diese Datei sie nennt und 🧑 sie für diesen Lauf freigegeben hat.

## Ergebnis

- AC-01 bis AC-05: Nachweise je Kriterium in RD1.1–RD1.4 geprüft, die Tests messen die Kriterien (AC-01 `bin/paper.test.ts`, AC-02 `golden/golden.test.ts` + `golden/public.test.ts`, AC-03 `rig/*.test.ts` + Sichtvergleich, AC-04 `golden/stil.test.ts`, AC-05 `vertrag/cli.vertrag.test.ts`).
- Prüfungen: `task check` grün; `task check:go` grün bis auf `cmd/k3c-load` › `TestGleicherSeedGleicheEingaben` (zeitabhängig unter Last, einzeln grün, RD1 ändert keinen Go-Code); `task check:race` lokal übersprungen (kein gcc). CI per `workflow_dispatch` auf `sprint/rd1` (Run 42279806963, `77960eac`): alle Jobs grün – Golden auf Linux byte-gleich zu Windows, Regel `exakt` bestätigt.
- Sprint-Review (Opus, ganzer Branch): nichts Kritisches; drei wichtige Befunde behoben: (1) Golden-Vergleich folgt jetzt `GOLDEN_REGEL` (`goldenAbweichungen`, Test „anderer Hash: exakt meldet …, toleranz vergleicht die Pixel“); (2) `validate` prüft auch die Render-Regeln (`rig/pruefen.ts`: Figur-Animationen, Reittier, Gangart, Beinteile; vier Tests); (3) Aufräumen löscht nur Reste beendeter Prozesse (Test „Temp-Ordner laufender Prozesse bleiben stehen“). Dazu: Reittier-Suche überspringt nicht lesbare Nachbarordner.
- Leichte Befunde, offen (für Folge-Tickets der Haupt-Session): Register-Import hängt an `render/ausgabe.ts` (render → rig); `render <ordner>` ohne Specs schreibt `index.paper.json = []`; `pixeldiff` prüft die PNG-Signatur nicht; kein Test über den echten Prozess-Einstieg (`bun …/paper.ts`); Pferd-Geometrie (Hüfte −56, HL1/HL2, Zügelpunkte, Boden −103) fest im Code statt aus Spec-Pivots (relevant für weitere Reittiere, MG*); Toleranzen in `gangarten.test.ts` grob; Task `paper` ohne `package.json`/`bun.lock` in `sources`; bun (JSC) vs. node (V8) bei `Math.sin` unter Linux ungeprüft (CI prüft node).
- Sichtvergleich mit dem Prototyp (Claude, Read-Tool): Idle, Lauf, Pferd-Galopp und Reiten stimmen in Form, Farben und Bewegung; Unterschiede: Saum fehlt (D-RD1-03), Zügel blasser; endgültige Beurteilung durch 🧑.
