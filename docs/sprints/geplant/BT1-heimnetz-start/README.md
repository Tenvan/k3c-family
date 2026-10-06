# BT1 · SRV · Server im Heimnetz finden, Windows-Starter, Start mit Seed

- **Status:** geplant
- **Domäne:** SRV
- **Prio:** niedrig
- **Reife:** Entwurf
- **Einschiebbar:** ja
- **Tickets:** B-040, B-041, B-095, B-048
- **Start-Commit:** –
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Geräte brauchen die IP (B-040), der PC-Start braucht die Shell (B-041), ein Seed lässt sich nicht per URL starten (B-095); Entscheidung 001 verweist unscharf auf die Standardbibliothek (B-048).

## Ziel

Ein neues Gerät tritt ohne IP-Eingabe bei, der Server startet am PC per Doppelklick, ein Seed per URL. Am Ende sichtbar: Handy findet den Server ohne IP; Doppelklick startet ihn am PC; URL startet Seed und Tiefe.

## Beteiligte und Zielgruppen

Familie an Handy und PC; 🧑 entscheidet den Weg der Server-Suche.

## Anforderungen

B-040 › Anforderungen; B-041 › Anforderungen; B-095 › Anforderungen; B-048 › Anforderungen.

## Nicht-Ziele

Internet-Zugriff, Konten.

## Regeln und Einschränkungen

SRV; keine neue Abhängigkeit ohne Begründung im Ticket.

## Beispiele

`game.html?autostart=1&fresh=1&seed=42&depth=2` → neues Spiel, Seed 42, Tiefe 2.

## Ausnahme- und Fehlerfälle

Server nicht gefunden → Eingabefeld für die Adresse bleibt.

## Akzeptanzkriterien

- **AC-01** Geräte finden den Server im Heimnetz (B-040/AC-01).
- **AC-02** Wails-Starter für Windows existiert (B-041/AC-01).
- **AC-03** Ein neues Spiel startet per URL mit eigenem Seed und gewählter Tiefe (B-095/AC-01, B-095/AC-02, B-095/AC-03).
- **AC-04** Die Wahl der Go-Standardbibliothek ist dort festgehalten, wo B-001 auf sie verweist (B-048/AC-01).

## Offene Fragen

- Weg der Server-Suche (mDNS, Broadcast, QR-Code): entscheidet 🧑 (B-040).

## Sessions

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BT1.1 Start mit Seed und Tiefe per URL, Nachweis B-048 (AC-03, AC-04).
- BT1.2 Server-Suche im Heimnetz (AC-01).
- BT1.3 Wails-Starter für Windows (AC-02).
- BT1.4 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
