import { BAND_FILL, BAND_LINE, type ChartBand } from './chartTheme';

/** Grünes Korridorband über die Plotbreite; `sy` bildet Werte auf Pixel ab. Offene Seiten laufen bis zum Plotrand. */
export function ChartBandShape({ band, sy, left, right, top, bottom }: {
  band: ChartBand;
  sy: (v: number) => number;
  left: number;
  right: number;
  top: number;
  bottom: number;
}) {
  if (band.lower === undefined && band.upper === undefined) return null;
  const yu = band.upper === undefined ? top : sy(band.upper);
  const yl = band.lower === undefined ? bottom : sy(band.lower);
  return (
    <g data-testid="band">
      <rect x={left} y={Math.min(yu, yl)} width={right - left} height={Math.abs(yl - yu)} fill={BAND_FILL} />
      {band.upper !== undefined && <line x1={left} x2={right} y1={yu} y2={yu} stroke={BAND_LINE} strokeDasharray="4 3" />}
      {band.lower !== undefined && <line x1={left} x2={right} y1={yl} y2={yl} stroke={BAND_LINE} strokeDasharray="4 3" />}
    </g>
  );
}
