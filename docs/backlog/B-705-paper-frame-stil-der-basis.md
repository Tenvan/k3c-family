# B-705 · Das Vorher-Bild im Review rendert mit dem Stil der Basis

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ43
- **Projekt:** WZG
- **Erstellt:** 2026-11-06
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** Klasse A, Claude, 2026-11-12

## Ausgangslage

Die Paper-CLI liest die Stil-Datei immer aus der Repo-Wurzel (`tools/paper/render/stil.ts`, `STIL_PFAD`). Das Review rendert das Vorher-Bild aus einem Schnappschuss-Baum der Basis, aber mit dem aktuellen Stil. Ein reiner Stilwechsel zeigt deshalb keinen Pixel-Unterschied (D-RV1-3, gemessen in RV1).

## Ziel

Vorher-Bilder im Review entstehen mit dem Stil der Basis.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- `paper frame` und `paper render` nehmen `--stil <datei>`; ohne Option gilt `public/grafik/stil.paper.json` (CLI-Teil, PT2.1).
- Das Review übergibt beim Vorher-Bild die Stil-Datei des Schnappschusses der Basis (Workbench-Teil, WZ43.2).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Vertrag: `docs/standards/paper-spec.md` › 8 (CLI) bekommt `--stil <datei>` für `frame` und `render`; Änderung nur über dieses Ticket (Projekt PAP). Der Schnappschuss-Baum der Basis liegt unter `.cache/paper/<renderKey>/basis/public/grafik/` und enthält schon `stil.paper.json` sowie die Reittier-Specs (`internal/review/cache.go`, `Quelle.schnappschuss`); das Review muss nur den Pfad übergeben (`Renderer.Frame` um den Stil erweitern, Attrappen in `review/cli_test.go` und `assetsui/helpers_test.go` nachziehen). Der Cache-Schlüssel enthält den `stilHash` der Basis bereits. Klasse A (Problem, DEV). CLI-Teil in PT2.1, Workbench-Teil in WZ43.2.

- **Bereich:** `tools/paper/cli/frame*.ts`, `tools/paper/bin/paper.ts`, `tools/paper/render/stil*.ts`, `docs/standards/paper-spec.md`, `tools/k3c-workbench/internal/review/**`, `tools/k3c-workbench/internal/assetsui/**`
- **Abhängig von:** –
- **Braucht 🧑:** –
- **Größe:** S
- **Sperre:** –

## Beispiele

Test-Repo (`tools/paper/fixtures/review-repo/erzeuge.ts`), Basis `HEAD~1`: Commit 2 macht Material `stein` heller, die Mauer-Spec bleibt gleich → Review meldet `mauer` mit `stil: true` und `pixelAnteil > 0` (Fixture: 0,170654 aus handgemachten PNGs).

## Ausnahme- und Fehlerfälle

- `--stil` zeigt auf eine fehlende Datei → Exit 2 mit Text auf stderr, kein Bild.
- Stil-Datei der Basis verletzt das Schema → Exit 1 (`fehler` je Regel); das Review zeigt beim Vorher-Bild den Fehlertext, Nachher bleibt.
- Fehlt in der Basis ein im Arbeitsstand genutztes Material, rendert Vorher mit dem Stil der Basis (Spec der Basis nutzt es ja nicht).

## Akzeptanzkriterien

- **AC-01** `paper frame` und `paper render` nehmen `--stil <datei>`; ohne Option gilt `public/grafik/stil.paper.json`.
- **AC-02** Das Review übergibt beim Vorher-Bild die Stil-Datei des Schnappschusses; ein Test mit reinem Stilwechsel zeigt `pixelAnteil > 0`.

## Offene Fragen

keine

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
