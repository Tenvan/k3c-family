# B-677 · Grafik, Sound und Quellen sind im Studio eine Seite Assets mit Fokus-Chips, Inspektor und einer Abspielleiste

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** WZ33
- **Projekt:** WZG
- **Erstellt:** 2026-10-30
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-30 🧑 Chat („dann jetzt umsetzen“)

## Ausgangslage

Das Asset-Studio (`tools/k3c-studio/frontend/src/App.tsx`, `PAGES`) hat drei Seiten für dieselben Dateien:

- **Grafik** mit den Reitern Slots, Szenen und Packs (`graphics/GraphicsPage.tsx`, `ScenesView.tsx`, `PacksView.tsx`).
- **Sound** mit den Reitern Slots, Szenen, Proben und Hörprobe (`sound/SoundPage.tsx`, `SoundScenes.tsx`, `Proben.tsx`, `Hoerprobe.tsx`).
- **Quellen** als eigene Seite mit Dialogen (`sources/`, B-673).

Die Slots-Reiter von Grafik und Sound nutzen dieselbe `assets/AssetsPage.tsx`.

Befund vom 2026-10-30 (Mock, siehe Notizen):

- Es gibt drei Kandidaten-Listen mit eigenem Status: Katalog (`CatalogPicker`), Recherche (`ResearchPanel`) und Vorschläge (Quellen-Seite).
- Es gibt sechs Abspielstellen mit eigenem Stopp und vier Filterleisten.
- Eine Netz-Datei in einen Slot zu bringen braucht sechs Schritte und zwei Seitenwechsel ohne Rückweg. `VorratAdopt` (`tools/assetkit/vorrat_adopt.go`) übernimmt die Datei und ordnet sie zugleich als letzte Präferenz zu, aber die Oberfläche trennt Laden und Übernehmen auf zwei Seiten.
- Filter gehen beim Wechsel zwischen Grafik und Sound verloren.
- Die Slot-Liste zeigt weder ein Bild noch ▶.

## Ziel

🧑 verwaltet alle Asset-Slots und Dateien auf einer Seite und stellt sie mit einem Klick auf ein Thema scharf: Grafik, Sound, Lücken, Netz, Lizenz oder Vorrat. Eine Lücke lässt sich aus dem Netz in zwei Schritten füllen, ohne die Seite zu wechseln.

## Beteiligte und Zielgruppen

- 🧑 Ralf sucht, prüft, lädt und ordnet Grafik und Ton im Studio zu.
- Claude (Studio-Chat) bekommt den Kontext der gewählten Slots.
- Sessions der Domäne DEV setzen um.

## Anforderungen

### Seite und Navigation

- Eine Seite **Assets** ersetzt Grafik, Sound und Quellen im Kopf. Welt, Planer, Monarch und Gegner bleiben.
- **Fokus-Chips** oben sind gespeicherte Filter mit Anzahl: Alles, Grafik, Sound, Lücken, Aus dem Netz, Lizenz prüfen, Vorrat.
- **Eigene Fokus-Chips** kommen schon im ersten Schritt: „Ansicht merken“ speichert Suche, Facetten, Sicht und Gruppierung unter einem Namen in den Studio-Einstellungen (`prefs`). Eigene Chips lassen sich umbenennen und löschen.
- **Sicht-Schalter** mit drei Sichten:
  - **Slots:** bei Grafik ein Raster, bei Sound eine Liste mit ▶ in der Zeile.
  - **Dateien:** jede Datei mit ihrer Stufe Netz, Vorrat, Spiel oder Präferenz n.
  - **Szene:** die Mini-Szenen im Spielmaßstab.
- **Gruppieren nach** Kategorie, Spielbereich, Quelle, Pack oder Slot.
- **Eine Suche und Facetten links**: Art, Kategorie, Status, Stufe, Quelle, Lizenz.
- Der Filterzustand bleibt beim Wechsel von Fokus und Sicht erhalten und übersteht einen Neustart.

### Inspektor

Der Inspektor hat drei Reiter:

- **Belegung:** Vorschau oben, Präferenzen mit Vergleich, Ausschnitt.
- **Kandidaten:** Spiel (Katalog), Vorrat und Netz (Recherche-Treffer und Vorschläge) in einer Liste, je Zeile mit Stufe, Lizenz-Marke und Größe. Dubletten werden nach Datei-URL bzw. Prüfsumme zusammengeführt.
- **Details:** heute `SlotInfo`.

**Zuordnen** ist eine einzige Aktion für jede Stufe:

- Bei Netz-Kandidaten erscheint die Bestätigung direkt in der Zeile (Größe, Lizenz, Ziel-Slot), kein Dialog.
- Danach laden, übernehmen und zuordnen in einem Backend-Aufruf.
- Ein Vorrat-Kandidat wird übernommen und zugeordnet; eine Spiel-Datei wird zugeordnet.

Die Sprünge aus anderen Seiten, aus Chat und Ticket mit Slot (`takePendingSlot`, `assignContext`) öffnen Assets mit dem gewählten Slot.

### Abspielen und Werkzeuge

- **Eine Abspielleiste** unten für die ganze Seite. Es spielt immer nur ein Ton.
- Die Leiste bietet Stopp, Überblenden, nah/fern und Lautstärke sowie die Tastatur der Hörprobe (↑↓, Enter, S, X, +/−).
- Der Reiter Hörprobe entfällt ersatzlos und geht in Fokus Sound und Abspielleiste auf.
- **Werkzeuge** öffnen rechts als Fläche statt als Reiter: Mischprobe, Lautheit, Abstand, Vorrat aufräumen, Aus Recherche übernehmen. Das Verhalten bleibt wie heute.

### Einheitliches Vokabular

- **Slot-Status:** aktiv, Lücke, Datei fehlt.
- **Stufe einer Datei:** Netz, Vorrat, Spiel, Präf. n.
- **Lizenz-Marke:** CC0/BY, SA, prüfen, gesperrt.
- Begriffe stehen im Glossar (Assets-Seite, Fokus-Chip, Sicht, Asset-Kandidat, Asset-Vorrat, Abspielleiste).

## Nicht-Ziele

- Neue Quellen jenseits der vier erlaubten (Wikimedia Commons bleibt draußen, siehe B-673).
- Lizenzen automatisch bestätigen: „bestätigt“ bleibt bei 🧑.
- Websuche im Studio-Chat (Sicherheitsänderung, siehe B-673 › Notizen).
- Die Seiten Welt, Planer, Monarch und Gegner (B-670–B-672).
- Änderungen am Spiel, am Slot-Register-Format oder am Server.

## Regeln und Einschränkungen

- Domäne DEV in diesen Ordnern:
  - `tools/k3c-studio/frontend/src/` (neuer Ordner `assets/` bzw. Feature-Ordner der Seite)
  - `tools/k3c-studio/backend/`
  - `tools/assetkit/`
  - Bausteine in `tools/webkit`
- Radix nur über `@webkit` und Atomic Design (`tests/atomicDesign.test.ts`); keine neue Abhängigkeit.
- Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.
- Das Studio schreibt `data/**` und `public/`, der Server liest nur (Entscheidung 004).
- Heruntergeladene Dateien bleiben untrusted: nur Bild und Ton mit erlaubter Endung, nichts ausführen (B-601).
- Lizenzregeln Q70 gelten: NC und ND nie, `SuggestionLicenseAllowed`.
- Zuordnen hängt die Datei als letzte Präferenz an, wie `VorratAdopt`. Hat der Slot noch keine Zuordnung, wird sie damit aktiv.
- Zuordnen prüft den Stand des Slot-Registers (wie `VorratAdoptIf`). Bei fremder Änderung bricht es ohne Schreiben ab.
- `soundtest.html` nutzt die Hörprobe-Logik (`soundtestLogic`) weiter und darf nicht brechen.
- Tests und Vorschauen auf Port 5196, nie auf 5192 oder 8091: Das Studio ist immer offen.

## Beispiele

- **Seite öffnen:** Fokus „Alles 214“, Sicht Slots, Facetten links, Inspektor rechts, Abspielleiste unten.
- **Fokus „Lücken“:** Es erscheinen nur Slots ohne aktive Zuordnung, gruppiert nach Kategorie. Taverne wählen, Reiter Kandidaten: sieben Kandidaten aus Spiel, Vorrat und Netz. Bei einem Netz-Kandidaten auf „Zuordnen“ klicken, die Zeile zeigt „212 KB · CC0 · in Taverne“, Bestätigen. Der Slot ist aktiv, die Lücke verschwindet aus der Liste.
- **Fokus „Sound“:** In der Zeile `music:nacht` auf ▶ klicken. Die Abspielleiste zeigt den Titel. Mit ↓ und Enter springt die Wiedergabe zum nächsten Slot, der alte Ton stoppt.
- **Eigene Ansicht merken:** Fokus Grafik, Facette Kategorie Gebäude, Sicht Szene, „Ansicht merken“ als „Gebäude im Spiel“. Nach einem Neustart steht der Chip neben den festen.
- **Sprung aus dem Gebäude-Planer** mit Slot `farm`: Assets öffnet sich mit Fokus Alles und `farm` im Inspektor.

## Ausnahme- und Fehlerfälle

- **Offline:** Netz-Kandidaten sind sichtbar, „Zuordnen“ ist gesperrt mit Hinweis. Vorrat und Spiel bleiben nutzbar.
- **Download schlägt fehl:** Die Zeile zeigt den Fehler, nichts wird übernommen oder zugeordnet. Ein erneuter Versuch ist in der Zeile möglich.
- **Slot-Register extern geändert:** Abbruch mit Hinweis „Neu laden“, nichts geschrieben.
- **Lizenz `pruefen` oder gesperrt:** „Zuordnen“ ist gesperrt, die Marke nennt den Grund.
- **Eigener Fokus-Chip mit nicht mehr vorhandener Facette** (z. B. gelöschte Kategorie): Der Chip lädt, die unbekannte Facette fällt weg, ein Hinweis erscheint.
- **Gleicher Kandidat als Recherche-Treffer und als Vorschlag:** Er erscheint einmal, mit beiden Herkünften im Tooltip.

## Akzeptanzkriterien

- **AC-01** `PAGES` enthält `assets` statt `grafik`, `sound` und `quellen`. Welt, Planer, Monarch und Gegner sind unverändert (Vitest).
- **AC-02** Die festen Fokus-Chips setzen ihre Facetten und zeigen die richtige Anzahl. „Ansicht merken“ legt einen eigenen Chip an, umbenennen und löschen funktioniert, und der Chip übersteht ein Neuladen (Vitest).
- **AC-03** Die Sichten Slots, Dateien und Szene und die Gruppierungen Kategorie, Spielbereich, Quelle, Pack und Slot zeigen dieselbe gefilterte Menge. Grafik erscheint als Raster, Sound als Liste mit ▶ (Vitest).
- **AC-04** Suche und Facetten bleiben beim Wechsel von Fokus und Sicht erhalten und überstehen ein Neuladen (Vitest).
- **AC-05** Das Backend liefert je Slot eine Kandidaten-Liste aus Katalog, Vorrat, Recherche und Vorschlägen mit Stufe und Lizenz, ohne Dubletten nach Datei-URL bzw. Prüfsumme. Geprüft durch einen Go-Test in `tools/assetkit` und im Backend.
- **AC-06** „Zuordnen“ eines Netz-Kandidaten erledigt Laden, Übernehmen und Zuordnen als letzte Präferenz in einem Aufruf mit Stand-Prüfung. Scheitert ein Schritt, wird nichts zugeordnet, und der Fehler steht in der Zeile (Go-Test, Vitest für die Bestätigung in der Zeile).
- **AC-07** Auf der ganzen Seite spielt höchstens ein Ton. Abspielleiste und Tastatur (↑↓, Enter, S, X, +/−) funktionieren. Der Reiter Hörprobe existiert nicht mehr, und `soundtest.html` läuft weiter (Vitest).
- **AC-08** Mischprobe, Lautheit, Abstand, Vorrat aufräumen und „Aus Recherche übernehmen“ sind über die Werkzeuge-Fläche erreichbar. Ihre bisherigen Tests sind grün oder ohne Verlust an geprüftem Verhalten übertragen.
- **AC-09** Sprünge mit Slot aus Planer, Monarch, Gegner, Chat und Ticket öffnen Assets mit diesem Slot. Der Ticket- und Rahmen-Kontext nutzt die Seiten-ID `assets`; der Zuordnungs-Kontext des Chats trägt weiter `seite` = `sound` oder `grafik` (Art des gewählten Slots), weil das Backend danach unterscheidet (Vitest).
- **AC-10** Die alten Seiten Grafik, Sound und Quellen sind entfernt, ohne tote Importe. `task check:studio` und `tests/atomicDesign.test.ts` sind grün, und keine Datei ist länger als 400 Zeilen.
- **AC-11** Browser-Pane (Port 5196, `?mock=1`, 1600×1000): Kopf, Fokus-Chips, Facetten, Liste und Inspektor sind ohne Scrollen sichtbar, die Abspielleiste steht fest unten.
- **AC-12** Manuelle Abnahme durch 🧑 im Studio-Fenster mit echten Daten: Eine Lücke wird aus dem Netz in zwei Schritten gefüllt, und eine eigene Ansicht bleibt nach dem Neustart erhalten.

## Offene Fragen

keine. Entschieden von 🧑 am 2026-10-30 im Chat:

- Die Seite heißt „Assets“.
- „Zuordnen“ bestätigt direkt in der Zeile.
- Die Hörprobe entfällt.
- Eigene Fokus-Chips kommen gleich im ersten Schritt.

## Notizen

- Die Mocks liegen im Artifact „Asset-Studio: Assets-Seite“ (https://claude.ai/artifact/U3mBTEvYgr3YcFNLpumS4K, privat). Es hat neun Flächen: Befund, System, Fokus Grafik, Fokus Sound, Fokus Lücken, Dateien aus dem Netz, Szene, Werkzeuge und Umbau.
- Die Quellen-Oberfläche aus B-673 (Reiter, Filterzeile, Gruppen nach Slot, Schreibwerkzeuge in `assetkit`) geht hier auf. Ihre Backend-Funktionen (`AddSuggestions`, `PlanSuggestions`, `research_add.go`) bleiben.
- B-673/AC-08 (manuelle Abnahme) vor WZ33 abschließen oder in AC-12 aufgehen lassen.
- Ungeprüft: wie viele Recherche-Treffer und Vorschläge heute auf dieselbe Datei-URL zeigen.
- **Umgesetzt 2026-10-30 (WZ33):** Backend `tools/assetkit/candidates*.go`, `backend/app_candidates.go` (RPC AssetCandidates, AssetCandidateAssign); Frontend `src/assetpage/`, `api/candidateTypes.ts`, webkit FocusBar, FacetRail, TransportBar. Alte Seiten Grafik, Sound, Quellen entfernt; die Tests mit Wert sind übertragen.
- **Auslegung AC-04:** Ein fester Chip ersetzt die Facetten, behält aber Suche, Sicht und Gruppierung; ein eigener Chip setzt auch diese.
- **Abweichungen:** Sicht Szene übernimmt die Fokus-Filter nicht (eigene Filter der Szenen); die Proben behalten ihren eigenen Stopp; NC-Treffer und -Katalogdateien sind nicht zuordenbar (strenger als zuvor), ebenso Pakete (zip, 7z).
- **Abgenommen 2026-10-31:** AC-12 bestanden (🧑 Ralf, WZ33.8).
- **Offen (jetzt B-678):** Wikimedia-Downloads mit Weiterleitung scheitern im Studio-Client möglicherweise (ungeprüft); SourceAdopt ist ohne Stand und Sperre aufrufbar (nur die Seite Assets ruft es seriell).
