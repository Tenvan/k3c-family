# RV1.4 · Animationen synchron, Prompt-Buttons am Review-Eintrag

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** review
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** rv1/4-animation-prompts
- **Abhängig von:** RV1.3
- **Tickets:** B-699
- **Kriterien:** AC-04, AC-06

## Ziel

Auf `develop` spielt die Review-Seite Vorher und Nachher einer Animation synchron mit gemeinsamem Frame-Schieber ab, und jeder Review-Eintrag hat Prompt-Buttons (Vorlagen `ort: review`), die den gefüllten Prompt in die Zwischenablage kopieren.

## Kontext

- **Stand RV1.3:** `tools/k3c-workbench/frontend/src/assets/review/` mit `ReviewPage`, `Compare.tsx` (Vorher/Nachher für Frame 0), `types.ts`, `api.ts`.
- **Animationen:** `AssetsGet(id).animationen` = Name → `{frames, fps?}` (`fps` Standard 8, `docs/standards/paper-spec.md` › 3). Ein gemeinsamer Frame-Index für beide Seiten, Bereich aus dem Arbeitsstand; liefert die Basis für einen Index einen Fehler (Animation kürzer oder fehlt), zeigt die Vorher-Seite „Frame fehlt in der Basis“. Frames je `id|animation|frame|basis` im Speicher halten, damit die Wiedergabe nicht jedes Mal neu anfragt.
- **Prompt-Vorlagen:** `AssetsPrompts()` = `data/asset-prompts.json` (Fixture `tools/paper/fixtures/ui/AssetsPrompts.json`); am Review-Eintrag nur `ort: review` (`aenderung-zuruecknehmen`, `nachbessern`). Platzhalter laut `docs/standards/paper-mcp.md` › 3: `{asset}`, `{rig}`, `{pfad}`, `{zustand}` (Feldänderungen als `pfad: alt → neu`, mit `; ` verbunden; Stiländerung „Stil geändert“; Status ungültig mit Fehlertexten), `{basis}` (gewählte Basis), `{eingabe}` (Pflicht bei `ort: review`, Freitextfeld am Button). `{teil}` kommt am Review-Eintrag nicht vor.
- Kopieren mit `navigator.clipboard.writeText`, Rückmeldung „Kopiert.“ bzw. Fehlertext; Muster: `src/balancing/HintsTab.tsx` und dessen Test (Clipboard-Mock per `Object.defineProperty`).
- `prompt.ts` (reine Funktion `fillPrompt`) ist exportiert, damit WB1 sie für `ort: asset|teil` mitnutzen kann.
- Bausteine nur aus `@webkit/*`; Frame-Schieber = `ValueSlider`.

## Erlaubte Dateien

- `tools/k3c-workbench/frontend/src/assets/review/`

## Nicht-Ziele

- Keine Prompt-Buttons am Asset im Katalog oder am Teil der Vorschau (WB1).
- Keine Änderung an `data/asset-prompts.json` oder an den MCP-Prompts (MC1).
- Kein Abspielen mit Physik-Neuberechnung im Client: Frames kommen fertig vom Server.

## Schritte

1. **Rot** (`prompt.test.ts`): `füllt alle Platzhalter` – je Vorlage `ort: review` aus der Fixture mit dem monarch-Eintrag aus `AssetsReview.json`, Basis `HEAD~1`, Eingabe „etwas dunkler“ → kein Platzhalter mehr (`/\{(asset|rig|teil|pfad|zustand|basis|eingabe)\}/`; das wörtliche `{rig: …}` der Vorlagen bleibt), Text enthält `monarch`, `{rig: figur}`, `public/grafik/figuren/monarch.paper.json`, `HEAD~1`, `etwas dunkler`; bei `nachbessern` zusätzlich `umhang.material: koenigsrot → purpur`; `Stiländerung im Zustand` – mauer → „Stil geändert“; `ungültig im Zustand` – banner → Fehlertext enthalten.
2. **Rot** (`PromptButtons.test.tsx`): `zeigt nur Vorlagen am Review-Eintrag` – zwei Buttons mit den Titeln der Fixture („Änderung zurücknehmen“, „Nachbessern …“); `Button gesperrt ohne Eingabe`; `kopiert den gefüllten Prompt` – nach Eingabe und Klick genau ein `writeText` mit dem Text aus `fillPrompt`, Meldung „Kopiert.“ (AC-06); `Fehler beim Kopieren sichtbar`.
3. **Rot** (`AnimationSync.test.tsx`, `vi.useFakeTimers`): `Schieber setzt beide Seiten` – `ValueSlider` „Frame“ auf 5 → `AssetsFrame` für Basis und Arbeitsstand mit Frame 5; `Wiedergabe synchron` – Play, nach 125 ms (fps 8) beide mit Frame 1, nach `frames` Schritten wieder 0; `Animation wechseln` – Auswahl `reiten` → beide Aufrufe mit `reiten`, Index 0; `Frame fehlt in der Basis` – Mock wirft für die Basis → Hinweis, Nachher läuft weiter; `Cache` – zweiter Durchlauf ruft `AssetsFrame` nicht erneut (AC-04).
4. **Grün:** `prompt.ts`, `PromptButtons.tsx`, `AnimationSync.tsx` (Animation-Auswahl, Play/Pause, Frame-Schieber), in `Compare.tsx` und `ReviewList.tsx` einhängen; Timer beim Verlassen der Seite aufräumen.
5. Vitest grün, Commit `feat(dev): synchrone Animationen und Prompt-Buttons im Review (RV1.4)`.

## Fertig, wenn

- [ ] AC-04: `AnimationSync.test.tsx` grün (Schieber, synchrone Wiedergabe, Animation wechseln, Cache).
- [ ] AC-06: `prompt.test.ts` und `PromptButtons.test.tsx` grün (kopierter Text vollständig gefüllt).
- [ ] `tests/atomicDesign.test.ts` grün (`task test -- atomicDesign`).
- [ ] `task check:diff` grün.

## Prüfen

```bash
task k3c-workbench:vitest
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
