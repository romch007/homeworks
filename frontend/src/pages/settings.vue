<template>
  <v-select
    v-model="selectedLanguage"
    :label="t('applicationLanguage')"
    :items="languages"
    :item-props="languageProp"
  >
    <template v-slot:selection="{ item, index }">
      <span v-if="item.raw.flag" :class="flagToClass(item.raw.flag)"></span>

      <span v-if="item.raw.name">{{ item.title }}</span>
      <span v-else>{{ t("useBrowserDefaultLanguage") }}</span>
    </template>

    <template v-slot:item="{ props, item }">
      <v-list-item v-bind="props" v-if="item.raw.name">
        <template v-slot:prepend>
          <span v-if="item.raw.flag" :class="flagToClass(item.raw.flag)"></span>
        </template>
      </v-list-item>

      <v-list-item
        v-else
        v-bind="props"
        :title="t('useBrowserDefaultLanguage')"
      ></v-list-item>
    </template>
  </v-select>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import "/node_modules/flag-icons/css/flag-icons.min.css";
import { useI18n } from "vue-i18n";

const { t, locale } = useI18n();

interface Language {
  code: string;
  flag?: string;
  name?: string;
}

const languages: Language[] = [
  { code: "browser" },
  { code: "en", flag: "gb", name: "English" },
  { code: "fr", flag: "fr", name: "Français" },
];

const selectedLanguage = ref<Language>(languages[0]);

watch(selectedLanguage, (newValue, _oldValue) => {
  if (newValue.code === "browser") {
    localStorage.removeItem("languageOverride");
    locale.value = navigator.language;
  } else {
    localStorage.setItem("languageOverride", newValue.code);
    locale.value = newValue.code;
  }
});

function languageProp(item: Language) {
  return { title: item.name };
}

function flagToClass(code: string) {
  return `fi fi-${code}`;
}
</script>

<style scoped>
.fi {
  border-radius: 2px;
  margin-right: 10px;
}
</style>
