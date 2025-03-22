<template>
  <!-- <v-btn class="mb-10" color="primary" block @click="showDialog = true"
    >New subject</v-btn
    > -->

  <v-skeleton-loader type="list-item" v-if="data === undefined" />

  <v-empty-state v-else-if="data?.length === 0" title="No subject to display">
    <template v-slot:actions>
      <v-btn color="primary" @click="onCreate">Create your first subject</v-btn>
    </template>
  </v-empty-state>

  <div class="d-flex flex-row justify-start flex-wrap ga-4" v-else>
    <v-card
      class="d-flex flex-column"
      v-for="subject in data"
      width="220"
      height="131"
    >
      <v-card-title>{{ subject.name }}</v-card-title>

      <v-card-subtitle
        >Created {{ dayjs(subject.created_at).fromNow() }}</v-card-subtitle
      >

      <v-spacer></v-spacer>

      <v-card-actions>
        <color-indicator
          v-if="subject.hex_color !== undefined"
          :color="subject.hex_color"
        ></color-indicator>

        <v-spacer></v-spacer>

        <v-btn
          icon="mdi-pencil"
          variant="text"
          @click="onEdit(subject)"
        ></v-btn>

        <v-btn icon="mdi-delete" variant="text"></v-btn>
      </v-card-actions>
    </v-card>

    <v-btn
      prepend-icon="mdi-plus"
      variant="tonal"
      width="220"
      height="131"
      stacked
      @click="onCreate"
      >New subject</v-btn
    >
  </div>

  <subject-dialog
    :variant="formVariant"
    :loading="formLoading"
    v-model:show="showDialog"
    v-model:name="formName"
    v-model:hex-color="formHexColor"
    @submit="dialogSubmit"
    @close="dialogClose"
  ></subject-dialog>
</template>

<script setup lang="ts">
import { ref } from "vue";
import useSWRV from "swrv";

import type { Subject } from "@/api";
import { createSubject, fetcher, updateSubject } from "@/api";
import dayjs from "@/dayjs";

const showDialog = ref(false);

const formVariant = ref<"create" | "edit">("create");
const formLoading = ref<boolean>(false);
const formCurrentSubjectId = ref<number>(-1);
const formName = ref("");
const formHexColor = ref("#FFFFFF");

async function dialogSubmit() {
  formLoading.value = true;

  if (formVariant.value === "create") {
    await createSubject(formName.value, formHexColor.value);
  } else if (formVariant.value === "edit") {
    await updateSubject(
      formCurrentSubjectId.value,
      formName.value,
      formHexColor.value
    );
  }

  formLoading.value = false;

  mutate();

  showDialog.value = false;
}

function dialogClose() {
  showDialog.value = false;
}

function onCreate() {
  showDialog.value = false;
  resetDialogFields();

  formVariant.value = "create";

  showDialog.value = true;
}

function onEdit(subject: Subject) {
  showDialog.value = false;
  formCurrentSubjectId.value = subject.id;
  formVariant.value = "edit";

  formName.value = subject.name;
  formHexColor.value = subject.hex_color ?? "#FFFFFF";

  showDialog.value = true;
}

function resetDialogFields() {
  formName.value = "";
  formHexColor.value = "#FFFFFF";
}

const { data, error, isValidating, mutate } = useSWRV<Subject[]>(
  "/api/subjects",
  fetcher
);
</script>
