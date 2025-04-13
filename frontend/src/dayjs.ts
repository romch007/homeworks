import dayjs from "dayjs";
import relativeTime from "dayjs/plugin/relativeTime";

import "dayjs/locale/fr";

dayjs.extend(relativeTime);
dayjs.locale(navigator.language);

export default dayjs;
