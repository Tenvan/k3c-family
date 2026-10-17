import { Button, Text, TextField } from '@radix-ui/themes';
import { useState } from 'react';
import { backend } from '../api';
import { loadText, savePref } from '../lib/prefs';
import { openGameReason } from './levelView';

const BASE_KEY = 'world.gameBase';
const DEFAULT_BASE = 'http://127.0.0.1:5183'; // Vite-Dev-Server von `task dev`, leitet /api an den Spielserver

/** „Im Spiel öffnen“ (B-460/AC-06): das Level neu oder voll ausgebaut im Browser starten, über die Landingpage des
 *  Spielservers unter der eingestellten Adresse (nur Loopback; Go prüft sie und öffnet den Browser). Ein neues Spiel gibt
 *  es nur für den Wald und passende Seeds, „voll ausgebaut“ braucht den Dev-Mode des Servers. */
export function GameLauncher({ seed, biome }: { seed: string; biome: string }) {
  const [base, setBase] = useState(() => loadText(BASE_KEY, DEFAULT_BASE));
  const ctx = `${seed}|${biome}`;
  const [noted, setNoted] = useState({ ctx, text: '' });
  const note = noted.ctx === ctx ? noted.text : ''; // ein Hinweis gilt nur für den Seed, zu dem er gehört
  const setNote = (text: string) => setNoted({ ctx, text });
  const [busy, setBusy] = useState(false);
  const reasonNew = openGameReason('new', seed, biome);
  const reasonFull = openGameReason('full', seed, biome);

  const open = async (mode: 'new' | 'full') => {
    setBusy(true);
    setNote(mode === 'full' ? 'Lege den Spielstand voll ausgebaut an …' : '');
    try {
      const res = await backend.worldOpenGame(base, seed, biome, mode);
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
      <Button variant="soft" disabled={busy || reasonFull !== null} title={reasonFull ?? 'Legt voll-<seed> mit allen Stufen und Gebäuden an (überschreibt ihn), nur im Dev-Mode des Servers'} onClick={() => void open('full')}>
        Voll ausgebaut öffnen
      </Button>
      <TextField.Root className="wd-base" aria-label="Spielserver" value={base} onChange={(e) => { setBase(e.target.value); savePref(BASE_KEY, e.target.value); }} />
      {(reasonNew ?? note) && <Text size="2" color={reasonNew && !note ? 'gray' : undefined} role="status">{note || `Im Spiel öffnen nicht möglich: ${reasonNew}`}</Text>}
    </div>
  );
}
