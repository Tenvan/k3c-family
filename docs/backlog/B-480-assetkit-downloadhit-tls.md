# B-480 · TestDownloadHit in tools/assetkit scheitert lokal am TLS-Zertifikat und blockiert task check:all

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`task check:studio` (und damit `task check:all`) bricht lokal bei `tools/assetkit` ab: `TestDownloadHit` (`import_test.go`) meldet `tls: failed to verify certificate: x509: certificate signed by unknown authority` für den Testserver (`httptest.NewTLSServer`, Client `srv.Client()`). Der Fehler tritt auch ohne Änderungen an Go-Dateien auf (festgestellt in WZ16.1/WZ16.2, 2026-10-18, Rechner von 🧑). Ursache nicht untersucht: möglich sind eine TLS-Abfangung durch Sicherheitssoftware auf dem Rechner (siehe Norton-Notiz im Projektspeicher) oder ein Client in `DownloadHit`/`fetchToTemp`, der das Zertifikat des Testservers nicht kennt.

## Ziel

`task check:all` läuft auf dem Entwickler-Rechner grün, oder der Test ist so geändert, dass er nicht von der lokalen TLS-Umgebung abhängt.

## Beteiligte und Zielgruppen

Entwickler (🧑), Agenten, die `task check:all` als Nachweis brauchen.

## Anforderungen

Ursache klären (Test gegen CI-Ergebnis vergleichen, Client in `DownloadHit` prüfen). Behebung entweder im Test (z. B. HTTP-Testserver statt TLS, wenn `https` nur beim Host-Filter geprüft wird) oder im Code. Die Regel „Download nur von `upload.wikimedia.org`, nur https“ bleibt erhalten.

## Nicht-Ziele

Keine Lockerung der Download-Regeln im Produktiv-Code; keine Änderung am Welt-Betrachter.

## Regeln und Einschränkungen

Domäne DEV (`tools/assetkit/`). Test ohne Netz und ohne systemweite Zertifikate.

## Beispiele

`task check:studio` auf dem Rechner von 🧑 → grün; in der CI ist `go test ./tools/assetkit` ebenfalls grün.

## Ausnahme- und Fehlerfälle

Liegt die Ursache nur an der Sicherheitssoftware des Rechners, bleibt der Code unverändert und das Ticket wird `verworfen` mit Hinweis im Projektspeicher.

## Akzeptanzkriterien

- **AC-01** Ursache festgestellt und im Ticket vermerkt.
- **AC-02** `go test ./...` in `tools/assetkit` ist auf dem Rechner von 🧑 grün, oder das Ticket ist mit Grund verworfen.

## Offene Fragen

keine

## Notizen

Entdeckt in WZ16.2 (Sprint WZ16). Die Frontend-Tests des Studios und alle übrigen Go-Tests von `task check:all` sind grün.
