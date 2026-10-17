# B-459 · Die Asset-Seiten für Grafik und Sound laufen in einer eigenen EXE getrennt von der Workbench

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** eingeplant
- **Sprint:** WZ6
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Assets-Seite steckt in der Workbench k3c-dev (`tools/k3c-dev/frontend/src/assets/`, Go `tools/k3c-dev/internal/assets/`). Ralf plant dort laufend Änderungen; jede Änderung baut die Workbench neu oder startet `wails dev` neu, damit fallen MCP-Verbindung und Planung kurz aus (zuletzt 2026-10-17 beim Session-Fix). Grafik und Sound teilen sich eine Seite (Umschalter `Grafik`/`Sound`). Daneben gibt es die Dev-Seiten `assetvorschau.html`, `grafiken.html`, `soundtest.html`, `figuren.html`, `aufstellung.html` und `leveltest.html`.

## Ziel

Ein eigenes Asset-Werkzeug (eigene EXE mit festem Pfad unter `bin/`, eigener Task) enthält je eine Seite für Grafik und für Sound; die Workbench bleibt beim Umbau des Asset-Werkzeugs ungestört. Es ist die Heimat für Welt-Betrachter (B-460), Chunk-Planung (B-461), Gebäude-Planer (B-462), Monarch- (B-463) und Gegner-Seite (B-464).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Neue Asset-Funktionen über den heutigen Stand hinaus (eigene Tickets B-460–B-464); Änderungen am Slot-Register-Format (`data/assets.json`); MCP-Tools `assets_*` neu schneiden (bleiben vorerst in k3c-dev, siehe Offene Fragen).

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Das Asset-Werkzeug startet als eigene EXE mit festem Pfad (`bin/`) über einen Task in `Taskfile.yml`; Proben nur auf Loopback (keine Firewall-Abfrage).
- **AC-02** Es hat je eine Seite für Grafik und für Sound mit dem heutigen Funktionsumfang: Slots, Filter, Präferenzen, Mini-Szene, Katalog, Recherche (`ResearchPanel`), Ausschnitt-Editor (`CutEditor`), Import.
- **AC-03** Die Workbench hat keine Assets-Seite mehr, sondern höchstens einen Verweis auf das Asset-Werkzeug; ein Neubau des Asset-Werkzeugs startet die Workbench nicht neu.
- **AC-04** Tests der Asset-Logik laufen in `task check` bzw. einem eigenen Prüfziel mit.
- **AC-05** `tools/k3c-assets/` ist in der Domänentabelle (`docs/arbeitsweise.md`) eingetragen; neue Begriffe stehen im Glossar.

## Offene Fragen

1. Technik: zweite Wails-App unter `tools/` (wie k3c-dev) oder Web-Seiten im Vite-Build mit kleinem Go-Dienst? Annahme: Wails-App `tools/k3c-assets`, teilt Go-Pakete mit k3c-dev. Folge für B-460: Ein Wails-Frontend muss `src/scenes/` und `public/` für die Live-Vorschau mitbauen.
2. Bleiben die MCP-Tools `assets_*` in k3c-dev (lesen und schreiben dieselbe `data/assets.json`) oder zieht das Asset-Werkzeug einen eigenen MCP-Server? Annahme: bleiben in k3c-dev; zwei Schreiber auf `data/assets.json` brauchen eine Regel (Neu laden bei Änderung).
3. Name des Werkzeugs (Arbeitstitel „Asset-Werkzeug“, `k3c-assets`); bisherige Namen „Ressourcen-Manager“ (B-298), „Asset-Manager“ (B-435), „Assets-Seite“ zusammenführen.
4. Was passiert mit `assetvorschau.html`, `grafiken.html`, `soundtest.html`? Entfernen berührt PLAT (`*.html`) und `src/tools/devTiles.ts`; GEB9 nutzt `src/tools/assetvorschau*.ts`. Annahme: bleiben bis nach GEB9.
5. Domäne von `tools/k3c-assets/`: Annahme DEV, Eintrag in die Domänentabelle als INF-Session.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
