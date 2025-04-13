<template>
  <div>
    <div
      v-if="subjects === undefined"
      class="d-flex flex-row justify-start flex-wrap ga-4"
    >
      <v-skeleton-loader
        min-width="220"
        min-height="131"
        type="card"
        v-for="i in [...Array(10).keys()]"
        :key="i"
      />
    </div>

    <v-empty-state v-else-if="subjects?.length === 0" :title="t('noSubject')">
      <template v-slot:actions>
        <v-btn color="primary" @click="onCreate">{{
          t("createYourFirstSubject")
        }}</v-btn>
      </template>
    </v-empty-state>

    <div class="d-flex flex-row justify-start flex-wrap ga-4" v-else>
      <v-card
        class="d-flex flex-column"
        v-for="subject in subjects"
        width="220"
        height="131"
      >
        <v-card-title>{{ subject.name }}</v-card-title>

        <v-card-subtitle
          >{{ t("created") }}
          {{ dayjs(subject.created_at).fromNow() }}</v-card-subtitle
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
            v-tooltip="t('edit')"
          ></v-btn>

          <v-btn
            icon="mdi-delete"
            variant="text"
            @click="onRemove(subject)"
            v-tooltip="t('remove')"
          ></v-btn>
        </v-card-actions>
      </v-card>

      <v-btn
        prepend-icon="mdi-plus"
        variant="tonal"
        width="220"
        height="131"
        stacked
        @click="onCreate"
        >{{ t("newSubject") }}</v-btn
      >
    </div>

    <subject-dialog
      :variant="formVariant"
      :loading="formLoading"
      v-model:show="showDialog"
      v-model:name="formName"
      v-model:hex-color="formHexColor"
      @submit="dialogSubmit"
    ></subject-dialog>

    <confirmation-dialog
      :loading="confirmationLoading"
      :title="confirmationDialogTitle"
      :text="t('subjectDeleteConfirmation')"
      actionText="Delete"
      v-model:show="showConfirmationDialog"
      @submit="confirmationSubmit"
    ></confirmation-dialog>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import useSWRV from "swrv";

import type { Subject } from "@/api";
import { createSubject, deleteSubject, fetcher, updateSubject } from "@/api";
import dayjs from "@/dayjs";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

definePage({
  meta: { title: "subjects" },
});

const showDialog = ref(false);
const showConfirmationDialog = ref(false);

const formVariant = ref<"create" | "edit">("create");
const formLoading = ref<boolean>(false);
const formCurrentSubjectId = ref<number>(-1);
const formName = ref("");
const formHexColor = ref<string>();

const subjectToDelete = ref<Subject | null>(null);
const confirmationLoading = ref<boolean>(false);

const confirmationDialogTitle = computed(
  () => `${t("remove")} '${subjectToDelete.value?.name}'`,
);

const { data: subjects, mutate } = useSWRV<Subject[]>("/api/subjects", fetcher);

async function dialogSubmit() {
  formLoading.value = true;

  if (formVariant.value === "create") {
    await createSubject(formName.value, formHexColor.value);
  } else if (formVariant.value === "edit") {
    await updateSubject(
      formCurrentSubjectId.value,
      formName.value,
      formHexColor.value,
    );
  }

  formLoading.value = false;

  mutate();

  showDialog.value = false;
}

async function confirmationSubmit() {
  if (subjectToDelete.value === null) return;

  confirmationLoading.value = true;

  await deleteSubject(subjectToDelete.value.id);

  confirmationLoading.value = false;

  mutate();

  showConfirmationDialog.value = false;
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

function onRemove(subject: Subject) {
  subjectToDelete.value = subject;
  showConfirmationDialog.value = true;
}

function resetDialogFields() {
  formName.value = "";
  formHexColor.value = undefined;
}
</script>
