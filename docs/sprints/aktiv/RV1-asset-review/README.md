# RV1 · DEV · Asset-Review mit Git-Diff, Pixel-Overlay und Teil-Markierung

- **Status:** aktiv
- **Projekt:** PAP
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-699
- **Start-Commit:** 8b0e2f18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-06, Chat (🧑: „alles bis zur Abnahme abarbeiten“; Defaults in B-701)

## Ausgangslage

Welle 1, Lane **RV1** (DEV, Lane `review`, parallel zu RD1, MC1, WB1). Vertrag: `docs/standards/paper-mcp.md` › Oberfläche (RPC); die Methoden liegen seit PV1.8 als Stubs in `internal/assetsui` (`Stubs{Root}` liest `tools/paper/fixtures/ui/<Methode>.json`) und sind in `uiMethods`/`wails.ts` (Typen `unknown`) und `app_assets.go` verdrahtet; `frontend/src/assets/review/index.tsx` ist ein Platzhalter `ReviewPage`. Hash-Regel: `tools/paper/schema/hash.ts` (nur TypeScript, Go portiert). CLI `validate`, `diff`, `pixeldiff`, `frame` aus `paper-spec.md` › 8; bis RD1 die Attrappe `tools/paper/fixtures/cli-stub.ts` (`frame` mit festen PNGs). Test-Repo: `tools/paper/fixtures/review-repo/erzeuge.ts` (`HEAD~1` = `32eb7a7`).

## Ziel

Alle Asset-RPC-Methoden liefern echte Daten, und die Startseite von Assets zeigt jede Asset-Änderung gegen eine wählbare Git-Basis mit Vorher/Nachher, Pixel-Diff-Overlay, Teil-Markierung, synchronen Animationen und Prompt-Buttons.

Am Ende sichtbar: Nach einer Änderung am Umhang-Material zeigt die Review-Seite den Monarchen mit umrandetem Umhang.

## Beteiligte und Zielgruppen

- **Profitiert:** Ralf als Gestalter der Assets (sieht jede Asset-Änderung eines Agenten auf einen Blick) und Agenten, die per Prompt-Button einen fertigen Auftrag bekommen.
- **Arbeitet:** autonome Agenten der Lane `review` (RV1.1–RV1.5).
- **Entscheidet und prüft:** 🧑 Ralf (Freigabe der Spec, manuelle Abnahme der Seite).
- **Nachbarn:** WB1 (hängt `ReviewPage` in die Navigation, nutzt `AssetsList`/`AssetsGet`), MC1 (`assets_review` und `assets_preview` mit derselben Basis-Regel), RD1 (echte CLI).

## Anforderungen

Entscheidung 005 › 6, 7.

## Nicht-Ziele

Navigation und Katalog (WB1), MCP (MC1).

## Regeln und Einschränkungen

Nur `tools/k3c-workbench/internal/assetsui/` (Besitzer aller Asset-RPC-Methoden), `tools/k3c-workbench/app_assets.go` (nur `assets()` auf den echten Dienst umstellen), `tools/k3c-workbench/internal/review/` (neu: Basis, `git show`, Cache `.cache/paper/<renderKey>/`, Diff) und `tools/k3c-workbench/frontend/src/assets/review/`. Signaturen der RPC-Methoden nur laut Vertrag (sonst Ticket); `frontend/src/api/wails.ts` gehört WB1 und bleibt bei `unknown`, Typen in `frontend/src/assets/review/types.ts`. Specs aus `git show` kanonisch hashen (Go-Port von `hash.ts`, gegen bun geprüft); `origin/develop` ohne Fetch → `basisStand: lokal`. Bilder, Diffs und Prüfungen nur über die CLI, kein Renderer in Go. Go-Tests rufen bun (CI-Job `k3c-workbench` hat bun), ohne bun scheitern sie, kein Skip. `ValueSlider` aus `@webkit/atoms` (PV1.8; der rohe Radix-`Slider` bleibt daneben).

## Beispiele

- Test-Repo, Basis `HEAD~1` → `AssetsReview` liefert monarch `geaendert` (`umhang.material: koenigsrot → purpur`, `pixelAnteil` 0.015326), pferd `neu`, fass `geloescht`, banner `ungueltig` (Material `scharlach`), mauer `geaendert` mit `stil: true` (0.170654).
- Gleiches Repo, Basis `HEAD` → nur monarch und banner.
- Review-Seite, Eintrag monarch → Vorher (rot) und Nachher (purpur) nebeneinander, Überblendregler, Schalter „Unterschiede“ legt das Overlay mit einer Box um den Umhang.
- Prompt-Button „Nachbessern …“ mit Eingabe „etwas dunkler“ → Zwischenablage enthält den Prompt mit `monarch`, Pfad, Basis, Feldänderung und Eingabe.

## Ausnahme- und Fehlerfälle

- Basis `main`, `HEAD~2` oder unbekannter Commit → `Basis „x“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/develop`, die Seite zeigt den Text statt der Liste.
- `origin/develop` ohne erreichbares Remote → lokaler Stand, `basisStand: lokal`, Hinweis auf der Seite.
- Nur Zeilenenden (CRLF) oder Schlüsselreihenfolge geändert → kein Eintrag.
- Unbekannte `id` → `Asset „x“ fehlt`; unbekannte Animation → Text mit den vorhandenen Animationen; leere Animation = erste Animation.
- CLI-Systemfehler (Exit 2) → Fehler mit stderr, kein halber Cache-Eintrag (atomar schreiben).
- Frame-Index außerhalb der Basis-Animation → Vorher zeigt „Frame fehlt in der Basis“, Nachher läuft weiter.

## Akzeptanzkriterien

- **AC-01** `AssetsReview` liefert für das Test-Repo die Fälle `neu`, `geaendert`, `geloescht`, `ungueltig`, `stil` exakt wie `fixtures/ui/` (Go-Test).
- **AC-02** Basis `HEAD`, `HEAD~1`, Commit und `origin/develop` wählbar; ungültige Basis → Fehlertext mit erlaubten Werten; CRLF ändert den Hash nicht (005/AC-08).
- **AC-03** Seite zeigt Vorher/Nachher, Überblendregler, Pixel-Diff-Overlay mit Boxen und markiert geänderte Teile (Vitest) (005/AC-08).
- **AC-04** Animationen laufen alt und neu synchron mit Frame-Schieber.
- **AC-05** Ungültige Spec als roter Eintrag mit Fehlertext, die übrigen werden angezeigt.
- **AC-06** Prompt-Buttons am Review-Eintrag (Vorlagen `ort: review`) kopieren den gefüllten Prompt (005/AC-09).
- **AC-07** `AssetsList`, `AssetsGet`, `AssetsStyle`, `AssetsPrompts` liefern echte Daten aus dem Checkout (Go-Test).

## Offene Fragen

Alle mit Default entschieden (gilt bis zur Freigabe durch 🧑):

- **Stil der Basis beim Rendern:** `frame` hat keinen Stil-Parameter. Default: Basis als Mini-Baum im Cache (`basis/public/grafik/…`); reicht das der echten CLI nicht, Ticket PAP für `--stil <datei>` (RV1.1).
- **Fixture `AssetsList.json`:** meldet pferd und mauer `geaendert: true`, laut Vertrag gilt `HEAD` (dort unverändert). Default: Test erwartet `false`, Ticket PAP zum Nachziehen der Fixture (RV1.2).
- **Reihenfolge der Review-Einträge:** Vertrag offen. Default: `ungueltig`, `geaendert`, `neu`, `geloescht`, je Status nach `id`; Tests vergleichen nach `id` sortiert.
- **Leere Animation:** Default = erste Animation der Spec (wie MCP), nötig für gelöschte Assets.
- **`app_assets.go`:** Default: RV1.2 darf `assets()` umstellen (sonst bliebe der Typname `Stubs` für echte Daten).
- **Umhang-Markierung:** Der Umhang ist kein Teil; Default: er wird über die Pixel-Diff-Box umrandet, die Teil-Markierung gilt nur für Ids aus `teile`.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| RV1.1 | `RV1.1-review-basis-diff.md` | Umsetzung | autonom | offen |
| RV1.2 | `RV1.2-assetsui-echte-daten.md` | Umsetzung | autonom | offen |
| RV1.3 | `RV1.3-review-seite-vergleich.md` | Umsetzung | autonom | offen |
| RV1.4 | `RV1.4-animation-prompts.md` | Umsetzung | autonom | offen |
| RV1.5 | `RV1.5-review.md` | Review | autonom | offen |

Lane `review`, nacheinander; RV1.3 braucht nur PV1.8 (Vitest gegen Fixtures) und darf vor RV1.2 laufen:

- RV1.1 Opus: `internal/review` Basis, `git show`, Hash-Port, Cache, Diff je Asset und Stil (AC-01, AC-02)
- RV1.2 Sonnet: `internal/assetsui` echte Daten, Pixel-Diff, Frame (AC-05, AC-07) – nach RV1.1
- RV1.3 Opus: `ReviewPage` Liste, Basis-Auswahl, Vergleich, Overlay, Teil-Markierung (AC-03, AC-05)
- RV1.4 Sonnet: Animationen synchron, Prompt-Buttons am Eintrag (AC-04, AC-06) – nach RV1.3
- RV1.5 Review · Opus – nach allen

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
