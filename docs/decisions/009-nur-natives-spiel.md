# 009 · Nur noch das native Spiel am PC, der Browser-Client entfällt

- **Status:** angenommen (Ralf, Dialog im Chat am 2026-11-12)
- **Datum:** 2026-11-12
- **Projekt:** NAT · Tickets: B-823 (Ablösung), B-822 (Testseiten, erledigt in CU2)
- **Vorgänger:** 001 (Engine auf Go-Server, Browser als Client), 008 (natives Spiel in Go + Ebitengine)

## Anlass

Das native Spiel (`native/k3c-game`, Projekt NAT) läuft am PC mit 1 und 2 Spielern, Menüs, Ton und Spielständen; die
Abnahme NG9.4 ist bestanden (2026-11-12). Daneben gibt es weiter den Browser-Client (`src/`, Phaser, Vite) mit dem
Go-Server als Spiel-Host (Entscheidung 001). Zwei Clients für dasselbe Spiel verdoppeln Pflege, Tests und Planung.

## Entscheidung

1. **Plattform ist nur noch der PC** (später ein Mini-PC am TV) mit dem nativen Spiel. Xbox/Edge als Zielplattform
   entfällt; das native Spiel läuft auf einer Xbox im Handelsmodus nicht.
2. **Das Online-Spiel über mehrere Geräte (`/ws`) entfällt.** Ob `k3c-server`, `engine/net`, `k3c-load` und `k3c-tui` als
   Werkzeuge bleiben oder fallen, klärt ein eigenes Ticket.
3. **Der Server liefert keinen Client mehr aus** (`dist/`).
4. **Die Präsentationsseite `site/` (GitHub Pages) bleibt**, statisch; Bilder aus den Papierschnitt-Assets in einem
   PLAT-Ticket.
5. **„Im Spiel ansehen“ der Workbench wird auf das native Spiel umgestellt** (eigenes DEV-Ticket).
6. **Zeitpunkt:** Die Ablösung (B-823) wird nach der Abnahme des nativen Spiels (NG9 inkl. Mini-PC) umgesetzt; bis dahin
   bleibt der Browser-Client lauffähig, wird aber nicht weiterentwickelt.
7. **Offene Planung für den Browser-Client wird gesammelt verworfen** (Projekte BED, LST, Sprints HW1, RX1, PF1 und die
   CLI-Tickets ohne Bezug zum nativen Spiel), mit Verweis auf B-823.

## Gründe

- Das native Spiel deckt den Zweck (Couch-Koop zu zweit am TV) ab, ohne Netz und Server.
- Ein Client statt zwei: weniger Code, schnellere Prüfläufe, eine Planung.
- Die Go-Simulation bleibt unverändert (Entscheidung 008, Punkt 3); nur der Weg der Darstellung ändert sich.

## Folgen

- Entscheidung 001 gilt nur noch für die Simulation in Go, nicht mehr für den Browser als Client.
- B-823 wird vor der Umsetzung in Teil-Tickets zerlegt: Browser-Client entfernen (`src/`, Vite, `package.json`,
  Dockerfile, CI), Server-Werkzeuge prüfen, `site/`-Bilder, „Im Spiel ansehen“ nativ.
- `CLAUDE.md` und `codemap.md` beschreiben danach nur noch das native Spiel.
- Risiko: Abnahmen, die bisher im Browser liefen (Seiten-Regeln, Xbox-Navigation), entfallen ersatzlos; Gamepad-Abnahmen
  laufen am PC bzw. Mini-PC (NG14.5, NG9.5).
