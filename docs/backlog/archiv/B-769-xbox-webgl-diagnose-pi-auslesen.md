# B-769 · Xbox-WebGL-Diagnose in der Textur-Testseite und Lese-Endpunkte für den Pi

- **Domäne:** CLI
- **Typ:** Problem
- **Prio:** hoch
- **Umgebung:** live
- **Status:** verworfen
- **Sprint:** RX1
- **Projekt:** LST
- **Erstellt:** 2026-11-09
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Textur-Testseite (B-764, v0.23.1) meldete auf der Xbox „Kein WebGL verfügbar“, obwohl die Pi-Logs zeigen: Spiel und Stress-Seite starten dort mit WebGL und verlieren den Kontext nach 2,4 bis 4,4 s bei 214 bis 347 MiB. Ursache der Meldung unklar (Vermutung: Im Panel hält das Spiel im Hintergrund ~540 MiB, der Kontext lässt sich nicht anlegen). Außerdem kamen Pi-Logs nur über SSH und eine fehlerhafte Nachrichtenkette an.

## Ziel

Ein Xbox-Lauf der Testseite liefert die Ursache und die echte Texturgrenze, und der Entwickler liest den Pi von seinem Rechner aus.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Testseite probiert 7 Wege, einen WebGL-Kontext zu bekommen (wie Phaser, WebGL 2, WebGL 1, ohne Antialias, low-power, minimal, experimental-webgl), hält je Versuch Erfolg, Grund (`webglcontextcreationerror`), Renderer, maxTex und Timer-Query-Erweiterung fest und zeigt sie an.
- Der Bericht geht schon beim Öffnen (`kind: gl-diagnose`) und bei jeder Messung, auch ohne Kontext (`noGl`), an `/api/report`.
- Im Panel gibt das Spiel vor der Messung seinen Speicher frei (Spiel zerstören, Guard aus); beim Schließen lädt das Dokument neu (`releasesGame`).
- Lese-Endpunkte nur im Dev-Mode, ohne Token, nur GET: `/api/dev/status`, `/api/dev/log?pattern=&limit=&mib=`, `/api/dev/reports[?name=]`.

## Nicht-Ziele

Kein Schreiben über die Endpunkte, kein Ersatz für `/api/status` mit Token, kein Texturbudget (B-764).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Test: Die Versuche und ihre Auswertung (`glProbe.test.ts`), die Freigabe-Seiten (`panelHost.test.ts`) und `ContextGuard.stop` (`contextGuard.test.ts`).
- **AC-02** Test: Die Lese-Endpunkte antworten ohne Dev-Mode 403, filtern das Log, listen und lesen Berichte und lehnen Pfade ab (`engine/net/dev_read_test.go`).
- **AC-03** Abnahme (🧑): Textur-Grenze auf der Xbox öffnen (Seiten > Performance), der Bericht liegt unter `/api/dev/reports` und nennt Grund und Grenze.

## Offene Fragen

keine

## Notizen

Verworfen 2026-11-12 (Entscheidung Ralf): abgelöst durch B-823 (nur noch natives Spiel am PC, Entscheidung 009).
