<template>
  <v-dialog v-model="show" max-width="500">
    <v-card :title="t('createNewHomework')">
      <v-form v-model="formValid" @submit.prevent="submit">
        <v-card-text>
          <v-text-field
            v-model="title"
            :label="t('title')"
            required
            :rules="titleRules"
          ></v-text-field>

          <v-textarea
            :label="t('description')"
            v-model="description"
          ></v-textarea>

          <v-date-input
            v-model="dueDate"
            :label="t('dueDate')"
            prepend-icon=""
            prepend-inner-icon="$calendar"
          ></v-date-input>

          <v-autocomplete
            v-model="subject"
            :label="t('subject')"
            :loading="subjects === undefined"
            :disabled="subjects == undefined"
            :items="subjects ?? []"
            :item-props="subjectItemProp"
            prepend-inner-icon="mdi-tag"
            :color="subject?.hex_color"
          >
            <template v-slot:item="{ props, item }">
              <v-list-item v-bind="props">
                <template v-slot:prepend>
                  <v-icon icon="mdi-tag" :color="item.raw.hex_color"></v-icon>
                </template>
              </v-list-item>
            </template>
          </v-autocomplete>
        </v-card-text>

        <v-card-actions>
          <v-spacer></v-spacer>

          <v-btn variant="plain" @click="show = false">{{ t("cancel") }}</v-btn>

          <v-btn
            color="primary"
            variant="tonal"
            type="submit"
            :loading="props.loading"
            >{{ t("create") }}</v-btn
          >
        </v-card-actions>
      </v-form>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { fetcher } from "@/api";
import type { Subject } from "@/api";
import useSWRV from "swrv";
import { watch } from "vue";
import { ref } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const props = defineProps<{ loading: boolean }>();

const show = defineModel<boolean>("show");

const title = defineModel<string>("title");
const description = defineModel<string>("description");
const dueDate = defineModel<Date>("dueDate");
const subject = defineModel<Subject>("subject");

const emit = defineEmits<{
  (e: "close"): void;
  (e: "submit"): void;
}>();

const formValid = ref(false);

const titleRules = [
  (v: any) => {
    if (v) return true;

    return t("youMustEnterATitle");
  },
];

function submit() {
  emit("submit");
}

const { data: subjects, mutate } = useSWRV<Subject[]>(
  "/api/subjects",
  fetcher,
  { revalidateOnFocus: false },
);

watch(show, (newValue, _oldValue) => {
  if (newValue) mutate();
});

function subjectItemProp(item: Subject) {
  return { title: item.name };
}
</script>
