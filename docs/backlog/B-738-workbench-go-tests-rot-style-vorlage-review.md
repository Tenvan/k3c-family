# B-738 · Workbench-Go-Tests rot: Style-Fixture, Rig-Vorlage vierbeiner ohne Gangarten, Review-Tests mit festen Commit-Hashes

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-07
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Beim `task check:diff` auf `sprint/mg9` (Basis `origin/develop` 5305c03d) lief die Workbench-Go-Suite voll (die Root-`Taskfile.yml` war geändert, B-737) und war rot, ohne dass die Änderungen daran beteiligt sind: `TestAssetsStyle` in `assetsui` und `mcpsrv` (Fixture `AssetsStyle` passt nicht zur Stil-Datei nach RD2.2), `TestRigVorlagenEchteCLI` in `paperassets` (Vorlage `vierbeiner` hat keine Animationen schritt/trab/galopp seit Regel `gangart-fehlt`, RD2.4), `review`-Tests (`TestVorherCache`, `TestBasisAufloesen`, `TestBasisOriginDevelopLokal`, `TestReviewHEAD1WieFixture`: Commit-Hash des Test-Repos ≠ Fixture `32eb7a7`, vermutlich Git-Konfiguration dieses Rechners).

## Ziel

Die Workbench-Go-Suite ist auf `develop` grün, `check:diff` mit geänderter Root-Taskfile läuft durch.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Fixture `AssetsStyle` aus der aktuellen `public/grafik/stil.paper.json` neu erzeugen.
- Rig-Vorlage `vierbeiner` (`paperassets`) mit den drei Gangarten oder `proportionen.gangarten` ausstatten.
- Review-Tests: Test-Repo mit festem Autor, Datum und ohne Signierung bauen (`GIT_CONFIG_GLOBAL`, `commit.gpgsign=false`, `core.autocrlf=false`), damit der Hash stabil ist, oder die Hash-Fixtures aus dem Lauf ableiten.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `task k3c-workbench:gotest` ist auf einem frischen Checkout von `develop` grün (Windows, autocrlf an).
- **AC-02** Die genannten Tests sind unverändert aussagekräftig (kein Auskommentieren).

## Offene Fragen

- Sind die Review-Tests nur auf diesem Rechner rot (Git-Konfiguration) oder auch in CI? (ungeprüft)

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
