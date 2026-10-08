# B-113 · Fünf Materialien, Lager-Maximum und Tragen zum Lager sind umgesetzt

- **Domäne:** SIM
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** offline
- **Status:** erledigt
- **Sprint:** SP13
- **Projekt:** –
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-02, Chat (Ralf) per /goal „SP13 vorbereiten und im Team komplett abarbeiten“, Revision 1

## Ausgangslage

`World.stock` kennt nur Holz, Stein, Kupfer ohne Maximum; der Bauer trägt zur Burg (`engine/sim/units.go` › `carry`). Eisen und Kristall gibt es nicht; Kupfer hat keine Verwendung.

## Ziel

Der Insel-Vorrat hat fünf Materialien, ein Maximum je Rohstoff (300 je Hub plus 300 je Lager) und Arbeiter bringen Material zum nächsten Lager oder zur Burg. Nutzen: Material wird planbar; das Lager ist ein Ausbauziel.

## Beteiligte und Zielgruppen

Spieler und Bauern; Messung mit B-099.

## Anforderungen

- Vorrat mit Holz, Stein, Kupfer, Eisen, Kristall (je Insel gemeinsam, B-100); Kapazität je Rohstoff = 300 × Hubs + 300 × Lager der Insel (Annahme: Summe).
- `data/economy.json` › `storage` (Basis 300, je Lager 300).
- Gebäude **Lager** (Hub-Stufe 2; Startwert 50 Stein + 20 Gold, HP 200, Bauzeit 8 s): erhöht die Kapazität; Bauern bringen Material zum nächsten Lager oder zur Burg ihres Hubs.
- Ist das Maximum voll, bleibt das Material liegen und der Arbeiter wartet; nichts geht verloren.
- Spielstand speichert den Vorrat mit fünf Materialien (`SAVE_VERSION`).

## Nicht-Ziele

Plantage und Adern (B-114), Ausbau (B-112), Anzeige (B-117).

## Regeln und Einschränkungen

`docs/rules/materialien-gebaeude.md` § 1 und § 3.2. Datei ≤ 400 Zeilen, Funktion ≤ 60, Schichtgrenzen aus `docs/arbeitsweise.md`; deterministisch (`engine/rng`), 2+ Spieler.

## Beispiele

Vorrat Stein 295/300, Bauer bringt 10 Stein → 5 werden aufgenommen, 5 bleiben beim Bauer, er wartet; ein Lager (+300) löst das.

## Ausnahme- und Fehlerfälle

Lager zerstört → Kapazität sinkt, Überschuss bleibt im Vorrat bis verbraucht (kein Verlust beim Absinken).

## Akzeptanzkriterien

- **AC-01** Test: Kapazität je Rohstoff entspricht der Formel mit n Hubs und m Lagern.
- **AC-02** Test: Bauer bringt Material zum nächstgelegenen Lager oder zur Burg; bei vollem Maximum wartet er und nichts geht verloren.
- **AC-03** Test: Eisen und Kristall sind im Vorrat, Spielstand speichert und lädt fünf Materialien; ein alter Stand wird geladen.
- **AC-04** Golden-Daten aktualisiert; `task check:go` grün.

## Offene Fragen

Wirkung beim Absinken der Kapazität (zerstörtes Lager): Überschuss bleibt (Annahme).

## Notizen

Aus R2.2. Abhängig von B-100 (Insel-Vorrat).

SP13 (2026-10-02) hat den SIM-Teil umgesetzt (Insel-Optionen, Grade, Wellenfaktor, Gradwechsel, fünf Materialien, Lager-Maximum, Lager-Gebäude, Tragen). Offen für den Rest: Protokoll (B-104, B-123), Raum (B-133), Dialog (B-105), Debug-Panel (B-107), Anzeige (B-117); Wirkung von Ziel und Niederlage-Modus: B-102.
