import { ActionButton, Button, Flex, IconButton, StatusBadge, Text, Tip } from '@webkit/atoms';
import { useRef, useState } from 'react';
import { backend, type AssetCandidate, type AssetKind } from '../api';
import { fileUrl } from '../assets/assets';
import { formatSize, MARK_LABEL } from '../lib/license';
import { canAssign, confirmText, hitId, markTip, markTone, needsConfirm, originTip, previewKind, rowError, sourceLine, stageText, stageTone, type RowError } from './candidateLogic';
import { playFile } from './transport';

interface Props {
  c: AssetCandidate;
  slotId: string;
  art: AssetKind;
  /** Stand des Slot-Registers aus der Kandidaten-Antwort (Schreibregel von AssetCandidateAssign) */
  stand: string;
  offline: boolean;
  /** Nach Zuordnen oder Bestätigen: Slot-Register und Kandidaten neu laden */
  onChanged: () => void;
  /** Register fremd geändert: nur die Kandidaten neu laden, damit `stand` stimmt */
  onStale: () => Promise<void>;
}

function Preview({ c, art }: { c: AssetCandidate; art: AssetKind }) {
  const kind = previewKind(c, art);
  if (kind === 'bild') return <img className="ap-thumb" src={fileUrl(c.vorschau)} alt="" loading="lazy" />;
  if (kind === 'ton') {
    return (
      <IconButton size="2" variant="soft" color="gray" aria-label={`Vorschau abspielen: ${c.titel}`} onClick={() => void playFile(c.titel, fileUrl(c.vorschau))}>▶</IconButton>
    );
  }
  return <span className="ap-thumb ap-thumb-none" aria-label="keine Vorschau" title="keine Vorschau">◻</span>;
}

/** Aktionen am Zeilenende: Seite öffnen, Lizenz bestätigen, Zuordnen (gesperrt mit Grund als Hinweis). */
function Actions({ c, reason, confirming, onStart, onLicense }: { c: AssetCandidate; reason: string; confirming: boolean; onStart: () => void; onLicense: () => void }) {
  return (
    <Flex gap="1" align="center" className="ap-cand-actions">
      {c.link && c.stufe === 'netz' && <Button size="1" variant="ghost" onClick={() => backend.openUrl(c.link!)}>Seite</Button>}
      {c.lizenzBestaetigen && <ActionButton size="1" variant="soft" color="green" onClick={onLicense}>Lizenz bestätigen</ActionButton>}
      {c.praef === 0 && (canAssign(c)
        ? <ActionButton size="1" onClick={onStart} disabled={confirming}>Zuordnen</ActionButton>
        : <Tip content={reason}><Button size="1" disabled>Zuordnen</Button></Tip>)}
    </Flex>
  );
}
/** Eine Zeile der Kandidaten-Liste: Vorschau, Titel, Stufe, Lizenz-Marke, Größe und Aktionen. Die Bestätigung eines
 *  Netz-Kandidaten und Fehler erscheinen in der Zeile selbst, kein Dialog. */
export function CandidateRow({ c, slotId, art, stand, offline, onChanged, onStale }: Props) {
  const [confirming, setConfirming] = useState(false);
  const [error, setError] = useState<RowError | null>(null);
  const last = useRef<() => Promise<unknown>>(() => Promise.resolve());
  const standRef = useRef(stand); // „Erneut versuchen“ nach Stale braucht den frisch geladenen Stand, nicht den der alten Closure
  standRef.current = stand;

  /** Führt die Aktion aus; Fehler bleiben in der Zeile (mit „Erneut versuchen“ für dieselbe Aktion), bei veraltetem Register lädt die Liste neu. */
  const attempt = async (run: () => Promise<unknown>) => {
    last.current = run;
    setError(null);
    try {
      await run();
      setConfirming(false);
      onChanged();
    } catch (e) {
      const err = rowError(e);
      setConfirming(false);
      setError(err);
      if (err.stale) await onStale();
    }
  };
  const assign = () => attempt(() => backend.assetCandidateAssign(slotId, c.id, standRef.current, c.groesse));
  const start = () => {
    if (!needsConfirm(c)) return assign();
    setError(null);
    setConfirming(true);
  };
  const reason = c.grund || (offline && c.stufe === 'netz' ? 'offline' : 'nicht zuordenbar');

  return (
    <li className="ap-cand" data-cand={c.id}>
      <div className="ap-cand-main">
        <Preview c={c} art={art} />
        <Tip content={originTip(c)}>
          <span className="ap-cand-text">
            <Text size="2" weight="medium" className="ap-cand-title">{c.titel}</Text>
            <span className="ap-dim">{sourceLine(c)}</span>
          </span>
        </Tip>
        <StatusBadge tone={stageTone(c)}>{stageText(c)}</StatusBadge>
        <Tip content={markTip(c)}><StatusBadge tone={markTone(c.marke)}>{MARK_LABEL[c.marke]}</StatusBadge></Tip>
        <span className="ap-size">{formatSize(c.groesse)}</span>
        <Actions c={c} reason={reason} confirming={confirming} onStart={start} onLicense={() => attempt(() => backend.assetsLicenseOk(slotId, hitId(c.id)))} />
      </div>
      {confirming && (
        <Flex gap="2" align="center" className="ap-cand-line" role="group" aria-label="Zuordnen bestätigen">
          <Text size="1">{confirmText(c, slotId)}</Text>
          <ActionButton size="1" onClick={assign}>Bestätigen</ActionButton>
          <Button size="1" variant="soft" color="gray" onClick={() => setConfirming(false)}>Abbrechen</Button>
        </Flex>
      )}
      {error && (
        <Flex gap="2" align="center" className="ap-cand-line ap-cand-error" role="alert">
          <Text size="1" color="red">{error.text}</Text>
          <ActionButton size="1" variant="soft" onClick={() => attempt(last.current)}>Erneut versuchen</ActionButton>
        </Flex>
      )}
    </li>
  );
}