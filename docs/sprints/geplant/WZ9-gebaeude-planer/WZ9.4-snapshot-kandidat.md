# WZ9.4 · Snapshot-Option Kandidaten-hub.json

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** SRV
- **Umgebung:** offline
- **Branch:** wz9/4-snapshot-kandidat
- **Abhängig von:** WZ9.3, WZ7.1 (WZ8.4 nur, falls schon erledigt)
- **Tickets:** B-462
- **Kriterien:** AC-04, AC-09, AC-10, AC-11

## Ziel

Die Snapshot-Funktion aus WZ7.1 nimmt eine Kandidaten-`hub.json` als Option, prüft sie mit der Funktion aus WZ9.3 und baut den Snapshot daraus, ohne EXE-Neubau und ohne globalen Zustand.

## Kontext

- Voraussetzung: WZ7.1 (Snapshot-Funktion mit Options-Struktur) und WZ9.3 (`ValidateHub`, `sim.CreateOptions` mit Kandidat). Ist WZ7.1 nicht fertig, Session `blockiert`. Fehlt die Options-Struktur oder hat sie globalen Zustand, ein Ticket für WZ7 anlegen, nicht in WZ9.4 umbauen.
- Ist zugleich „geplante Chunks“ (WZ8.4) gesetzt, gelten die Kandidaten-Linien und die Planung wird gegen sie geprüft, nicht gegen die eingebetteten (Beispiel: Linie 5 bei 126 macht die Ader bei Hub +136 ungültig).
- `engine/room` importiert nichts aus `engine/net`. Ohne Option gilt die eingebettete `hub.json`; gleiche Eingabe ergibt byte-gleichen Snapshot.

## Erlaubte Dateien

- `engine/room/` (Snapshot-Funktion, Options-Struktur, Tests)
- `engine/net/` nur, wenn die Option ins Protokoll der Studio-Anbindung muss (dann Protokoll-Regeln aus `arbeitsweise.md` beachten, sonst Ticket)
- `testdata/hub/` (Fixture nur lesen)

## Nicht-Ziele

Kein Studio-Code (WZ9.5), keine Änderung an `engine/sim` über WZ9.3 hinaus, keine Änderung an `data/hub.json` oder `testdata/golden/`, kein globaler Zustand, kein Umbau von WZ7.1.

## Schritte

1. Stand von WZ7.1, WZ8.4 und WZ9.3 prüfen; fehlt WZ7.1, `blockiert`.
2. Option „Kandidaten-hub.json“ in die Options-Struktur der Snapshot-Funktion aufnehmen.
3. Mit gesetzter Option: `ValidateHub` aufrufen, bei Befunden Fehler mit Liste zurückgeben; sonst Bau über `sim.CreateOptions`.
4. Vorrang: Option „geplante Chunks“ zusammen mit Kandidat → Kandidaten-Linien.
5. Go-Tests: Determinismus (zweimal byte-gleich), Kandidat (Rüstkammer ab Stufe 3) gegen eingebettet, ungültiger Kandidat, Vorrang, 7-Stufen-Fixture.
6. `task check:go`.

## Fertig, wenn

- [ ] AC-11: Option, Prüfung, Vorrang und Tests wie oben grün.
- [ ] AC-04: Snapshot mit Kandidat zeigt die Rüstkammer in Hub-Stufe 3, ohne Option nicht.
- [ ] AC-09: Test mit der 7-Stufen-Fixture grün.
- [ ] AC-10: `data/hub.json` und `testdata/golden/` unverändert.
- [ ] `task check:go` grün.

## Prüfen

```bash
task check:go
git diff --stat origin/develop -- data/hub.json testdata/golden
```

Unter k3c-dev: `check_run` mit `checkout`. Keine manuelle Prüfung.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
