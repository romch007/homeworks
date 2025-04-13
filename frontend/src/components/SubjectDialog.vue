<template>
  <v-dialog v-model="show" max-width="400">
    <v-card :title="title">
      <v-form v-model="formValid" @submit.prevent="submit">
        <v-card-text>
          <v-text-field
            v-model="name"
            :label="t('name')"
            required
            :rules="nameRules"
            class="mb-3"
          ></v-text-field>

          <color-input v-model="hexColor"></color-input>
        </v-card-text>

        <v-card-actions>
          <v-spacer></v-spacer>

          <v-btn variant="plain" @click="show = false">{{ t("cancel") }}</v-btn>

          <v-btn
            color="primary"
            variant="tonal"
            type="submit"
            :loading="props.loading"
            >{{ action }}</v-btn
          >
        </v-card-actions>
      </v-form>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const props = defineProps<{
  variant: "create" | "edit";
  loading: boolean;
}>();

const show = defineModel<boolean>("show");
const name = defineModel<string>("name");
const hexColor = defineModel<string>("hex-color");

const emit = defineEmits<{
  (e: "close"): void;
  (e: "submit"): void;
}>();

const formValid = ref(false);

const nameRules = [
  (v: any) => {
    if (v) return true;

    return t("youMustEnterAName");
  },
];

const title = computed(() =>
  props.variant === "create"
    ? t("createNewSubject")
    : `${t("edit")} '${name.value}'`,
);
const action = computed(() =>
  props.variant === "create" ? t("create") : t("update"),
);

function submit() {
  emit("submit");
}
</script>
