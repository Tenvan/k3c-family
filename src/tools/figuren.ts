import { installPageChrome } from '../core/shell';
import { installSelection } from './selection';
import { applyTexts, t } from './texts';
import { allFigures, installPadScroll, renderReference } from './spriteReference';

installPageChrome();
applyTexts();
installPadScroll();
renderReference(document.getElementById('cards')!, allFigures(), false, installSelection('k3c-auswahl-figuren', t('sel.figuren')));
