# B-1266 · Review-Befunde WE6: gocyclo in services/config.go, MCP-Beschreibung ohne endBoss/skins/events, Go-Tests rot

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** offen
- **Sprint:** WN4
- **Projekt:** WZN
- **Erstellt:** 2026-11-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

Befunde aus WE6.5: (1) `task check:dev` rot nur durch gocyclo in `tools/k3c-workbench/internal/services/config.go:107` (`check`, Komplexität 18 > 15), nicht WE6. (2) Die Beschreibung des Welten-MCP nennt `endBoss`, `skins`, `events` nicht, kein MCP-Roundtrip-Test (B-987/AC-03). (3) `task check:go` rot: `engine/internal/trace` (nacht-300 Golden), `engine/room` (TestKampfInselwechselLetzteInselOhneTausch, TestVictoryEinmal), `engine/sim` (TestVorpostenBauNurOhneGefahr, Nil-Zeiger); Go-Engine ist eingefroren, Ursache ungeklärt. (4) `task check` rot nur durch das Fremd-Verzeichnis `.work/ru33` (237 Testdateien), nicht WE6.

## Ziel

Die Befunde sind behoben oder einzeln entschieden.

## Beteiligte und Zielgruppen

Entwickler.

## Anforderungen

- config.go unter Komplexität 15.
- MCP-Beschreibung und ein Roundtrip-Test für die neuen Felder.
- Ursache der Go-Testfehler klären (veraltete Referenz oder Regression).

## Nicht-Ziele

Neue Funktionen.

## Regeln und Einschränkungen

Go-Engine eingefroren (Entscheidung 013, Entwurf).

## Beispiele

-

## Ausnahme- und Fehlerfälle

-

## Akzeptanzkriterien

- **AC-01** `check_run task:check:dev` grün.
- **AC-02** MCP-Roundtrip-Test für endBoss, skins, events.
- **AC-03** `task check:go` grün oder Ausnahmen begründet.

## Offene Fragen

Keine.

## Notizen

Angelegt von WE6.5.
