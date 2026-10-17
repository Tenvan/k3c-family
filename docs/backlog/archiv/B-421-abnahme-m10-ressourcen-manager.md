# B-421 · Abnahme M10: Ressourcen-Manager in k3c-dev und Mini-Szenen durch 🧑 beobachten

- **Domäne:** DEV
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** –
- **Projekt:** ABN
- **Erstellt:** 2026-10-14
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Sprint M10 (B-298, B-299) ist gebaut und mit Tests belegt. Die Beobachtungs-Kriterien hat nur der Agent im Browser-Pane mit Mock-Backend gesehen. Das k3c-dev-Fenster (Wails) mit dem echten Register, der Dateidialog und eine echte Netzsuche sind offen (`docs/sprints/erledigt/M10-ressourcen-manager/`, Ergebnisse M10.3 bis M10.5).

## Ziel

🧑 hat die Beobachtungs-Kriterien von B-298 und B-299 in k3c-dev mit echtem Register gesehen; B-298 und B-299 können archiviert werden.

## Beteiligte und Zielgruppen

🧑 (Ralf) beobachtet und entscheidet; der Agent behebt Funde.

## Anforderungen

- Beobachtung im Wails-Fenster von k3c-dev (`task k3c-dev`) mit `data/assets.json`, Vite-Dienst läuft.
- Für die Netzsuche ein freesound-Token in `.work/freesound-token` (nur lokal).

## Nicht-Ziele

Neue Funktionen; Befund aus B-420 (Credit-Test für Importe) ist ein eigenes Ticket.

## Regeln und Einschränkungen

Abnahmen und Freigaben nur durch 🧑 (`docs/arbeitsweise.md` › SDD). Lizenz bestätigen geschieht nur in der Oberfläche (Q70). Echte Importe nur nach Rücksprache einchecken.

## Beispiele

Reiter „Assets“ öffnen, Slot `farm` wählen, Suche „house“, Treffer mit „+“ anhängen → Mini-Szene Gebäude zeigt die Zuordnung.

## Ausnahme- und Fehlerfälle

Quelle der Recherche nicht erreichbar → Hinweis an der Quelle, die übrigen liefern weiter (M10.4). Fehlt der freesound-Token, meldet die Quelle „token fehlt“.

## Akzeptanzkriterien

- **AC-01** (B-298/AC-01) Im k3c-dev-Fenster zeigt der Reiter „Assets“ die Umschalter Grafik und Sound mit den echten Slots (127, davon 26 Lücken) und den Filtern (Beobachtung 🧑).
- **AC-02** (B-298/AC-03) Ein Ausschnitt wird im Fenster gespeichert (z. B. `hub:3`) und in der Vorschau angezeigt; ein Ausschnitt außerhalb des Bildes wird abgelehnt (Beobachtung 🧑).
- **AC-03** (B-298/AC-04) „Recherche“ an einem Slot liefert echte Treffer mit Link, Quelle und Lizenz (OpenGameArt, Commons, mit Token freesound); sie bleiben nach Neustart des Fensters stehen (Beobachtung 🧑).
- **AC-04** (B-298/AC-05) Der Dateidialog „Importieren“ legt eine Datei unter `public/…/kandidaten/` ab, die Credit-Zeile steht mit „Lizenz prüfen“, „Lizenz bestätigen“ setzt sie auf bestätigt (Beobachtung 🧑; Testdatei danach nicht einchecken).
- **AC-05** (B-298/AC-07) Die eingebettete Mini-Szene im Slot-Detail folgt der gewählten Präferenz; nach dem Speichern lädt sie neu (Beobachtung 🧑).
- **AC-06** (B-299/AC-01) `assetvorschau.html` zeigt alle fünf Grafik-Szenen im Spielmaßstab, Lücken als rote Platzhalter (Beobachtung 🧑).
- **AC-07** (B-299/AC-02) Auf `assetvorschau.html` spielen `sfx:coin` und der Wechsel `music:tag` → `music:nacht` auf Knopfdruck (Beobachtung 🧑 mit Ton).

## Offene Fragen

keine

## Notizen

Entstanden aus der Review-Session M10.6. Danach B-298 und B-299 auf erledigt setzen.
