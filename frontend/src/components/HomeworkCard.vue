<template>
  <v-card min-width="330">
    <subject-indicator
      class="px-2 mt-3"
      v-if="!!homework.subject"
      :subject="homework.subject"
    />

    <v-card-title>{{ homework.title }}</v-card-title>

    <v-card-subtitle v-if="!!homework.description">{{
      homework.description
    }}</v-card-subtitle>

    <v-card-subtitle class="font-italic" v-else
      >No description provided</v-card-subtitle
    >

    <v-spacer></v-spacer>

    <v-card-actions class="px-3">
      <due-indicator
        v-if="!!homework.due_date"
        :dueDate="homework.due_date"
        :done="homework.done"
      />

      <v-spacer></v-spacer>

      <v-btn
        icon="mdi-check"
        :loading="statusChangeLoading"
        @click.stop="changeHomeworkStatus(homework, true)"
        v-tooltip="'Mark as done'"
        v-if="!homework.done"
      ></v-btn>

      <v-btn
        icon="mdi-undo"
        :loading="statusChangeLoading"
        @click.stop="changeHomeworkStatus(homework, false)"
        v-tooltip="'Mark as unfinished'"
        v-else
      ></v-btn>

      <v-btn icon="mdi-pencil" variant="text" v-tooltip="'Edit'"></v-btn>

      <v-btn icon="mdi-delete" variant="text" v-tooltip="'Remove'"></v-btn>
    </v-card-actions>
  </v-card>
</template>

<script setup lang="ts">
import { updateHomeworkStatus, type Homework } from "@/api";
import { ref } from "vue";

const { homework } = defineProps<{ homework: Homework }>();
const emit = defineEmits<{ (e: "mutate"): void }>();

const statusChangeLoading = ref<boolean>(false);

async function changeHomeworkStatus(homework: Homework, done: boolean) {
  statusChangeLoading.value = true;

  await updateHomeworkStatus(homework.id, done);

  statusChangeLoading.value = false;

  emit("mutate");
}
</script>
