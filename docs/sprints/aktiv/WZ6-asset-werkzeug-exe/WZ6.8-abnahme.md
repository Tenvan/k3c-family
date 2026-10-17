# WZ6.8 · Abnahme des Asset-Studios durch Ralf

- **Status:** offen
- **Typ:** Review
- **Agent:** Mensch
- **Domäne:** DEV
- **Umgebung:** live
- **Branch:** wz6/8-abnahme
- **Abhängig von:** WZ6.7
- **Tickets:** B-459, B-421
- **Kriterien:** AC-08

## Ziel

🧑 nimmt am PC das Asset-Studio ab (AC-08): Grafik- und Sound-Seite, die umgezogene Vorschau, die Workbench ohne Assets-Seite und deren Ruhe beim Neubau des Asset-Studios. Außerdem entscheidet 🧑 über B-421 (`erledigt` mit Verweis auf WZ6.8 oder `verworfen`).

## Kontext

Die Abnahme-Liste schreibt WZ6.7 unten in `### Abnahme-Liste`. Start: `task k3c-studio:build`, dann `bin/k3c-studio.exe` oder der Verweis „Asset-Studio öffnen“ in der Workbench; die Workbench über `task k3c-dev`. **Umgebung live:** Die Workbench läuft, das Asset-Studio ist gebaut, für die Netzsuche liegt ein freesound-Token in `.work/freesound-token` (nur lokal, B-421 › Anforderungen). Claude bereitet vor und trägt das Urteil von 🧑 ein, urteilt aber nicht selbst. Fenster- oder Browser-Prüfungen durch Claude gibt es nur mit Freigabe von 🧑 für diesen Lauf.

**B-421** (Abnahme M10, Projekt ABN) prüft die Workbench-Assets-Seite und `assetvorschau.html`, die es nach WZ6 nicht mehr gibt. Seine Punkte nimmt 🧑 gegen das Asset-Studio ab:

- B-421/AC-01 → Grafik- und Sound-Seite: echte Slots aus `data/assets.json` (Zahl und Lücken aktuell per `assets_list`) und Filter.
- B-421/AC-02 → Ausschnitt-Editor: `hub:3` speichern und in der Vorschau sehen; ein Ausschnitt außerhalb des Bildes wird abgelehnt.
- B-421/AC-03 → Recherche: echte Treffer mit Link, Quelle und Lizenz; sie bleiben nach Neustart des Studios stehen.
- B-421/AC-04 → Import: Datei unter `public/…/kandidaten/`, Credit mit „Lizenz prüfen“, „Lizenz bestätigen“ setzt sie auf bestätigt. Die Testdatei danach nicht einchecken.
- B-421/AC-05 → Die Mini-Szene folgt der gewählten Präferenz und lädt nach dem Speichern neu.
- B-421/AC-06 → Grafik-Seite, Vorschau: alle fünf Grafik-Szenen im Spielmaßstab, Lücken als rote Platzhalter.
- B-421/AC-07 → Sound-Seite: `sfx:coin` und der Wechsel `music:tag` → `music:nacht` auf Knopfdruck (mit Ton).

Laut B-421 › Notizen werden B-298 und B-299 nach erledigter Abnahme ebenfalls `erledigt`.

### Abnahme-Liste

Was 🧑 am PC sieht (Studio: `task k3c-studio:build`, dann `bin/k3c-studio.exe`):

1. **Studio-Fenster (WZ6/AC-01):** EXE startet als eigenes Fenster; kein Port offen (nur Loopback bzw. gar kein Listener). Zweiter Start holt das erste Fenster nach vorn.
2. **Seite Grafik, Slots (B-421/AC-01):** echte Slots aus `data/assets.json` (Zahl wie `assets_list`), Filter Kategorie/Typ/Status/Suche, Lücken sichtbar.
3. **Ausschnitt-Editor (B-421/AC-02):** `hub:3` speichern, in der Vorschau sehen; ein Ausschnitt außerhalb des Bildes wird abgelehnt. Testwert danach zurücknehmen.
4. **Recherche (B-421/AC-03):** echte Treffer mit Link, Quelle und Lizenz; nach Neustart des Studios stehen sie noch.
5. **Import (B-421/AC-04):** Datei unter `public/…/kandidaten/`, Credit „Lizenz prüfen“, „Lizenz bestätigen“ setzt auf bestätigt. Testdatei nicht einchecken.
6. **Mini-Szene (B-421/AC-05):** folgt der gewählten Präferenz und lädt nach dem Speichern neu.
7. **Grafik, Reiter Szenen (B-421/AC-06, AC-06 WZ6):** alle fünf Szenen (Figuren, Gebäude, Umgebung inkl. Hintergründe, Objekte, HUD) im Spielmaßstab, Lücken als rote Platzhalter. Reiter Packs wie `grafiken.html`.
8. **Seite Sound (B-421/AC-07):** `sfx:coin` hörbar; Musikwechsel `music:tag` → `music:nacht` mit Überblendung; Umgebungsschleife; Stopp; Reiter Hörprobe wie `soundtest.html`.
9. **Workbench ohne Assets-Seite (AC-03):** kein Reiter „Assets“; „Asset-Studio öffnen“ startet das Studio; bei fehlender EXE die Meldung „Asset-Studio nicht gebaut (`task k3c-studio:build`)“.
10. **Neubau-Probe (AC-03, aus WZ6.5):** `task k3c-studio:build` bei laufender Workbench; Workbench und MCP bleiben davon unberührt.
11. **Neuladen bei MCP-Änderung (AC-07):** `assets_set` per MCP während das Studio offen ist; Studio lädt neu, Auswahl, Filter und Eingaben bleiben; Speichern auf veraltetem Stand meldet „Register geändert, bitte erneut speichern“.
12. **Entwicklerseite:** keine Kachel „Asset-Vorschau“ mehr; `grafiken.html` und `soundtest.html` laufen unverändert.
13. **Entscheidung B-421** (erledigt mit Verweis auf WZ6.8 samt B-298/B-299, oder verworfen) und Abnahme AC-08.

## Erlaubte Dateien

Nur diese Dateien/Ordner dürfen geändert oder angelegt werden. Alles andere → Ticket anlegen, nicht ändern.

- Planungsdateien von WZ6, B-459 und B-421, bei Abnahme von B-421 auch Status von B-298 und B-299; neue Tickets für Befunde; `docs/sprints/README.md` (Fahrplan). Alles über k3c-dev.

## Nicht-Ziele

Code ändern oder Befunde in dieser Session beheben (→ Ticket); urteilen oder abhaken anstelle von 🧑; über B-421 ohne 🧑 entscheiden; Prüfungen von `grafiken.html` und `soundtest.html` an Xbox oder TV (bleiben bestehen, eigene Sessions); Spec-Freigaben oder Kriterien umformulieren.

## Schritte

1. Claude bereitet vor: `workbench_status` und `svc_status`, `task k3c-studio:build`, Abnahme-Liste und B-421-Punkte bereitstellen.
2. 🧑 geht die Abnahme-Liste am PC durch, einschließlich der B-421-Punkte im Studio und einer in WZ6.5 offen gebliebenen Neubau-Probe.
3. Claude trägt das Urteil von 🧑 je Punkt mit Datum und Quelle ins Ergebnis ein; Befunde werden Tickets.
4. 🧑 entscheidet über B-421: `erledigt` mit Verweis auf das Ergebnis von WZ6.8, danach B-298 und B-299 `erledigt` (B-421 › Notizen), oder `verworfen` mit Grund. Claude setzt das per `plan_set`.
5. Bei Abnahme: B-459 `erledigt`, WZ6 `erledigt` (Ordner nach `erledigt/`), Fahrplan anpassen. Commit `docs(dev): Abnahme WZ6 (WZ6.8)`, `git merge origin/develop`, push auf `sprint/wz6`. Ist der Sprint-PR schon gemergt, bekommt dieser Commit einen eigenen PR gegen `develop`.

## Fertig, wenn

- [ ] AC-08: Urteil von 🧑 je Punkt der Abnahme-Liste im Ergebnis, mit Datum und Quelle, einschließlich B-421/AC-01–AC-07 gegen das Asset-Studio.
- [ ] B-421 ist von 🧑 entschieden: `erledigt` mit Verweis auf WZ6.8 (B-298, B-299 ebenfalls `erledigt`) oder `verworfen` mit Grund.
- [ ] Bei Abnahme: B-459 und WZ6 `erledigt`; `check_run task:test` mit `pattern: planning` grün.

## Prüfen

```bash
task test -- planning
```

Manuelle Prüfungen am PC (Asset-Studio- und Workbench-Fenster) macht 🧑. Claude prüft im Fenster oder Browser nur, wenn 🧑 das für diesen Lauf freigegeben hat.

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
