# B-846 · Der Spielserver startet und läuft ohne dist/ und liefert keinen Browser-Client mehr aus

- **Domäne:** SRV
- **Typ:** Schuld
- **Prio:** niedrig
- **Umgebung:** offline
- **Status:** offen
- **Sprint:** –
- **Projekt:** NAT
- **Erstellt:** 2026-11-13
- **Spec:** Entwurf
- **Revision:** 1
- **Freigabe:** –

## Ausgangslage

Teil von B-823 (Entscheidung 009, Punkt 3). `cmd/k3c-server` startet nicht ohne `dist/index.html` (`cmd/k3c-server/main.go:123`) und liefert `dist/` aus.

## Ziel

Der Server startet ohne `dist/`, liefert keine Client-Seiten mehr aus; API und Werkzeug-Endpunkte bleiben.

## Beteiligte und Zielgruppen

🧑 Ralf (Freigabe, Domäne SRV); Claude (Umsetzung).

## Anforderungen

- Kein Start-Abbruch ohne `dist/`; `K3C_DIST` und Auslieferung entfallen.
- Smoke-Test der CI prüft eine API-Route statt `/`.

## Nicht-Ziele

Online-Spiel oder Protokoll entfernen (B-850).

## Regeln und Einschränkungen

Domäne SRV: Freigabe durch 🧑 (nicht Klasse A).

- **Bereich:** `cmd/k3c-server/**`, `.github/workflows/ci.yml`
- **Abhängig von:** –
- **Braucht 🧑:** Freigabe
- **Größe:** S
- **Sperre:** taskfile

## Beispiele

Server ohne `dist/` gestartet → läuft, `/api/...` antwortet, `/` liefert 404.

## Ausnahme- und Fehlerfälle

Alte Lesezeichen auf `/` → 404 mit Hinweis auf das native Spiel.

## Akzeptanzkriterien

- **AC-01** Test in `cmd/k3c-server`: Start ohne `dist/` ok, `/` antwortet 404.
- **AC-02** CI-Smoke-Test prüft eine API-Route.

## Offene Fragen

keine

## Notizen

Zerlegung B-823 am 2026-11-13 (Claude, AC-01 von B-823).
