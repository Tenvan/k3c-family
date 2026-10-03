# W1 · SIM · Hub-Ausbau und Mauerstufen

- **Status:** geplant
- **Domäne:** SIM
- **Reife:** Entwurf
- **Einschiebbar:** nein
- **Tickets:** B-112
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Der Hub hat feste Bauplätze, Mauer und Turm nur die Holzstufe, es gibt weder Hub-Ausbau noch Reparatur (B-112). Voraussetzung ist der Insel-Vorrat aus dem Lager (SP13).

## Ziel

Hub-Stufen 1 bis 5 und Mauer-/Turm-Stufen 1 bis 5 sind in der Simulation spielbar, Reparatur zwischen den Wellen eingeschlossen.

Am Ende sichtbar: `task check:go` grün, Tests für Ausbau, Zerstörung und Reparatur, aktualisierte Golden-Daten.

## Beteiligte und Zielgruppen

Spieler (2+ Monarchen) und Bauern; Werte pflegt REG; 🧑 gibt die Spec frei.

## Anforderungen

B-112 › Anforderungen.

## Nicht-Ziele

Gebäude-Wirkungen (W3), Protokoll (W5), Anzeige (W6, B-117).

## Regeln und Einschränkungen

Werte nur in `data/`, Logik und Tests in `engine/sim/`; deterministisch (`engine/rng`), mit 2+ Spielern gleichzeitig. Golden-Daten nach dem Golden-Ablauf (B-137) aktualisieren, Spielstand-Änderungen nach der Migrationsregel (B-137). Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`. Der Sprint bleibt in der Domäne SIM. Quelle: `docs/rules/materialien-gebaeude.md`.

## Beispiele

Hub-Stufe 1, Vorrat 100 Stein, 50 Gold gezahlt → Material abgebucht → Bauer baut → Hub-Stufe 2 → Steinmauer baubar.

## Ausnahme- und Fehlerfälle

Material fehlt → Ausbau wartet. Ausbau während einer Welle → erlaubt, der Bauer baut nach der Gefahr.

## Akzeptanzkriterien

- **AC-01** Hub-Ausbau auf Stufe 2 bis 5 bucht Gold und Material laut `hub.json` ab; Gebäude höherer Stufen sind vor dem Ausbau nicht bezahlbar (Test) (B-112/AC-01, B-112/AC-03).
- **AC-02** Mauer und Turm werden am selben Platz auf Stufe 2 bis 5 ausgebaut, Kosten und HP laut Daten, Stufe 3 braucht Hub-Stufe 3 (Test) (B-112/AC-02).
- **AC-03** Zerstörung setzt die Mauer-Stufe zurück, die Hub-Stufe bleibt; Reparatur stellt HP zwischen den Wellen ohne Gold und Material her (Test) (B-112/AC-04, B-112/AC-05).
- **AC-04** Spielstand speichert Hub- und Gebäude-Stufen, Golden-Daten sind aktualisiert, `task check:go` grün (B-112/AC-06).

## Offene Fragen

keine

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- W1.1 Hub-Ausbau und Freischaltung nach Hub-Stufe in `data/hub.json` und `engine/sim/` (AC-01).
- W1.2 Mauer-/Turm-Stufen, Zerstörung, Reparatur, Spielstand und Golden (AC-02, AC-03, AC-04).
- W1.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
