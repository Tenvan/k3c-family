# B-707 · Die Render-CLI rendert einen Frame mit einer übergebenen Stil-Datei (frame --stil)

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** PAP
- **Erstellt:** 2026-11-06
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die echte Render-CLI (RD1) liest die Stil-Datei immer aus der Repo-Wurzel (`tools/paper/render/stil.ts`, `STIL_PFAD`). Die Asset-Review (RV1) rendert „Vorher“ aus einem Schnappschuss der Basis im Cache, dort liegt auch die Stil-Datei der Basis, die CLI ignoriert sie aber. Folge: Eine reine Stiländerung (z. B. Material `stein` heller) zeigt im Review keinen Pixel-Unterschied, Vorher und Nachher sind gleich. Vertragslücke: `docs/standards/paper-spec.md` › 8 kennt kein `--stil` bei `frame`/`render`. B-701 D-RV1-3.

## Ziel

`frame` (und `render`) nehmen optional `--stil <datei>`; ohne Angabe gilt die Stil-Datei der Repo-Wurzel. `internal/review` übergibt bei Basis-Frames die Stil-Datei des Schnappschusses.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** `paper-spec.md` › 8 beschreibt `--stil <datei>` für `frame` und `render` (Änderung nur über dieses Ticket, Projekt PAP).
- **AC-02** CLI-Test: derselbe Frame mit zwei Stil-Dateien (Material `stein` verschieden) ergibt verschiedene PNG-Hashes.
- **AC-03** Go-Test in `internal/review`: Review des Test-Repos gegen `HEAD~1` meldet für `mauer` einen `pixelAnteil` > 0 mit dem echten Renderer, ohne Frame-Attrappe.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
