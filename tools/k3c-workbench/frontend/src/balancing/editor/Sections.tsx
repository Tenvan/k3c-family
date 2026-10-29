import type { ReactNode } from 'react';
import { Flex, IconButton, SelectField, Text, TextField } from '@webkit/atoms';
import type { BalanceBaseline } from '../../api';
import { CHOICES, GRADES, HELPS, isMix, MIX_SEP, mixCount, PER_PLAYER, resizeMix, SOLO_BOTS, WORLDS } from '../help';
import { HelpField } from '../HelpButton';
import type { BalanceForm } from '../spec';
import { Choice, type ChoiceOption } from './Choice';
import { ALL_SEEDS } from './estimate';

export type Patch = (p: Partial<BalanceForm>) => void;

/** Karte eines Abschnitts: Titel links, Zusammenfassung rechts. */
export function Card({ title, summary, children }: { title: string; summary?: string; children: ReactNode }) {
  return (
    <section className="bal-ed-card" aria-label={title}>
      <div className="bal-ed-chead">
        <Text weight="bold">{title}</Text>
        {summary && <Text size="1" color="gray" className="bal-ed-sum">{summary}</Text>}
      </div>
      {children}
    </section>
  );
}

/** Optionen plus der aktuelle Wert, falls er (z. B. aus einem alten Lauf) nicht dazugehört. */
const withCurrent = (options: ChoiceOption[], value: string): ChoiceOption[] =>
  options.some((o) => o.value === value) ? options : [...options, { value, label: value }];

const worldOptions = (value: string) => withCurrent([{ value: '', label: 'Eine Stufe' }, ...WORLDS.map((w) => ({ value: w, label: w === 'standard' ? 'Ganze Insel 1' : w }))], value);
const helpOptions = (value: string) => withCurrent([{ value: '', label: 'keine' }, ...HELPS.map((h) => ({ value: h, label: h }))], value);
const gradeOptions = (value: string) => withCurrent(GRADES.map((g) => ({ value: g, label: g })), value);

export function scenarioSummary(f: BalanceForm): string {
  const where = f.world === '' ? `eine Stufe${f.depth === '' ? '' : `, Tiefe ${f.depth}`}` : `Insel 1, Hilfen ${f.helps || 'keine'}`;
  return `${where} · ${f.grade || 'normal'} · ${f.days.trim() ? `${f.days.trim()} Tage` : 'Tage je Ziel'}`;
}

/** 1 · Szenario: Welt, Tiefe (nur eine Stufe), Dev-Hilfen (nur mit Welt), Grad, Tage. */
export function ScenarioSection({ form, patch }: { form: BalanceForm; patch: Patch }) {
  const stage = form.world === '';
  // Mit Welt gibt es keine Tiefe, ohne Welt keine Hilfen: das jeweils Verborgene wird geleert.
  const setWorld = (world: string) => patch(world === '' ? { world, helps: '' } : { world, depth: '' });
  return (
    <Card title="1 · Szenario" summary={scenarioSummary(form)}>
      <div className="bal-ed-fields">
        <HelpField name="world" hint={false}><Choice label="Welt" value={form.world} options={worldOptions(form.world)} onChange={setWorld} /></HelpField>
        {stage && (
          <HelpField name="depth" hint={false}>
            <SelectField aria-label="Tiefe" value={form.depth} options={CHOICES.depth} onChange={(depth) => patch({ depth })} />
          </HelpField>
        )}
        {!stage && <HelpField name="helps" hint={false}><Choice label="Hilfen" value={form.helps} options={helpOptions(form.helps)} onChange={(helps) => patch({ helps })} /></HelpField>}
        <HelpField name="grade" hint={false}>
          <Choice label="Grad" value={form.grade || 'normal'} options={gradeOptions(form.grade || 'normal')} onChange={(g) => patch({ grade: g === 'normal' ? '' : g })} />
        </HelpField>
        <HelpField name="days" hint={false}>
          <TextField.Root aria-label="Tage" className="bal-ed-num" placeholder="Ziel" value={form.days} onChange={(e) => patch({ days: e.target.value })} />
          <Text size="1" color="gray">leer = Tage je Ziel</Text>
        </HelpField>
      </div>
      {!stage && form.helps !== '' && <Text size="1" color="amber" role="note">Mit Hilfen sagt der Lauf nichts über die echte Balance; für den Meilenstein „keine“ wählen.</Text>}
    </Card>
  );
}

const BOT_OPTIONS: ChoiceOption[] = CHOICES.bot.filter((o) => o.value !== PER_PLAYER);

/** Profile je Spieler (Spieler 1 zuerst) bzw. ein Profil für alle. */
function BotChoice({ form, patch }: { form: BalanceForm; patch: Patch }) {
  const n = mixCount(form.players);
  const mix = isMix(form.bot);
  const parts = mix ? resizeMix(form.bot, n).split(MIX_SEP) : [];
  const solo = (SOLO_BOTS as readonly string[]).includes(form.bot) ? form.bot : 'saver';
  const options = mix || BOT_OPTIONS.some((o) => o.value === form.bot) ? BOT_OPTIONS : [...BOT_OPTIONS, { value: form.bot, label: form.bot }];
  const modes: ChoiceOption[] = [
    { value: 'alle', label: 'Ein Profil für alle' },
    { value: 'je', label: 'Je Spieler', disabled: n < 2, title: n < 2 ? 'Braucht mindestens 2 Spieler' : undefined },
  ];
  const setPart = (i: number) => (v: string) => patch({ bot: parts.map((p, j) => (j === i ? v : p)).join(MIX_SEP) });
  return (
    <HelpField name="bot" hint={false}>
      <Choice label="Bot-Zuordnung" value={mix ? 'je' : 'alle'} options={modes} onChange={(m) => patch({ bot: m === 'je' ? Array(n).fill(solo).join(MIX_SEP) : parts[0] })} />
      {!mix && <SelectField aria-label="Bot" value={form.bot} options={options} onChange={(bot) => patch({ bot })} />}
      {mix && (
        <Flex gap="2" wrap="wrap">
          {parts.map((b, i) => (
            <Flex key={i} align="center" gap="2" className="bal-ed-bot">
              <Text size="1" color="gray">Spieler {i + 1}</Text>
              <SelectField aria-label={`Bot Spieler ${i + 1}`} size="1" value={b} options={SOLO_BOTS.map((x) => ({ value: x, label: x }))} onChange={setPart(i)} />
            </Flex>
          ))}
        </Flex>
      )}
    </HelpField>
  );
}

/** 2 · Spieler und Bots: Spieler 1–4 mit −/+, Bot als ein Profil oder je Spieler. */
export function PlayersSection({ form, patch }: { form: BalanceForm; patch: Patch }) {
  const n = mixCount(form.players);
  const set = (v: number) => patch({ players: String(v), bot: isMix(form.bot) ? resizeMix(form.bot, v) : form.bot });
  return (
    <Card title="2 · Spieler und Bots" summary={`${n} Spieler · ${isMix(form.bot) ? 'Profil je Spieler' : form.bot || 'Bot je Ziel'}`}>
      <div className="bal-ed-fields">
        <HelpField name="players" hint={false}>
          <Flex align="center" gap="2">
            <IconButton size="1" variant="soft" color="gray" aria-label="Ein Spieler weniger" disabled={n <= 1} onClick={() => set(n - 1)}>−</IconButton>
            <Text weight="bold" role="status" aria-label="Spieler" className="bal-ed-count">{n}</Text>
            <IconButton size="1" variant="soft" color="gray" aria-label="Ein Spieler mehr" disabled={n >= 4} onClick={() => set(n + 1)}>+</IconButton>
            {form.players === '' && <Text size="1" color="gray">Vorgabe der Ziele</Text>}
          </Flex>
        </HelpField>
        <BotChoice form={form} patch={patch} />
      </div>
    </Card>
  );
}

const SEED_STEPS = [1, 8, 20];

export function seedsSummary(f: BalanceForm, listMode: boolean): string {
  if (listMode || f.seedList.trim() !== '') return `${f.seedList.split(',').filter((s) => s.trim()).length} Seeds aus der Liste`;
  return f.seeds === '' ? 'alle Seeds der Ziele' : `${f.seeds} ${f.seeds === '1' ? 'Seed' : 'Seeds'}`;
}

/** 3 · Umfang: Seeds 1, 8, 20, Alle oder eine Liste; Seeds und Seed-Liste schließen sich aus. */
export function ScopeSection({ form, patch, listMode, onListMode }: { form: BalanceForm; patch: Patch; listMode: boolean; onListMode: (on: boolean) => void }) {
  const list = listMode || form.seedList.trim() !== '';
  const n = Number(form.seeds);
  const steps = form.seeds !== '' && Number.isInteger(n) && !SEED_STEPS.includes(n) ? [...SEED_STEPS, n].sort((a, b) => a - b) : SEED_STEPS;
  const value = list ? 'liste' : form.seeds === '' ? 'alle' : form.seeds;
  const options: ChoiceOption[] = [...steps.map((s) => ({ value: String(s), label: String(s) })), { value: 'alle', label: `Alle (${ALL_SEEDS})` }, { value: 'liste', label: 'Liste …' }];
  const pick = (v: string) => {
    onListMode(v === 'liste');
    patch(v === 'liste' ? { seeds: '' } : v === 'alle' ? { seeds: '', seedList: '' } : { seeds: v, seedList: '' });
  };
  return (
    <Card title="3 · Umfang" summary={seedsSummary(form, listMode)}>
      <Flex gap="3" align="center" wrap="wrap">
        <HelpField name="seeds" hint={false}><Choice label="Seeds" value={value} options={options} onChange={pick} /></HelpField>
        {list && (
          <HelpField name="seedList" hint={false}>
            <TextField.Root aria-label="Seed-Liste" placeholder="k3c-dev,7,12" value={form.seedList} onChange={(e) => patch({ seedList: e.target.value })} />
          </HelpField>
        )}
      </Flex>
    </Card>
  );
}

/** 5 · Vergleich und Name: Baseline als Vergleich und Label. */
export function CompareSection({ form, patch, baselines }: { form: BalanceForm; patch: Patch; baselines: BalanceBaseline[] }) {
  const options = [{ value: '', label: 'Baseline des Repos' }, ...baselines.map((b) => ({ value: b.name, label: `${b.name} · ${b.run}` }))];
  if (!options.some((o) => o.value === form.baseline)) options.push({ value: form.baseline, label: form.baseline });
  return (
    <Card title="5 · Vergleich und Name" summary={form.baseline || 'Baseline des Repos'}>
      <div className="bal-ed-fields">
        <HelpField name="baseline" hint={false}><SelectField aria-label="Baseline" value={form.baseline} options={options} onChange={(baseline) => patch({ baseline })} /></HelpField>
        <HelpField name="label" hint={false}><TextField.Root aria-label="Label" value={form.label} onChange={(e) => patch({ label: e.target.value })} /></HelpField>
      </div>
    </Card>
  );
}
