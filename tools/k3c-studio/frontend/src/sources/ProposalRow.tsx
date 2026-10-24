import { ActionButton, Button, Flex, Progress, StatusBadge, Text } from '@webkit/atoms';
import { CheckField } from '@webkit/molecules';
import { useState } from 'react';
import type { SourceProgress, SourceProposal } from '../api';
import { playChoice, stopAll } from '../sound/audio';
import { downloadable, formatBytes, percent, prioLabel, STATUS_LABEL, STATUS_TONE, usage } from './sourcesLogic';

/** Vorschau eines geladenen Eintrags (läuft aus dem Vorrat, ohne Netz): Bild bei Grafik, Hörprobe bei Ton. */
function Preview({ s }: { s: SourceProposal }) {
  const [status, setStatus] = useState('');
  if (!s.vorschau) return null;
  if (s.art === 'grafik') return <img src={s.vorschau} alt={`Vorschau ${s.titel}`} style={{ maxHeight: 40, imageRendering: 'pixelated' }} />;
  const play = async () => setStatus((await playChoice({}, { datei: s.vorschau! })) ? 'Spielt' : 'Nicht abspielbar');
  return (
    <Flex gap="2" align="center">
      <ActionButton size="1" variant="soft" onClick={play}>Hörprobe</ActionButton>
      <Button size="1" variant="soft" color="gray" onClick={() => { stopAll(); setStatus(''); }}>Stopp</Button>
      {status && <Text size="1" color="gray">{status}</Text>}
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
    <Flex direction="column" style={{ minWidth: 200, flex: 1 }}>
      <Text weight="medium">{s.titel}</Text>
      <Text size="1" color="gray">{s.slot} · {s.art === 'sound' ? 'Ton' : 'Grafik'} · {s.quelle}{s.urheber ? ` · ${s.urheber}` : ''}</Text>
      {(s.hinweis || reason) && <Text size="1" color={reason ? 'red' : 'gray'}>{reason || s.hinweis}</Text>}
      {s.fehler && <Text size="1" color="red" role="alert">Fehler: {s.fehler}</Text>}
    </Flex>
  );
}

function Loading({ s, job }: { s: SourceProposal; job?: SourceProgress }) {
  if (!job || (job.status !== 'wartet' && job.status !== 'laedt')) return null;
  return (
    <Flex direction="column" gap="1" style={{ width: 120 }}>
      <Progress value={percent(job)} aria-label={`Fortschritt ${s.titel}`} />
      <Text size="1" color="gray">{percent(job)} %</Text>
    </Flex>
  );
}

/** Eine Zeile der Vorschlagsliste; Ungeladenes ist offline (und wenn gesperrt oder verworfen) ausgegraut. */
export function ProposalRow({ s, offline, checked, job, onCheck, onAdopt }: RowProps) {
  const dim = s.status !== 'geladen' && (offline || !downloadable(s));
  const loaded = s.status === 'geladen';
  return (
    <Flex gap="3" align="center" wrap="wrap" data-testid="proposal" data-id={s.id} data-dim={dim} style={{ opacity: dim ? 0.5 : 1, padding: '4px 0', borderBottom: '1px solid var(--gray-5)' }}>
      <CheckField label="" aria-label={`${s.titel} wählen`} checked={checked} disabled={offline || !downloadable(s)} onCheckedChange={(c) => onCheck(c === true)} />
      <Info s={s} />
      <Text size="2">{s.lizenz}</Text>
      <Text size="2">{formatBytes(s.groesse)}</Text>
      <Text size="1" color="gray">Prio {prioLabel(s.prioritaet)}</Text>
      <StatusBadge tone={STATUS_TONE[s.status]}>{STATUS_LABEL[s.status]}</StatusBadge>
      {loaded && <Text size="1" color="gray" title={s.pruefsumme}>{usage(s)}</Text>}
      <Loading s={s} job={job} />
      {loaded && <Preview s={s} />}
      {loaded && !s.verwendet && <ActionButton size="1" variant="soft" onClick={onAdopt}>Übernehmen</ActionButton>}
    </Flex>
  );
}
