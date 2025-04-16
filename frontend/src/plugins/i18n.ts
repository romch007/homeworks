import { createI18n } from "vue-i18n";

import en from "@/locales/en.json";
import fr from "@/locales/fr.json";

type MessageSchema = typeof en;

export default createI18n<[MessageSchema], "en" | "fr">({
  locale: localStorage.getItem("languageOverride") ?? navigator.language,
  fallbackLocale: "en",
  messages: { en, fr },
});
