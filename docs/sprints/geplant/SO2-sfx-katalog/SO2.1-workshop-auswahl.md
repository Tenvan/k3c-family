# SO2.1 · Workshop: Sounds je Ereignis hören und wählen, Katalog anlegen

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Branch:** so2/1-workshop-auswahl
- **Abhängig von:** SO1.4, SO3.2
- **Tickets:** B-167
- **Kriterien:** AC-01

## Ziel

🧑 hat je Ereignis einen Kandidaten gewählt oder „Lücke“ bestätigt; `docs/assets/sounds.md` listet jedes Ereignis mit dieser Entscheidung, Quelle und Lizenz.

## Kontext

- **Schon entschieden (nicht neu verhandeln):** Q15 (`docs/fragenkatalog.md` › Beschlüsse vom 2026-10-03): Stil **Retro/Chiptune, kindgerecht, Nacht leise**; Quellen **Kenney, OpenGameArt, freesound** (nur CC0 oder CC-BY), selbst erzeugte Retro-SFX per Web Audio als Lückenfüller. Q08 nennt zwölf Pflicht-Ereignisse: Treffer, Kill, Münze auf/gegeben, Pfeil, Schlag, Bau-Fortschritt/fertig, Tod, Wiederbeleben, Skill, Nacht naht, Portal.
- **Ereignisliste für den Katalog** (B-167 › Anforderungen): Münze aufheben und geben, Schlag, Pfeil, Treffer, Gegner-Tod, Bauen und Fertig, Hub-Ausbau, Nacht naht, Portal öffnet, Tod und Wiederbeleben, Skill je Klasse, Boss-Auftritt, UI-Klicks. Die zwölf aus Q08 sind für den Spieleabend-Build Pflicht; **ob Hub-Ausbau, Skill je Klasse, Boss-Auftritt und UI-Klicks beschafft werden oder vorerst Lücke mit Ticket bleiben, entscheidet 🧑 im Workshop** (Q08 und Q15 sagen dazu nichts). Zuordnung zu den Protokoll-Events: `docs/protocol.md` › Ereignisse (`hit`, `kill`, `arrow`, `strike`, `coinPickup`, `coinGive`, `buildProgress`, `revive`; Tod, Bau fertig, Skill, Nacht naht, Portal über `playerDown`, `built`, `skillPoint`, `dusk`, `arrived`).
- **Offen für 🧑 (nichts vorwegnehmen):** welcher Kandidat je Ereignis klingt gut und kindgerecht; Dauer etwa 30 bis 45 Minuten. Die Auswahl erfolgt auf `soundtest.html` (SO3.2: Kandidatenliste aus `public/audio/kandidaten.json`, Abspielen über den Audio-Kern aus SO1); der Agent läuft mit und trägt die Entscheidungen ein (wie GR2.2, F1.4).
- **Vorbereitung durch den Agenten (diese Datei verlangt sie):** vor dem Termin je Ereignis höchstens drei Kandidaten aus den Quellen von Q15 in `public/audio/kandidaten.json` eintragen (Gruppe „Ereignis“, Name, Datei, Quelle, Urheber, Lizenz; Format der Datei legt SO3.2 fest) und die Dateien klein halten (kurze SFX, Format laut SO1.2/B-166). Nur CC0 oder CC-BY. Download nur von der verlinkten Quelle, mit Lizenzangabe; was nicht auffindbar ist, wird als Lücke geführt (Füller per Web Audio erst in SO2.2). Vorbild der Tabelle: `docs/assets/zuordnung.md` (B-161, GR1).
- **Katalog-Spalten** (B-167): Ereignis, Sound-Datei, Quelle, Urheber, Lizenz, Status (`zugeordnet` oder fett markierte Lücke mit Ticket).

## Erlaubte Dateien

- `docs/assets/sounds.md` (neu)
- `public/audio/kandidaten.json` und `public/audio/kandidaten/` (nur Kandidaten-Einträge und -Dateien)
- diese Datei (Ergebnis), Sprint-README (nur Tabelle der Sessions), `docs/backlog/` (nur Status und neue Tickets für Lücken)

## Nicht-Ziele

Einbinden ins Spiel (SO2.3), Credits-Datei und Aufräumen nicht gewählter Kandidaten (SO2.2), Musik (SO4), eigene Kompositionen.

## Schritte

1. Vorbereitung (Agent): Kandidaten eintragen, Katalog-Gerüst `docs/assets/sounds.md` mit allen Ereignissen anlegen (Status zunächst `offen`).
2. Termin: 🧑 hört je Ereignis auf `soundtest.html` und wählt einen Kandidaten oder bestätigt „Lücke“; bei Q08-Ereignissen ohne Treffer ist „Lücke, Füller per Web Audio“ zulässig.
3. Agent trägt die Entscheidungen mit Datum in den Katalog ein; Lücken fett mit Ticket (neue Tickets nach `docs/vorlagen/ticket.md`, Index in `docs/backlog/README.md`).
4. Ergebnis eintragen (Zahl gewählt, Zahl Lücken, Besonderheiten), `Status: fertig`.

## Fertig, wenn

- [ ] AC-01: `docs/assets/sounds.md` nennt jedes Ereignis der Liste mit gewähltem Sound samt Quelle, Urheber und Lizenz oder als Lücke mit Ticket, bestätigt von 🧑 mit Datum.

## Prüfen

Manuell durch 🧑 (Browser, `soundtest.html`, möglichst am TV).

## Ergebnis

–
