# B-495 · Die Aktionsliste nennt respec, wenn CanRespec erlaubt

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** SKL
- **Erstellt:** 2026-10-18
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

`engine/net/actions.go` (`actionsOf`) nennt `attack`, `skill` und `learn`, aber nicht `respec`; der Kopfkommentar verweist auf B-270. Mit SK1.1 gibt es `sim.CanRespec(w, p)` und `sim.CanLearn(w, p, id)` ohne Seiteneffekt, die Aktionsliste nutzt sie noch nicht. `PlayerAction` kennt `respec` bereits (B-410).

## Ziel

Der Server nennt `respec` in der Aktionsliste eines Spielers genau dann, wenn `CanRespec` nil liefert; der Client erfindet keine Regel.

## Beteiligte und Zielgruppen

Entwickler (Server), Spieler über das Aktionen-Overlay (S3).

## Anforderungen

- `actionsOf` ruft `sim.CanRespec` auf und fügt `{"action": "respec"}` hinzu, wenn kein Fehler kommt (ohne gelernte Skills ebenfalls, Beschluss 🧑 2026-10-06).
- Reihenfolge laut `docs/protocol.md` beibehalten; Protokoll-Doku und Tests nachziehen, falls die Liste dort beschrieben ist.

## Nicht-Ziele

Client-Anzeige (S3), Ausbilden und Tauschen (B-410), lernbare Skills je Spieler (B-285).

## Regeln und Einschränkungen

SRV; keine Regel in `engine/net` nachbauen, nur `sim.CanRespec` aufrufen. Protokoll-Änderungen (Doku, `testdata/protocol/`) brauchen eine eigene Session für beide Enden, falls sich das Format ändert.

## Beispiele

Spieler am Tag an der Burg → Aktionsliste enthält `respec`. Nacht oder fern der Burg → fehlt.

## Ausnahme- und Fehlerfälle

Gefallener Monarch (`RespawnIn` > 0): keine Aktionen, wie heute.

## Akzeptanzkriterien

- **AC-01** Ein Test in `engine/net` zeigt `respec` in der Aktionsliste am Tag an der Burg und nicht in der Nacht oder fern der Burg.
- **AC-02** `task check:go` ist grün, `testdata/protocol/` ist unverändert, sofern das Format gleich bleibt.

## Offene Fragen

keine

## Notizen

Entstanden im Review von SK1 (SK1.4); `CanRespec` kam mit SK1.1.
