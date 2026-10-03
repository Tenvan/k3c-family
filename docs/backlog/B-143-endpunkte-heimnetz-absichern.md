# B-143 · Restore-, Save- und Report-Endpunkte sind im Heimnetz abgesichert

- **Domäne:** SRV
- **Typ:** Problem
- **Prio:** mittel
- **Status:** eingeplant
- **Sprint:** F4
- **Erstellt:** 2026-10-02
- **Spec:** freigegeben
- **Revision:** 1
- **Freigabe:** 2026-10-03, Chat (Ralf), mit Sprint F4

## Ausgangslage

`engine/net/handler.go` registriert `POST /api/save/restore`, `POST /api/save`, `POST /api/report` und `POST /api/clientlog` ohne Anmeldung; nur `/api/status*` verlangt das Token (`engine/net/status.go` › `authorized`). Jedes Gerät im Heimnetz kann damit einen Spielstand zurückspielen oder überschreiben und Berichte schreiben. Der Pi hängt dauerhaft im Netz. Es gibt Größenlimits (z. B. `store.MaxReportBytes` 256 KB, Client-Log 64 KB je Anfrage), aber keine Begrenzung auf vertrauenswürdige Geräte.

## Ziel

Schreibende Endpunkte sind im Heimnetz gegen versehentlichen und fremden Zugriff geschützt, ohne dass die Familie auf der Xbox ein Passwort eintippen muss. Nutzen: Ein Fremdgerät im WLAN kann keine Spielstände zerstören.

## Beteiligte und Zielgruppen

Familie (Xbox, Handys, Tablets), Betreiber des Pi; 🧑 entscheidet das Schutzmodell (Beschluss Q17).

## Anforderungen

- Das Schutzmodell (z. B. Restore nur mit Token, Save/Report nur aus privaten Adressbereichen, Rate-Limit) steht als Entscheidung 🧑 in der Spec, bevor Code entsteht.
- `POST /api/save/restore` ist nach dem Modell geschützt; das Spiel selbst (Lobby, WebSocket) funktioniert unverändert ohne Eingabe eines Geheimnisses.
- Jeder abgelehnte Zugriff steht im Log (Zeit, Pfad, Gerät gekürzt, Grund) und antwortet 401 oder 403.
- Es gibt einen Test je geschütztem Endpunkt für erlaubt und abgelehnt.

## Nicht-Ziele

Öffentliche Erreichbarkeit aus dem Internet, Benutzerkonten, HTTPS-Zertifikate (`certs/` bestehen bereits).

## Regeln und Einschränkungen

Domäne SRV (`engine/net/`); Standardbibliothek zuerst; Token nie in Logs; keine neue Abhängigkeit ohne Zustimmung von 🧑. Der Spielbetrieb ohne gesetztes Token bleibt möglich (wie heute bei `/api/status`).

## Beispiele

Handy im WLAN ruft `POST /api/save/restore` ohne Berechtigung auf → 401, ein Eintrag im Log; die Lobby auf demselben Handy lädt und speichert weiter.

## Ausnahme- und Fehlerfälle

Server hinter einem Proxy (Adresse nicht die des Geräts) → das Modell nennt, welche Adresse gilt. Token nicht gesetzt → Verhalten laut Entscheidung (Standard: Restore aus, wie `/api/status` ohne Token).

## Akzeptanzkriterien

- **AC-01** Go-Test: `POST /api/save/restore` ohne Berechtigung antwortet 401 oder 403, mit Berechtigung 200; der Spielstand ist nur im zweiten Fall geändert (`task check:go`).
- **AC-02** Go-Test: Lobby, WebSocket und `GET /api/save` funktionieren ohne Berechtigung unverändert (`task check:go`).
- **AC-03** Go-Test: Ein abgelehnter Zugriff erzeugt genau einen Logeintrag ohne Token im Text.
- **AC-04** Die README beschreibt das Schutzmodell in höchstens 10 Zeilen; `task check:go` grün.

## Offene Fragen

Schutzmodell (Token, Adressbereich, beides) und ob `POST /api/save` und `/api/report` auch geschützt werden: `docs/fragenkatalog.md` Q17, entscheidet 🧑. Blockiert die Freigabe der Sprint-Spec F4.

## Notizen

Aus Plan Lücke 14. Verwandt: B-027 (Status-Token, bereits umgesetzt).
