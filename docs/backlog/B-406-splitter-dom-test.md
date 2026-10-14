# B-406 · Doppelklick und Ziehen des Splitters sind in einer DOM-Umgebung getestet

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ4
- **Projekt:** WZG
- **Erstellt:** 2026-10-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-14, 🧑 im Chat (Revision 1)

## Ausgangslage

WZ3.1/AC-06: Die Ziehberechnung des Splitters ist als `splitShare` und `clampSplit` getestet (`Splitter.test.tsx`). Doppelklick (`onDoubleClick={onReset}`) und der Pointer-Ablauf sind ohne DOM-Umgebung nicht prüfbar; im Root-Vitest gibt es kein jsdom und kein happy-dom.

## Ziel

Doppelklick (setzt zurück) und Ziehen (setzt den Anteil) des Splitters sind in einer DOM-Umgebung getestet. Entscheidung nötig: DOM-Umgebung als neue Dev-Abhängigkeit (🧑).

## Beteiligte und Zielgruppen

Entwickler und CI, die `task check` ausführen; 🧑 entscheidet über die neue Dev-Abhängigkeit.

## Anforderungen

- Der Test rendert `Splitter` (`tools/k3c-dev/frontend/src/tasks/TaskBars.tsx`) in einer DOM-Umgebung und löst echte Ereignisse aus (Doppelklick, `pointerdown`/`pointermove`/`pointerup`).
- Läuft offline und deterministisch im Root-Vitest (`task test`, `task check`), ohne Dienste.
- Die DOM-Umgebung wird nur für diese Testdatei aktiv (Docblock `@vitest-environment`), alle übrigen Tests bleiben in Node.

## Nicht-Ziele

Umbau des Splitters oder der Planungsseite; Tests weiterer Komponenten in DOM-Umgebung (bei Bedarf eigenes Ticket); Änderung der Lint-Regeln.

## Regeln und Einschränkungen

Komplexitäts-Budget (Datei ≤ 400 Zeilen). Keine neue Abhängigkeit ohne Zustimmung von 🧑 (mit der Spec-Freigabe): vorgeschlagen ist **`happy-dom`** als einzige neue devDependency (leichter als jsdom, bringt `PointerEvent`). Da Vitest vom Root läuft, trägt `package.json` im Root sie ein; `tools/k3c-dev/frontend/package.json` bleibt unverändert.

## Beispiele

- Doppelklick auf den Balken → `onReset` wird genau einmal aufgerufen.
- `pointerdown`, dann `pointermove` auf 25 % der Breite des Elternelements → `onChange(25)`; Bewegung auf 95 % → `onChange(80)`.
- Nach `pointerup` löst eine weitere `pointermove` kein `onChange` mehr aus.

## Ausnahme- und Fehlerfälle

Elternelement ohne messbare Box (`getBoundingClientRect` fehlt) → `pointerdown` tut nichts, `onChange` wird nicht gerufen. happy-dom liefert Boxen mit 0; der Test setzt die Box per `vi.spyOn`.

## Akzeptanzkriterien

- **AC-01** `tools/k3c-dev/frontend/src/planning/SplitterDom.test.tsx` läuft in `happy-dom` und prüft: Doppelklick ruft `onReset`; Ziehen (`pointerdown` + `pointermove`) ruft `onChange` mit dem geklemmten Anteil; nach `pointerup` keine Änderung mehr. `task test -- SplitterDom` ist grün.

## Offene Fragen

Neue devDependency `happy-dom` im Root (🧑 mit der Spec-Freigabe). Alternative ohne Abhängigkeit gibt es nicht: `renderToStaticMarkup` löst keine Ereignisse aus.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
