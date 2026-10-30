# B-673 · Die Quellen-Seite ist kompakt nach Slot gegliedert, und alle vier Quellen lassen sich mit Vorschlägen füllen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** –
- **Projekt:** WZG
- **Erstellt:** 2026-10-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-30 🧑 Chat („specs sind alle freigegeben“)

## Ausgangslage

Seite Quellen des Studios (`tools/k3c-studio/frontend/src/sources/`, Backend `tools/k3c-studio/backend/app_sources.go`, B-601). Oben stehen vier Karten für OpenGameArt, Kenney, itch.io und freesound, fest aus `assets.SuggestionSources` (`tools/assetkit/suggestions.go`), mit Lizenzregeln und Zählern. Darunter Auswahl-Summe und Knöpfe, dann `ListFilter` mit sieben Chip-Zeilen (Quelle, Art, Slot, Lizenz, Status, Verwendung; etwa 170 px) und eine flache Liste mit 8–10 Elementen je Zeile (`ProposalRow`). Gesehen am 2026-10-30 im Browser-Pane (1600×1000, `?mock=1`).

Die Liste kommt allein aus `data/assets-vorschlaege.json` (WZ29.5). Dort stehen 41 Einträge, alle von OpenGameArt (17 Ton, 24 Grafik), alle `offen`. Neue Einträge kann das Studio nicht anlegen; es gibt keinen Schreibweg außer von Hand, geprüft von `CheckSuggestions`. Kenney, itch.io und freesound stehen deshalb dauerhaft auf 0.

Daneben gibt es die Recherche-Treffer der Workbench (`data/assets-recherche.json`, B-298, MCP `assets_hit_add`): 128 Treffer, davon 65 OpenGameArt, 33 Kenney, 30 Wikimedia Commons, mit `lizenzStatus`. Die Quellen-Seite kennt sie nicht. Wikimedia Commons ist keine erlaubte Quelle in `SuggestionSources`.

## Ziel

🧑 sieht auf einen Blick, welcher Slot noch eine Lücke hat und welche Kandidaten es dafür gibt, und kann aus jeder Quelle neue Vorschläge bekommen, ohne die JSON-Datei von Hand zu pflegen.

## Beteiligte und Zielgruppen

- 🧑 Ralf sucht und lädt Grafik und Ton für Lücken im Studio.
- Claude (Studio-Chat) sucht Vorschläge und trägt sie ein.
- Sessions der Domäne DEV setzen um.

## Anforderungen

### Oberfläche

- **Quellen als Reiter:** „Alle“ und je Quelle ein Reiter mit Anzahl der Vorschläge. Unter dem gewählten Reiter in einer Zeile die Lizenzregeln und ein Link zur Quellseite. Die Karten entfallen.
- **Leere Quelle:** statt einer leeren Liste ein Hinweis mit den Knöpfen „Claude in <Quelle> suchen lassen“ und „Aus Recherche übernehmen“.
- **Eine Filterzeile:** Suche, Art (Alle · Ton · Grafik) und Status (Offen · Geladen · Verwendet · Alle) als Segment-Knöpfe; Slot, Lizenz und „gesperrte zeigen“ unter „Filter ▾“. Der Filterzustand bleibt gemerkt wie heute (`quellen.filter`).
- **Liste nach Slot gruppiert:** Gruppenkopf mit Slot, Anzahl Kandidaten und „Lücke“ bzw. „belegt“. Je Zeile: Auswahl (nur ladbare), Vorschau (Bild klein bzw. ▶ für Hörprobe, nur geladen), Titel mit Quelle und Art darunter, Lizenz, Größe, rechts Status oder Aktion („Übernehmen“ bei geladen und unverwendet). Prio, Urheber, Prüfsumme und Hinweis im Tooltip oder beim Aufklappen der Zeile.
- **Aktionsleiste nur bei Auswahl:** „n gewählt · Größe“, „Auswahl leeren“, „Herunterladen“ (Bestätigungsdialog wie heute). „Vorrat aufräumen“ und „Neu laden“ als Symbol-Knöpfe in der Filterzeile.
- Sprung aus anderen Seiten mit Slot (`takePendingSlot`) setzt den Slot-Filter und klappt die Gruppe auf.

### Vorschläge füllen

- **Aus Recherche übernehmen:** Recherche-Treffer mit direkter Datei-URL, erlaubter Quelle und `lizenzStatus` erkannt oder bestätigt werden als Vorschlag `offen` in `data/assets-vorschlaege.json` übernommen (Slot, Titel, Quelle, Seite, Datei-URL, Urheber, Lizenz, Größe per HEAD). Vorher zeigt ein Dialog die Liste mit Anzahl und Größe; geschrieben wird erst nach Bestätigung. Treffer ohne Datei-URL oder mit `pruefen` werden mit Grund übersprungen.
- **Claude suchen lassen:** Der Knopf öffnet den Studio-Chat mit einem vorbereiteten Auftrag (Quelle, Slot oder alle Lücken, Art). Claude sucht, prüft die Lizenz auf der Quellseite (Q70) und schreibt Einträge über ein neues Backend-Werkzeug, das `checkSuggestion` je Eintrag anwendet und atomar schreibt. Ungültige Einträge werden mit Grund abgelehnt, nichts wird geladen.
- Schreiben der Vorschlagsliste nur über eine Funktion in `tools/assetkit` (anhängen, Duplikate nach Datei-URL ablehnen, ID vergeben); Studio und Workbench nutzen dieselbe.

## Nicht-Ziele

- Automatisches Herunterladen ohne Bestätigung.
- Neue Quellen jenseits der vier (Wikimedia Commons siehe offene Fragen).
- Änderungen an Vorrat, Übernehmen und Aufräumen über die Oberfläche hinaus.
- Lizenzen selbst freigeben: „bestätigt“ bleibt bei 🧑.

## Regeln und Einschränkungen

- Domäne DEV: `tools/k3c-studio/frontend/src/sources/`, `tools/k3c-studio/backend/`, `tools/assetkit/`, Bausteine in `tools/webkit`.
- Heruntergeladene Dateien bleiben untrusted (nur Bild und Ton mit erlaubter Endung, nichts ausführen), wie B-601.
- Lizenzregeln Q70: NC und ND nie; `SuggestionLicenseAllowed` gilt auch für übernommene Treffer.
- Das Studio schreibt `data/**`, der Server liest nur (Entscheidung 004).
- Radix nur über `@webkit`, Atomic Design; keine neue Abhängigkeit; Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

- Seite öffnen → Reiter „Alle 41 · OpenGameArt 41 · Kenney 0 · itch.io 0 · freesound 0“, eine Filterzeile, Liste nach Slot gruppiert.
- Reiter „Kenney“ → Hinweis „Noch keine Vorschläge von Kenney“ mit „Aus Recherche übernehmen“; Klick → Dialog nennt z. B. 20 übernehmbare und 13 übersprungene Treffer mit Grund; nach Bestätigung zeigt der Reiter „Kenney 20“.
- „Claude in freesound suchen lassen“ für Slot `skill-effekt:heiler` → Chat öffnet mit Auftrag; Claude trägt drei Kandidaten mit CC0/CC-BY ein, einen mit NC lehnt das Werkzeug ab.
- Zwei Einträge wählen → Aktionsleiste „2 gewählt · 111 kB · Herunterladen“.

## Ausnahme- und Fehlerfälle

- Offline → Hinweis wie heute, Suchen und Übernehmen aus Recherche gesperrt (Größe per HEAD nicht möglich), Geladenes bleibt nutzbar.
- Datei-URL schon in der Liste → Eintrag wird nicht doppelt angelegt, Dialog nennt ihn als übersprungen.
- Datei extern geändert während des Schreibens → Abbruch mit Hinweis „Neu laden“, nichts geschrieben.
- Recherche-Treffer von Wikimedia Commons → übersprungen mit Grund „Quelle nicht erlaubt“.
- Slot existiert nicht mehr im Register → Eintrag abgelehnt (`SuggestionTargets`).

## Akzeptanzkriterien

- **AC-01** Browser-Pane (Port 5196, `?mock=1`, 1600×1000): Reiter, Filterzeile und mindestens die ersten zwei Slot-Gruppen sind ohne Scrollen sichtbar; die Filter nehmen zugeklappt höchstens eine Zeile ein.
- **AC-02** Die Liste ist nach Slot gruppiert mit „Lücke“/„belegt“ im Kopf; Reiter und Filter wirken zusammen (Vitest).
- **AC-03** Eine Quelle ohne Vorschläge zeigt den Hinweis mit beiden Knöpfen (Vitest).
- **AC-04** „Aus Recherche übernehmen“ schreibt nur Treffer mit Datei-URL, erlaubter Quelle und Lizenz erkannt/bestätigt, ohne Duplikate, erst nach Bestätigung; übersprungene stehen mit Grund im Dialog (Go-Test in `tools/assetkit` und Backend, Vitest für den Dialog).
- **AC-05** Das Schreibwerkzeug für Claude lehnt ungültige Einträge (Lizenz, Quelle, Slot, Endung) mit Grund ab und schreibt gültige atomar (Go-Test).
- **AC-06** Die Aktionsleiste erscheint nur bei Auswahl und zeigt Anzahl und Größe; Herunterladen läuft über den bestehenden Bestätigungsdialog (Vitest).
- **AC-07** Bestehende Tests von `SourcesPage`, `sourcesLogic`, `NetworkSwitch` und dem Backend sind grün oder ohne Verlust an geprüftem Verhalten angepasst.
- **AC-08** Manuelle Abnahme durch 🧑 mit echten Daten: mindestens eine weitere Quelle außer OpenGameArt hat Vorschläge, einer davon ist geladen und übernommen.

## Offene Fragen

- Wikimedia Commons als fünfte erlaubte Quelle aufnehmen (30 Recherche-Treffer)? Default: nein, eigenes Ticket mit Lizenzregeln. Entscheidet 🧑.
- Recherche-Treffer nach der Übernahme in `assets-recherche.json` behalten oder markieren? Default: behalten, Vorschlag verweist per Datei-URL. Entscheidet 🧑.
- Soll das Schreibwerkzeug auch als MCP-Tool der Workbench verfügbar sein (z. B. `assets_suggest_add`)? Default: ja, gleiche Funktion aus `tools/assetkit`. Entscheidet 🧑.
- Ticket teilen in Oberfläche und Vorschläge-füllen? Default: ein Ticket, zwei Sessions. Entscheidet 🧑 beim Planen.

## Notizen

- Mock aus dem Chat vom 2026-10-30 (Widget „studio_quellen_mock“).
- Geschwister-Tickets der Studio-Bedienung: B-670 (Gebäude-Planer), B-671 (Monarch), B-672 (Gegner).
- Zählung am 2026-10-30: `assets-vorschlaege.json` 41 Einträge (OpenGameArt), `assets-recherche.json` 128 Treffer (OpenGameArt 65, Kenney 33, Wikimedia Commons 30).
- **Umgesetzt 2026-10-30:** assetkit `suggestions_add.go` (`AddSuggestions`, `PlanSuggestions`, `HeadSize` mit 10 s Frist), `research_add.go`; Backend `app_sources_add.go`, `chat_sources.go`, `chat_http.go`; Frontend `sources/` (`SourceTabs`, `SourcesFilterBar`, `SourceList`, `sourcesFilter`), `chat/chatBridge.ts`, `chat/ChatThread.tsx` (Vorbelegung der Chat-Eingabe), `styles/sources.css`.
- AC-01 Browser-Pane 5196 `?mock=1` 1600×1000: Reiter, eine Filterzeile, Gruppen sichtbar, Aktionsleiste „1 gewählt · 25 kB“. AC-02, AC-03, AC-06, AC-07 Vitest; AC-04, AC-05 Go-Tests (`suggestions_add_test.go`, `app_sources_add_test.go`); `check:diff` grün. Review ohne schwere Befunde (HEAD nur https auf erlaubte Hosts, Weiterleitungen geprüft).
- Abweichung: Der Studio-Chat hat keine Websuche (`--tools ""`). „Claude suchen lassen“ schlägt aus dem Modellwissen vor; das Backend prüft jeden Eintrag (Lizenz, Quelle, Slot, HEAD) und markiert ihn mit „Lizenz vor dem Laden auf der Quellseite prüfen (Q70)“. Websuche wäre eine Sicherheitsänderung, nicht gemacht.
- Befund: Mit den echten Daten bringt „Aus Recherche übernehmen“ nichts; nur die 30 Wikimedia-Treffer haben eine Datei-URL, und Wikimedia ist nicht erlaubt.
- Entschieden nach Default: Wikimedia bleibt draußen; Treffer bleiben in der Recherche; MCP-Tool der Workbench nicht gebaut (die gemeinsame Funktion steht in assetkit). Grenze: Die Sperre beim Schreiben gilt nur im eigenen Prozess (Kommentar in `AddSuggestions`).
- Offen: AC-08 manuelle Abnahme durch 🧑 mit echten Daten.
