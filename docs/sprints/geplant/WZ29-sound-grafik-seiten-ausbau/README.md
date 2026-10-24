# WZ29 · DEV · Ausbau der Sound- und Grafik-Seite: Hörproben, Vorschauen, Filter, AI-Chat

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-597, B-598, B-599, B-600
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Ralf (2026-10-24): Für Skills gibt es im Studio keine vernünftige Hörprobe, Szenen sind schlecht sortier- und filterbar, und der AI-Chat soll Ton und Grafik unterstützen; dasselbe gilt für die Grafik-Seite. WZ28 (B-594) ergänzt vorher die Slot-Details.

## Ziel

Sound- und Grafik-Seite haben differenzierte Hörproben und Vorschauen (auch Skills und Skill-Effekte), einheitlich filter- und sortierbare Szenen und eine Chat-Unterstützung mit bestätigten Vorschlägen. Am Ende sichtbar: Skill-Szene mit Hörprobe, Skill-Effekte-Szene, Filter und ein Chat-Vorschlag für einen Ton.

## Beteiligte und Zielgruppen

Ralf; Agent DEV; 🧑 nimmt am PC ab.

## Anforderungen

B-597, B-598, B-599, B-600 › Anforderungen.

## Nicht-Ziele

Neue Töne oder Grafiken selbst beschaffen (macht der Chat auf Ralfs Bestätigung), Spiel-Logik, Balancing.

## Regeln und Einschränkungen

Läuft nach WZ28.1 (Slot-Details) und SO8 (Abbildung Skill → Slot); Frontend nur im Studio, Atomic Design über `@webkit`; Zeichnen/Hören mit den Funktionen des Spiels; Chat nur online, Seiten funktionieren offline; keine stille Schreibaktion des Chats.

## Beispiele

Szenen › Skills › Filter „Heiler“ → Hörprobe → Chat: „Ton für passiven Skill regeneration vorschlagen“ → Karte → Übernehmen.

## Ausnahme- und Fehlerfälle

Datei oder Sheet fehlt → Hinweis in der Zeile. Chat offline → „offline“, Seiten bleiben nutzbar.

## Akzeptanzkriterien

- **AC-01** Gemeinsamer Filter-/Sortier-Baustein, beide Seiten nutzen ihn (B-599).
- **AC-02** Sound-Szenen Skills, Gegner/Bosse, Bauen, Reiten, UI, Warnungen, Misch-, Lautheits- und Abstandsprobe (B-597).
- **AC-03** Grafik-Szenen Skill-Effekte, Schilder/Gerüste, Gegner je Biom, Reittiere, Bosse, Biom und Licht wählbar (B-598).
- **AC-04** Der Chat erhält den Seitenkontext, Vorschläge als Karten, Schreiben nur nach „Übernehmen“, Lizenz- und Download-Regeln erzwungen, offline sauber (B-600/AC-01 bis AC-04).
- **AC-05** Ralf hat Szenen, Filter und einen Chat-Vorschlag für Ton und Grafik am PC abgenommen (B-600/AC-05).

## Offene Fragen

keine blockierende; Pflicht-Szenen und Chat-Aktionen laut Tickets als Default, Ralf ergänzt bei der Abnahme. Spec-Freigabe steht aus.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ29.1 | – | Umsetzung | autonom | offen |
| WZ29.2 | – | Umsetzung | autonom | offen |
| WZ29.3 | – | Umsetzung | autonom | offen |
| WZ29.4 | – | Umsetzung | autonom | offen |
| WZ29.5 | – | Umsetzung | autonom | offen |
| WZ29.6 | – | Review | autonom | offen |
| WZ29.7 | – | Workshop | Mensch | offen |

- WZ29.1 (DEV) Filter-/Sortier-Baustein in `@webkit` mit reinen Funktionen und Tests, Einbau in Sound- und Grafik-Szenen (AC-01).
- WZ29.2 (DEV) Sound-Szenen: Skills, Gegner/Bosse, Bauen, Reiten, UI, Warnungen (AC-02).
- WZ29.3 (DEV) Misch-, Lautheits- und Abstandsprobe (AC-02).
- WZ29.4 (DEV) Grafik-Szenen: Skill-Effekte, Schilder/Gerüste, Gegner je Biom, Reittiere, Bosse, Biom und Licht (AC-03).
- WZ29.5 (DEV) Chat-Unterstützung: Kontext, Vorschlagskarten, Bestätigung, Lizenz- und Download-Regeln, offline (AC-04).
- WZ29.6 (DEV) Review mit Offline-Nachweis (alle Kriterien).
- WZ29.7 (Mensch) Abnahme am PC (AC-05).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
