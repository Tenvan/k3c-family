# B-688 · Downloads folgen Weiterleitungen nur zu erlaubten Hosts

- **Domäne:** DEV
- **Typ:** Problem
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ39
- **Projekt:** WZG
- **Erstellt:** 2026-10-31
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** autonom, Claude, 2026-11-02

## Ausgangslage

Fund aus dem Team-Lauf WZG (2026-10-31, worker-3, Ungeprüft/vermutet): `http.DefaultClient` in `tools/k3c-studio/backend/app_assets.go:109` und `tools/k3c-workbench/internal/mcpsrv/tools_assets_research.go:151` folgt Weiterleitungen auf beliebige Hosts. Die Host-Prüfung gilt nur für die Start-URL (SSRF-relevant).

## Ziel

Treffer-Downloads folgen Weiterleitungen nur zu Hosts, die auch für die Start-URL erlaubt wären.

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Jeder Weiterleitungsschritt wird gegen dieselbe Host-Erlaubnis geprüft wie die Start-URL (nur https, keine lokalen oder privaten Adressen).
- Eine Weiterleitung auf einen nicht erlaubten Host bricht den Download mit klarer Meldung ab.
- Die Weiterleitung innerhalb erlaubter Hosts (siehe `TestDownloadHitFolgtWeiterleitung`) bleibt möglich.

## Nicht-Ziele

Was ausdrücklich nicht dazugehört, mit Ticket-Nummer, falls es später kommt.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Go-Test (httptest): Weiterleitung auf einen nicht erlaubten Host oder eine lokale Adresse bricht den Download ab, keine Datei entsteht.
- **AC-02** Go-Test: Weiterleitung zwischen erlaubten Hosts funktioniert weiter.

## Offene Fragen

keine

## Notizen

Zusammenhang: B-678 AC-03. Welche Hosts erlaubt sind, ergibt sich aus der bestehenden Prüfung der Start-URL; keine neue Liste erfinden.
