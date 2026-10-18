# BT1 · SRV, DEV · Server im Heimnetz finden, Windows-Starter, Start mit Seed

- **Status:** aktiv
- **Projekt:** REL
- **Domäne:** SRV, DEV
- **Reife:** bereit
- **Tickets:** B-041, B-095, B-048
- **Start-Commit:** c7ea4fd7
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-18, Chat (Ralf), Revision 1

## Ausgangslage

Geräte brauchen die IP (B-040), der PC-Start braucht die Shell (B-041), ein Seed lässt sich nicht per URL starten (B-095); Entscheidung 001 verweist unscharf auf die Standardbibliothek (B-048).

## Ziel

Der Server startet am PC per Doppelklick, ein Seed startet per URL. Am Ende sichtbar: Doppelklick startet den Server am PC; URL startet Seed und Tiefe. (Das Finden des Servers ohne IP per QR-Code für das Handy entfällt seit 2026-10-11, B-040 ist verworfen.)

## Beteiligte und Zielgruppen

Familie an PC und Xbox; 🧑 betreibt den Server im Heimnetz.

## Anforderungen

B-041 › Anforderungen; B-095 › Anforderungen; B-048 › Anforderungen. (B-040 › Anforderungen entfällt, B-040 ist verworfen.)

## Nicht-Ziele

Internet-Zugriff, Konten.

## Regeln und Einschränkungen

Domänen SRV (BT1.1, Protokoll mit beiden Enden) und DEV (BT1.2, `tools/k3c-starter`); keine neue Abhängigkeit außer Wails (bereits im Repo). Protokoll-Änderung nur in einer eigenen Session (Arbeitsweise › Protokoll). AC-02 verlangt eine Abnahme am PC durch 🧑 (live).

## Beispiele

`game.html?autostart=1&fresh=1&seed=42&depth=2` → neues Spiel, Seed 42, Tiefe 2.

## Ausnahme- und Fehlerfälle

Port belegt → Meldung im Fenster des Starters (B-041); ungültiger Seed oder ungültige Tiefe → Fehlermeldung des Servers, kein Raum (B-095). Die Server-Suche entfällt (B-040 verworfen).

## Akzeptanzkriterien

- **AC-01** Entfällt (Beschluss 🧑 2026-10-11, B-040 verworfen: QR-Code und Handy); ehemals „Geräte finden den Server im Heimnetz“ (B-040/AC-01).
- **AC-02** Der Windows-Starter `tools/k3c-starter` existiert und läuft per Doppelklick (B-041/AC-01, B-041/AC-02).
- **AC-03** Ein neues Spiel startet per URL mit eigenem Seed und gewählter Tiefe (B-095/AC-01, B-095/AC-02, B-095/AC-03).
- **AC-04** Die Wahl der Go-Standardbibliothek ist dort festgehalten, wo B-001 auf sie verweist (B-048/AC-01).

## Offene Fragen

keine (Entscheidungen 🧑 2026-10-18: neues Protokollfeld `seed` (B-095), Entscheidung 001 um einen Satz ergänzen (B-048), Starter als `tools/k3c-starter` in Domäne DEV (B-041); B-040 verworfen).

## Sessions

| Nr. | Datei | Typ | Agent | Status |
|---|---|---|---|---|
| BT1.1 | `BT1.1-seed-per-url.md` | Umsetzung | autonom | fertig |
| BT1.2 | `BT1.2-windows-starter.md` | Umsetzung | autonom | offen |
| BT1.3 | `BT1.3-review.md` | Review | autonom | offen |

Entwurf. Vor dem Aktivieren jede Session als Datei nach `docs/vorlagen/session.md` schreiben, die Kriterien in Klammern werden ihr Feld `Kriterien`.

- BT1.1 Start mit Seed und Tiefe per URL, Nachweis B-048 (AC-03, AC-04).
- BT1.2 Wails-Starter für Windows (AC-02); Vorbild sind die Wails-Fenster von k3c-dev und Asset-Studio.
- BT1.3 Review (Code-Sprint): alle Kriterien prüfen.

## Abnahme

–
