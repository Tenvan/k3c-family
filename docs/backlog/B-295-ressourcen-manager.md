# B-295 · Ein ResourcenManager in k3c-dev ordnet jedem Grafik- und Sound-Slot Assets mit Präferenz zu

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** mittel
- **Status:** offen
- **Sprint:** –
- **Erstellt:** 2026-10-05
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Die Zuordnung von Grafiken und Sounds läuft über Chat-Fragen, statische Fundseiten (`docs/funde/`) und Markdown-Tabellen (`docs/assets/zuordnung*.md`, `public/audio/kandidaten.json`). Es gibt keine Übersicht aller Stellen, an denen ein Asset eingebaut werden muss, keine Präferenz-Reihenfolge, keine Ausschnitte und keine wiederholbare Web-Recherche. B-252 (Client-Seite, nur Grafik, nur Text-Export) deckt das nicht ab, weil der Client nicht ins Repo schreiben und nicht im Netz suchen darf. Entscheidung 🧑 im Chat am 2026-10-05: Manager in k3c-dev, Mini-Szenen als Dev-Seite im Client (B-296).

## Ziel

🧑 sieht in k3c-dev alle Grafik-Slots und alle Sound-Slots (getrennte Ansichten), wählt je Slot ein oder mehrere Assets aus dem Katalog mit Suche, ordnet sie nach Präferenz, legt bei Grafiken Ausschnitte fest und lässt je Slot nach Alternativen im Netz suchen. Das Ergebnis liegt als Daten im Repo und ist in einer Mini-Szene (B-296) sichtbar.

## Beteiligte und Zielgruppen

🧑 wählt und ordnet. Der Agent bereitet Recherche und Slot-Register über MCP vor. Spieler sehen nur das Ergebnis im Spiel.

## Anforderungen

- **Slot-Register** in `data/` (z. B. `data/assets.json`, einzige Quelle): je Slot ID, Art (Grafik/Sound), Kategorie (für die Mini-Szene), Beschreibung, Zielmaß (Grafik) bzw. Bus (Sound), Liste der Zuordnungen in Präferenz-Reihenfolge. `docs/assets/zuordnung*.md` und `public/audio/kandidaten.json` werden Ansicht oder gehen darin auf.
- **Katalog**: alle Dateien unter `public/grafik/`, `public/sprites/`, `public/audio/` mit Quelle, Urheber, Lizenz aus den CREDITS-Dateien; Suche nach Name, Pack, Tag, Lizenz.
- **Getrennte Ansichten** Grafik und Sound in k3c-dev; Slot-Liste mit Status (zugeordnet, Lücke, nur Platzhalter).
- **Mehrfachauswahl mit Präferenz**: mehrere Assets je Slot, Reihenfolge per Ziehen; nur das erste ist im Spiel aktiv, die weiteren sind Reserve und Vergleich.
- **Ausschnitte** bei Grafiken: Rechteck (x, y, w, h) bzw. Frame-Raster je Zuordnung, Vorschau im Maßstab `UNIT_PX`.
- **Web-Recherche als Vorschlagsliste** je Slot: Treffer mit Titel, Quelle (Link), Urheber, Lizenz (soweit erkennbar) und Vorschau (Bild bzw. Hörprobe, sonst nur Link). Eine API ist keine Voraussetzung: Treffer kommen aus Such-APIs (z. B. freesound), aus gelesenen Suchseiten (OpenGameArt, itch.io, Kenney, Wikimedia Commons) oder vom Agenten über ein MCP-Tool (Agent sucht im Netz, legt Treffer zum Slot ab). Treffer bleiben je Slot gespeichert, bis 🧑 sie verwirft.
- **Übernahme**: automatisch nur, wo die Quelle einen direkten Download mit maschinenlesbarer Lizenz bietet; sonst lädt 🧑 manuell herunter und übernimmt die Datei per **Import** (Datei wählen, Treffer zuordnen). Der Import legt sie unter `public/…/kandidaten/` ab und trägt Quelle, Urheber, Lizenz aus dem Treffer in die Credits ein. Lizenz ohne Bestätigung → „Lizenz prüfen“, bis 🧑 bestätigt.
- **MCP-Tools** für den Agenten: Slots lesen, Zuordnung setzen, Recherche-Treffer anlegen und lesen.
- Einbettung der Mini-Szenen aus B-296 als iframe vom Vite-Port des Worktrees.

## Nicht-Ziele

Grafik oder Sound bearbeiten (außer Ausschnitt), Einbau neuer Slots in den Renderer, Steuerung per Controller, Betrieb auf der Xbox.

## Regeln und Einschränkungen

Lizenzregel Q70 (`docs/fragenkatalog.md`): CC0, CC-BY, OGA-BY, CC-BY-SA, OFL (Schriften); CC-BY-NC nur als markierter Lückenfüller. Der Manager zeigt die Lizenz je Asset als Kennzeichen, markiert NC-Assets und warnt beim Ausschnitt an SA-Assets (Credit: Ausschnitt unter CC-BY-SA). Nicht übernehmbar: ND und Lizenzen ohne Weitergabe der Rohdateien (Pixabay, Mixkit, CraftPix-Freebies, Zapsplat, Sonniss). Credits sofort (B-165). freesound-Token nur lokal (nicht im Repo). Domäne SRV (`tools/k3c-dev/`); das Slot-Register in `data/` legt SIM bzw. REG an (Grenzfall klären). Logging mit Emojis.

## Beispiele

Slot `building.farm` (Grafik, Kategorie Gebäude) → Suche „farm“ → 🧑 wählt `house-a` (Präferenz 1, Ausschnitt 0,0,64,48) und Kenney Farm (Präferenz 2) → Mini-Szene Gebäude zeigt `house-a`. Slot `sfx.coin.pickup` (Sound) → Recherche liefert freesound „coin“ CC0 (Hörprobe, Übernahme direkt) und einen itch.io-Fund (nur Link, „Lizenz prüfen“) → 🧑 lädt den itch.io-Fund manuell, importiert ihn zum Treffer, bestätigt die Lizenz → liegt unter `public/audio/kandidaten/` mit Credit.

## Ausnahme- und Fehlerfälle

Datei fehlt → Slot zeigt Platzhalter mit Pfad. Quelle nicht erreichbar, Suchseite geändert oder Token fehlt → Hinweis an der Quelle, übrige Quellen liefern weiter. Keine Vorschau ladbar → Treffer nur mit Link. Lizenz unklar → nicht übernehmbar ohne Bestätigung 🧑.

## Akzeptanzkriterien

- **AC-01** k3c-dev zeigt Grafik-Slots und Sound-Slots in getrennten Ansichten aus dem Slot-Register (Beobachtung).
- **AC-02** Je Slot lassen sich mehrere Katalog-Assets per Suche wählen und ordnen; die Reihenfolge steht danach im Slot-Register (Go-Test).
- **AC-03** Ein Ausschnitt einer Grafik wird gespeichert und in der Vorschau angezeigt (Go-Test + Beobachtung).
- **AC-04** Die Recherche zeigt je Slot Vorschläge mit Link, Quelle und Lizenz, auch von Quellen ohne API; Treffer bleiben gespeichert (Go-Test mit aufgezeichneten Antworten + Beobachtung).
- **AC-05** Eine manuell geladene Datei lässt sich zu einem Treffer importieren; Ablage und Credit-Eintrag stimmen, unbestätigte Lizenz bleibt „Lizenz prüfen“ (Go-Test).
- **AC-06** MCP-Tools für Slots, Zuordnung und Recherche-Treffer antworten (`dev:test`).
- **AC-07** Die Mini-Szene aus B-296 ist eingebettet und zeigt die aktive Zuordnung (Beobachtung).

## Offene Fragen

- Wer legt das Slot-Register an (SIM/REG oder SRV als Grenzfall) und wie werden bestehende Zuordnungen migriert?

Entschieden (🧑, Chat 2026-10-05): B-252 ist verworfen und geht hier auf; das Spiel nutzt nur die erste Präferenz, weitere sind Reserve und Vergleich; Recherche ohne API ist erlaubt (Vorschläge mit Link, manueller Download und Import); Lizenzregel Q70.

## Notizen

Gegenstück im Client: B-296 (Mini-Szenen). Verwandt: B-252, B-193, B-167, B-168, B-165. Quellen-Übersicht aus dem Chat vom 2026-10-05.
