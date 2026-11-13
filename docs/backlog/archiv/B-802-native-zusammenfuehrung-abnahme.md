# B-802 · Das native Spiel läuft von Start bis Spielstand durch, ist gemessen und am PC abgenommen

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** NG9
- **Projekt:** NAT
- **Erstellt:** 2026-11-11
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-11-11 🧑 Chat (Ralf-Queue natives Spiel)

## Ausgangslage

Nach NG2–NG8 sind Welt, Figuren, Bauten, Eingabe, HUD, Menüs und Ton einzeln gebaut und getestet. Ob das Ganze vom Start bis zum Spielstand läuft, wie schnell es ist und was gegenüber dem Browser-Client fehlt, ist offen.

## Ziel

Das native Spiel läuft vom Start über Lobby und Spiel bis Speichern und Laden durch, ist am Entwicklungs-PC gemessen, als Paket für den Mini-PC gebaut und von 🧑 am PC abgenommen; Lücken zum Browser-Client sind Tickets.

## Beteiligte und Zielgruppen

🧑 Ralf (Abnahme am PC, Mini-PC), Familie, Claude (Zusammenführung, Messung).

## Anforderungen

- Ende-zu-Ende-Test ohne Fenster: Start → Lobby mit 2 Spielern → kurzes Spiel mit festen Befehlen → Speichern → Laden → gleicher Stand.
- Funktionsabgleich: Liste der Darstellungs-Elemente aus `src/scenes/` (render, hud, hints, effects, Szenen), je Element nativ ja oder nein; jedes Nein ist ein Ticket im Projekt NAT.
- Messlauf am Entwicklungs-PC (Bildrate, Grafikspeicher, Rechenzeit je Bild, Sim-Takt), je einmal.
- Paket für den Mini-PC (`task native:game:paket`) mit EXE, Grafik und Messskript.

## Nicht-Ziele

Sim-, Balance- und Lasttest-Läufe (gesperrt seit 2026-11-11); Abschalten von Server und Browser-Client (B-775 › Offene Fragen); neue Funktionen.

## Regeln und Einschränkungen

Leistungsmessung der Darstellung ist erlaubt und läuft je einmal. Abnahmen am Gerät sind Mensch-Sessions und keine Abhängigkeit des Reviews.

Planungsangaben für die Arbeitswellen (`docs/arbeitsweise.md` › Parallelität und Wellen), je eine Zeile, Werte ohne Freitext; Pflicht ab `Spec: freigegeben`:

- **Bereich:** `native/k3c-game/game/**`, `native/k3c-game/e2e/**`, `native/Taskfile.yml`, `native/measure.ps1`
- **Abhängig von:** NG2, NG3, NG4, NG5, NG6, NG7, NG8
- **Braucht 🧑:** Freigabe, Abnahme, Gerät
- **Größe:** M
- **Sperre:** taskfile

## Beispiele

`task native:game:e2e` grün; `task native:game:measure` schreibt `reports/native-game-*.json`; Ralf spielt am PC 15 Minuten zu zweit.

## Ausnahme- und Fehlerfälle

Ein Teil fehlt oder bricht im Zusammenspiel → Ticket mit Befund, kein Umbau im Review. Grafikspeicher über dem Wert des Prototyps → Befund mit Maßnahme als Ticket.

## Akzeptanzkriterien

- **AC-01** Ende-zu-Ende-Test ohne Fenster grün (Start bis Laden, 2 Spieler, gleicher Welt-Hash nach Laden).
- **AC-02** Funktionsabgleich mit `src/scenes/` als Liste im Ergebnis, jede Lücke als Ticket im Projekt NAT.
- **AC-03** Messbericht Entwicklungs-PC unter `reports/` und Paket für den Mini-PC gebaut.
- **AC-04** 🧑 spielt am PC zu zweit mit 2 Gamepads, Befund und Entscheidung in den Notizen (B-775/AC-03 am PC).

## Offene Fragen

keine

## Notizen

Angelegt 2026-11-11 bei der Planung der Umsetzung (NG9). Mini-PC-Teil: B-786 im selben Sprint.

**Abnahme am PC (NG9.4), 🧑 Ralf, 2026-11-12:** bestanden. Standard-Abnahme mit 1 und 2 Spielern an der Tastatur läuft, Menüs gehen. Abweichung: mit Tastatur statt zwei Gamepads; der Rest (Gamepads, Ton, weitere Punkte der Checkliste) kommt später in NG14.5/NG15.3 bzw. am Mini-PC (NG9.5). Keine Befunde.
