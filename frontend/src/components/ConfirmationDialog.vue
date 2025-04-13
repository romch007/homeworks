<template>
  <v-dialog v-model="show" max-width="400">
    <v-card :title="props.title" :text="props.text">
      <template v-slot:actions>
        <v-spacer></v-spacer>

        <v-btn variant="plain" @click="show = false"> {{ t("cancel") }} </v-btn>

        <v-btn
          variant="elevated"
          color="error"
          :loading="props.loading"
          @click="submit"
        >
          {{ props.actionText }}
        </v-btn>
      </template>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const props = defineProps<{
  title: string;
  text: string;
  actionText: string;
  loading: boolean;
}>();

const show = defineModel<boolean>("show");

const emit = defineEmits<{ (e: "submit"): void }>();

function submit() {
  emit("submit");
}
</script>
