/**
 * plugins/index.ts
 *
 * Automatically included in `./src/main.ts`
 */

// Plugins
import vuetify from "./vuetify";
import i18n from "./i18n";
import router from "../router";

// Types
import type { App } from "vue";

export function registerPlugins(app: App) {
  app.use(vuetify).use(i18n).use(router);
}
