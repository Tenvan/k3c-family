# WZ29 · DEV · Ausbau der Sound- und Grafik-Seite: Hörproben, Vorschauen, Filter, AI-Chat

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-597, B-598, B-599, B-600, B-601, B-602
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 2
- **Freigabe:** autonom, Claude, 2026-10-24, Revision 2 (Abhängigkeit SO8.2 statt SO8; Ziel von Ralf: WZG komplett umsetzen, soweit autonom möglich; Revision 1: 2026-10-24, Ralf im Chat)

## Ausgangslage

Ralf (2026-10-24): Für Skills gibt es im Studio keine vernünftige Hörprobe, Szenen sind schlecht sortier- und filterbar, und der AI-Chat soll Ton und Grafik unterstützen; dasselbe gilt für die Grafik-Seite. WZ28 (B-594) ergänzt vorher die Slot-Details.

## Ziel

Sound- und Grafik-Seite haben differenzierte Hörproben und Vorschauen (auch Skills und Skill-Effekte), einheitlich filter- und sortierbare Szenen, eine Chat-Unterstützung mit bestätigten Vorschlägen und einen Offline-Vorrat: eine gefüllte Download-Vorschlagsliste für Ton und Grafik, aus der Ralf online empfohlene Assets vorab lädt. Am Ende sichtbar: Skill-Szene mit Hörprobe, Skill-Effekte-Szene, Filter, ein Chat-Vorschlag und ein Offline-Test mit geladenem Vorrat.

## Beteiligte und Zielgruppen

Ralf; Agent DEV; 🧑 nimmt am PC ab.

## Anforderungen

B-597, B-598, B-599, B-600, B-601, B-602 › Anforderungen.

## Nicht-Ziele

Neue Töne oder Grafiken selbst beschaffen (macht der Chat auf Ralfs Bestätigung), Spiel-Logik, Balancing.

## Regeln und Einschränkungen

**Abhängigkeiten (aktivieren erst danach):** WZ28.1 (Slot-Details, B-594, erledigt) und SO8.2 (Abbildung Skill → Slot im Client, Sprint SO8 aus Projekt SND). Die Hörprobe SO8.4 mit Ralf ist keine Voraussetzung (Revision 2: vorher „SO8 erledigt“, das hätte WZ29 an eine Mensch-Session gebunden). Frontend nur im Studio, Atomic Design über `@webkit`; Zeichnen und Hören mit den Funktionen des Spiels, keine zweite Darstellung; Chat und Downloads nur online, alle Seiten funktionieren offline; keine stille Schreibaktion des Chats; der Vorrat bleibt in `.work/vorrat/` (gitignored), in Git landen nur gewählte Dateien mit Credit und Katalog-Eintrag (B-601).

## Beispiele

Szenen › Skills › Filter „Heiler“ → Hörprobe → Chat: „Ton für passiven Skill regeneration vorschlagen“ → Karte → Übernehmen.

## Ausnahme- und Fehlerfälle

Datei oder Sheet fehlt → Hinweis in der Zeile. Chat offline → „offline“, Seiten bleiben nutzbar.

## Akzeptanzkriterien

- **AC-01** Gemeinsamer Filter-/Sortier-Baustein, beide Seiten nutzen ihn (B-599).
- **AC-02** Sound-Szenen Skills, Gegner/Bosse, Bauen, Reiten, UI, Warnungen, Misch-, Lautheits- und Abstandsprobe (B-597).
- **AC-03** Grafik-Szenen Skill-Effekte, Schilder/Gerüste, Gegner je Biom, Reittiere, Bosse, Biom und Licht wählbar; dazu die Szene „Kampf-Anzeigen“ mit Mock-Zuständen für Boss-Leiste, Phasen, Warnkreis, Event-Banner und Mondanzeige (B-598, B-602).
- **AC-04** Der Chat erhält den Seitenkontext, Vorschläge als Karten, Schreiben nur nach „Übernehmen“, Lizenz- und Download-Regeln erzwungen, offline sauber (B-600/AC-01 bis AC-04).
- **AC-05** Die Download-Vorschlagsliste für Ton und Grafik ist gefüllt (mindestens drei Kandidaten je Lücke und Skill-Linie, Lizenz geprüft), das Studio lädt Auswahl nach Bestätigung, legt Credit und Katalog an, räumt auf und zeigt Geladenes offline spielbar (B-601/AC-01 bis AC-04).
- **AC-06** Ralf hat Szenen, Filter, einen Chat-Vorschlag, den Offline-Vorrat und die Kampf-Anzeigen am PC abgenommen (B-600/AC-05, B-601/AC-05, B-602/AC-03).

## Offene Fragen

keine blockierende; Pflicht-Szenen und Chat-Aktionen laut Tickets als Default, Ralf ergänzt bei der Abnahme.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ29.1 | `WZ29.1-filter-baustein.md` | Umsetzung | autonom | offen |
| WZ29.2 | `WZ29.2-sound-szenen.md` | Umsetzung | autonom | offen |
| WZ29.3 | `WZ29.3-misch-lautheit-abstand.md` | Umsetzung | autonom | offen |
| WZ29.4 | `WZ29.4-grafik-szenen.md` | Umsetzung | autonom | offen |
| WZ29.5 | `WZ29.5-vorschlagsliste.md` | Umsetzung | autonom | offen |
| WZ29.6 | `WZ29.6-quellen-seite.md` | Umsetzung | autonom | offen |
| WZ29.7 | `WZ29.7-chat-zuordnung.md` | Umsetzung | autonom | offen |
| WZ29.8 | `WZ29.8-review.md` | Review | autonom | offen |
| WZ29.9 | `WZ29.9-abnahme-pc.md` | Workshop | Mensch | offen |

- WZ29.1 (DEV) Filter-/Sortier-Baustein in `@webkit` mit reinen Funktionen und Tests, Einbau in Sound- und Grafik-Szenen (AC-01).
- WZ29.2 (DEV) Sound-Szenen: Skills, Gegner/Bosse, Bauen, Reiten, UI, Warnungen (AC-02).
- WZ29.3 (DEV) Misch-, Lautheits- und Abstandsprobe (AC-02).
- WZ29.4 (DEV) Grafik-Szenen: Skill-Effekte, Schilder/Gerüste, Gegner je Biom, Reittiere, Bosse, Biom und Licht (AC-03).
- WZ29.5 (DEV) **Vorschlagsliste füllen (vor der Download-Funktion):** Recherche für Ton und Grafik, `data/assets-vorschlaege.json` mit Quelle, Datei-URL, Lizenz, Größe, Priorität; Lizenzen auf den Quellseiten prüfen; Test der Liste (AC-05, B-601/AC-01).
- WZ29.6 (DEV) **Eigene Seite „Quellen“** im Studio für Ton und Grafik: Quellen-Übersicht, Vorschlagsliste mit Filter, Download mit Bestätigung, Credit und Katalog-Eintrag, Offline-Anzeige, Aufräumen, Sprung von Sound- und Grafik-Seite (AC-05).
- WZ29.7 (DEV) Chat-Unterstützung: Kontext, Vorschlagskarten, Bestätigung, Lizenz- und Download-Regeln, offline; nutzt die Vorschlagsliste (AC-04).
- WZ29.8 (DEV) Review mit Offline-Nachweis (alle Kriterien).
- WZ29.9 (Mensch) Abnahme am PC (AC-06).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
