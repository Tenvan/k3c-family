import { Button, SelectField, Text } from '@webkit/atoms';
import { useCallback, useEffect, useState } from 'react';
import { backend } from '../api';
import type { GameServer } from '../api/types';
import { loadText, savePref } from '../lib/prefs';
import { openGameReason } from './levelView';

const TARGET_KEY = 'world.gameTarget';
const ENGINE = 'engine';
// Die interne Engine des Studios (B-507/AC-02) läuft auf seinem eigenen Server: dessen Adresse (unter Vite das Backend,
// vite.config.ts setzt VITE_STUDIO_API). Der Mock im Test und im Studio-Vite kennt keinen und nimmt den Vite-Dev-Server von `task dev`.
const ENGINE_BASE = backend.mock ? 'http://127.0.0.1:5183' : import.meta.env.VITE_STUDIO_API || window.location.origin;

/** Text eines Spielservers in der Liste: Adresse, Heimnetz-Adresse (nur Anzeige), Zustand. */
function serverLabel(s: GameServer): string {
  const lan = s.lan ? ` · im Heimnetz ${s.lan}` : '';
  const state = !s.reachable ? ' – nicht erreichbar' : s.dev ? '' : ' – ohne Dev-Mode';
  return `Spielserver ${s.base}${lan}${state}`;
}

/** „Im Spiel öffnen“ (B-460/AC-06, B-513): das Level neu oder voll ausgebaut im Browser starten, wahlweise in der internen
 *  Engine des Studios oder auf einem erkannten Spielserver (nur Loopback; Go prüft die Adresse, die Seite öffnet den Browser).
 *  Ein neues Spiel gibt es nur für den Wald und passende Seeds; „voll ausgebaut“ legt den Spielstand im gewählten Ziel an. */
export function GameLauncher({ seed, biome }: { seed: string; biome: string }) {
  const [wanted, setWanted] = useState(() => loadText(TARGET_KEY, ENGINE));
  const [servers, setServers] = useState<GameServer[]>([]);
  const ctx = `${seed}|${biome}`;
  const [noted, setNoted] = useState({ ctx, text: '' });
  const note = noted.ctx === ctx ? noted.text : ''; // ein Hinweis gilt nur für den Seed, zu dem er gehört
  const setNote = (text: string) => setNoted({ ctx, text });
  const [busy, setBusy] = useState(false);
  const reasonNew = openGameReason('new', seed, biome);
  const reasonFull = openGameReason('full', seed, biome);
  // eine gemerkte Wahl gilt nur, solange der Server erreichbar ist; sonst die interne Engine
  const target = servers.some((s) => s.base === wanted && s.reachable) ? wanted : ENGINE;

  const search = useCallback(async () => {
    try {
      setServers((await backend.worldGameServers()).servers ?? []);
    } catch {
      setServers([]);
    }
  }, []);
  useEffect(() => void search(), [search]);

  const choose = (value: string) => {
    setWanted(value);
    savePref(TARGET_KEY, value);
  };

  const open = async (mode: 'new' | 'full') => {
    setBusy(true);
    setNote(mode === 'full' ? 'Lege den Spielstand voll ausgebaut an …' : '');
    try {
      const res = target === ENGINE ? await backend.worldOpenGame(ENGINE_BASE, seed, biome, mode) : await backend.worldOpenGameServer(target, seed, biome, mode);
      setNote(res.error || `Im Browser geöffnet: Spielstand „${res.save}“`);
    } catch (e) {
      setNote(e instanceof Error ? e.message : String(e));
    }
    setBusy(false);
  };

  return (
    <div className="wd-bar wd-launch">
      <Button disabled={busy || reasonNew !== null} title={reasonNew ?? 'Neues Spiel mit diesem Seed; ein vorhandener Spielstand dieses Namens wird weitergespielt'} onClick={() => void open('new')}>
        Im Spiel öffnen
      </Button>
      <Button variant="soft" disabled={busy || reasonFull !== null} title={reasonFull ?? 'Legt voll-<seed> mit allen Stufen und Gebäuden an (überschreibt ihn); auf dem Spielserver nur im Dev-Mode'} onClick={() => void open('full')}>
        Voll ausgebaut öffnen
      </Button>
      <SelectField className="wd-base" aria-label="Spiel läuft auf" value={target} onChange={choose} options={[
        { value: ENGINE, label: `Interne Engine des Studios (${ENGINE_BASE})` },
        ...servers.map((s) => ({ value: s.base, label: serverLabel(s), disabled: !s.reachable })),
      ]} />
      <Button variant="ghost" disabled={busy} title="Spielserver erneut prüfen" onClick={() => void search()}>
        Neu suchen
      </Button>
      {(reasonNew ?? note) && <Text size="2" color={reasonNew && !note ? 'gray' : undefined} role="status">{note || `Im Spiel öffnen nicht möglich: ${reasonNew}`}</Text>}
    </div>
  );
}
