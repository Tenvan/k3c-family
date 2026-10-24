# B-596 · Eine Balancing-Seite fährt Simulationen und Berichte ohne Internet und ohne Claude

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ28
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Balancing-Läufe starten bisher über Claude und MCP (`sim_test`, `sim_run`, `internal/balance` in k3c-workbench). Ralf (2026-10-24): Er braucht eine eigene Simulations- und Balancing-Seite, die komplett ohne Internet und Claude Läufe mit Berichten fährt, auch auf dem Notebook.

## Ziel

Eine Seite führt Balancing-Simulationen aus und zeigt Berichte, ohne dass Claude oder Internet nötig sind.

## Beteiligte und Zielgruppen

Ralf (Balancing und Spieleabende, Projekt BAL); Agent DEV.

## Anforderungen

- Neue Seite „Balancing“ in k3c-workbench (oder Studio, je nach Spec), Backend nutzt dieselbe Engine und dieselben Läufe wie `sim_test`/`sim_run` (`internal/balance`, `engine/sim`).
- Lauf konfigurieren: Biom/Welt, Seeds oder Anzahl, Bot-Profil, Grad, Spieleranzahl, Tiefe, Dauer; gespeicherte Voreinstellungen.
- Start, Fortschritt live, Abbruch; mehrere Läufe nacheinander; Läufe gehen im Hintergrund weiter.
- Bericht je Lauf: Zielkorridor-Prüfung (Pass/Fail), Kennzahlen (Holz je Nacht, Hub-Stufen-Zeit, Verluste, Gold), Diagramme, Zeitreihen, Abweichung zu den Zielen aus `balance-targets.json`.
- Berichte speichern (Ordner `reports/` oder `.work/balance`), Liste, Vergleich zweier Läufe, Export als Markdown und JSON.
- Werte-Varianten: einen Wert aus `data/` überschreiben (nur im Lauf, ohne Datei zu ändern) und Läufe vergleichen.
- Läuft offline: keine externen Schriften, Skripte oder API-Aufrufe.

## Nicht-Ziele

Neue Balancing-Regeln und Zielkorridore (BAL4–BAL7); Client-Lasttests mit Browsern (`k3c-load`, TST).

## Regeln und Einschränkungen

Testläufe nur über denselben Weg wie `sim_test` (B-348), Engine deterministisch (`engine/rng`); Backend `tools/k3c-workbench` bindet die Engine als Bibliothek; Frontend Atomic Design über `@webkit`; keine Änderung an `data/`-Dateien durch Varianten; Laufzeit kann Minuten dauern, Seite blockiert nicht.

## Beispiele

20 Seeds Wald, Profil „Mauern zuerst“, 3 Spieler → Bericht mit Korridor-Prüfung, Holz je Nacht und Diagramm; Vergleich mit dem Lauf von gestern.

## Ausnahme- und Fehlerfälle

Lauf bricht ab → Teilbericht mit Fehlermeldung. Ziele fehlen → Bericht ohne Prüfung mit Hinweis.

## Akzeptanzkriterien

- **AC-01** Das Backend startet, zeigt und bricht Balancing-Läufe ab, ohne MCP (Go-Test).
- **AC-02** Die Seite konfiguriert Läufe und zeigt den Fortschritt live (Vitest, Beobachtung).
- **AC-03** Berichte enthalten Korridor-Prüfung, Kennzahlen und Diagramme; sie lassen sich speichern, vergleichen und exportieren.
- **AC-04** Wert-Varianten im Lauf, ohne Dateien zu ändern (Go-Test).
- **AC-05** Ohne Netzwerk läuft alles (Test, Beobachtung am Notebook).

## Offene Fragen

Welche Kennzahlen im Bericht Pflicht sind: Defaults aus `docs/rules/zielkorridore.md` und `balance-targets.json`; 🧑 ergänzt bei der Abnahme. Seite in Workbench (Default) oder Studio?

## Notizen

Aus Ralfs Wunsch 2026-10-24. Spec: Entwurf. Sprint WZ28. Zusammenhang: B-589 (BAL7), B-348.
