# WZ62 · DEV · Studio: feste Welten, Welt-Filter für Katalog und Sound, Größenvergleich mit Lineal, Welten-Editor-Link

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** bereit
- **Tickets:** B-1168, B-1169, B-1170, B-1171, B-1172, B-1173
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 Ralf, 2026-11-20

## Ausgangslage

Das Studio (Workbench, Bereich Assets) zeigt nur die Custom World Nordmeer, kennt keinen Welt-Filter mit Vererbung und hat vier Zoomstufen im Größenvergleich.

## Ziel

Das Studio listet alle vier festen Welten, Katalog und Sound filtern nach Welt (gesamt oder exklusiv), der Größenvergleich ist eine fensterfüllende Ansicht mit dem Größenstufen-Lineal, und der Welten-Editor springt in den Katalog der Welt.

## Beteiligte und Zielgruppen

🧑 nutzt das Studio und nimmt die Seiten am PC ab. Agenten setzen um, die Review-Session prüft.

## Anforderungen

`B-1168` feste Welten, `B-1169` Welt-Filter mit Vererbung, `B-1170` Größenvergleich mit Lineal (nach LA1.1), `B-1171` Link aus dem Welten-Editor (nach B-1169), `B-1172` Welt löschen, `B-1173` Werte-Tabelle benutzerfreundlich (Lane zweit, Modell opus).

## Nicht-Ziele

Neue Welt-Dateien (WE3), Weltwahl im Spiel (B-1165), neue Assets, Review-Seite des Studios.

## Regeln und Einschränkungen

Oberfläche nur über `@webkit` (Atomic Design, Radix nur dort, rohe Controls nur in `atoms/`); Datei ≤ 400 Zeilen, Funktion ≤ 60 Zeilen; Prüfläufe über `check_run`. Der Größenvergleich (WZ62.3) wartet auf LA1.1.

Planungsangaben für die Arbeitswellen:

- **Bereich:** `tools/k3c-workbench/frontend/src/assets/**`, `tools/k3c-workbench/frontend/src/world/**`, `tools/k3c-workbench/internal/world/**`, `tools/k3c-workbench/internal/assetsui/**`, `tools/k3c-workbench/app_world.go`, `tools/k3c-workbench/app_studio.go`, `tools/k3c-workbench/app_assets.go`, `tools/webkit/**`, `tools/assetkit/**`
- **Abhängig von:** –
- **Braucht 🧑:** Abnahme
- **Sperre:** –

## Beispiele

Filter „Welt = Sengoku, exklusiv“ zeigt nur Assets, die für Sengoku gebaut sind; „gesamt“ zeigt zusätzlich die geerbten der Standard-Welt mit Herkunft.

## Ausnahme- und Fehlerfälle

Eine feste Welt ohne Datei erscheint mit Hinweis „noch nicht umgesetzt“; ein Asset ohne Weltzuordnung zählt zur Basis.

## Akzeptanzkriterien

- **AC-01** (B-1168) Die Weltliste des Studios nennt die vier festen Welten und jede Datei aus `data/worlds`.
- **AC-02** (B-1169) Katalog und Sound zeigen alle Assets einer Welt (gesamt) und nur die exklusiven; geerbte tragen die Herkunft.
- **AC-03** (B-1170) Der Größenvergleich ist eine fensterskalierende Ansicht mit Lineal der Größenstufen, ohne Zoomstufen.
- **AC-04** (B-1171) Der Welten-Editor verlinkt in den Katalog der Welt (exklusiv oder gesamt).
- **AC-05** (B-1172) Eine Welt lässt sich im Welten-Editor löschen; feste Welten und Welten mit Erben werden abgelehnt.
- **AC-06** (B-1173) Die Werte-Tabelle im Welten-Editor hat Gruppen, Suche, Filter, Standardwert und Abweichung, Rücksetzen je Zeile und geprüfte Eingabe.

## Offene Fragen

Spec ist ein Entwurf und braucht die Freigabe von 🧑. Sessions sind noch nicht geschrieben.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ62.1 | `WZ62.1-feste-welten.md` | Umsetzung | autonom | fertig |
| WZ62.2 | `WZ62.2-welt-filter-vererbung.md` | Umsetzung | autonom | fertig |
| WZ62.3 | `WZ62.3-groessenvergleich-lineal.md` | Umsetzung | autonom | offen |
| WZ62.4 | `WZ62.4-link-welten-editor-katalog.md` | Umsetzung | autonom | offen |
| WZ62.5 | `WZ62.5-review.md` | Review | autonom | offen |
| WZ62.6 | `WZ62.6-welt-loeschen.md` | Umsetzung | autonom | offen |
| WZ62.7 | `WZ62.7-werte-tabelle.md` | Umsetzung | autonom | offen |

Bei `Reife: Entwurf` genügen Stichpunkte mit den Kriterien in Klammern statt Session-Dateien. Vor dem Aktivieren
wird jede Session als eigene Datei nach `docs/vorlagen/session.md` geschrieben. Keine eigenen Sim- oder
Balance-Sessions vor der Abnahme: Sim, Balance, Gen und `check:race` laufen im Abnahme-Ticket (B-742).

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets und das Abnahme-Ticket für Sim, Balance und Gen
(genau eins, `docs/arbeitsweise.md` › Sim, Balance und Gen). Bis dahin `–`.
