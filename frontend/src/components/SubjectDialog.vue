<template>
  <v-dialog v-model="show" max-width="400">
    <v-card :title="title">
      <v-form v-model="formValid" @submit.prevent="submit">
        <v-card-text>
          <v-text-field
            v-model="name"
            label="Name"
            required
            :rules="nameRules"
          ></v-text-field>

          <color-input v-model="hexColor"></color-input>
        </v-card-text>

        <v-card-actions>
          <v-spacer></v-spacer>

          <v-btn variant="plain" @click="close">Cancel</v-btn>

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

    return "You must enter a name";
  },
];

const title = computed(() =>
  props.variant === "create" ? "Create a new subject" : "Edit a subject"
);
const action = computed(() =>
  props.variant === "create" ? "Create" : "Update"
);

function close() {
  emit("close");
}

function submit() {
  emit("submit");
}
</script>
