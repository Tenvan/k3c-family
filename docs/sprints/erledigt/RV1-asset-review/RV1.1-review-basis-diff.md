# RV1.1 · internal/review: Basis, git show, Cache, Diff je Asset und Stil

- **Status:** fertig
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Lane:** review
- **Modell:** opus
- **Umgebung:** offline
- **Branch:** rv1/1-review-basis-diff
- **Abhängig von:** PV1.8
- **Tickets:** B-699
- **Kriterien:** AC-01, AC-02

## Ziel

Auf `develop` liegt das neue Paket `tools/k3c-workbench/internal/review`: Es löst eine Basis auf, liest Specs und Stil-Datei der Basis per `git show`, hasht sie kanonisch wie `hash.ts`, rendert „Vorher“-Frames in den Cache `.cache/paper/<renderKey>/` und liefert je Asset den Review-Eintrag (`neu`, `geaendert`, `geloescht`, `ungueltig`, `stil`) exakt wie `tools/paper/fixtures/ui/AssetsReview.json`.

## Kontext

- **Vertrag:** `docs/standards/paper-mcp.md` › 1 (Regel `basis`) und › 4 Oberfläche (RPC): Typ `Review` = `{basis, basisCommit, basisStand: aktuell|lokal, assets: [{id, rig, pfad, status, fehler?, felder: [{pfad, alt, neu}], teile: [id], stil, pixelAnteil}]}`; Regeln für `status`, `stil`, `pixelAnteil` und Fehlertexte stehen dort wörtlich. Hash: `docs/standards/paper-spec.md` › 6, CLI: › 8.
- **Hash (`tools/paper/schema/hash.ts`):** `canonical` = Schlüssel sortiert, ohne Leerraum, `\r\n` in Strings → `\n`; `specHash`/`stilHash` = SHA-256 hex der kanonischen Form ohne `$schema`; `renderKey` = SHA-256 über `specHash:stilHash:RENDERER_VERSION` (`paper-0+resvg-js-2.6.2`). Go portiert das. **Fallstricke:** Zahlen wie JS `JSON.stringify` ausgeben (`1.50` → `1.5`, `-0` → `0`, kein `json.Number`-Literal durchreichen), Strings ohne HTML-Escaping (`SetEscapeHTML(false)`), Schlüssel sortiert (nur ASCII-Schlüssel in Specs, daher Byte-Sortierung = JS-Sortierung).
- **CLI:** `bun tools/paper/bin/paper.ts <befehl>` (RD1); solange die Datei fehlt, die Attrappe `bun tools/paper/fixtures/cli-stub.ts`. Exit 0 = JSON auf stdout, 1 = `{ok: false, fehler}`, 2 = Systemfehler (stderr). Die Attrappe rechnet `validate`, `diff`, `pixeldiff` echt, `frame` liefert feste PNGs (monarch `reiten` → `monarch-galopp-0.png`), kennt also keine Materialänderung.
- **Lücke im Vertrag:** `frame` hat keinen Stil-Parameter, „Vorher“ braucht aber die Stil-Datei der Basis. Default: `review` legt die Basis als Mini-Baum im Cache ab (`.cache/paper/<renderKey>/basis/public/grafik/stil.paper.json` und `.../<gruppe>/<id>.paper.json`) und ruft `frame` auf diese Spec; liest die echte CLI den Stil nicht relativ zur Spec, Ticket im Projekt PAP (`plan_create`, Erweiterung `--stil <datei>`) und weiterarbeiten.
- **Test-Repo:** `bun tools/paper/fixtures/review-repo/erzeuge.ts <leerer Ordner>` erzeugt ein Git-Repo (Commit 1 Basis, Commit 2 pferd neu/fass gelöscht/Stil `stein` heller, Arbeitsstand: Umhang purpur, banner mit `scharlach` ungültig); `HEAD~1` = `32eb7a7`. Basis `HEAD` → monarch `geaendert` + banner `ungueltig`; `HEAD~1` → alle fünf Fälle.
- **`pixelAnteil` der Fixture** stammt aus handgemachten PNGs (`tools/paper/fixtures/render/monarch/monarch-galopp-0.png` → `monarch-galopp-0-purpur.png`, `render/mauer/mauer-1.png` → `mauer-1-stil.png`). Der Test ersetzt daher nur `Frame` durch eine Attrappe mit diesen PNGs; `validate`, `diff`, `pixeldiff` laufen über die echte CLI-Attrappe.
- Bun läuft im CI-Job `k3c-workbench` (Windows, `setup-bun`); Go-Tests dürfen bun aufrufen. Ohne bun: `t.Fatal` mit Hinweis auf `requirements.md`, **kein** Skip.

## Erlaubte Dateien

- `tools/k3c-workbench/internal/review/` (neu)

## Nicht-Ziele

- Keine RPC-Methoden und kein `app_assets.go` (RV1.2), keine Oberfläche (RV1.3/RV1.4).
- Kein eigener Renderer und kein Pixel-Vergleich in Go: Bilder und Diffs kommen von der CLI.
- Keine Änderung an `tools/paper/` (Domäne PAP); Lücken → Ticket.

## Schritte

1. **Rot – Hash** (`hash_test.go`): `TestKanonischWieHashTS` – `Kanonisch({b:[2,1],a:{d:1,c:"x"}})` = `{"a":{"c":"x","d":1},"b":[2,1]}`; `TestSpecHashWieBun` – `SpecHash` von `tools/paper/fixtures/specs/monarch.paper.json` und `StilHash` von `public/grafik/stil.paper.json` gleich dem Wert aus `bun -e` mit `hash.ts`; `TestSpecHashCRLF` – Datei mit CRLF-Zeilenenden und umsortierten Schlüsseln hat denselben Hash (AC-02); `TestRenderKey` – Änderung von Spec-, Stil-Hash oder `RendererVersion` ändert den Schlüssel.
2. **Grün – `hash.go`:** `Kanonisch(any) string`, `SpecHash([]byte) (string, error)`, `StilHash = SpecHash`, `RenderKey(spec, stil string) string`, Konstante `RendererVersion` mit Kommentar „gleich `RENDERER_VERSION` in hash.ts“.
3. **Rot – Basis** (`basis_test.go`, Helfer `testRepo(t)` erzeugt das Test-Repo in `t.TempDir()` über bun): `TestBasisAufloesen` – `""` und `HEAD` → HEAD, `HEAD~1` → `basisCommit` `32eb7a7`, volle und kurze SHA (7 Zeichen) → derselbe Commit, `basisStand` `aktuell`; `TestBasisOriginDevelopLokal` – `git update-ref refs/remotes/origin/develop HEAD~1` ohne Remote → Commit `32eb7a7`, `basisStand` `lokal`; `TestBasisUngueltig` – `main`, `HEAD~2`, `xyz`, `deadbeef` (Format gültig, Commit fehlt) → Fehlertext exakt `Basis „x“ ungültig: HEAD, HEAD~1, Commit-SHA oder origin/develop` (AC-02).
4. **Grün – `basis.go` und `git.go`:** `Aufloesen(root, basis) (Stand{Basis, Commit, BasisStand}, error)`; `origin/develop` versucht `git fetch origin develop` (Timeout 10 s), Erfolg → `aktuell`, sonst `lokal`. `Show(root, commit, pfad) ([]byte, bool, error)` (`git show <commit>:<pfad>`, fehlend = `false`), `SpecsInBasis(root, commit)` (`git ls-tree -r --name-only <commit> -- public/grafik`), `SpecsImArbeitsstand(root)` (Dateisystem, auch unversioniert); beide nur `*.paper.json` ohne `index.paper.json` und `stil.paper.json`. Git immer mit `-c core.autocrlf=false` und ohne Shell.
5. **Rot – CLI und Cache** (`cli_test.go`, `cache_test.go`): `TestCLIValidateUngueltig` – banner aus dem Test-Repo → `fehler[0]` = `{teile[0].lagen[0].material, material-unbekannt, …}`; `TestCLIDiffUmhang` – `diff` alt/neu monarch → `felder` = `[{umhang.material, koenigsrot, purpur}]`, `teile` = `[]`; `TestCLISystemfehler` – Exit 2 → Go-Fehler mit stderr; `TestVorherCache` – zweiter Aufruf für dieselbe Basis ruft `Frame` nicht erneut (Zähler der Attrappe), Datei unter `.cache/paper/<renderKey>/` vorhanden.
6. **Grün – `cli.go` und `cache.go`:** Schnittstelle `Renderer{Validate, Diff, Frame, PixelDiff}`, Umsetzung `CLI{Root}` (wählt `bin/paper.ts`, sonst `fixtures/cli-stub.ts`); Cache schreibt atomar (Temp-Datei + Umbenennen), Dateiname `<animation>-<frame>[@<variante>].png`.
7. **Rot – Review** (`review_test.go`): `TestReviewHEAD1WieFixture` – `Vergleich(repo, "HEAD~1", r)` mit `r` = CLI-Attrappe, deren `Frame` die PNGs aus `render/` liefert (monarch alt/neu, mauer alt/neu); Ergebnis nach `id` sortiert gleich `tools/paper/fixtures/ui/AssetsReview.json` nach `id` sortiert, alle Felder inklusive `basisCommit`, `fehler`, `teile`, `stil`, `pixelAnteil` (0.015326, 0.170654) (AC-01); `TestReviewHEAD` – Basis `HEAD` liefert genau monarch `geaendert` und banner `ungueltig`; `TestReviewNurCRLF` – monarch im Arbeitsstand nur mit CRLF neu geschrieben → kein Eintrag für monarch (AC-02); `TestReviewAusgabeGeaendert` – nur `<gruppe>/<id>/manifest.json` anders → `geaendert` ohne Felder.
8. **Grün – `review.go`:** `Vergleich(root, basis string, r Renderer) (Ergebnis, error)`; Status-Regeln laut Vertrag (neu/geloescht `pixelAnteil` 1, ungueltig 0, `stil` = `stilHash` anders); `felder`/`teile` aus `diff --stil-alt --stil-neu`; `pixelAnteil` = `pixeldiff` von Frame 0 der ersten Animation. Reihenfolge: `ungueltig`, `geaendert`, `neu`, `geloescht`, je Status nach `id`. Typen mit JSON-Tags exakt wie der Vertrag, damit RV1.2 sie als Alias übernimmt.
9. `go test ./internal/review/` grün, dann Commit `feat(dev): Review-Kern mit Basis, Cache und Diff (RV1.1)`.

## Fertig, wenn

- [ ] AC-01: `go test ./internal/review/ -run TestReviewHEAD1WieFixture` (in `tools/k3c-workbench`) grün; Vergleich mit `AssetsReview.json` ohne Ausnahme-Felder.
- [ ] AC-02: `TestBasisAufloesen`, `TestBasisOriginDevelopLokal`, `TestBasisUngueltig`, `TestSpecHashCRLF`, `TestReviewNurCRLF` grün.
- [ ] `TestSpecHashWieBun` grün (Go-Hash = `hash.ts`).
- [ ] `task check:diff` grün (bzw. `check_run task:check:diff` mit `checkout`).

## Prüfen

```bash
cd tools/k3c-workbench && go test ./internal/review/
task check:diff
```

Einmal vor dem Commit; während der Arbeit nur gezielte Tests (`docs/arbeitsweise.md` › Tempo im Lauf). Code-Review vor dem Commit (Pfad- und Shell-Code: `git`/`bun`-Aufrufe).

## Ergebnis

- AC-01 geprüft: `TestReviewHEAD1WieFixture` grün – alle Felder gleich `AssetsReview.json` (nach `id`), inklusive `pixelAnteil` 0.015326/0.170654; Reihenfolge ungueltig, geaendert, neu, geloescht.
- AC-02 geprüft: `TestBasisAufloesen`, `TestBasisOriginDevelopLokal`, `TestBasisUngueltig`, `TestSpecHashCRLF`, `TestReviewNurCRLF` grün; `TestSpecHashWieBun` (Go-Hash = `hash.ts`).
- Prüfung: `go test ./...` in `tools/k3c-workbench` grün (simrun-TempDir-Cleanup einmal flakig, Wiederholung grün), `golangci-lint run ./internal/review/` 0 Befunde. `task check:diff` lief über 10 min (Diff gegen develop enthält ganz `feature/pap`), daher Gate wie genannt.
- Review (Sonnet): Befunde 1–8, 10, 12 behoben mit `haertung_test.go` (Animation `idle@spieler2`, eindeutige Temp-Dateien, Render aus Schnappschuss im Cache, kaputte Spec = `ungueltig`, Git-Fehler nicht verschluckt, Pfad-Härtung, eigene Cache-Kennung der Attrappe, `GIT_TERMINAL_PROMPT=0`, `ls-tree -z`).
- Abweichungen: `stil` gilt je Asset (genutztes Material/Einzelton, sonst Papier/Licht = alle), weil die Fixture `monarch` mit `stil: false` verlangt; zusätzlich `Geaendert()` für `AssetsList`. Beides als D-Zeile in B-701.
