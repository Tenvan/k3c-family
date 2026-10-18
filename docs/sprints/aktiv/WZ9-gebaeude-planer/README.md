# WZ9 · SRV, DEV · Gebäude-Planer für die Freischaltung je Hub-Stufe

- **Status:** aktiv
- **Projekt:** WZG
- **Domäne:** SRV, DEV
- **Reife:** bereit
- **Tickets:** B-462
- **Start-Commit:** b1b6062a
- **Spec:** freigegeben
- **Revision:** 3
- **Freigabe:** 2026-10-18, Ralf im Chat (Revision 3)

## Ausgangslage

B-462 › Ausgangslage. Der Sprint setzt den Vorlauf WZ15 voraus (Regeltabelle erzeugt, Prüfung einer Kandidaten-`hub.json` ohne globalen Zustand, Linien als Parameter in `engine/level`, 7-Stufen-Fixture); WZ15 ist aus WZ9 abgespalten worden (Entscheidung 🧑 2026-10-17, 8 Sessions waren über dem Budget). `engine/sim` liest `hub.json` per `go:embed`, ein ungespeicherter Stand lässt sich daher nur über eine Kandidaten-Option im Snapshot zeigen (SRV-Session WZ9.1). Voraussetzung für die DEV-Sessions: Asset-Studio (WZ6) und Live-Vorschau (WZ7); WZ9.1 braucht die Snapshot-Funktion WZ7.1.

## Ziel

Ein Gebäude-Planer im Asset-Studio zeigt und bearbeitet, welcher Bauplatz ab welcher Hub-Stufe in welcher Tiefe verfügbar ist, prüft über das Go-Backend (Funktion aus WZ15, inklusive geplanter Chunks aus B-461, sofern WZ8 erledigt ist), schreibt `data/hub.json` und den erzeugten Block der Regeltabelle format-stabil und zeigt das Ergebnis in der Live-Vorschau, auch ungespeichert und ohne EXE-Neubau.

Am Ende sichtbar: im Asset-Studio die Matrix mit Befunden und REG-Hinweis, die Live-Vorschau in der gewählten Hub-Stufe; von 🧑 abgenommen.

## Beteiligte und Zielgruppen

B-462 › Beteiligte und Zielgruppen. Im Sprint: Agenten setzen WZ9.1–WZ9.3 um (SRV, DEV) und prüfen in WZ9.4; 🧑 gibt die Spec frei und nimmt in WZ9.5 ab.

## Anforderungen

B-462 › Anforderungen (Matrix, Bearbeitbar, Schreiben, Schreibregel, REG-Hinweis, Live-Vorschau). Sprint-eigen:
- **Voraussetzung WZ15:** Prüfung (`ValidateHub`), `sim.CreateOptions` mit Kandidat, Linien als Parameter in `engine/level`, Generator-Paket `tools/regeltabelle/tabelle` (`Block`, `Ersetzen`, Gegenprobe) und die 7-Stufen-Fixture liegen vor.
- **Reihenfolge:** Erst baut SRV die Kandidaten-Option in den Snapshot (WZ9.1), dann der Planer (WZ9.2 Matrix, WZ9.3 Bearbeiten), dann Review (WZ9.4) und Abnahme (WZ9.5).
- **Kandidat ohne globalen Zustand:** Die Snapshot-Option (WZ9.1) nimmt die Kandidaten-`hub.json` als Parameter; kein `sim.UseData`. Die Options-Struktur der Snapshot-Funktion legt WZ7.1 an.
- **Vorrang im Snapshot:** Sind „geplante Chunks“ (WZ8.4) und Kandidaten-`hub.json` zugleich gesetzt, gelten die Kandidaten-Linien, und die Planung wird gegen sie geprüft.
- **Planer schreibt zwei Dateien (Entscheidung 8):** zuerst `data/hub.json`, dann den erzeugten Block in § 3 (Funktionen aus WZ15.2), jeweils atomar; Schreibregel für beide Dateien.
- **Bauplätze und Pflichtplätze (Entscheidung 5, Frage 12):** hinzufügen und entfernen nur mit bekannten Arten; Pflichtplätze prüft die Funktion aus WZ15.3; Abnahmepunkt in WZ9.5.
- **Kein Datenwechsel im Sprint:** `data/hub.json`, Regeltabelle und `testdata/golden/` bleiben unverändert; Tests arbeiten auf Kopien.
- **Abhängigkeiten:** WZ9.1 braucht WZ15.3 und WZ7.1; WZ9.2 braucht WZ6, WZ7.3, WZ9.1 und WZ15.2; WZ9.3 die Schreibregel aus B-459/AC-07.

## Nicht-Ziele

B-462 › Nicht-Ziele, besonders: Werte und Kosten (`data/buildings.json`, `levels`, REG); neue Gebäudearten und Felder (SIM); mehr Hub-Stufen selbst (B-458); Grafik der Ausbaustufen (GEB9); Chunk-Planung (B-461, WZ8); Golden-Update im Sprint. Regeltabellen-Generator und Prüfung gehören zu WZ15.

## Regeln und Einschränkungen

B-462 › Regeln und Einschränkungen. Für den Sprint:
- **Domänen nacheinander:** SRV (WZ9.1: `engine/room/`, `engine/net/`, Kandidaten-Option der Snapshot-Funktion) → DEV (WZ9.2–WZ9.5: `tools/k3c-studio/`). Die Review-Session trägt die Domäne der letzten Umsetzung (DEV) und prüft den Diff beider Domänen.
- **Grenzfall Werkzeug schreibt REG-Daten:** Das Speichern im Planer ist eine REG-Änderung durch 🧑 außerhalb des Sprints; kein Sprint-Commit enthält eine geänderte `data/hub.json` oder Regeltabelle.
- **Schichten:** `engine/room` importiert nichts aus `engine/net`.
- **Abgrenzung:** GEB9 ändert `data/hub.json` nicht und braucht den Planer nicht; B-458 nutzt ihn später für die neue Stufenfolge.
- **Glossar:** „Gebäude-Planer“ trägt WZ6.1 ein; WZ9 trägt keinen Begriff ein.
- Keine neue Abhängigkeit; Datei ≤ 400, Funktion ≤ 60 Zeilen; Firewall-Regel des Asset-Studios (B-459), Generator nur als gebaute EXE.

## Beispiele

B-462 › Beispiele. Sprint-eigen:
- Nach WZ9.1: Snapshot mit Kandidaten-`hub.json` (Rüstkammer ab Stufe 3) zeigt die Rüstkammer in Hub-Stufe 3, ohne Option nicht; zweimal derselbe Aufruf → byte-gleich.
- Nach WZ9.3: Rüstkammer auf Stufe 3 im Planer → Live-Vorschau zeigt sie sofort → Speichern → eine geänderte Zeile in `data/hub.json` und der neu erzeugte Block in § 3 (alles auf Kopien getestet), REG-Hinweis; „Regeltabelle veraltet“ nur, wenn das Schreiben des Blocks scheitert.

## Ausnahme- und Fehlerfälle

B-462 › Ausnahme- und Fehlerfälle. Sprint-eigen:
- WZ15 noch nicht erledigt, wenn WZ9.1 oder WZ9.2 an der Reihe ist → `blockiert`.
- WZ6 oder WZ7.3 noch nicht fertig, wenn WZ9.2 an der Reihe ist → WZ9.2 `blockiert` (Ablauf laut `docs/arbeitsweise.md`).
- WZ7.1 (Snapshot-Funktion) noch nicht fertig, wenn WZ9.1 an der Reihe ist → WZ9.1 `blockiert`; WZ9.2 und WZ9.3 folgen erst danach. Fehlt die Options-Struktur oder hat sie globalen Zustand, wird ein Ticket für WZ7 angelegt, statt sie in WZ9.1 umzubauen.
- WZ8 noch nicht erledigt → die Prüfung (WZ15.3) lässt geplante Chunks aus, Hinweis im Planer, Ticket zur Nachrüstung; WZ9 läuft weiter.

## Akzeptanzkriterien

- **AC-01** `B-462/AC-01` Matrix Hub-Stufe × Bauplatz mit Platz und Tiefe.
- **AC-02** `B-462/AC-02` (Planer) Speichern nur nach Prüfung mit der Funktion aus WZ15.3 (auch für hinzugefügte und entfernte Plätze, `gateHubLevel`, Händler, `wallUnits`; mit WZ8 inklusive geplanter Chunks); abgelehnte Änderungen lassen die Dateien byte-gleich.
- **AC-03** `B-462/AC-03` Abweichungen zur erzeugten Regeltabelle sichtbar (Berechnung im Go-Backend mit der Funktion aus WZ15.2); beim Speichern schreibt der Planer den erzeugten Block in § 3 (nur zwischen den Markern, atomar, Fehler im zweiten Schritt → „Regeltabelle veraltet“).
- **AC-04** `B-462/AC-04` Live-Vorschau zeigt den geprüften Stand des Planers, auch ungespeichert und ohne EXE-Neubau.
- **AC-05** `B-462/AC-05` Hinweis auf REG-Änderung (`data/hub.json` und Regeltabelle) und Goldens.
- **AC-06** 🧑 nimmt den Gebäude-Planer im Asset-Studio ab und bestätigt die Pflichtplatz-Liste (Abnahme-Session, Agent: Mensch).
- **AC-07** `B-462/AC-06` Format stabil, atomar, Schreibregel bei extern geänderter Datei (`data/hub.json` und Regeltabelle).
- **AC-08** `B-462/AC-08` (Matrix, Snapshot) Mehr Hub-Stufen ohne Umbau: Matrix und Snapshot-Option zeigen mit der 7-Stufen-Fixture aus WZ15 7 Stufen (Tests), keine feste 5 im Code.
- **AC-09** `B-462/AC-09` Die Snapshot-Funktion aus WZ7.1 nimmt die Kandidaten-`hub.json` als Option (Options-Struktur, kein globaler Zustand), prüft und baut daraus; ohne Option gilt die eingebettete; bei zugleich gesetzten „geplanten Chunks“ gelten die Kandidaten-Linien und die Planung wird gegen sie geprüft; Go-Tests für Determinismus, Kandidat gegen eingebettet, ungültigen Kandidaten und den Vorrang (Linie 5 bei 126 macht die Ader bei Hub +136 ungültig).
- **AC-10** Der Sprint ändert weder `data/hub.json` noch `testdata/golden/`: `git diff --stat origin/develop -- data/hub.json testdata/golden` ist leer.

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat): B-462 › Offene Fragen 1–6, 8–10 (siehe Ticket); Zuschnitt: REG, INF und SIM als Vorlauf-Sprint WZ15, WZ9 behält SRV und DEV (5 Sessions). Frage 7 ist durch 9 beantwortet.

Aus dem Code festgelegt, 🧑 bestätigt bei der Abnahme (WZ9.5): **Pflichtplätze** (B-462 › Offene Fragen 12): Werkstatt, Treppe runter, Treppe hoch und Farm je Seite; Kaserne, Taverne, Lager, Heilplatz, Schmiede und Rüstkammer sind entfernbar. Die Tabelle mit den Gründen steht in WZ15.3 › Kontext.

Offen: keine.

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| WZ9.1 | `WZ9.1-snapshot-kandidat.md` | Umsetzung | autonom | offen |
| WZ9.2 | `WZ9.2-planer-matrix.md` | Umsetzung | autonom | offen |
| WZ9.3 | `WZ9.3-planer-bearbeiten.md` | Umsetzung | autonom | offen |
| WZ9.4 | `WZ9.4-review.md` | Review | autonom | offen |
| WZ9.5 | `WZ9.5-abnahme.md` | Workshop | Mensch | offen |

- WZ9.1 (SRV) Snapshot nimmt Kandidaten-`hub.json` als Option: Prüfung mit der Funktion aus WZ15.3 und Bau aus dem Kandidaten über die Options-Struktur aus WZ7.1 und `sim.CreateOptions`, ohne EXE-Neubau; bei zugleich gesetzter Option „geplante Chunks“ (WZ8.4, falls erledigt) gelten die Kandidaten-Linien; Go-Tests für Determinismus, Kandidat gegen eingebettet, ungültigen Kandidaten, Vorrang und 7-Stufen-Fixture (AC-09, AC-04, AC-08, AC-10).
- WZ9.2 (DEV) Gebäude-Planer im Asset-Studio: Matrix aus `data/hub.json`, Abweichung zur Regeltabelle über das Go-Backend, Live-Vorschau folgt dem Planer-Stand (Kandidat) und der gewählten Hub-Stufe (AC-01, AC-03, AC-04, AC-08).
- WZ9.3 (DEV) Bearbeiten (Stufe, Platz, Tiefe, `needsDeeper`, `gateHubLevel`, Händler, `wallUnits`, Plätze bekannter Arten hinzufügen und entfernen) mit Prüfung über das Go-Backend, atomares format-stabiles Schreiben von `data/hub.json` und danach des erzeugten Blocks der Regeltabelle, Schreibregel bei externer Änderung beider Dateien, REG-Hinweis (AC-02, AC-03, AC-05, AC-07, AC-10).
- WZ9.4 (DEV) Review: Kriterien, Diff SRV und DEV, Begriffs-Treue, PR (alle außer AC-06).
- WZ9.5 (DEV) Abnahme durch 🧑 im Asset-Studio (Agent: Mensch): Matrix, Beispiel Rüstkammer, Live-Vorschau ungespeichert, erzeugter Block und REG-Hinweis, Pflichtplatz-Liste bestätigen (AC-06).

Domänen-Reihenfolge: SRV → DEV.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
