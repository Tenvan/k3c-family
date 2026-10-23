# B-474 · TestDownloadHit scheitert lokal am TLS-Zertifikat (Norton), check:all rot

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** WZ26
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`task check:all` scheitert lokal in `check:studio`: `TestDownloadHit` (`tools/assetkit/import_test.go:131`) meldet „tls: failed to verify certificate: x509: certificate signed by unknown authority“ beim Abruf vom `httptest.NewTLSServer` mit `srv.Client()`. In der CI ist der Test grün. Lokal liegt Norton Antivirus mit Traffic-Filter vor (`SSLKEYLOGFILE=\\.\nllMonFltProxy…`, `NODE_EXTRA_CA_CERTS`), vermutlich fängt er auch die Verbindung auf 127.0.0.1 ab. Ursache nicht bestätigt (Test gegen den Norton-Filter, z. B. mit Ausnahme, nicht geprüft).

## Ziel

`task check:all` läuft lokal auf dem Entwicklerrechner grün: Ursache klären (Norton oder Test) und den Test so absichern, dass er lokal und in der CI besteht.

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

- **AC-01** Prüfbar formuliert: Test, Befehl, Datei oder Beobachtung am TV mit erwartetem Ergebnis.

## Offene Fragen

Entscheidung, betroffener Umfang, wer entscheidet (🧑). Sonst `keine`.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
