# B-866 · check_run nimmt Testmuster mit Oder-Strich, Dach und Dollar an

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** WZ54
- **Projekt:** WZG
- **Erstellt:** 2026-11-14
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-14
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

WZ49.3: `patternRule` in `check_run` lässt kein `|`, `^` oder `$` zu; Muster wie `TestA|TestB` gehen nicht.

## Ziel

`check_run` nimmt Go- und Vitest-Muster mit Alternativen und Ankern an, ohne Shell-Einschleusung zu erlauben.

## Beteiligte und Zielgruppen

Agenten bei gezielten Tests; Claude (Umsetzung).

## Anforderungen

- Erlaubt: Buchstaben, Ziffern, `_`, `-`, `/`, `.`, `|`, `^`, `$`, `(`, `)`.
- Weitergabe ohne Shell-Auswertung (Argument-Liste oder Quoting) belegt.

## Nicht-Ziele

Andere Parameter.

## Regeln und Einschränkungen

Klasse A (Problem, DEV); Eingabe-Code: Review Pflicht.

- **Bereich:** `tools/k3c-workbench/internal/mcpsrv/check_*.go`
- **Abhängig von:** WZ49
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** exe

## Beispiele

`check_run go:test pattern=TestA|TestB` → beide Tests laufen.

## Ausnahme- und Fehlerfälle

`;`, Backtick, `&` → abgelehnt mit erlaubtem Zeichenvorrat.

## Akzeptanzkriterien

- **AC-01** Test: `TestA|TestB` und `^TestA$` laufen, `;` wird abgelehnt.

## Offene Fragen

keine

## Notizen

Befund WZ49.3 (2026-11-14).
