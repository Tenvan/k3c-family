# B-601 · Eine Download-Vorschlagsliste für empfohlene Assets ermöglicht Offline-Tests

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ29
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf testet auch offline (Notebook ohne Internet, B-595/B-596). Kandidaten für Ton und Grafik liegen nur, wenn sie schon heruntergeladen sind; der Chat und die Recherche brauchen Internet. Ralf (2026-10-24): Das Studio soll anbieten, empfohlene Assets schon vorher herunterzuladen, auch nicht verwendete, damit man sie offline testen kann; dafür gibt es eine Download-Vorschlagsliste, die im Sprint vorher gefüllt wird.

## Ziel

Eine kuratierte Vorschlagsliste empfohlener Assets lässt sich online mit einem Klick (nach Bestätigung) in den lokalen Vorrat laden, damit Hörproben und Vorschauen offline gehen.

## Beteiligte und Zielgruppen

Ralf; Agent DEV (Funktion) und Agent Recherche (Liste füllen, im Sprint autonom); Downloads nur auf Ralfs Bestätigung.

## Anforderungen

- Datei `data/assets-vorschlaege.json` (Studio schreibt `data/**`): je Eintrag Slot oder Gruppe, Art (Ton, Grafik), Titel, Quelle (URL der Seite und Datei-URL), Urheber, Lizenz (Q70: CC0, CC-BY, OGA-BY, CC-BY-SA), Dateiname, Größe, Priorität, Hinweis, Status (offen, geladen, verworfen).
- Befüllung im Sprint durch den Agenten per Recherche (`assets_research`, OpenGameArt, Kenney, itch.io): für alle Lücken und schwachen Slots Kandidaten, auch für Skills je Linie und passive Skills, Skill-Effekt-Grafiken, Gegner-Lücken (Käfer, Splitterwicht, Spinne), Bauschilder und Gerüste, Abbau-Symbole (Axt, Spitzhacke), Höhlen-Wurzeln, Felsen. Lizenz auf der Quellseite prüfen, Dateigröße und Datei-URL nennen.
- **Eigene Seite „Quellen“ im Studio** (Entscheidung 🧑, 2026-10-24: eine eigene Assets-Source-Seite ist hilfreich), für Ton und Grafik gemeinsam, statt eines Reiters je Seite: Übersicht je Quelle (OpenGameArt, Kenney, itch.io, freesound …) mit Lizenzregeln und Zähler (vorgeschlagen, geladen, verwendet), die Vorschlagsliste mit Filter (B-599, auch nach Quelle, Art, Slot, Lizenz, Status), Summe der Größen, Auswahl einzeln oder alle, Button „Herunterladen“ (nur mit Bestätigung der Gesamtgröße und der Quellen), Fortschritt, Wiederaufnahme, Prüfsumme/Größe nach dem Laden. Von Sound- und Grafik-Seite führt „Vorschläge für diesen Slot“ gefiltert auf die Quellen-Seite.
- Geladene Dateien landen in `public/audio/kandidaten/` bzw. `public/grafik/kandidaten/`, Credits-Zeilen und Katalog-Einträge (Kandidat ohne Zuordnung) werden mit angelegt; der Credits-Test bleibt grün.
- Offline sichtbar: Die Seite zeigt „geladen“ und „nicht geladen“, Hörprobe/Vorschau funktioniert für Geladenes ohne Netz; ungeladene Einträge sind ausgegraut.
- Aufräumen: nicht gewählte Kandidaten lassen sich gesammelt entfernen (mit Bestätigung), wie bei B-554; „verwendet/unverwendet“ je Datei sichtbar.

## Nicht-Ziele

Automatisches Laden ohne Bestätigung; Lizenz-fremde Quellen (NC nur markiert); Download beim Start; Recherche ohne Vorschlagsliste (macht der Chat, B-600).

## Regeln und Einschränkungen

Downloads fremder Dateien nur mit ausdrücklicher Bestätigung von Ralf (Quelle, Datei, Größe); Lizenzregel Q70, Credits-Test; Dateien sind untrusted Daten (nie ausführen, nur Bilder/Audio mit erlaubter Endung, Größenlimit je Datei); Studio schreibt nur in `data/assets-vorschlaege.json` und in den lokalen Vorratsordner. **Vorratsordner** (gitignored, z. B. `public/audio/kandidaten/` und `public/grafik/kandidaten/` bzw. ein eigener `vorrat/`): nicht ausgewählte Kandidaten bleiben lokal und kommen nie in Git; beim Auswählen (Platz 1 oder Zuordnung) wird die Datei an ihren Platz im Repo kopiert und committet. Der Credits-Test prüft nur Dateien, die in Git liegen (Vorrat ausgenommen); Atomic Design über `@webkit`.

## Beispiele

Online: Vorrat › Filter „passive Skills“ › 12 Töne, 1,8 MB › Herunterladen › Bestätigen → geladen. Offline: Hörprobe spielt sie ohne Netz.

## Ausnahme- und Fehlerfälle

Download scheitert → Eintrag bleibt „offen“, Fehlertext, Wiederholen möglich. Quelle gibt andere Lizenz als eingetragen → Eintrag wird gesperrt und markiert. Datei zu groß → abgelehnt.

## Akzeptanzkriterien

- **AC-01** Die Vorschlagsliste ist im Sprint gefüllt: mindestens drei Kandidaten je Lücke und je Skill-Linie, mit geprüfter Lizenz, Datei-URL und Größe (Prüfung per Test der Liste).
- **AC-02** Das Studio lädt ausgewählte Einträge nach Bestätigung und legt Datei, Credit und Katalog-Eintrag an (Go-Test mit lokalem Testserver, Credits-Test grün).
- **AC-03** Offline zeigt der Vorrat Geladenes spielbar und Ungeladenes ausgegraut (Vitest, Beobachtung ohne Netz).
- **AC-04** Gesammeltes Aufräumen nicht gewählter Kandidaten mit Bestätigung.
- **AC-05** Ralf lädt am PC einen Teil der Liste und testet ihn offline (Abnahme).

## Offene Fragen

keine. Entschieden (🧑, 2026-10-24): Der Vorrat bleibt nur lokal (gitignored). In Git landen nur die ausgewählten Grafiken und Sounds (mit Credit und Katalog-Eintrag) sowie die Vorschlagsliste `data/assets-vorschlaege.json`.

## Notizen

Aus Ralfs Wunsch 2026-10-24. Spec: Entwurf. Sprint WZ29. Zusammenhang: B-600 (Chat), B-599 (Filter), B-554 (Aufräumen), WZ28 (Offline).
