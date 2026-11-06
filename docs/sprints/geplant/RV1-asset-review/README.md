# RV1 · DEV · Asset-Review mit Git-Diff, Pixel-Overlay und Teil-Markierung

- **Status:** geplant
- **Projekt:** PAP
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-699
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Welle 1, Lane **RV1** (DEV, Lane `review`, parallel zu RD1, MC1, WB1). Vertrag: `docs/standards/paper-mcp.md` › Oberfläche (RPC); die Methoden liegen seit PV1.8 als Stubs in `internal/assetsui` und sind in `uiMethods`/`wails.ts` verdrahtet. CLI `diff`, `pixeldiff`, `frame` aus `paper-spec.md`; bis RD1 die Attrappe. Test-Repo: `tools/paper/fixtures/review-repo/`.

## Ziel

Alle Asset-RPC-Methoden liefern echte Daten, und die Startseite von Assets zeigt jede Asset-Änderung gegen eine wählbare Git-Basis mit Vorher/Nachher, Pixel-Diff-Overlay, Teil-Markierung, synchronen Animationen und Prompt-Buttons.

Am Ende sichtbar: Nach einer Änderung am Umhang-Material zeigt die Review-Seite den Monarchen mit umrandetem Umhang.

## Beteiligte und Zielgruppen

Wer profitiert, wer arbeitet, wer entscheidet oder prüft (🧑)?

## Anforderungen

Entscheidung 005 › 6, 7.

## Nicht-Ziele

Navigation und Katalog (WB1), MCP (MC1).

## Regeln und Einschränkungen

Nur `tools/k3c-workbench/internal/assetsui/` (Besitzer aller Asset-RPC-Methoden), `tools/k3c-workbench/internal/review/` (neu: Basis, `git show`, Cache `.cache/paper/<renderKey>/`, Diff) und `tools/k3c-workbench/frontend/src/assets/review/`. Signaturen der RPC-Methoden nur laut Vertrag (sonst Ticket). Specs aus `git show` kanonisch hashen (PV1.3 `hash.ts`); `origin/develop` ohne Fetch → `basisStand: lokal`. `ValueSlider` aus `@webkit/atoms` (PV1.8; der rohe Radix-`Slider` bleibt daneben). Typen der Asset-Methoden in `frontend/src/assets/review/types.ts`; `frontend/src/api/wails.ts` gehört WB1 und bleibt bei `unknown`.

## Beispiele

Typische Situation → erwartetes Ergebnis.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten.

## Akzeptanzkriterien

- **AC-01** `AssetsReview` liefert für das Test-Repo die Fälle `neu`, `geaendert`, `geloescht`, `ungueltig`, `stil` exakt wie `fixtures/ui/` (Go-Test).
- **AC-02** Basis `HEAD`, `HEAD~1`, Commit und `origin/develop` wählbar; ungültige Basis → Fehlertext mit erlaubten Werten; CRLF ändert den Hash nicht (005/AC-08).
- **AC-03** Seite zeigt Vorher/Nachher, Überblendregler, Pixel-Diff-Overlay mit Boxen und markiert geänderte Teile (Vitest) (005/AC-08).
- **AC-04** Animationen laufen alt und neu synchron mit Frame-Schieber.
- **AC-05** Ungültige Spec als roter Eintrag mit Fehlertext, die übrigen werden angezeigt.
- **AC-06** Prompt-Buttons am Review-Eintrag (Vorlagen `ort: review`) kopieren den gefüllten Prompt (005/AC-09).
- **AC-07** `AssetsList`, `AssetsGet`, `AssetsStyle`, `AssetsPrompts` liefern echte Daten aus dem Checkout (Go-Test).

## Offene Fragen

keine

## Sessions

Reife Entwurf, Lane `review`:

- RV1.1 Opus: `internal/review` Basis, `git show`, Cache, Diff je Asset und Stil (AC-01, AC-02)
- RV1.2 Sonnet: `internal/assetsui` echte Daten, Pixel-Diff, Frame (AC-05, AC-07)
- RV1.3 Opus: `ReviewPage` Liste, Vergleich, Overlay, Teil-Markierung (AC-03)
- RV1.4 Sonnet: Animationen synchron, Prompt-Buttons am Eintrag (AC-04, AC-06)
- RV1.5 Review · Opus

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
