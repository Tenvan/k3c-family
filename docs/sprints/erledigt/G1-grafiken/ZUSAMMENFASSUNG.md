# G1 · Zusammenfassung

Regelbasiert aus der Sprint-README erzeugt (`plan_digest`, B-696); maßgeblich bleibt [README.md](README.md).

- **Titel:** Referenzseite für die gewählten Grafik-Packs
- **Projekt:** –

## Ziel

Die zwölf Packs liegen mit Lizenz im Repo und sind auf `grafiken.html` ansehbar. Am Ende sichtbar: Kachel „Alle Grafiken“ → Seite mit allen Packs, Urhebern, Lizenzen, Quellen und den Bildern in wählbarer Vergrößerung.

## Ergebnis

Reviews 2026-10-01 (G1.2, G1.4): `task check` grün (701 Tests), `task build` nimmt Seite und `public/grafik/` (921 KB) auf; keine schweren Befunde; Kopieren der Auswahl über http ergänzt. Prüfer war dieselbe Sitzung wie die Umsetzung. AC-01 bis AC-03 und AC-05 durch `grafikPacks.test.ts` und `selection.test.ts` belegt (zwölf Packs mit Lizenzdatei, Credits in `CREDITS.md` und `lizenzen.html`, Auswahl auf drei Seiten). **AC-04 und AC-05 abgenommen 2026-10-01 durch 🧑 (Ralf)** am lokalen Browser: „passt alles“, alle zwölf Packs übernommen (auch Warped Caves unter CC BY 3.0). Einzelschritte und die …

## Kriterien

| Kriterium | Status | Inhalt |
|---|---|---|
| AC-01 | geprüft | Alle zwölf Packs liegen unter `public/grafik/<pack>/` mit `LICENSE.txt`, ohne Musik, Code … |
| AC-02 | geprüft | `public/grafik/CREDITS.md` und `lizenzen.html` nennen jedes Pack mit Quelle, Urheber und L … |
| AC-03 | – | `grafiken.html` ist von der Landingpage erreichbar und zeigt alle Packs mit allen Bildern … |
| AC-04 | – | 🧑 hat die Seite angesehen und die Packs abgenommen (B-087/AC-04). |
| AC-05 | – | Auf `grafiken.html`, `figuren.html` und `aufstellung.html` kann 🧑 Figuren, Packs und Bilde … |

## Tickets

| Ticket | Status | Kriterien | Titel |
|---|---|---|---|
| B-087 | erledigt | AC-01 geprüft, AC-02 geprüft, AC-03 –, AC-04 –, AC-05 – | Eine Referenzseite zeigt die gewählten CC0-Grafik-Packs für Gebäude, Ressourcen … |
