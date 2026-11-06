# WB1.3 · Prompt-Buttons, Teil-Auswahl per Klick

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** ui
- **Modell:** sonnet
- **Umgebung:** offline
- **Branch:** wb1/3-prompt-buttons
- **Abhängig von:** WB1.2
- **Tickets:** B-699
- **Kriterien:** AC-03

## Ziel

Im Katalog kopieren Prompt-Buttons am Asset (`ort: asset`) und am angeklickten Teil der Vorschau (`ort: teil`) den fertig gefüllten Prompt aus `AssetsPrompts` in die Zwischenablage; ein Klick in die Vorschau wählt das Teil. Die Platzhalter-Ersetzung liegt als reine Funktion vor, die auch RV1 für `ort: review` nutzen kann.

## Kontext

- Vertrag `docs/standards/paper-mcp.md` › 3 Prompt-Vorlagen: `AssetsPrompts()` → `[{id, titel, ort, text}]`, `ort` ∈ `asset`, `teil`, `review`. Platzhalter **nur** `{asset}` (ID), `{rig}`, `{teil}` (Pflicht bei `teil`), `{pfad}` (Spec-Pfad), `{zustand}` (kurzer Zustand, z. B. „gültig, Version 1“), `{basis}` (Review-Basis), `{eingabe}` (Freitext, Pflicht bei `review`). Fixture `tools/paper/fixtures/ui/AssetsPrompts.json`: zehn Vorlagen, sechs `asset`, zwei `teil`, zwei `review`; Text beginnt mit „Rufe zuerst `assets_guide {rig: {rig}}` auf“.
- Entscheidung 005 › 5: Prompt-Buttons am Asset, am angeklickten Teil (Klick in die Vorschau wählt das Teil) und am Review-Eintrag (der gehört RV1).
- Voraussetzung WB1.2: Katalog mit Vorschau aus `AssetsFrame` → `{bild, teile: {teilId: [x, y, b, h]}}` in Pixeln des Frames; das Bild wird skaliert angezeigt.
- Vorbild für Kopieren und „In Claude öffnen“: `frontend/src/planning/PromptParts.tsx` (`navigator.clipboard.writeText`, `backend.openInClaude(prompt)`, Rückmeldung über `useFeedback`). Kopieren und „In Claude öffnen“ sind keine schreibenden Controls (B-701 D-12); `readonly.test.tsx` aus WB1.2 muss grün bleiben.
- Teil-Treffer: Boxen überlappen (z. B. `rumpf` umschließt `oberarm-h`, `oberarm-h` und `oberarm-v` haben dieselbe Box). Regel (D-05 der Sprint-README): kleinste Fläche gewinnt, bei Gleichstand das spätere Teil in `Spec.teile` (oben gezeichnet); Klick außerhalb aller Boxen hebt die Auswahl auf.

## Erlaubte Dateien

- `tools/k3c-workbench/frontend/src/assets/prompts/` (neu: `fill.ts`, `PromptButtons.tsx`, Tests)
- `tools/k3c-workbench/frontend/src/assets/catalog/` (Vorschau-Klick, Buttons einhängen)
- `tools/k3c-workbench/frontend/src/assets/readonly.test.tsx`, `assets/codemap.md`

## Nicht-Ziele

Keine Prompt-Buttons am Review-Eintrag (RV1.4, nutzt `fill.ts`); keine Änderung der Vorlagen (`data/asset-prompts.json` gehört PAP/MC1); keine Go-Änderung.

## Schritte

1. **Rot:** `prompts/fill.test.ts`: `fillPrompt(vorlage, werte)` ersetzt alle sieben Platzhalter, auch mehrfach vorkommende; ein fehlender Pflichtwert (`teil` bei `ort: teil`, `eingabe` bei `ort: review`) liefert einen Fehler statt eines halben Prompts; ein unbekannter Platzhalter `{xyz}` bleibt nicht stehen, sondern führt zum Fehler (Vorlagen-Fehler sichtbar machen); `zustandVon(eintrag, spec)` liefert „gültig, Version 1“ bzw. „ungültig: `feld · regel`“. Für jede der zehn Fixture-Vorlagen: Ergebnis enthält keine `{`…`}` mehr und beginnt mit „Rufe zuerst `assets_guide {rig: figur}` auf“ für `monarch`.
2. **Rot:** `prompts/hit.test.ts`: `teilAn(teile, reihenfolge, x, y)` mit den Boxen aus `AssetsFrame.json` (Klick in den Kopf → `kopf`, in den Oberarm → `oberarm-v` laut Reihenfolge, außerhalb → keins); Umrechnung Anzeige- in Bildpixel über natürliche und angezeigte Größe.
3. **Rot:** `catalog/CatalogPrompts.test.tsx`: Am gewählten Asset stehen die sechs `asset`-Buttons mit `titel`; Klick kopiert den gefüllten Prompt (`navigator.clipboard.writeText` gemockt, Text enthält `monarch` und den Pfad); Klick in die Vorschau markiert das Teil (Rahmen über der Box) und zeigt die zwei `teil`-Buttons, deren Prompt das Teil nennt; ohne gewähltes Teil keine `teil`-Buttons.
4. Umsetzen: `fill.ts` (reine Funktionen), `PromptButtons.tsx` (Molekül der App: Buttons je Vorlage, Kopieren und „In Claude öffnen“ wie `PromptParts.tsx`), Klick-Auswahl und Teil-Rahmen in der Katalog-Vorschau.
5. `readonly.test.tsx` um die neuen Buttons ergänzen (Kopieren und `openInClaude` erlaubt); Codemap nachziehen.
6. Grün: gezielte Vitest-Läufe, dann einmal `task check:diff`.
7. Commit `feat(dev): Prompt-Buttons am Asset und am Teil (WB1.3)`.

## Fertig, wenn

- [ ] AC-03: `bunx vitest run --config tools/k3c-workbench/frontend/vitest.config.ts prompts CatalogPrompts` grün (Platzhalter-Ersetzung aller Fixture-Vorlagen, Pflichtwerte, Teil-Treffer, Kopieren).
- [ ] AC-02 bleibt: `bunx vitest run --config tools/k3c-workbench/frontend/vitest.config.ts readonly` grün.
- [ ] `task check:diff` grün.

## Prüfen

```bash
task check:diff
```

Während der Arbeit nur gezielt: `bunx vitest run --config tools/k3c-workbench/frontend/vitest.config.ts <muster>`; mit k3c-workbench `check_run task:check:diff` mit `checkout`.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
