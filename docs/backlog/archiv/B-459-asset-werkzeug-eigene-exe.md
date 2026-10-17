# B-459 · Die Asset-Seiten für Grafik und Sound laufen in einer eigenen EXE getrennt von der Workbench

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** ?
- **Status:** erledigt
- **Sprint:** WZ6
- **Projekt:** WZG
- **Erstellt:** 2026-10-17
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** 2026-10-17, Chat (Ralf)

## Ausgangslage

Die Assets-Seite steckt in der Workbench k3c-dev (`tools/k3c-dev/frontend/src/assets/`, Go `tools/k3c-dev/internal/assets/`). Ralf plant dort laufend Änderungen; jede Änderung baut die Workbench neu oder startet `wails dev` neu, damit fallen MCP-Verbindung und Planung kurz aus (zuletzt 2026-10-17 beim Session-Fix). Grafik und Sound teilen sich eine Seite (Umschalter `Grafik`/`Sound`). Daneben gibt es die Dev-Seiten `assetvorschau.html`, `grafiken.html`, `soundtest.html`, `figuren.html`, `aufstellung.html` und `leveltest.html`.

## Ziel

Das Asset-Studio (`tools/k3c-studio`, Wails-App, EXE mit festem Pfad unter `bin/`, eigener Task) enthält je eine Seite für Grafik und für Sound und übernimmt die Dev-Seiten `assetvorschau.html`, `grafiken.html` und `soundtest.html`; die Workbench bleibt beim Umbau des Asset-Studios ungestört. Es ist die Heimat für Welt-Betrachter (B-460), Chunk-Planung (B-461), Gebäude-Planer (B-462), Monarch- (B-463) und Gegner-Seite (B-464).

## Beteiligte und Zielgruppen

🧑 pflegt Grafik und Sound im Asset-Studio und ändert es laufend; Agenten nutzen weiter die MCP-Tools `assets_*` der Workbench; beide arbeiten auf `data/assets.json`.

## Anforderungen

- Eigene Wails-App `tools/k3c-studio`, EXE `bin/k3c-studio.exe`, unabhängig von k3c-dev baubar und startbar.
- Gemeinsame Asset-Logik (Go) in einem Modul, das k3c-dev und Asset-Studio nutzen (kein `internal/` über Modulgrenzen).
- Spiel-Code aus `src/` (Audio, Modelle, Figuren-Referenz) liest das Asset-Studio nur, ändert ihn nicht.
- Keine neue Abhängigkeit ohne Ticket; Datei ≤ 400, Funktion ≤ 60 Zeilen.

## Nicht-Ziele

Neue Asset-Funktionen über den heutigen Stand hinaus (eigene Tickets B-460–B-464); Änderungen am Slot-Register-Format (`data/assets.json`); eigener MCP-Server im Asset-Studio; Entfernen von `soundtest.html` und `grafiken.html` (bleiben für die Xbox); `figuren.html`, `aufstellung.html` (B-463/B-464) und `leveltest.html` (B-460) ziehen erst mit ihren Tickets um.

## Regeln und Einschränkungen

Domäne DEV; `Taskfile.yml`, CI (`.github/workflows/ci.yml`), Lint-Konfiguration (`.oxlintrc.json`), Wurzel-`vite.config.ts` (nur Test-Ausschluss und Seitenfilter), `.gitignore`/`.dockerignore`, `requirements.md` und Struktur-Abschnitt von `CLAUDE.md` werden für das neue Werkzeug als Grenzfall in DEV mitgeändert (wie GEB1.2 für `tools/gebaeude`). Firewall: EXE mit festem Pfad, kein `go run`, Proben nur auf `127.0.0.1`.

## Beispiele

- Ralf baut das Asset-Studio neu, während die Workbench läuft → MCP bleibt verbunden, Planung unverändert.
- Ein Agent setzt per `assets_set` eine Präferenz → das offene Asset-Studio zeigt sie nach dem Neuladen, Auswahl und Filter bleiben.
- Asset-Studio › Sound › Musik → Wechsel `music:tag` → `music:nacht` hörbar (bisher `assetvorschau.html?szene=musik`).

## Ausnahme- und Fehlerfälle

- `data/assets.json` wurde seit dem Laden von einem anderen Prozess geändert (MCP) → das Asset-Studio speichert nicht, lädt neu und meldet „Register geändert, bitte erneut speichern“; offene Eingaben bleiben erhalten.
- `bin/k3c-studio.exe` fehlt → der Verweis in der Workbench meldet „Asset-Studio nicht gebaut (`task k3c-studio:build`)“.
- Datei aus `public/` fehlt → Lücke markiert wie heute.

## Akzeptanzkriterien

- **AC-01** Das Asset-Studio startet als eigene EXE mit festem Pfad (`bin/k3c-studio.exe`) über einen Task in `Taskfile.yml`; Proben nur auf Loopback (keine Firewall-Abfrage).
- **AC-02** Es hat je eine Seite für Grafik und für Sound mit dem heutigen Funktionsumfang der Workbench-Seite: Slots, Filter, Präferenzen, Mini-Szene, Katalog, Recherche (`ResearchPanel`), Ausschnitt-Editor (`CutEditor`), Import.
- **AC-03** Die Workbench hat keine Assets-Seite mehr, sondern einen Verweis, der das Asset-Studio startet; ein Neubau des Asset-Studios startet die Workbench nicht neu.
- **AC-04** Tests der Asset-Logik (Go und Frontend) laufen in `task check:all` und in der CI mit.
- **AC-05** `tools/k3c-studio/` ist in der Domänentabelle (`docs/arbeitsweise.md`) als DEV eingetragen; Asset-Studio und die neuen Begriffe stehen im Glossar.
- **AC-06** Der gesamte Inhalt von `assetvorschau.html` ist im Asset-Studio: Grafik-Szenen (Figuren, Gebäude mit Stufenreihen und Vorlagen-Texten, Umgebung mit Hintergründen, Objekte, HUD) auf der Grafik-Seite, Sound-Szenen (Ereignis-SFX, Musik mit Wechsel, Umgebungsgeräusche) auf der Sound-Seite. `assetvorschau.html`, `src/tools/assetvorschau*.ts` (die Logik zieht ins Asset-Studio um), Kachel, Texte und Verweise sind entfernt. Die Inhalte von `grafiken.html` und `soundtest.html` gibt es zusätzlich im Asset-Studio; beide Seiten bleiben für Prüfungen am TV (Xbox) bestehen.
- **AC-07** Die MCP-Tools `assets_*` bleiben in k3c-dev; ändert ein anderer Prozess `data/assets.json`, lädt das Asset-Studio neu, ohne Auswahl, Filter und offene Eingaben zu verlieren; ein Speichern auf veraltetem Stand wird abgelehnt (siehe Ausnahme- und Fehlerfälle).

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat):
1. **Technik:** Wails-App `tools/k3c-studio` wie k3c-dev, teilt Go-Pakete mit k3c-dev; das Frontend baut für die Live-Vorschau (B-460) `src/scenes/` und `public/` mit.
2. **Name:** Asset-Studio, EXE und Ordner `k3c-studio`.
3. **MCP-Schnitt:** `assets_*` bleiben in k3c-dev; das Asset-Studio lädt bei Dateiänderung von `data/assets.json` neu (AC-07).
4. **Dev-Seiten:** `assetvorschau.html` zieht ins Asset-Studio um und wird entfernt. `soundtest.html` und `grafiken.html` bleiben für Prüfungen am TV (SO4, SO2, HW1.9, GR8); ihre Inhalte kommen zusätzlich ins Asset-Studio (AC-06).
5. **Domäne:** DEV, Eintrag in die Domänentabelle per INF-Session.

Offen: keine.

## Notizen

B-421 (Abnahme M10 Ressourcen-Manager) prüft `assetvorschau.html` und die Workbench-Assets-Seite, die WZ6 entfernt; seine Abnahmepunkte (u. a. `sfx:coin`, Musikwechsel) gehen in die Abnahme WZ6.8 über, dort entscheidet 🧑 über B-421. Revision 2 (2026-10-17): AC-06 um Sound-Szenen, Hintergründe und Vorlagen ergänzt, AC-07 um die Schreibregel, Ausnahmefälle und Grenzfälle der Domäne nachgetragen (Review-Befunde).
