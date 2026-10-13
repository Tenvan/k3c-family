# Drittanbieter-Hinweise

## catch-me-up (Oliver Wang)

Die Zusammenfassungs-Funktion von tenvan-cc folgt dem Konzept von catch-me-up (https://github.com/oliverow/catch-me-up, Version 0.1.3).
`assets/catchup.en.md` ist dessen `INSTRUCTIONS.md` unverändert, `assets/catchup.de.md` eine deutsche Neufassung. Der Code ist eine eigene TypeScript-Umsetzung.

```text
MIT License

Copyright (c) 2026 Oliver Wang

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## claude-skins (hellosverre) und prismantis (Nahum Litvin, Oliver Wang)

Die Chat-Skins von tenvan-cc (`hooks/skins.ts`, Zeichnung in `hooks/register.tsx`) übernehmen Ideen aus claude-skins (Tool-Symbole, Diff-Karte, Details mit Code-Element, Prompt-Rahmen, Hervorhebungen, Code- und Tabellenkarten, Mermaid-Karten, Formelkarten; Parser und Layout in `hooks/mermaid.ts` sind eigener Code) und prismantis (ruhige Ergebnisse, echte Fehlerzeile). Beide stehen unter der MIT-Lizenz (Copyright (c) 2026 hellosverre bzw. Copyright (c) 2026 Nahum Litvin). Der Code ist eine eigene Umsetzung; einzig `mdLiteral` folgt in der Technik (Satzzeichen per Backslash maskieren, führende Leerzeichen als geschützte Leerzeichen) der Funktion `literal` aus claude-skins `hooks/rows.tsx`. Lizenztext wie oben (MIT).

## Mindful-Claude (Halluton)

Die Atemübung im Band folgt der Idee von Mindful-Claude (https://github.com/halluton/Mindful-Claude, Stand 411c9c4): Atemübungen hrv, sigh, box und 4-7-8, solange Claude arbeitet, die Phase auch in der Arbeitszeile. Kein Code übernommen; die Umsetzung (`hooks/breath.ts`, SMIL-Svg statt Terminal-Client) ist eigen. Lizenz: MIT, Copyright (c) 2026 Halluton.

## guided-mr (Alex Jacobs)

Die Merge-Request-Funktion übernimmt Ideen aus guided-mr (https://github.com/alexjacobs08/guided-mr, Stand 46027a2): MRs über `glab` lesen, Status im Pane, Aktionen per Taste. Das Repo hat keine Lizenzdatei; es wurde kein Code übernommen, die Umsetzung (`hooks/mr.ts`, Teile in `hooks/register.tsx`) ist eigen.
