# WZ9 · DEV · Gebäude-Planer für die Freischaltung je Hub-Stufe

- **Status:** geplant
- **Projekt:** WZG
- **Domäne:** DEV
- **Reife:** Entwurf
- **Tickets:** B-462
- **Start-Commit:** – (wird beim Aktivieren gesetzt: `git rev-parse --short origin/develop`)
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

B-462 › Ausgangslage: Die Freischaltung je Hub-Stufe steht in `data/hub.json` (`sites`, `islandSites`, `wallLines`, `merchant`) und wird an drei Stellen von Hand gepflegt (JSON, Regeltabelle § 3, Test-Kopie `hubLevelRule`); die Platz-Regeln gibt es nur als Go-Tests (`engine/sim/site_data_test.go`). B-458 (Hub wächst kleinteiliger) braucht die Zuordnung und profitiert vom Planer; GEB9 braucht den Planer nicht. `engine/sim` liest `hub.json` per `go:embed`, ein ungespeicherter Stand lässt sich daher nur über eine Kandidaten-Option im Snapshot zeigen (SRV-Session WZ9.4). Voraussetzung für die DEV-Sessions: Asset-Studio (WZ6) und Live-Vorschau (WZ7); WZ9.4 braucht die Snapshot-Funktion WZ7.1; die REG-, INF- und SIM-Sessions hängen nicht daran.

## Ziel

Ein Gebäude-Planer im Asset-Studio zeigt und bearbeitet, welcher Bauplatz ab welcher Hub-Stufe in welcher Tiefe verfügbar ist, prüft über das Go-Backend (inklusive geplanter Chunks aus B-461, sofern WZ8 erledigt ist), schreibt `data/hub.json` format-stabil und zeigt das Ergebnis in der Live-Vorschau, auch ungespeichert und ohne EXE-Neubau; die Regeltabelle in § 3 wird aus der JSON erzeugt.

Am Ende sichtbar: im Asset-Studio die Matrix mit Befunden und REG-Hinweis, die Live-Vorschau in der gewählten Hub-Stufe; im Terminal `task rules:update` ohne Diff und ein grüner Go-Test Block = JSON; von 🧑 abgenommen.

## Beteiligte und Zielgruppen

B-462 › Beteiligte und Zielgruppen. Im Sprint: Agenten setzen WZ9.1–WZ9.7 um (REG, INF, SIM, SRV, DEV); 🧑 entscheidet die Offenen Fragen vor der Freigabe und nimmt in WZ9.8 ab.

## Anforderungen

B-462 › Anforderungen. Sprint-eigen:
- **Reihenfolge:** Erst legt REG das Zielformat der Tabelle von Hand fest (WZ9.1), dann erzeugt INF genau diesen Block (WZ9.2), dann stellt SIM die Prüfung bereit (WZ9.3), dann baut SRV die Kandidaten-Option in den Snapshot (WZ9.4); der Planer (WZ9.5, WZ9.6) baut auf Generator, Prüfung und Snapshot-Option auf.
- **Generator in Go:** `tools/regeltabelle` (Task, EXE `bin/regeltabelle`) und das Paket `tools/regeltabelle/tabelle` (reine Funktion, vom Studio-Backend importiert); das Studio zeigt nur die Abweichung, es schreibt die Tabelle nie (B-462 › Offene Fragen 10).
- **Kandidat ohne globalen Zustand:** Prüfung (WZ9.3) und Snapshot-Option (WZ9.4) nehmen die Kandidaten-`hub.json` als Parameter; kein `sim.UseData`, keine globale Umschaltung. Die Options-Struktur der Snapshot-Funktion legt WZ7.1 an.
- **Linien als Parameter in `engine/level` (darf WZ9.3 ändern):** Die Kandidaten-Linien erreichen Generator und Planungs-Prüfung nur über Parameter (Standard eingebettet, kein globaler Zustand). Hat WZ8 sie angelegt (WZ8.2, WZ8.3), baut WZ9.3 darauf auf; sonst legt WZ9.3 sie an (Zuschnitt wie B-461 › Anforderungen), auch ohne Planung. `sim.CreateIsland` nimmt die Options-Struktur aus WZ8.3; WZ9.3 ergänzt darin den Kandidaten als Feld (fehlt die Struktur, legt WZ9.3 sie an und passt alle Aufrufer an). **`sim.CreateOptions` ist eine neue Struktur nur für die Erzeugung, getrennt vom vorhandenen `sim.IslandOptions` (`engine/sim/island_options.go`, im Spielstand gespeichert); sie wird nicht gespeichert, Spielstand-Format und `IslandSaveVersion` bleiben unverändert.**
- **Geplante Chunks (B-461, WZ8):** Die Prüfung aus WZ9.3 prüft sie mit, sofern WZ8 erledigt ist (Annahme: WZ7 → WZ8 → WZ9), gegen die Kandidaten-Linien; sonst Hinweis „geplante Chunks nicht geprüft“ und ein Ticket.
- **Vorrang im Snapshot:** Sind „geplante Chunks“ (WZ8.4) und Kandidaten-`hub.json` (WZ9.4) zugleich gesetzt, gelten die Kandidaten-Linien, und die Planung wird gegen sie geprüft, nicht gegen die eingebetteten.
- **Kein Datenwechsel im Sprint:** `data/hub.json` und `testdata/golden/` bleiben unverändert; Tests arbeiten auf Kopien. Die 7-Stufen-Fixture (AC-09) liegt unter `testdata/` (Vorschlag: `testdata/hub/`), nicht in `data/`; ihre Werte gibt der Sprint nicht vor, sie muss die Prüfung bestehen.
- **Abhängigkeiten:** WZ9.4 braucht die Snapshot-Funktion WZ7.1; WZ9.5 braucht das Asset-Studio (WZ6), die Live-Vorschau (WZ7.3) und WZ9.4; WZ9.6 die Schreibregel aus B-459/AC-07.

## Nicht-Ziele

B-462 › Nicht-Ziele, besonders: Werte und Kosten (`data/buildings.json`, `levels`, REG); neue Gebäudearten und Felder (SIM); mehr Hub-Stufen selbst (B-458); Grafik der Ausbaustufen (GEB9); Chunk-Planung (B-461, WZ8); Golden-Update im Sprint.

## Regeln und Einschränkungen

B-462 › Regeln und Einschränkungen. Für den Sprint:
- **Domänen nacheinander:** REG (WZ9.1: nur `docs/rules/materialien-gebaeude.md` § 3) → INF (WZ9.2: `Taskfile.yml`, `tools/regeltabelle/`, `tests/projectRules.test.ts`) → SIM (WZ9.3: `engine/sim/`, bei Bedarf `engine/level/`, keine Golden-Änderung) → SRV (WZ9.4: `engine/room/`, `engine/net/`, Kandidaten-Option der Snapshot-Funktion) → DEV (WZ9.5–WZ9.8: `tools/k3c-studio/`). Reihenfolge der Domänen: REG → INF → SIM → SRV → DEV. Die Review-Session trägt die Domäne der letzten Umsetzung (DEV) und prüft den Diff aller fünf Domänen.
- **Grenzfall Werkzeug schreibt REG-Daten:** Das Speichern im Planer ist eine REG-Änderung durch 🧑 außerhalb des Sprints; kein Sprint-Commit enthält eine geänderte `data/hub.json`.
- **Schichten:** `engine/room` importiert nichts aus `engine/net`. **`engine/level` darf WZ9.3 ändern:** Mauerlinien als Parameter von Generator und Planungs-Prüfung (Standard eingebettet, kein globaler Zustand), soweit WZ8 sie nicht schon angelegt hat; `engine/level` importiert weiter nichts aus `engine/sim`, `engine/room`, `engine/net`, und die Golden-Daten bleiben byte-gleich.
- **Abgrenzung:** GEB9 ändert `data/hub.json` nicht und braucht den Planer nicht; B-458 nutzt ihn später für die neue Stufenfolge (neue `levels` und Plätze kommen dort über REG/SIM).
- **Glossar:** „Gebäude-Planer“ trägt WZ6.1 ein; WZ9 trägt keinen Begriff ein.
- Keine neue Abhängigkeit; Datei ≤ 400, Funktion ≤ 60 Zeilen; Firewall-Regel des Asset-Studios (B-459), Generator nur als gebaute EXE.

## Beispiele

B-462 › Beispiele. Sprint-eigen:
- Nach WZ9.2: `task rules:update` auf `develop`-Stand → kein Diff in `docs/rules/materialien-gebaeude.md`; Go-Test und Marker-Test grün.
- Nach WZ9.3: `go test ./engine/sim` grün, `testdata/golden/` unverändert, `hubLevelRule` nicht mehr im Code; die 7-Stufen-Fixture besteht die Prüfung.
- Nach WZ9.4: Snapshot mit Kandidaten-`hub.json` (Rüstkammer ab Stufe 3) zeigt die Rüstkammer in Hub-Stufe 3, ohne Option nicht; zweimal derselbe Aufruf → byte-gleich.
- Nach WZ9.6: Rüstkammer auf Stufe 3 im Planer → Live-Vorschau zeigt sie sofort → Speichern → Diff nur in einer Zeile, REG-Hinweis, Meldung „Regeltabelle veraltet“.

## Ausnahme- und Fehlerfälle

B-462 › Ausnahme- und Fehlerfälle. Sprint-eigen:
- WZ6 oder WZ7.3 noch nicht fertig, wenn WZ9.5 an der Reihe ist → WZ9.5 `blockiert` (Ablauf laut `docs/arbeitsweise.md`); WZ9.1–WZ9.3 bleiben gültig.
- WZ7.1 (Snapshot-Funktion) noch nicht fertig, wenn WZ9.4 an der Reihe ist → WZ9.4 `blockiert`; WZ9.5 und WZ9.6 folgen erst danach, WZ9.1–WZ9.3 bleiben gültig. Fehlt die Options-Struktur in WZ7.1 oder hat sie globalen Zustand, wird ein Ticket für WZ7 angelegt, statt sie in WZ9.4 umzubauen.
- WZ8 noch nicht erledigt (WZ9.3 an der Reihe) → die Prüfung lässt geplante Chunks aus, Hinweis im Planer, Ticket zur Nachrüstung; WZ9 läuft weiter.
- WZ9.1 findet eine Abweichung zwischen Tabelle und `data/hub.json` → nicht still angleichen: 🧑 entscheidet, welche Seite gilt; bis dahin `blockiert`.
- WZ9.3 lässt einen Golden-Test brechen → Fehler im Umbau, kein `task golden:update`.
- WZ9.3 sprengt das Budget (Variante der Sim-Funktionen ohne globalen Zustand) → Session teilen (Prüfung / Bau aus Kandidat), Sprint-Umfang neu bewerten, Kriterien nicht umformulieren.

## Akzeptanzkriterien

- **AC-01** `B-462/AC-01` Matrix Hub-Stufe × Bauplatz mit Platz und Tiefe.
- **AC-02** `B-462/AC-02` Prüfung über eine Funktion in `engine/sim` ohne globalen Zustand (eine Quelle der Regeln), mit WZ8 inklusive geplanter Chunks aus `data/islands.json` gegen die Kandidaten-Linien (Linien als Parameter in `engine/level`); erst dann Schreiben von `data/hub.json`.
- **AC-03** `B-462/AC-03` Abweichungen zur erzeugten Regeltabelle sichtbar (Berechnung im Go-Backend mit der Funktion des Tasks).
- **AC-04** `B-462/AC-04` Live-Vorschau zeigt den geprüften Stand des Planers, auch ungespeichert und ohne EXE-Neubau.
- **AC-05** `B-462/AC-05` Hinweis auf REG-Änderung und Goldens.
- **AC-06** 🧑 nimmt den Gebäude-Planer im Asset-Studio ab (Abnahme-Session, Agent: Mensch).
- **AC-07** `B-462/AC-06` Format stabil, atomar, Schreibregel bei extern geänderter Datei.
- **AC-08** `B-462/AC-07` Regeltabelle in § 3 aus `data/hub.json` erzeugt (Go-Generator, Task `rules:update`), Go-Test Block = JSON, Marker-Test.
- **AC-09** `B-462/AC-08` Mehr Hub-Stufen ohne Umbau: Eine Fixture mit 7 `levels` unter `testdata/`, die die Prüfung besteht, ohne vorgegebene Werte; sie darf Farm, Linie 1, `payRangeUnits` (soweit die Prüfung es als Parameter nimmt), Inset und Streuung frei wählen. Matrix, Prüfung, Generator und Snapshot-Option zeigen 7 Stufen (Tests), keine feste 5 im Code.
- **AC-10** Der Sprint ändert weder `data/hub.json` noch `testdata/golden/`: `git diff --stat origin/develop -- data/hub.json testdata/golden` ist leer.
- **AC-11** `B-462/AC-09` Die Snapshot-Funktion aus WZ7.1 nimmt die Kandidaten-`hub.json` als Option (Options-Struktur, kein globaler Zustand), prüft und baut daraus; ohne Option gilt die eingebettete; bei zugleich gesetzten „geplanten Chunks“ gelten die Kandidaten-Linien und die Planung wird gegen sie geprüft; Go-Tests für Determinismus, Kandidat gegen eingebettet, ungültigen Kandidaten und den Vorrang (Linie 5 bei 126 macht die Ader bei Hub +136 ungültig).

## Offene Fragen

Entschieden von 🧑 am 2026-10-17 (Chat): B-462 › Offene Fragen 1–4 (Planer schreibt `data/hub.json` nach Go-Prüfung mit REG-Hinweis; Regeltabelle aus der JSON erzeugt per Task mit Test; erweiterbar für B-458, Matrix mit `sites`, `islandSites`, `wallLines`, Händler; Live-Vorschau aus WZ7 zeigt die Änderung) und B-462 › Offene Fragen 9 (eigene SRV-Session: Snapshot nimmt Kandidaten-`hub.json` als Option, Options-Struktur ohne globalen Zustand; Domänen REG → INF → SIM → SRV → DEV).

Offen, vor der Freigabe (entscheidet 🧑):
1. B-462 › Offene Fragen 5, 6, 8 und 10 (Plätze hinzufügen, `wallUnits` bearbeitbar, Tabelle beim Speichern erzeugen, Generator in Go statt TS). Frage 7 (Vorschau ungespeichert) ist durch Entscheidung 9 beantwortet.
2. **Umfang:** 8 Sessions überschreiten das Budget von 3–6 je Sprint (`docs/arbeitsweise.md`). Annahme: so lassen, weil REG, INF und SIM klein sind und die Abnahme getrennt laufen soll; Alternative: REG/INF/SIM als eigener Vorlauf-Sprint (WZ9.4 hängt an WZ7.1 und bliebe im Hauptsprint). Risiko: WZ9.3 ist wegen der Variante ohne globalen Zustand die größte Session und kann sich teilen.

## Sessions

- WZ9.1 (REG) Regeltabelle „Hub-Stufe → Gebäude“ in § 3 auf die erzeugbare Form umstellen: Marker, Spalten (Hub-Stufe, Material, Hub-Plätze mit Platz und Tiefe, Mauerlinie und Tor, Händler), Inhalt = heutige `data/hub.json`, Abgleich ohne Abweichung (AC-08, AC-10).
- WZ9.2 (INF) Generator in Go (`tools/regeltabelle`, Funktion in `tools/regeltabelle/tabelle`) und Task `rules:update` schreiben den Block aus `data/hub.json` byte-gleich zu WZ9.1; Go-Test vergleicht Block und JSON, `tests/projectRules.test.ts` prüft die Marker; Test mit der 7-Stufen-Fixture unter `testdata/` (AC-08, AC-09, AC-10).
- WZ9.3 (SIM) Exportierte Prüfung einer Kandidaten-`hub.json` (Parameter, kein globaler Zustand) in `engine/sim` aus den Regeln von `site_data_test.go`, mit WZ8 inklusive geplanter Chunks aus `data/islands.json` gegen die Kandidaten-Linien; `engine/level` darf geändert werden: Mauerlinien als Parameter von Generator und Planungs-Prüfung (Standard eingebettet), soweit WZ8.2/WZ8.3 sie nicht angelegt haben; `sim.CreateOptions` (aus WZ8.3, sonst hier angelegt, alle Aufrufer angepasst) bekommt den Kandidaten als Feld; Variante der Plätze-, Stufen- und Linien-Berechnung für den Bau aus einem Kandidaten (Grundlage für WZ9.4); Tests darauf umstellen, `hubLevelRule` entfernen, Fixture mit 7 `levels` unter `testdata/`, die die Prüfung besteht (Werte frei, nicht vorgegeben), Goldens unverändert (AC-02, AC-09, AC-10).
- WZ9.4 (SRV) Snapshot nimmt Kandidaten-`hub.json` als Option: Prüfung mit der Funktion aus WZ9.3 und Bau aus dem Kandidaten über die Options-Struktur aus WZ7.1 und `sim.CreateOptions`, ohne EXE-Neubau; bei zugleich gesetzter Option „geplante Chunks“ (WZ8.4) gelten die Kandidaten-Linien, die Planung wird gegen sie geprüft; Go-Tests für Determinismus, Kandidat gegen eingebettet, ungültigen Kandidaten, den Vorrang und die 7-Stufen-Fixture (AC-11, AC-04, AC-09, AC-10).
- WZ9.5 (DEV) Gebäude-Planer im Asset-Studio: Matrix aus `data/hub.json`, Abweichung zur Regeltabelle über das Go-Backend, Live-Vorschau folgt dem Planer-Stand (Kandidat) und der gewählten Hub-Stufe (AC-01, AC-03, AC-04, AC-09).
- WZ9.6 (DEV) Bearbeiten mit Prüfung über das Go-Backend, atomares format-stabiles Schreiben, Schreibregel bei externer Änderung, REG-Hinweis (AC-02, AC-05, AC-07, AC-10).
- WZ9.7 (DEV) Review: Kriterien, Diff aller fünf Domänen, Begriffs-Treue, PR (alle außer AC-06).
- WZ9.8 (DEV) Abnahme durch 🧑 im Asset-Studio (Agent: Mensch): Matrix, Beispiel Rüstkammer, Live-Vorschau ungespeichert, REG-Hinweis (AC-06).

Domänen-Reihenfolge: REG → INF → SIM → SRV → DEV.

## Abnahme

Wird von der Review-Session (Doku-Sprint: letzte Session) ausgefüllt, höchstens fünf Zeilen: Datum, Kriterien
(Verweis auf Session-Ergebnisse), behobene Befunde, neue Tickets. Bis dahin `–`.
