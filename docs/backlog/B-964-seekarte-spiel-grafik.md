# B-964 · Die Seekarte im Spiel ist eine gestaltete Papierschnitt-Karte statt gezeichneter Formen

- **Domäne:** PAP
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** offline
- **Status:** eingeplant
- **Sprint:** GS2
- **Projekt:** WZG
- **Erstellt:** 2026-11-15
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 🧑 2026-11-15
- **Duplikat von:** –
- **Ersetzt:** –

## Ausgangslage

🧑 2026-11-15: Im Welten-Editor entsteht eine Seekarte zur Bearbeitung der Inseln (B-963). Das Spiel bekommt mit B-962 eine Seekarte am Hafen, vorerst aus gezeichneten Formen (dort Nicht-Ziel: Grafik-Assets).

Stand im Code: B-962 ist umgesetzt und in `main` (Session WE4.2). Das Overlay liegt in `native/k3c-game/hud/seekarte.go` (Zustand `seaView`, Knoten je Insel: abgeschlossen, aktuell, wählbar, gesperrt), `seekarte_input.go` (Links/Rechts/A/B) und `drawsea.go` (Kreise und Linien). Die Route kommt als `view.RouteIsland` (ID, Name, Etappe), die Ziele aus `sim.SwitchState`. Weltbilder je Welt stehen in `data/paper.json` › `weltbilder` (groß und klein, Gruppe `hintergruende`), geladen über `native/k3c-game/art/worldimages.go`. Vorhandene Welten: `standard`, `sengoku`, `nil`, `britannien`; „Polar“ gibt es nicht.

## Ziel

Sobald das Schiff fertig ist, zeigt das Spiel die Seekarte als grafisch schöne Karte im Papierschnitt-Stil: gleiche Inhalte wie B-962 (Etappen, Inseln, abgeschlossen, wählbar, gesperrt), aber gestaltet.

Dieses Ticket liefert die Assets und das Diorama im Overlay. Die Schiffsfahrt im Vollbild ist B-977.

## Beteiligte und Zielgruppen

Spielende am TV (1–2 Monarchen im Split-Screen). Claude legt die Assets über `assets_*` an und setzt um, 🧑 gibt frei, sieht die Assets im Review der Workbench und nimmt am TV ab.

## Anforderungen

Entscheidungen von 🧑 (wörtlich übernommen):

- Gestaltete Karte statt gezeichneter Formen; Inhalte und Bedienung bleiben wie in B-962.
- **Stil (🧑 2026-11-15): Diorama mit Weltbildern.** Das Meer besteht aus Papierschichten, jede Insel erscheint als Miniatur aus den Weltbildern ihrer Welt (Sengoku, Nil, Polar).
- **Schiff (🧑 2026-11-15): fährt die Route.** Das Schiff liegt an der aktuellen Insel; nach der Wahl mit A fährt es als kurze Animation über die Papierwellen zur Zielinsel, danach lädt die Insel (dient als Übergang). (Fahrt: B-977; hier liegt das Schiff an der aktuellen Insel.)
- **Miniaturen (🧑 2026-11-15): ein Bild je Welt mit Varianten.** Je Welt ein Miniatur-Satz aus dem Weltbild; Inseln unterscheiden sich über Größe, Tiefe (Stufenzahl) und kleine Aufsätze. Neue Inseln im Editor brauchen kein neues Asset.
- **Anzeige (🧑 2026-11-15): Overlay, Fahrt im Vollbild.** Die Wahl läuft als Overlay in der Hälfte des Spielers am Steg (wie B-962), der andere spielt weiter; erst die Schiffsfahrt nach A übernimmt den ganzen Bildschirm.
- Assets nur über MCP `assets_*` (Entscheidung 005).
- Funktioniert mit 2 Spielern im Split-Screen.

Ergänzt:

- **R1 Assets:** Meer aus drei Papierschichten (hinten, Mitte, vorn mit Wellenkante), je Welt eine Insel-Miniatur aus dem Weltbild (`standard`, `sengoku`, `nil`, `britannien`), ein kleines Schiff, drei Aufsätze (z. B. Turm, Baum, Fels) und die Zustandszeichen abgeschlossen (Fahne), gesperrt (Nebel/Schloss). Zuordnung in `data/paper.json` › `seekarte` (Welt-ID → Miniatur, dazu Meer, Schiff, Aufsätze, Zeichen).
- **R2 Varianten ohne neues Asset:** Größe der Miniatur aus der Inselgröße, Zahl der sichtbaren Schichten unter der Insel aus der Stufenzahl (`depths`), Aufsatz aus der Insel-ID (fester Hash, kein Zufall zur Laufzeit). Dieselbe Insel sieht immer gleich aus, bei beiden Spielern.
- **R3 Zustände:** aktuell (Schiff liegt an), abgeschlossen (Fahne), wählbar (hell, Zeiger springt nur hierhin), gesperrt (blass, Nebel); die Etappen bleiben als Spalten von links nach rechts erkennbar, Name der gewählten Insel und Hinweiszeile bleiben wie in B-962.
- **R4 Bedienung unverändert:** `seekarte_input.go` und die Tests aus B-962 bleiben gültig; A, B, Links/Rechts wie bisher.
- **R5 Rückfall:** Fehlt ein Asset oder ist es nicht ladbar (`Library.Usable`), zeichnet das Overlay wie bisher die Formen aus `drawsea.go`. Eine Welt ohne Eintrag nutzt die Miniatur von `standard`.
- **R6 Leistung:** Keine Allokation je Frame; Texturen im Budget des Papierschnitts (Prüfung `k3c-grafik`).

## Nicht-Ziele

Regeln der Zielwahl (B-961), Bedienung der Karte (B-962), Editor-Seekarte (B-963), Schiffsfahrt im Vollbild (B-977), Andenken und Chronik (B-966), Miniatur für „Polar“ (entsteht mit der Welt).

## Regeln und Einschränkungen

- Entscheidung 005: Assets nur über `assets_*`, Review im Bereich Assets der Workbench, Fachprüfung `k3c-grafik`; PNG und `manifest.json` sind nicht eingecheckt (B-737), eingecheckt werden Specs, `index.paper.json`, `golden.json`.
- Gen-Läufe (`gen:paper`, Grafik-Golden) nur am Feature bzw. im Abnahme-Ticket (B-742).

- **Bereich:** `native/k3c-game/hud/**`, `native/k3c-game/art/**`, `native/k3c-game/view/**`, `public/grafik/hintergruende/**`, `public/grafik/objekte/**`, `data/paper.json`
- **Abhängig von:** WE4.2
- **Braucht 🧑:** Abnahme
- **Größe:** M
- **Sperre:** golden

## Beispiele

Sengoku, Insel 1 abgeschlossen, Schiff fertig, Spieler 1 am Steg: In seiner Hälfte öffnet das Diorama. Links die Reisfeld-Ebene mit Fahne, in der nächsten Spalte Bambuswälder (drei Schichten, Aufsatz Turm) mit dem Zeiger, rechts blass die Bergfestung im Nebel. Das Schiff liegt an der Reisfeld-Ebene. Spieler 2 spielt in seiner Hälfte weiter.

## Ausnahme- und Fehlerfälle

- Assets nicht gerendert (frischer Checkout ohne `task gen:paper`) → Rückfall auf die Formen, kein Absturz, kein leeres Overlay.
- Welt ohne eigene Miniatur → Miniatur von `standard`.
- Mehr Inseln, als in die Zelle passen (Editor-Welt mit vielen Inseln) → Miniaturen werden kleiner, Reihenfolge und Zustände bleiben lesbar; Untergrenze der Größe wie die Knoten heute.
- Beide Spieler am Steg → jede Hälfte zeigt ihr eigenes Overlay mit eigenem Zeiger (wie B-962).

## Akzeptanzkriterien

- **AC-01** Die Assets (Meer in drei Schichten, vier Insel-Miniaturen, Schiff, drei Aufsätze, Zeichen) sind über `assets_*` angelegt, in `data/paper.json` › `seekarte` zugeordnet und im Review der Workbench sichtbar; `assets_list` zeigt sie ohne Fehler.
- **AC-02** Go-Test in `native/k3c-game/hud`: Das Overlay wählt je Insel Miniatur (nach Welt), Größe, Schichtzahl (nach Stufenzahl) und Aufsatz (nach Insel-ID) deterministisch; zwei Spieler sehen dieselbe Insel gleich.
- **AC-03** Go-Test: Zustände aktuell, abgeschlossen, wählbar, gesperrt ergeben die Zeichen aus R3; die Tests aus B-962 (`seekarte_test.go`) bleiben grün.
- **AC-04** Go-Test: Ohne ladbare Assets zeichnet das Overlay die bisherigen Formen (Rückfall); im Ruhezustand keine Allokation je Frame.
- **AC-05** TV (🧑): Sengoku zu zweit, Schiff fertig: Die Karte ist als Diorama erkennbar, Zustände und Zeiger sind aus 3 m lesbar, der zweite Spieler spielt weiter.

## Offene Fragen

- „Abhängig von: Schiff fertig“ → Default: gemeint ist die Spielbedingung (Schiff gebaut), kein offenes Ticket; technisch hängt das Ticket nur an B-962 (Session WE4.2, in `main`) (vorläufig, 🧑 bestätigt).
- Miniaturen für welche Welten? → Default: die vier vorhandenen (`standard`, `sengoku`, `nil`, `britannien`); „Polar“ folgt mit der Welt (vorläufig).
- Aufsätze → Default: drei neutrale Motive für alle Welten (Turm, Baum, Fels), keine welt-eigenen (vorläufig, 🧑 bestätigt).
- Sperre `golden` → Default: gesetzt, weil neue Papierschnitt-Assets das Grafik-Golden der Gruppen ändern (vorläufig).

## Notizen

Idee aus der Ideenrunde k3c-07 (2026-11-15), Grundzüge mit 🧑 geklärt. 2026-11-15 aus WE4 nach GS2 verschoben und zerlegt: Schiffsfahrt im Vollbild → B-977.
