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

Das Asset-Studio (`tools/k3c-studio`, Wails-App, EXE mit festem Pfad unter `bin/`, eigener Task) enthält je eine Seite für Grafik und für Sound und übernimmt die Dev-Seiten `assetvorschau.html`, `grafiken.html` und `soundtest.html`; die Workbench bleibt beim Umbau des Asset-Studios ungestört. Es ist die Heimat für Welt-Betrachter (B-460), Chunk-Planung (B-461), Gebäude-Planer (B-462), Monarch- (B-463) und Gegner-Seite (B-464).

## Beteiligte und Zielgruppen

Wer spielt, entwickelt, betreibt oder entscheidet (🧑)? Keine Verantwortlichen erfinden.

## Anforderungen

- Was das Ergebnis können muss, auch Qualität (deterministisch, 2+ Spieler, Leistung).

## Nicht-Ziele

Neue Asset-Funktionen über den heutigen Stand hinaus (eigene Tickets B-460–B-464); Änderungen am Slot-Register-Format (`data/assets.json`); eigener MCP-Server im Asset-Studio; Entfernen von `soundtest.html` und `grafiken.html` (bleiben für die Xbox); `figuren.html`, `aufstellung.html` (B-463/B-464) und `leveltest.html` (B-460) ziehen erst mit ihren Tickets um.

## Regeln und Einschränkungen

Regeln aus `CLAUDE.md`, Entscheidungen (`docs/decisions/`), Domäne, Komplexitäts-Budget, Verträge (Protokoll, Spielstand).

## Beispiele

Typische Situation → erwartetes Ergebnis. Passt nichts: `nicht relevant` mit Grund.

## Ausnahme- und Fehlerfälle

Ungültige oder seltene Situation → gewolltes Verhalten. Passt nichts: `nicht relevant` mit Grund.

## Akzeptanzkriterien

- **AC-01** Das Asset-Studio startet als eigene EXE mit festem Pfad (`bin/`) über einen Task in `Taskfile.yml`; Proben nur auf Loopback (keine Firewall-Abfrage).
- **AC-02** Es hat je eine Seite für Grafik und für Sound mit dem heutigen Funktionsumfang der Workbench-Seite: Slots, Filter, Präferenzen, Mini-Szene, Katalog, Recherche (`ResearchPanel`), Ausschnitt-Editor (`CutEditor`), Import.
- **AC-03** Die Workbench hat keine Assets-Seite mehr, sondern höchstens einen Verweis auf das Asset-Studio; ein Neubau des Asset-Studios startet die Workbench nicht neu.
- **AC-04** Tests der Asset-Logik laufen in `task check` bzw. einem eigenen Prüfziel mit.
- **AC-05** `tools/k3c-studio/` ist in der Domänentabelle (`docs/arbeitsweise.md`) als DEV eingetragen; Asset-Studio und die neuen Begriffe stehen im Glossar.
- **AC-06** Der Inhalt von `assetvorschau.html` (Spielmaßstab je Kategorie, Gebäude-Stufenreihen) ist auf der Grafik-Seite des Asset-Studios; `assetvorschau.html`, ihr Skript (`src/tools/assetvorschau*.ts`), ihre Kachel und Verweise (u. a. `docs/standards/gebaeude-grafik.md`) sind entfernt. Die Inhalte von `grafiken.html` und `soundtest.html` gibt es zusätzlich im Asset-Studio; beide Seiten bleiben für Prüfungen am TV (Xbox) bestehen.
- **AC-07** Die MCP-Tools `assets_*` bleiben in k3c-dev; ändert ein Agent `data/assets.json`, lädt das Asset-Studio neu, ohne Eingaben zu verlieren.

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat):
1. **Technik:** Wails-App `tools/k3c-studio` wie k3c-dev, teilt Go-Pakete mit k3c-dev; das Frontend baut für die Live-Vorschau (B-460) `src/scenes/` und `public/` mit.
2. **Name:** Asset-Studio, EXE und Ordner `k3c-studio`.
3. **MCP-Schnitt:** `assets_*` bleiben in k3c-dev; das Asset-Studio lädt bei Dateiänderung von `data/assets.json` neu (AC-07).
4. **Dev-Seiten:** `assetvorschau.html` zieht ins Asset-Studio um und wird entfernt. `soundtest.html` und `grafiken.html` bleiben für Prüfungen am TV (SO4, SO2, HW1.9, GR8); ihre Inhalte kommen zusätzlich ins Asset-Studio (AC-06).
5. **Domäne:** DEV, Eintrag in die Domänentabelle per INF-Session.

Offen: keine.

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
