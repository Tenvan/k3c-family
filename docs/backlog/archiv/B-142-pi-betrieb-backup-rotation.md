# B-142 · Spielstände werden außerhalb des Pi gesichert, Berichte und Logs rotieren

- **Domäne:** SRV
- **Typ:** Idee
- **Prio:** hoch
- **Umgebung:** live
- **Status:** erledigt
- **Sprint:** F4
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint F4

## Ausgangslage

Der Server legt Sicherungen eines Spielstands unter `saves/backups/<slot>/` ab, je Slot bleiben 5 (`engine/store/backups.go`, `BackupKeep`). Sie liegen auf demselben Datenträger wie der Spielstand (auf dem Pi: dieselbe SD-Karte, Volume `k3c-data`, `compose.yaml`). Berichte unter `reports/` (`engine/store/reports.go`) und das Client-Log (`cmd/k3c-server/logging.go`, `/api/clientlog`) haben keine Obergrenze für die Gesamtgröße; die SD-Karte kann volllaufen. Ein Netzlast-Ziel für den Pi (KB/s je Client) gibt es nicht.

## Ziel

Der Spielstand liegt zusätzlich auf einem zweiten Gerät, Reports und Client-Log füllen den Datenträger nicht, und ein Netzlast-Ziel ist als Zahl festgelegt. Nutzen: Ein SD-Kartenausfall kostet keine Spielstände, der Pi läuft dauerhaft.

## Beteiligte und Zielgruppen

🧑 betreibt den Pi und entscheidet das Backup-Ziel (Beschluss Q18); Familie spielt.

## Anforderungen

- Backup-Skript oder -Task kopiert `saves/` auf ein zweites Gerät (PC oder NAS, Ziel nach Q18); der Befehl steht in der README (Abschnitt Docker/Pi); es läuft ohne Zugriff auf Tokens aus dem Image.
- Reports und Client-Log haben eine Obergrenze (Anzahl Dateien oder Gesamtgröße in MB, Zahl aus der Umsetzung begründet); die ältesten werden zuerst gelöscht, Spielstände nie.
- Netzlast-Ziel (KB/s je Client) steht in `docs/protocol.md`; Messung gehört zu B-140.

## Nicht-Ziele

Pi-Einrichtung und Lastmessung (SP11), Absicherung der Endpunkte (B-143), Protokoll der Ereignisse (B-140), automatische Cloud-Sicherung.

## Regeln und Einschränkungen

Domäne SRV (`engine/store/`, `cmd/`, Docker, README); Standardbibliothek zuerst, keine neue Abhängigkeit ohne Zustimmung von 🧑. Löschen von Berichten und Logs nur in deren Ordnern, nie in `saves/`.

## Beispiele

120 Berichte im Ordner `reports/` bei Obergrenze 100 → die 20 ältesten werden gelöscht, der Server loggt die Zahl.

## Ausnahme- und Fehlerfälle

Zweites Gerät nicht erreichbar → das Backup meldet den Fehler mit Zeit im Log, der Spielbetrieb läuft weiter. Datenträger fast voll → Rotation läuft vor dem Schreiben des neuen Berichts.

## Akzeptanzkriterien

- **AC-01** Go-Test: Bei überschrittener Obergrenze löscht die Rotation die ältesten Berichte und lässt `saves/` unberührt (`task check:go`).
- **AC-02** Go-Test: Das Client-Log überschreitet seine Obergrenze nicht über 10 000 Meldungen (`task check:go`).
- **AC-03** Die README beschreibt den Backup-Befehl auf ein zweites Gerät; ein Testlauf mit einem lokalen Zielordner kopiert alle Dateien aus `saves/` (Befehl, Ausgabe).
- **AC-04** `docs/protocol.md` nennt ein Netzlast-Ziel in KB/s je Client (Sichtprüfung); `task check` grün.

## Offene Fragen

Backup-Ziel PC oder NAS und Rhythmus: `docs/fragenkatalog.md` Q18, entscheidet 🧑.

## Notizen

Aus Plan Lücken 14 und 15 (SP11-Ergänzung). Das eigentliche Backup am Pi richtet 🧑 ein (Agent: Mensch), der Agent liefert Skript und Doku.

Beschluss 2026-10-03 (Q18): Backup-Ziel ist ein USB-Stick am Pi; Restore-Probe einmal durchspielen. Hinweis: gleiches Gerät wie der Pi, schützt nur vor SD-Karten-Ausfall, nicht vor Verlust des Pi.
