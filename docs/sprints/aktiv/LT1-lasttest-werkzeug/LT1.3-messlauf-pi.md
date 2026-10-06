# LT1.3 · Messlauf am Pi über eine Nacht

- **Status:** offen
- **Typ:** Workshop
- **Agent:** Mensch
- **Umgebung:** live
- **Branch:** lt1/3-messlauf-pi
- **Abhängig von:** LT1.2
- **Tickets:** B-175, B-042
- **Kriterien:** AC-06

## Ziel

🧑 hat am Pi 2 Räume × 3 Spieler über eine Nacht gemessen; Tabelle und Bewertung stehen in dieser Datei und in B-042.

## Kontext

Hardware-Session nach `docs/arbeitsweise.md` › Hardware entkoppelt: keine Abhängigkeit des Reviews LT1.4; bis zur Messung gilt das angenommene Ziel aus B-042 (Tick-p99 < 10 ms bei 30 Hz, 2 Räume × 3 Spieler; Handmessung 2026-10-03: Nacht 10,2 bis 10,3 ms). Dauer etwa 15 Minuten Arbeit plus eine Nacht Laufzeit. Der Pi-Server braucht `K3C_STATUS_TOKEN`. Ein Agent kann den Bericht danach auswerten und die Tabelle übertragen.

## Erlaubte Dateien

- diese Datei (Ergebnis, Status), `docs/backlog/B-042-*.md` (Ergebnis, Status, Archiv), `docs/backlog/README.md`
- Sprint-README (nur Tabelle der Sessions), `reports/` (Bericht)

## Nicht-Ziele

Code-Änderungen; Optimierungen am Server. Ziel verfehlt → Ticket (SRV), keine stille Absenkung des Ziels.

## Schritte

1. Stand mit `task load` auf den Pi bringen bzw. vom Rechner im Heimnetz gegen den Pi starten: `task load -- -url http://<pi>:8080 -token <T> -rooms 2 -players 3 -duration night`.
2. Nach dem Lauf: Markdown-Tabelle und Bewertung in das Ergebnis dieser Datei und in B-042 übernehmen.
3. Bewertung `verfehlt` oder `knapp` → Ticket mit Messwerten (SRV).
4. B-042 auf `erledigt` setzen und archivieren, wenn die Bewertung vorliegt; `Status: fertig`.

## Fertig, wenn

- [ ] AC-06: Tabelle und Bewertung des Pi-Laufs stehen im Ergebnis und in B-042; B-042 ist archiviert.

## Prüfen

Manuell durch 🧑 am Pi.

## Ergebnis

–
