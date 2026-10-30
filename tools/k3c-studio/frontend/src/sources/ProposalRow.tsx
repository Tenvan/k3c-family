import { ActionButton, Button, Flex, IconButton, Link, Progress, StatusBadge, Text } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import { useState } from 'react';
import type { SourceProgress, SourceProposal } from '../api';
import { playChoice, stopAll } from '../sound/audio';
import { downloadable, formatBytes, percent, prioLabel, STATUS_LABEL, STATUS_TONE, usage } from './sourcesLogic';

/** Vorschau eines geladenen Eintrags (läuft aus dem Vorrat, ohne Netz): kleines Bild bei Grafik, ▶ bei Ton. */
function Preview({ s }: { s: SourceProposal }) {
  const [status, setStatus] = useState('');
  if (!s.vorschau) return null;
  if (s.art === 'grafik') return <img src={s.vorschau} alt={`Vorschau ${s.titel}`} style={{ maxHeight: 28, maxWidth: 40, imageRendering: 'pixelated' }} />;
  const play = async () => setStatus((await playChoice({}, { datei: s.vorschau! })) ? 'Spielt' : 'Nicht abspielbar');
  return (
    <Flex gap="1" align="center">
      <IconButton size="1" variant="soft" aria-label="Hörprobe" onClick={play}>▶</IconButton>
      {status === 'Spielt' && <IconButton size="1" variant="soft" color="gray" aria-label="Stopp" onClick={() => { stopAll(); setStatus(''); }}>■</IconButton>}
      {status === 'Nicht abspielbar' && <Text size="1" color="gray">{status}</Text>}
    </Flex>
  );
}

interface RowProps {
  s: SourceProposal;
  offline: boolean;
  checked: boolean;
  job?: SourceProgress;
  onCheck: (checked: boolean) => void;
  onAdopt: () => void;
}

function Info({ s }: { s: SourceProposal }) {
  const reason = s.sperrgrund ?? (s.lizenzErlaubt ? '' : `Lizenz nicht erlaubt: ${s.lizenz}`);
  return (
    <Flex direction="column" style={{ minWidth: 0 }}>
      <Text weight="medium" size="2" title={s.hinweis} className="src-title">{s.titel}</Text>
      <Text size="1" color="gray">{s.quelle} · {s.art === 'sound' ? 'Ton' : 'Grafik'}</Text>
      {reason && <Text size="1" color="red">{reason}</Text>}
      {s.fehler && <Text size="1" color="red" role="alert">Fehler: {s.fehler}</Text>}
    </Flex>
  );
}

function Loading({ s, job }: { s: SourceProposal; job?: SourceProgress }) {
  if (!job || (job.status !== 'wartet' && job.status !== 'laedt')) return null;
  return (
    <Flex direction="column" gap="1" style={{ width: 100 }}>
      <Progress value={percent(job)} aria-label={`Fortschritt ${s.titel}`} />
      <Text size="1" color="gray">{percent(job)} %</Text>
    </Flex>
  );
}

/** Aufgeklappte Zeile: Priorität, Urheber, Prüfsumme, Hinweis und Link zur Seite des Angebots. */
function Details({ s }: { s: SourceProposal }) {
  return (
    <Flex gap="3" wrap="wrap" className="src-detail" data-testid="proposal-detail">
      <Text size="1" color="gray">Prio {prioLabel(s.prioritaet)}</Text>
      {s.urheber && <Text size="1" color="gray">Urheber: {s.urheber}</Text>}
      {s.pruefsumme && <Text size="1" color="gray">SHA-256: {s.pruefsumme}</Text>}
      {s.hinweis && <Text size="1" color="gray">{s.hinweis}</Text>}
      <Link size="1" href={s.seite} target="_blank" rel="noopener noreferrer">Seite des Angebots</Link>
    </Flex>
  );
}

/** Eine Zeile der Vorschlagsliste: Auswahl, Vorschau, Titel, Lizenz, Größe, Status oder Aktion; Ungeladenes ist offline
 *  (und wenn gesperrt oder verworfen) ausgegraut. */
export function ProposalRow({ s, offline, checked, job, onCheck, onAdopt }: RowProps) {
  const [open, setOpen] = useState(false);
  const dim = s.status !== 'geladen' && (offline || !downloadable(s));
  const loaded = s.status === 'geladen';
  return (
    <div className="src-row" data-testid="proposal" data-id={s.id} data-dim={dim} style={{ opacity: dim ? 0.5 : 1 }}>
      <CheckField label="" aria-label={`${s.titel} wählen`} checked={checked} disabled={offline || !downloadable(s)} onCheckedChange={(c) => onCheck(c === true)} />
      <div className="src-prev">{loaded && <Preview s={s} />}</div>
      <Info s={s} />
      <Text size="1" className="src-lic">{s.lizenz}</Text>
      <Text size="1" className="src-size">{formatBytes(s.groesse)}</Text>
      <Flex gap="2" align="center" justify="end" className="src-act">
        <Loading s={s} job={job} />
        {!job || (job.status !== 'wartet' && job.status !== 'laedt') ? <StatusBadge tone={STATUS_TONE[s.status]}>{STATUS_LABEL[s.status]}</StatusBadge> : null}
        {loaded && <Text size="1" color="gray" title={s.pruefsumme}>{usage(s)}</Text>}
        {loaded && !s.verwendet && <ActionButton size="1" variant="soft" onClick={onAdopt}>Übernehmen</ActionButton>}
        <Button size="1" variant="ghost" color="gray" aria-expanded={open} aria-label={`${s.titel}: Details`} onClick={() => setOpen(!open)}>{open ? '▾' : '▸'}</Button>
      </Flex>
      {open && <Details s={s} />}
    </div>
  );
}
