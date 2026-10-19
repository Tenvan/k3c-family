# B-488 · Welten und Inseln im Asset-Studio anlegen und bearbeiten, Welt per Prompt füllen

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ18
- **Projekt:** WZG
- **Erstellt:** 2026-10-18
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-18, Ralf im Chat (Revision 1); mit WZ18 und WZ19 bestätigt 2026-10-19, Ralf im Chat

## Ausgangslage

Wunsch von 🧑 (2026-10-18, Chat). Heute gibt es eine einzige Datei `data/islands.json` mit einer Liste von Inseln (heute nur Insel 1); der Editor des Asset-Studios (WZ8, WZ17) bearbeitet nur die geplanten Chunks einer Stufe. Es gibt keinen Begriff „Welt“ als eigene Datei, keinen Editor zum Anlegen und Ändern von Inseln und keine Hilfe, eine Welt mit einem Prompt zu füllen. Offen im Plan: Anzahl und Inhalt weiterer Inseln (`docs/rules/stufen.md` § 7); B-103 hat Inselwechsel gebaut, aber ausdrücklich keine weiteren Inseln. B-435 (Gebäude per Prompt im Asset-Manager) ist die nächste Verwandtschaft für den Prompt-Gedanken.

## Ziel

Im Asset-Studio lassen sich Welten (eine Welt hat n Inseln, jede Welt eine eigene JSON-Datei unter `data/worlds/`) und Inseln anlegen und bearbeiten. Ein Chat rechts im Studio spricht über die Claude Code CLI mit einem Modell und füllt damit eine Welt: Ein Button liefert ein Grundgerüst-Prompt, 🧑 erweitert es um eigene Wünsche, die Antwort wird geprüft und als Entwurf geladen. Der Raum wählt beim Anlegen eine Welt, der Spielstand merkt sich Welt und Insel, mehrere Inseln sind im Spiel spielbar.

## Beteiligte und Zielgruppen

🧑 (Ralf) entwirft Welten und Inseln im Asset-Studio, formuliert die Wünsche für den Prompt, entscheidet über Format und Spec-Freigabe und nimmt am PC ab. Spielende profitieren erst in Stufe B (Welt-Wahl im Raum). Agenten setzen die Sessions je Domäne um, ein getrennter Reviewer prüft.

## Anforderungen

**Entwurf zur Freigabe (Claude, Stand 2026-10-18); die Entscheidungen stehen unter „Offene Fragen“.**

**Zuschnitt:** eine Spec, zwei Sprints (Reihenfolge nach WZ17): **Sprint 1** Welten im Spiel (REG, SIM, SRV, DEV: Welt-Dateien, Spiel wählt die Welt, Welten-Editor); **Sprint 2** Chat und Prompt (DEV, Go: Chat rechts, Claude Code CLI, Welt füllen).

**Welt-Datei und Spiel (Sprint 1: REG, SIM, SRV)**
- Ordner `data/worlds/`, je Welt eine Datei `<id>.json` mit `id`, `name`, `notes` und `islands` (Liste in fester Reihenfolge). Eine Insel hat die Felder von heute (`id`, `name`, `depths`, `depthScaling`, `plannedChunks`) und die Chunk-Anzahl je Stufe aus WZ17. `data/islands.json` wird zu `data/worlds/standard.json` (Inhalt unverändert), alle Pfade in Sim, Studio und Tests ziehen mit.
- Die Prüfung des Ladens (`checkIsland`: Tiefen, Miniboss je Tiefe, Endboss in der tiefsten Stufe, Faktoren > 0, `plannedChunks` gegen die Mauerlinien) gilt für jede Welt; Fehler nennen Welt, Insel, Tiefe und Platz.
- Die Sim lädt Welten über einen Parameter statt über die globale Liste (`islandDefs`); die Standard-Welt bleibt Voreinstellung.
- Der Raum wählt beim Anlegen eine Welt (Standard `standard`; die Wahl gibt es nur im Dev-Mode). Der Spielstand speichert Welt-ID und Insel-ID statt des Index in `islands.json` (Spielstand-Version +1); alte Stände (Version 5) werden auf Welt `standard` mit dem alten Index migriert. Mehrere Inseln und der Inselwechsel (B-103) laufen mit Welten, die mehr als eine Insel haben; Obergrenze und Inhalt weiterer Inseln bleiben offen (Welt-Dateien), Balancing bleibt Sache der REG-Runden.
- Das Protokoll trägt die Welt-ID dort, wo der Client die Insel kennen muss (Version erhöhen, Beispiel in `docs/protocol.md`).

**Editor Welten und Inseln (Sprint 1: DEV, Seite „Welten Editor“)**
- Welten: anlegen (leer oder als Kopie), umbenennen, auswählen. Inseln einer Welt: anlegen, Reihenfolge ändern, Name und id, Stufen (Tiefen aus den vorhandenen Biomen), Skalierungstabelle, Planung der Chunks und Chunk-Anzahl (Editor aus WZ8/WZ17).
- Entwurf zuerst: ungespeicherte Änderungen mit sofortiger Prüfung (Fehler am Feld) und Vorschau; Speichern nur Gültiges, atomar, format-stabil, Stand-Prüfung, REG-Hinweis (wie WZ8 AC-12).

**Chat und Welt füllen (Sprint 2: DEV, Go für den Prozess-Start)**
- Ein allgemeiner Chat rechts im Studio (auf allen Seiten, ein-/ausblendbar), gebaut mit **assistant-ui**, schickt Nachrichten an die Claude Code CLI (`claude -p`, nicht-interaktiv, mit der vorhandenen Anmeldung von 🧑; kein Schlüssel im Studio) und zeigt die Antwort fortlaufend. Lizenz, Größe und React-Version von assistant-ui belegt die Umsetzungs-Session; die neue Abhängigkeit gibt 🧑 vorher frei.
- Der Aufruf der CLI läuft nur in Go (Wails-Binding), mit festem Arbeitsordner, ohne Schreibrechte und ohne Werkzeuge für das Modell (nur Text rein, Text raus); Zeitlimit; Abbruch-Knopf; austauschbar durch einen Test-Ersatz (kein echter Modellaufruf in der CI).
- **Chat-Verlauf:** wird im Ordner `.chats` (im Repo-Stamm, von git ignoriert) gespeichert und nach **einer Woche** gelöscht (Aufräumen beim Start des Studios und beim Speichern); nur lokal, nie eingecheckt.
- Auf der Seite Welten Editor erzeugt der Button „Prompt-Grundgerüst“ aus den Daten einen Text (Format der Welt-Datei mit Beispiel, gültige Tiefen und Biome, vorhandene Chunk-Arten und Objektarten, Größen-Tabelle, Prüfregeln, Abschnitt „MEINE WÜNSCHE:“) und legt ihn in das Eingabefeld des Chats; 🧑 ergänzt und sendet. Die JSON-Antwort wird geprüft (wie beim Laden) und als **Entwurf** geladen; bei Fehlern steht der Grund am Entwurf, nichts wird ungeprüft geschrieben.

## Nicht-Ziele

- API-Schlüssel, direkte API-Aufrufe und Cloud-Dienste aus dem Studio; Werkzeuge, Dateizugriff oder Code-Ausführung für das Modell im Chat.
- Neue Biome, neue Bosse, Balancing der Skalierung (REG-Runden BR1/BR2), Ruinen und Gegnerlager (B-468), Variante „Ebenen“ (`stufen.md` § 1).
- Spielstände älterer Versionen verwerfen (sie werden migriert).
- Automatisches Speichern einer vom Modell gelieferten Welt.

## Regeln und Einschränkungen

- Daten gehören nach `data/`, werden eingebettet (`data/embed.go`); Generator und Sim bleiben deterministisch (`engine/rng`).
- Heute lädt `engine/sim/island_data.go` die Inseln einmal beim Start (`islandDefs = loadIslandDefs()`, Panic bei Fehler) und der Spielstand Version 5 speichert die Insel als Index in `islands.json` (`engine/sim/island_save_v5.go`). Der Umbau auf Welten ändert Spielstand-Format, `IslandSaveVersion`, Protokoll und Raum-Anlegen: Golden- und Speicher-Tests zuerst, Migration alter Stände mit Test.
- Eine Session hat genau eine Domäne (REG Format/Workshop, SIM Laden/Prüfung/Spielstand, SRV Raum/Protokoll/Vorschau, DEV Editor und Chat); der Sprint hat deutlich mehr Sessions als der Richtwert 3–6 und wird beim Planen ggf. in zwei Sprints mit gemeinsamer Spec geschnitten (Einschätzung vor der Freigabe).
- **Neue Abhängigkeit:** ein Chat-Paket (Node) im Studio-Frontend; Freigabe durch 🧑 vor der Umsetzung, Lizenz und Größe in der Session belegt. Die Claude Code CLI ist eine Laufzeit-Voraussetzung nur auf dem Rechner von 🧑, nicht der CI und nicht des Spiels.
- Sicherheit: kein Schlüssel im Studio, kein Modell-Zugriff auf Dateien (nur Text), kein Prompt-Inhalt aus Spielständen oder Netz; die Antwort des Modells ist Daten und wird wie jede Datei geprüft.
- Golden-Daten bleiben ohne Welt-Wahl byte-gleich; eine geänderte Standard-Welt ist eine Entscheidung von 🧑.
- Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen.

## Beispiele

- 🧑 legt die Welt „Nordmeer“ als Kopie der Standard-Welt an, fügt „Insel 2“ mit den Tiefen 0–2 hinzu, setzt die Chunk-Anzahl der Wald-Stufe auf „klein“ (12) und speichert → `data/worlds/nordmeer.json` entsteht, Golden bleibt unverändert; beim Anlegen eines Raums wählt 🧑 „Nordmeer“, nach dem Endboss von Insel 1 wechseln alle gemeinsam auf Insel 2.
- 🧑 klickt „Prompt-Grundgerüst“, ergänzt im Chat „Vier Inseln, die dritte ist ein reines Höhlenabenteuer“ und sendet → das Modell antwortet mit einer Welt-Datei; die Prüfung nennt „Insel 3: kein Endboss in der tiefsten Stufe 2“, der Entwurf bleibt, 🧑 korrigiert im Editor oder fragt im Chat nach.
- Antwort ist kein JSON → Fehler „Antwort nicht lesbar“ am Entwurf, nichts geladen.
- Alter Spielstand (Version 5, Insel-Index 0) → wird beim Laden auf Welt `standard`, Insel `island1` migriert.

## Ausnahme- und Fehlerfälle

- Doppelte Welt-id oder Insel-id → abgelehnt mit Nennung; id nur Kleinbuchstaben, Ziffern, Bindestrich (kein Pfadzugriff über die id).
- Welt-Datei von außen geändert → Stand-Prüfung, Entwurf bleibt (wie WZ8 AC-12, B-486).
- Welt ohne Insel, Insel ohne Tiefen, Tiefe ohne Biom, Insel ohne Miniboss oder Endboss → abgelehnt mit Grund (Prüfung des Ladens).
- Prompt-Antwort mit Feldern, die das Format nicht kennt, oder mit Werten außerhalb der Grenzen → die Prüfung benennt sie; unbekannte Felder werden nicht still verworfen.
- Große Welten (n Inseln) bleiben im Editor bedienbar; Obergrenze n und Inhalt weiterer Inseln sind offen (`stufen.md` § 7).

## Akzeptanzkriterien

Entwurf zur Freigabe:
- **AC-01** Regeltext und Glossar („Welt“, „Welt-Datei“, „Chat“) als Beschluss von 🧑; Format `data/worlds/<id>.json` mit Beispiel. (Sprint 1)
- **AC-02** Go-Test: Jede Welt-Datei besteht die Prüfung des Ladens; ungültige Welt/Insel wird mit Welt, Insel, Tiefe und Platz abgelehnt (je Fall ein Test); `standard.json` hat denselben Inhalt wie die bisherige `islands.json`; Golden unverändert. (Sprint 1)
- **AC-03** Go-Test: Der Raum legt mit gewählter Welt an (nur im Dev-Mode wählbar); der Spielstand speichert Welt-ID und Insel-ID (Version +1); ein Stand der alten Version wird auf `standard` migriert; mit 2 Spielern und einer Welt mit mindestens zwei Inseln funktioniert der Inselwechsel. (Sprint 1)
- **AC-04** Protokoll-Test und Beispiel in `docs/protocol.md` für die Welt-ID; Versionen erhöht. (Sprint 1)
- **AC-05** Go- und Frontend-Test: Welt anlegen (leer, Kopie), umbenennen, Insel anlegen, Reihenfolge ändern, Stufen, Skalierung, Planung und Chunk-Anzahl bearbeiten; Speichern nur Gültiges, atomar, format-stabil, Stand-Prüfung, REG-Hinweis; Tests nur auf Kopien. (Sprint 1)
- **AC-06** Go-Test mit Test-Ersatz für die CLI: Der Aufruf sendet den Prompt, liefert die Antwort fortlaufend, bricht bei Zeitlimit und auf Knopfdruck ab, startet nie mit Schreibrechten oder Werkzeugen; kein echter Modellaufruf in der CI. (Sprint 2)
- **AC-07** Test: Der Chat-Verlauf liegt in `.chats` (von git ignoriert); Einträge älter als eine Woche werden beim Start gelöscht, jüngere bleiben. (Sprint 2)
- **AC-08** Frontend-Test: Der Button erzeugt das Prompt-Grundgerüst aus den Daten; die Antwort wird geprüft und als Entwurf geladen, Fehler stehen am Entwurf; der Chat ist auf allen Seiten ein- und ausblendbar. (Sprint 2)
- **AC-09** 🧑 nimmt am PC eine Welt mit mindestens zwei Inseln ab, einmal von Hand (Sprint 1) und einmal per Chat gefüllt (Sprint 2), als ungespeicherten Entwurf, und startet sie im Spiel.
- **AC-10** Review je Sprint: `task check:all` und `task check:go` grün, Golden-Diff und Migration geprüft, neue Abhängigkeit belegt (Sprint 2), `git status` ohne eingecheckte Welt oder Chat-Verlauf.

## Offene Fragen

Entschieden von 🧑 am 2026-10-18 (Chat):
- Eine Spec, **zwei Sprints** (Sprint 1 Welten im Spiel, Sprint 2 Chat und Prompt), beide **nach WZ17**.
- Standard-Welt unter `data/worlds/` (`islands.json` entfällt).
- Modell über die Claude Code CLI (`claude`), allgemeiner Chat rechts im Studio mit **assistant-ui** (die neue Abhängigkeit gibt 🧑 vor der Umsetzung frei).
- Alte Spielstände (Version 5) werden auf Welt `standard` migriert.
- Die Welt wählt beim Anlegen eines Raums nur der Dev-Mode.
- Obergrenze und Inhalt weiterer Inseln bleiben offen (Welt-Dateien, von Hand oder per Chat).
- Chat-Verlauf im Ordner `.chats`, Gültigkeit eine Woche.

Nichts Blockierendes offen. Nächster Schritt: Freigabe der Spec durch 🧑, dann Sprint-Entwürfe (Sprint 1 und 2) mit Sessions.

## Notizen

Zwei Sprints mit gemeinsamer Spec (Freigabe 2026-10-18): WZ18 (Welten im Spiel, Feld `Sprint`) und WZ19 (Chat und Prompt, nach WZ18). Das Feld `Sprint` nimmt nur einen Sprint, daher steht WZ19 hier. Beide setzen WZ17 (Chunk-Anzahl je Stufe) voraus.
