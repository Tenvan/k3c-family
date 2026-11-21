import { useEffect, useRef, useState } from 'react';
import type { WorldDetail } from '../api';
import { CrossSection } from './CrossSection';
import { islandAnchor } from './crossModel';
import { SeaMap } from './SeaMap';
import { useCross } from './useCross';
import type { StageChoice } from './useWorld';

// Übersicht des Welten-Editors (B-963): Seekarte links (sticky) und Querschnitt aller Inseln rechts. Ein Klick auf einen
// Knoten der Seekarte scrollt zur Insel und hebt sie kurz hervor; ein Klick auf einen Streifen wählt die Stufe zum Bearbeiten.

const HIT_MS = 1200;

interface Props {
  detail: WorldDetail;
  seed: string;
  choice: StageChoice | null;
  onPick: (island: string, stage: number) => void;
  onEtappe?: (island: string, etappe: number | null) => Promise<boolean> | void;
  /** Token `<insel>:<stufe>#<n>`: diese Stufe neu laden */
  refresh?: string;
  /** Gewählte Insel: nur sie im Querschnitt; die Seekarte wählt dann statt zu springen */
  selected?: string;
  onSelect?: (island: string) => void;
}

export function Overview({ detail, seed, choice, onPick, onEtappe, refresh, selected, onSelect }: Props) {
  const models = useCross(detail, seed, refresh);
  const root = useRef<HTMLDivElement>(null);
  const timer = useRef<ReturnType<typeof setTimeout> | undefined>(undefined);
  const [hit, setHit] = useState<string | null>(null);
  useEffect(() => () => clearTimeout(timer.current), []);
  const jump = (islandId: string) => {
    root.current?.querySelector(`#${islandAnchor(islandId)}`)?.scrollIntoView?.({ behavior: 'smooth', block: 'start' });
    setHit(islandId);
    clearTimeout(timer.current);
    timer.current = setTimeout(() => setHit(null), HIT_MS);
  };
  return (
    <div className="wd-overview" ref={root}>
      <SeaMap detail={detail} models={models} onJump={onSelect ?? jump} active={selected} />
      <CrossSection detail={detail} models={models} choice={choice} hit={hit} onPick={onPick} onEtappe={onEtappe} only={selected} />
    </div>
  );
}
