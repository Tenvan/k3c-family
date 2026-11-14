# M3 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** k3c-dev III: Dienste führen
- **Projekt:** –

## Ziel

`k3c-dev` startet, überwacht und stoppt die Entwicklungs-Dienste; Agenten nutzen dafür `svc_*`-Tools. Am Ende sichtbar: `svc_start Vite` startet den Dev-Server, `svc_status` zeigt ihn mit PID, CPU und Speicher, `console_tail Vite` seine Ausgabe, `svc_stop Vite` beendet ihn samt Kindprozessen.

## Ergebnis

- 2026-09-30, leichtes Review über `9a85ce3..main` (25 Code-Dateien) durch drei Reviewer (Sonnet): Controller, Prozesse/Übernahme, Tools/Verdrahtung. Kriterien: AC-01 (M3.1), AC-02 und AC-03 (M3.2), AC-04 und AC-05 (M3.3) geprüft. - Behoben (schwer): `Kill` nach `Wait` schickte `taskkill /T /F` an eine freie, evtl. neu vergebene PID; `svc_stop force` beendete die PID von der Übernahme ungeprüft; übernommener bzw. eigener Prozess, der nicht endet, galt als `gestoppt`. - Behoben (bestätigte Vermutungen): Port-Besitzer nur an 127.0.0.1/0.0.0.0/:: und nie PID ≤ 4; `Seq` je Status gegen ungeordnete …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Konfiguration und Zustandsmaschine mit Health-Prüfung und Auto-Restart (B-067/AC-01, B-067 … |
| AC-02 | geprüft | Übernahme, `Port belegt` und Metriken (B-067/AC-03, B-067/AC-04). |
| AC-03 | geprüft | Konsole und Log-Level-Zähler je Dienst (B-067/AC-06). |
| AC-04 | geprüft | `svc_*`-Tools und Beenden (B-067/AC-05). |
| AC-05 | geprüft | Von Hand geprüft, `check:dev` grün (B-067/AC-07). |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-067 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 geprüft, AC-04 geprüft, AC-05 geprüft | k3c-dev startet, überwacht und stoppt die Entwicklungs-Dienste, auch für Agenten |
