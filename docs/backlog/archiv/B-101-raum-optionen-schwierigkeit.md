# B-101 · Raum-Optionen und fünf Schwierigkeitsgrade wirken in der Simulation

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** SP13
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf) per /goal „SP13 vorbereiten und im Team komplett abarbeiten“, Revision 1

## Ausgangslage

Wellen und Gegner sind fest (`data/waves.json`), unabhängig von der Spieleranzahl; es gibt keine Grade und keine Raum-Optionen.

## Ziel

Ein Raum hat Optionen (Schwierigkeitsgrad Dev/Leicht/Normal/Hart/Ultra, Ziel, Niederlage-Modus); die Wellen wachsen mit der Spieleranzahl. Nutzen: Dev leichter, Live stufbar, ohne Debug-Panel.

## Beteiligte und Zielgruppen

Spieler und Entwickler; Werte pflegt REG.

## Anforderungen

- `data/difficulty.json` mit Faktoren je Grad (Wellengröße, Gegner-HP, Gegner-Schaden; Startwerte aus `docs/rules/wirtschaft.md` § 4); die Wirtschaft bleibt gleich.
- `data/waves.json`: Faktor `perExtraPlayer` 0,5; Wellengröße × (1 + 0,5 × (Spieler der Insel − 1)), gerundet.
- Der Grad wirkt ab der nächsten Welle, nie rückwirkend; er steht im Spielstand und im Raum.
- Ziel und Niederlage-Modus als Raum-Optionen im Raum und im Spielstand (Wirkung: B-102).

## Nicht-Ziele

Anlegen-Dialog (B-105), Debug-Panel (B-107), Protokoll (B-104), Siegvarianten und Niederlage-Modi (B-102).

## Regeln und Einschränkungen

`docs/rules/wirtschaft.md` § 4, `stufen.md` § 5; Werte nur in `data/`. Dev ist nur im Dev-Mode wählbar. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Raum Normal, 1 Spieler, Welle 3 → Gegnerzahl laut Tabelle; mit 3 Spielern ×2; Grad Hart ×1,25 HP ×1,3 darüber.

## Ausnahme- und Fehlerfälle

Unbekannter Grad im Spielstand → Normal und Hinweis im Log. Dev in Live angefordert → abgelehnt.

## Akzeptanzkriterien

- **AC-01** Test: Wellengröße je Spieleranzahl 1–4 entspricht der Formel (gerundet).
- **AC-02** Test: je Grad gelten die Faktoren aus `difficulty.json` für Wellengröße, HP und Schaden.
- **AC-03** Test: ein Gradwechsel mitten in der Nacht wirkt erst ab der nächsten Welle.
- **AC-04** Spielstand speichert und lädt Grad, Ziel und Niederlage-Modus.
- **AC-05** Golden-Daten sind aktualisiert; `task check:go` grün.

## Offene Fragen

keine

## Notizen

Aus R1.2 und R1.3. Abhängigkeit B-104 (Protokoll) für die Anzeige.

SP13 (2026-10-02) hat den SIM-Teil umgesetzt (Insel-Optionen, Grade, Wellenfaktor, Gradwechsel, fünf Materialien, Lager-Maximum, Lager-Gebäude, Tragen). Offen für den Rest: Protokoll (B-104, B-123), Raum (B-133), Dialog (B-105), Debug-Panel (B-107), Anzeige (B-117); Wirkung von Ziel und Niederlage-Modus: B-102.
