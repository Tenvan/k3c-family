# B-596 · Eine Balancing-Seite fährt Simulationen und Berichte ohne Internet und ohne Claude

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** eingeplant
- **Sprint:** WZ28
- **Projekt:** WZG
- **Erstellt:** 2026-10-24
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-24, Ralf im Chat (mit WZ28 Revision 1)

## Ausgangslage

Balancing-Läufe starten bisher über Claude und MCP (`sim_test`, `sim_run`, `internal/balance` in k3c-workbench). Ralf (2026-10-24): Er braucht eine eigene Simulations- und Balancing-Seite, die komplett ohne Internet und Claude Läufe mit Berichten fährt, auch auf dem Notebook.

## Ziel

Eine Seite führt Balancing-Simulationen für Kampf und Wirtschaft aus und zeigt Berichte, ohne dass Claude oder Internet nötig sind; Werte lassen sich automatisch gegen die Zielkorridore abstimmen.

## Beteiligte und Zielgruppen

Ralf (Balancing und Spieleabende, Projekt BAL); Agent DEV.

## Anforderungen

- Neue Seite „Balancing“ in k3c-workbench (Entscheidung 🧑, 2026-10-24); Backend nutzt dieselbe Engine und dieselben Läufe wie `sim_test`/`sim_run` (`internal/balance`, `engine/sim`).
- Lauf konfigurieren: Biom/Welt, Seeds oder Anzahl, Bot-Profil, Grad, Spieleranzahl, Tiefe, Dauer; gespeicherte Voreinstellungen.
- Start, Fortschritt live, Abbruch; mehrere Läufe nacheinander in einer Warteschlange; Läufe gehen im Hintergrund weiter.
- Bericht je Lauf: Zielkorridor-Prüfung (Pass/Fail), Kennzahlen, Diagramme, Zeitreihen, Abweichung zu den Zielen aus `balance-targets.json`. Kennzahlen: die vorhandenen (Holz je Nacht, Hub-Stufen-Zeit, Verluste, Gold) plus neue, wo sie helfen (z. B. Rohstoff-Fluss je Rohstoff und Nacht, Zeit bis zur ersten Mauer/Werkstatt, Gold-Bilanz je Phase, Überlebensquote je Nacht und Grad, Anteil Wartezeit der Bauern). Der Agent legt sie nach Bedarf an und dokumentiert sie in `docs/rules/zielkorridore.md`.
- Berichte speichern (Ordner `reports/` oder `.work/balance`), Liste, Vergleich zweier Läufe, Export als Markdown und JSON.
- Werte-Varianten: einen Wert aus `data/` überschreiben (nur im Lauf, ohne Datei zu ändern) und Läufe vergleichen.
- **Automatisches Balancing** (Schwerpunkt, Entscheidung 🧑: das Spiel ist weit, jetzt fehlt hauptsächlich möglichst automatisiertes Balancing): Parameter-Sweep über ein Raster von Werten, Bewertung gegen die Zielkorridore, automatische Suche (z. B. Bisektion/Raster) nach Werten, die alle Korridore treffen, und ein Vorschlag mit Begründung; Läufe ohne Eingriff in der Warteschlange (über Nacht). Der Vorschlag wird nie selbst in `data/` geschrieben, sondern als Diff zur Übernahme angeboten.
- Läuft offline: keine externen Schriften, Skripte oder API-Aufrufe.

## Nicht-Ziele

Neue Balancing-Regeln und Zielkorridore selbst (BAL4–BAL7; die Seite prüft gegen sie); Client-Lasttests mit Browsern (`k3c-load`, TST).

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
- **AC-06** Parameter-Sweep und automatische Suche liefern einen Vorschlag mit Begründung und einem Diff, ohne `data/` zu ändern (Go-Test mit kleinem Szenario).
- **AC-07** Neue Kennzahlen sind im Bericht und in `docs/rules/zielkorridore.md` beschrieben.

## Offene Fragen

keine. Entschieden (🧑, 2026-10-24): Seite in der Workbench; Kennzahlen nach Bedarf des Agenten, auch neue; Schwerpunkt automatisiertes Balancing.

## Notizen

Aus Ralfs Wunsch 2026-10-24; Ergänzung 2026-10-24: **Die Wirtschaft gehört ausdrücklich dazu** (Rohstoff-Fluss, Gold-Bilanz, Kosten und Bauzeiten, Hub-Ausbau, Lager, Bürger und Berufe, Händler). Kennzahlen und Korridore der Wirtschaft aus `docs/rules/wirtschaft.md` und `zielkorridore.md`; Sweep und automatische Suche gelten für Wirtschafts-Werte (z. B. Kosten in `data/buildings.json`, `data/hub.json`, Holz je Baum) genauso wie für Kampf-Werte. Spec: Entwurf. Sprint WZ28. Zusammenhang: B-589 (BAL7), B-348.
