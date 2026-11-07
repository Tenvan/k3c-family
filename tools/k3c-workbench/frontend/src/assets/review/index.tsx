import { Button, Flex, Heading, Text } from '@webkit/atoms';
import { ChipSwitch, LoadingNote } from '@webkit/molecules';
import { ListDetail } from '@webkit/templates';
import { useCallback, useEffect, useState, type RefCallback } from 'react';
import { text } from './fehlertext';
import { BasisSelect } from './BasisSelect';
import { Compare } from './Compare';
import { PromptButtons } from './PromptButtons';
import { SizeCompare } from '../size/SizeCompare';
import { ReviewList } from './ReviewList';
import { cachedReview, clearReviewCache, loadReview } from './reviewStore';
import type { Review, ReviewAsset } from './types';

/** Review-Seite im Bereich Assets (RV1, Vertrag docs/standards/paper-mcp.md › Oberfläche): jede Asset-Änderung gegen
 *  eine wählbare Git-Basis mit Vorher/Nachher, Pixel-Diff-Overlay und markierten Teilen. Ändert nichts. */
export function ReviewPage() {
  const [basis, setBasis] = useState('HEAD');
  const [auswahl, setAuswahl] = useState<string | null>(null);
  const [stand, setStand] = useState(0);
  const { review, laedt, fehler } = useReview(basis, stand);
  const eintrag = review?.assets.find((a) => a.id === auswahl);
  const [detail, hoehe] = useHoehe();
  const kopf = (
    <>
      <Heading size="4">Asset-Review</Heading>
      <Flex gap="3" align="center" wrap="wrap">
        <BasisSelect basis={basis} onChange={setBasis} />
        <Button size="1" variant="soft" disabled={laedt} onClick={() => {
          clearReviewCache();
          setStand((s) => s + 1);
        }}>
          Neu laden
        </Button>
      </Flex>
      {laedt && <LoadingNote id="review" text={`Vergleicht mit ${basis} …`} />}
      {review && (
        <Text size="2" color="gray">
          Basis {review.basis} ({review.basisCommit}) · {review.assets.length} Assets geändert
        </Text>
      )}
      {review?.basisStand === 'lokal' && <Text size="2" color="amber">Basis-Stand: lokal (ohne Fetch)</Text>}
      {fehler && (
        <Text size="2" color="red" role="alert">
          {fehler}
        </Text>
      )}
    </>
  );
  // Liste und Detail scrollen je für sich (WB2/AC-02)
  return (
    <ListDetail header={kopf} listWidth={520} list={review && <ReviewList assets={review.assets} auswahl={auswahl} onSelect={setAuswahl} felderHoehe={hoehe} />}>
      {review && eintrag && (
        <Flex ref={detail} key={`${eintrag.id}@${review.basisCommit}#${stand}`} direction="column" gap="4">
          <Detail asset={eintrag} review={review} />
          <PromptButtons asset={eintrag} basis={review.basis} />
        </Flex>
      )}
    </ListDetail>
  );
}

/** Höhe der Ansicht rechts (Vorher/Nachher bis zu den Prompts): so hoch darf die Feldliste links werden (0 = unbekannt). */
function useHoehe(): [RefCallback<HTMLDivElement>, number] {
  const [hoehe, setHoehe] = useState(0);
  const ref = useCallback<RefCallback<HTMLDivElement>>((el) => {
    if (!el) return;
    const ro = new ResizeObserver(() => setHoehe(el.offsetHeight));
    ro.observe(el);
    return () => {
      ro.disconnect();
      setHoehe(0);
    };
  }, []);
  return [ref, hoehe];
}

/** Review einer Basis aus dem Cache (B-732, reviewStore.ts); beim Neuladen (`stand`) bleibt das alte Ergebnis stehen, bis
 *  das neue kommt. */
function useReview(basis: string, stand: number) {
  const [review, setReview] = useState<Review | null>(() => cachedReview(basis) ?? null);
  const [laedt, setLaedt] = useState(false);
  const [fehler, setFehler] = useState('');
  useEffect(() => {
    let aktiv = true;
    setFehler('');
    const hit = cachedReview(basis);
    if (hit) {
      setReview(hit);
      return;
    }
    setLaedt(true);
    loadReview(basis).finally(() => aktiv && setLaedt(false)).then(
      (r) => aktiv && setReview(r),
      (e) => {
        if (!aktiv) return;
        setReview(null);
        setFehler(text(e));
      },
    );
    return () => {
      aktiv = false;
    };
  }, [basis, stand]);
  return { review, laedt, fehler };
}

type Ansicht = 'vergleich' | 'groesse';
/** Vorher/Nachher oder Größenvergleich des Arbeitsstands mit dem Monarchen (B-733); gilt für alle Einträge. */
const ANSICHTEN = [{ value: 'vergleich', label: 'Vorher/Nachher' }, { value: 'groesse', label: 'Größenvergleich' }] as const;
let letzteAnsicht: Ansicht = 'vergleich';

function Detail({ asset, review }: { asset: ReviewAsset; review: Review }) {
  const [ansicht, setAnsicht] = useState<Ansicht>(letzteAnsicht);
  const waehle = (a: Ansicht) => {
    letzteAnsicht = a;
    setAnsicht(a);
  };
  const groesse = ansicht === 'groesse' && asset.status !== 'geloescht';
  return (
    <>
      <ChipSwitch label="Ansicht" options={ANSICHTEN} value={ansicht} onChange={waehle} />
      {ansicht === 'groesse' && asset.status === 'geloescht' && <Text size="2" color="gray">Gelöscht: kein Arbeitsstand für den Größenvergleich</Text>}
      {groesse ? <SizeCompare id={asset.id} /> : ansicht === 'vergleich' && <Compare asset={asset} basis={review.basisCommit} />}
    </>
  );
}
