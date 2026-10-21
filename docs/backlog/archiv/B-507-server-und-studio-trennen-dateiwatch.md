# B-507 · Spielserver und Asset-Studio sind getrennt, der Server beobachtet die Dateien des Studios

- **Domäne:** DEV
- **Typ:** Schuld
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** TS1
- **Projekt:** WZG
- **Erstellt:** 2026-10-20
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-20, Ralf im Chat („anlegen und umsetzen“, Revision 1)

## Ausgangslage

Das Studio-Backend `engine/studio` liegt im Engine-Modul. Der Spielserver bindet es ein (`cmd/k3c-server/studio.go`) und liefert das Studio-Frontend unter `/studio/` aus (Build nach `dist/studio/`, Kachel `studio.html`, Vite-Ausnahme für `/api/studio`, nur Dev-Mode und Loopback, `K3C_STUDIO_LAN`). Dasselbe Backend nutzt das Wails-Fenster `tools/k3c-studio` (eigenes Go-Modul, Port 8091). Der Server lädt `data/` per `go:embed` zur Build-Zeit. `deploy-pages.yml` führt `task pages` aus: voller Build, Spiel-Client unter `app/`, nur wegen der Bilder der Präsentationsseite `site/`. Der Chat-Code von WZ19.1 (`engine/studio/chat_*.go`) liegt ebenfalls dort.
Entscheidung von 🧑 (2026-10-20, Chat): Spieler ändern über den Server keine Assets mehr; der Server soll nur Server sein.

## Ziel

Server, Assets und AI-Code sind vollständig getrennt. Der Server macht nur Server-Dinge, das Asset-Studio nur Asset-Dinge, der AI-Code für Assets liegt nur im Studio. Was im Studio gebaut und getestet wird, nutzt der laufende Spielserver ohne manuellen Neustart.

## Beteiligte und Zielgruppen

🧑 (Ralf) entwirft im Studio und betreibt den kleinen Heimserver (Xbox, Heimnetz), entscheidet und nimmt am PC ab. Spielende im Heimnetz sehen einen Neustart des Servers. Agenten setzen die Sessions um, ein getrennter Reviewer prüft.

## Anforderungen

**Trennung**
- Das Studio-Backend (heute `engine/studio`, samt `chat_*`) zieht nach `tools/k3c-studio/` (eigenes Go-Modul). `engine/` und `cmd/k3c-server` importieren nichts aus `tools/`; ein Test erzwingt das.
- Das Studio bindet die Engine als Bibliothek ein (`engine/sim`, `engine/level` …). Vorschau und „Im Spiel öffnen“ laufen im Studio ohne Spielserver, mit eigenen Testspielständen. Die Richtung ist einseitig: Das Studio darf `engine/…` importieren, nie umgekehrt.
- Die Studio-Seite des Servers entfällt komplett: `/studio/`, `/api/studio`, Kachel und `studio.html`, die Vite-Ausnahme, `dist/studio/`, `K3C_STUDIO_LAN`, der Dev-Mode-Zwang für das Studio; `task build` und `task serve` bauen kein Studio. Das Studio-Frontend baut in einen eigenen Ordner unter `tools/k3c-studio/`.
- Schnittstelle zwischen beiden sind die Dateien: Das Studio schreibt `data/**`, Grafiken und Sound, der Server liest sie und schreibt dort nie. Das Welt-Dateiformat ist als Vertrag beschrieben und getestet.

**Datei-Watch im Server (immer aktiv, auch produktiv)**
- Der Server beobachtet alles, was das Studio ändern kann: `data/**` (Welten, Balancing, Gebäude …), Grafiken und Sound.
- Er lädt neu, wo das einfach geht; sonst startet er verzögert neu. Mehrere Änderungen werden entprellt. Keine eigenen Refresh-Wege je Dateityp.
- Neustarts sind auch in laufenden Runden erlaubt (kleiner Heimserver); Spielstände werden wiederhergestellt (🩹). Vorher erscheint ein sichtbarer Hinweis „Server startet neu“ im Spiel.
- Log-Meldungen mit Emoji (🔁 Neustart, ⏳ Frist).

**GitHub Pages**
- `deploy-pages` baut nichts außer der Seite aus `site/`: kein Node, kein Frontend-Build. Die Bilder der Seite liegen als Kopien in `site/`; `tests/pagesSite.test.ts` prüft, dass jede Referenz auf eine Datei in `site/` zeigt. `task pages` entfällt oder kopiert nur.

**Dokumentation**
- Eine Entscheidung in `docs/decisions/` löst die Studio-Seite des Servers ab; B-459, B-488 und WZ19 verweisen darauf. `CLAUDE.md` (Struktur), `codemap.md` und Taskfile-Beschreibungen folgen.
- WZ19.2 (Chat im Studio) wird angepasst: Chat global rechts in allen Studio-Seiten (Welt zuerst), Anbindung der Provider nur über CLI und ACP, automatisch erkannt und zum Verbinden vorgelegt.

## Nicht-Ziele

Neue Studio-Funktionen, eigene Neuladewege je Dateityp, Änderungen an Spielregeln, Protokoll und Spielstand-Format, ein Hot-Reload ohne Neustart als Pflicht, weitere Seiten der Workbench k3c-dev.

## Regeln und Einschränkungen

`CLAUDE.md`: deterministisch, nur `engine/rng`, Emoji-Logs, Datei ≤ 400, Funktion ≤ 60 Zeilen, Seiten-Regeln der Landingpage (Kachel entfernen statt umbiegen). Entscheidung 001 (Go-Server) bleibt. Der Server bleibt ohne Studio-Abhängigkeit baubar. Sicherheit des Chats aus B-488 bleibt: Loopback, keine Werkzeuge, kein Schlüssel.

## Beispiele

- 🧑 ändert im Studio eine Welt und speichert → der laufende Spielserver nimmt sie nach kurzer Zeit an (Neuladen oder Neustart mit Hinweis).
- Ein Spieler öffnet die Landingpage → keine Kachel „Welten bauen“, `/studio/` antwortet 404.
- Push auf `main` → Pages enthält nur `site/`.

## Ausnahme- und Fehlerfälle

- Studio schreibt eine halbe Datei → Entprellen, Prüfung beim Laden; ungültige Datei: alter Stand bleibt, Fehler im Log (❌), Server läuft weiter.
- Neustart schlägt fehl → Server bleibt oder startet mit dem letzten gültigen Stand; Meldung im Log.
- Studio ohne Server gestartet → Vorschau läuft trotzdem (eigene Engine).

## Akzeptanzkriterien

- **AC-01** `engine/studio` ist nach `tools/k3c-studio/` umgezogen; ein Test (go list oder Import-Prüfung) belegt, dass `engine/` und `cmd/k3c-server` nichts aus `tools/` importieren; `task check:go` grün.
- **AC-02** Das Studio startet Vorschau und „Im Spiel öffnen“ mit eingebundener Engine und eigenen Testspielständen, ohne Spielserver (Go-Test).
- **AC-03** Der Server hat keine Studio-Seite mehr: `/studio/` und `/api/studio` antworten 404 (Test), Kachel und `studio.html`, Vite-Regel, `dist/studio` und `K3C_STUDIO_LAN` sind entfernt; `task build` baut kein Studio.
- **AC-04** Der Server beobachtet `data/**`, Grafiken und Sound immer; eine Änderung führt entprellt zu Neuladen oder verzögertem Neustart (Test mit temporärem Ordner und eingespeister Uhr); ein Neustart in laufender Runde zeigt vorher den Hinweis „Server startet neu“; ungültige Dateien lassen den Server laufen.
- **AC-05** `deploy-pages` baut nur `site/` ohne Node; die Bilder liegen in `site/`; `tests/pagesSite.test.ts` grün.
- **AC-06** Eine neue Entscheidung in `docs/decisions/`, `CLAUDE.md`, `codemap.md` und die Spec B-488/WZ19 sind angepasst; WZ19.2 beschreibt Chat global rechts und Provider über CLI und ACP; `task test -- planning` grün.
- **AC-07** Abnahme durch 🧑 am PC: im Studio geänderte Welt erscheint im laufenden Spielserver, Studio läuft ohne Server.

## Offene Fragen

keine (entschieden von 🧑 am 2026-10-20 im Chat: Studio bindet die Engine selbst ein; Watch immer aktiv, auch produktiv; Neustarts auch in Runden; Pages nur `site/` mit Bildkopien).

## Notizen

Links, Messwerte, verworfene Ansätze. Darf leer bleiben (`–`).
