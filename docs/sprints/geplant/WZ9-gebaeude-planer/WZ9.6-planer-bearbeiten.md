# WZ9.6 · Gebäude-Planer: Bearbeiten, Prüfen, Schreiben

- **Status:** offen
- **Typ:** Umsetzung
- **Agent:** autonom
- **Domäne:** DEV
- **Umgebung:** offline
- **Branch:** wz9/6-planer-bearbeiten
- **Abhängig von:** WZ9.5
- **Tickets:** B-462
- **Kriterien:** AC-02, AC-03, AC-05, AC-07, AC-10

## Ziel

Der Gebäude-Planer bearbeitet die Freischaltung (Stufe, Platz, Tiefe, `needsDeeper`, `gateHubLevel`, Händler-Zahlziele, `wallUnits`, Plätze bekannter Arten hinzufügen und entfernen), prüft sie über das Go-Backend (Funktion aus WZ9.3), schreibt `data/hub.json` und danach den erzeugten Block der Regeltabelle atomar und format-stabil, mit Schreibregel bei extern geänderten Dateien und dem Hinweis auf REG-Änderung und Goldens.

## Kontext

- Baut auf WZ9.5 (Matrix, Kandidat, Live-Vorschau) und der Schreibregel aus B-459/AC-07 (atomar, bei extern geänderter Datei nicht überschreiben).
- Das Speichern im Planer ist eine REG-Änderung durch 🧑 außerhalb des Sprints: Kein Sprint-Commit enthält eine geänderte `data/hub.json` oder Regeltabelle; Tests arbeiten auf Kopien (temporäres Verzeichnis).
- Prüfung ausschließlich über die Go-Funktion aus WZ9.3 (eine Quelle der Regeln), nicht im Frontend nachbauen. Format-stabil: Schlüsselreihenfolge, Einrückung und Zahlenformat der Datei bleiben, Diff nur in geänderten Zeilen.
- Das Ersetzen des Blocks zwischen den Markern kommt aus `tools/regeltabelle/tabelle` (Funktion aus WZ9.2), nicht aus `main`; das Studio baut es nicht nach.
- Beispiel: Rüstkammer auf Stufe 3 → Live-Vorschau zeigt sie sofort → Speichern → zwei Diffs: eine Zeile in `data/hub.json` und der neu erzeugte Block in § 3; Hinweis „REG-Änderung, Goldens prüfen“. Die Meldung „Regeltabelle veraltet“ erscheint nur, wenn das Schreiben des Blocks scheitert.

## Erlaubte Dateien

- `tools/k3c-studio/` (Backend, `frontend/`, Tests)
- `tools/assetkit/` nur, wenn die gemeinsame Schreibregel dort liegt (Änderung abwärtskompatibel)
- Lesen: `tools/regeltabelle/tabelle/`; geschrieben wird `docs/rules/materialien-gebaeude.md` nur zur Laufzeit des Studios und in Tests nur als Kopie in `t.TempDir()`

## Nicht-Ziele

Keine echte Änderung an `data/hub.json`, der Regeltabelle oder `testdata/golden/` im Sprint-Commit. Keine Werte oder Kosten (`data/buildings.json`, `levels`), keine neuen Gebäudearten, kein `task golden:update`. Das Schreiben des Blocks zur Laufzeit gehört dagegen zu dieser Session.

## Schritte

1. Frontend: Zellen der Matrix bearbeiten (`hubLevel`, `offsetUnits`, `fromDepth`, `needsDeeper` je Platz; `gateHubLevel`; `wallUnits[k]`; Händler `buyOffsetUnits`, `sellOffsetUnits`, `onlyDepth`), Plätze bekannter Arten hinzufügen und entfernen (Hinweis, dass alte Spielstände den entfernten Platz verlieren); Änderung als Kandidat führen, Live-Vorschau folgt.
2. Backend: Kandidat mit der Funktion aus WZ9.3 prüfen, Befunde (inklusive geplanter Chunks, sofern WZ8) anzeigen; Speichern nur ohne Befund.
3. Backend: atomares, format-stabiles Schreiben nach `data/hub.json`, danach den erzeugten Block (Funktionen aus `tools/regeltabelle/tabelle`) zwischen den Markern in `docs/rules/materialien-gebaeude.md` § 3, ebenfalls atomar; Text außerhalb der Marker bleibt byte-gleich. Scheitert der zweite Schritt, bleibt `data/hub.json` gespeichert, Meldung „Regeltabelle veraltet, `task rules:update`“.
4. Schreibregel: Hash beider Dateien beim Laden merken; hat sich eine seither geändert, nicht schreiben, Neuladen anbieten, Eingaben behalten.
5. Hinweis nach dem Speichern: „REG-Änderung: Commit mit Begründung (`data/hub.json` und Regeltabelle), `task golden:update` nur bei gewollter Änderung“.
6. Tests auf Kopien: Rüstkammer-Beispiel mit Ein-Zeilen-Diff in `data/hub.json`, Platz hinzufügen und entfernen, `gateHubLevel`, Händler und `wallUnits` ändern, extern geänderte Datei (je Datei), ungültiger Kandidat, atomares Schreiben, Fehler im Tabellenschritt.
7. `data/hub.json`, Regeltabelle und `testdata/golden/` per `git diff --stat` unverändert bestätigen.

## Fertig, wenn

- [ ] AC-02: Speichern nur nach Prüfung über die Go-Funktion, auch für hinzugefügte und entfernte Plätze, `gateHubLevel`, Händler und `wallUnits` (Test).
- [ ] AC-03: Beim Speichern wird der erzeugte Block geschrieben, Text außerhalb der Marker bleibt byte-gleich, Fehler im zweiten Schritt meldet „Regeltabelle veraltet“ (Tests auf Kopien).
- [ ] AC-05: Hinweis auf REG-Änderung und Goldens nach dem Speichern (Test).
- [ ] AC-07: Format stabil (Ein-Zeilen-Diff), atomar, Schreibregel bei extern geänderter Datei (`data/hub.json` und Regeltabelle; Tests auf Kopien).
- [ ] AC-10: `git diff --stat origin/develop -- data/hub.json testdata/golden` leer.
- [ ] Prüfläufe grün.

## Prüfen

```bash
task check
task check:go
git diff --stat origin/develop -- data/hub.json testdata/golden
```

Unter k3c-dev: `check_run` mit `checkout`. Keine manuelle Prüfung; die gehört 🧑 (WZ9.8).

## Ergebnis

Wird am Ende der Session ausgefüllt: Nachweis je Kriterium (`AC-01 geprüft: task check grün`,
`AC-02 verschoben: Grund, B-0NN`), wer manuell geprüft hat, Abweichungen vom Plan, neue Tickets. Bis dahin `–`.
