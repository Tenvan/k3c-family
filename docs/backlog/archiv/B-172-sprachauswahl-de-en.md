# B-172 · Der Client hat Deutsch und Englisch mit Sprachauswahl in den Optionen

- **Domäne:** CLI
- **Typ:** Idee
- **Prio:** mittel
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** S5
- **Erstellt:** 2026-10-03
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint S5

## Ausgangslage

Alle Texte des Clients stehen deutsch und verstreut in den Szenen (`src/scenes/HudScene.ts`, `LobbyScene.ts`, `lobbyLogic.ts`, `src/online/clientConnection.ts` › `TEXT`). 🧑 hat am 2026-10-03 entschieden (Fragenkatalog Q05): Deutsch und Englisch, Auswahl in den Optionen, neue Texte zentral sammeln.

## Ziel

Alle Spieltexte kommen aus zentralen Textdateien (de, en); die Sprache wird in der Optionen-Szene (B-146) gewählt und je Gerät gespeichert. Nutzen: Englisch ohne Umbau, auch für einen späteren itch.io-Auftritt (B-023).

## Beteiligte und Zielgruppen

Spieler am TV und am Handy; Umsetzung durch Agent; 🧑 prüft die englischen Texte.

## Anforderungen

- Zentrale Textverwaltung `src/core/texts.ts` (oder gleichwertig) mit Schlüsseln und Platzhaltern; Deutsch ist Standard und Rückfall, wenn ein englischer Text fehlt.
- Sprachwahl in der Optionen-Szene, gespeichert je Gerät (wie die übrigen Optionen aus B-146), wirkt ohne Neuladen oder nach Neuladen (offen, siehe unten).
- Alle bestehenden Texte des Clients wandern in die Textdateien; neue Texte (Hinweise B-148, Menüs, Skills) von Anfang an dort.
- Serverseitige Fehlertexte (`message` im Protokoll) bleiben Codes, der Client übersetzt über den Code.

## Nicht-Ziele

Weitere Sprachen, Übersetzung von Dokumentation, Lokalisierung der Sound-Dateien.

## Regeln und Einschränkungen

`src/scenes` rechnet nichts; Texte nur lesen. Keine neue Abhängigkeit ohne Ticket. Länge der englischen Texte darf die Mindest-Schriftgrößen aus B-136 nicht sprengen. Datei ≤ 400 Zeilen, Funktion ≤ 60.

## Beispiele

Sprache Englisch gewählt, Lobby öffnet → „Play“ statt „Spielen“, Verbindungsstatus „Server unreachable“.

## Ausnahme- und Fehlerfälle

Schlüssel fehlt in Englisch → deutscher Text, ein Eintrag im Client-Log. Gespeicherte Sprache unbekannt → Deutsch.

## Akzeptanzkriterien

- **AC-01** Test: Jeder Schlüssel der deutschen Datei hat einen englischen Text, sonst schlägt der Test fehl.
- **AC-02** Test: Fehlender Schlüssel ergibt den deutschen Text, unbekannte gespeicherte Sprache ergibt Deutsch.
- **AC-03** Die Optionen-Szene wechselt die Sprache, sie bleibt nach Neuladen erhalten.
- **AC-04** Im Client stehen keine deutschen Text-Literale mehr außerhalb der Textdateien (Prüfung per Test oder Skript über `src/scenes` und `src/online`).
- **AC-05** 🧑 hat die englischen Texte am Gerät gelesen und abgenommen.

## Offene Fragen

Sprachwechsel sofort oder erst nach Neuladen (Szenen bauen Texte beim Start auf): Entscheidung bei der Umsetzung, 🧑 prüft.

## Notizen

Aus Fragenkatalog Q05 (2026-10-03). Ersetzt die Antwort „nur Deutsch“ aus B-145.
