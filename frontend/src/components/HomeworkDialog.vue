<template>
  <v-dialog v-model="show" max-width="500">
    <v-card title="Create a new homework">
      <v-form v-model="formValid" @submit.prevent="submit">
        <v-card-text>
          <v-text-field
            v-model="title"
            label="Title"
            required
            :rules="titleRules"
          ></v-text-field>

          <v-textarea label="Description" v-model="description"></v-textarea>

          <v-date-input
            v-model="date"
            label="Due date"
            prepend-icon=""
            prepend-inner-icon="$calendar"
          ></v-date-input>

          <v-autocomplete
            label="Subject"
            :loading="subjects === undefined"
            :disabled="subjects == undefined"
            :items="subjects ?? []"
            :item-props="subjectItemProp"
          ></v-autocomplete>
        </v-card-text>

        <v-card-actions>
          <v-spacer></v-spacer>

          <v-btn variant="plain" @click="show = false">Cancel</v-btn>

          <v-btn
            color="primary"
            variant="tonal"
            type="submit"
            :loading="props.loading"
            >Create</v-btn
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

const props = defineProps<{ loading: boolean }>();

const show = defineModel<boolean>("show");

const title = defineModel<string>("title");
const description = defineModel<string>("description");
const date = defineModel<string>("date");

const emit = defineEmits<{
  (e: "close"): void;
  (e: "submit"): void;
}>();

const formValid = ref(false);

const titleRules = [
  (v: any) => {
    if (v) return true;

    return "You must enter a title";
  },
];

function submit() {
  emit("submit");
}

const { data: subjects, mutate } = useSWRV<Subject[]>("/api/subjects", fetcher);

watch(show, (newValue, _oldValue) => {
  if (newValue) mutate();
});

function subjectItemProp(item: Subject) {
  return { title: item.name };
}
</script>
