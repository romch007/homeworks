<template>
  <v-text-field
    v-model="searchQuery"
    placeholder="Search for homework..."
    variant="outlined"
    prepend-inner-icon="mdi-magnify"
  ></v-text-field>

  <v-row>
    <v-col cols="2">
      <v-select
        v-model="completionFilter"
        label="Filter by homework completion"
        :items="['All', 'Unfinished', 'Done']"
      ></v-select>
    </v-col>

    <v-col cols="4">
      <v-autocomplete
        v-model="selectedSubjects"
        :loading="subjects === undefined"
        :disabled="subjects === undefined"
        :items="subjects"
        :item-props="subjectFilterItemProps"
        label="Filter by subjects"
        multiple
        chips
        closable-chips
      >
        <template v-slot:chip="{ props, item }">
          <v-chip v-bind="props" :color="item.raw.hex_color" variant="flat">{{
            item.title
          }}</v-chip>
        </template>
      </v-autocomplete>
    </v-col>
  </v-row>

  <div v-if="homeworks === undefined" class="d-flex flew-wrap ga-4">
    <v-skeleton-loader
      width="510"
      type="card"
      v-for="i in [...Array(3).keys()]"
      :key="i"
    />
  </div>

  <v-empty-state
    v-else-if="homeworks?.length === 0"
    title="No homework to display"
  ></v-empty-state>

  <v-row dense v-else>
    <v-col v-for="homework in homeworks" cols="4">
      <v-card class="cursor-pointer" v-ripple @click="homeworkClick(homework)">
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
            @click.stop="changeHomeworkStatus(homework, true)"
            v-tooltip="'Mark as done'"
            v-if="!homework.done"
          ></v-btn>

          <v-btn
            icon="mdi-undo"
            @click.stop="changeHomeworkStatus(homework, false)"
            v-tooltip="'Mark as not finished'"
            v-else
          ></v-btn>
        </v-card-actions>
      </v-card>
    </v-col>
  </v-row>
</template>

<script lang="ts" setup>
import { fetcher, updateHomeworkStatus } from "@/api";
import type { Homework, Subject } from "@/api";
import useSWRV from "swrv";
import { ref } from "vue";
import { useRouter } from "vue-router";

const router = useRouter();

definePage({
  meta: { title: "Homeworks" },
});

const { data: subjects } = useSWRV<Subject[]>("/api/subjects", fetcher);

const selectedSubjects = ref<Subject[]>([]);

function subjectFilterItemProps(item: Subject) {
  return { title: item.name };
}

const searchQuery = ref<string>("");
const completionFilter = ref<"All" | "Unfinished" | "Done">("All");

const {
  data: homeworks,
  error,
  mutate,
} = useSWRV<Homework[]>(() => {
  const params = new URLSearchParams();

  if (searchQuery.value.length > 0) {
    params.append("search", searchQuery.value);
  }

  if (selectedSubjects.value.length > 0) {
    params.append(
      "subject_ids",
      selectedSubjects.value.map((s) => s.id).join(","),
    );
  }

  if (completionFilter.value !== "All") {
    params.append("done", completionFilter.value === "Done" ? "true" : "false");
  }

  const queryString = params.toString();
  return `/api/homeworks${queryString ? "?" + queryString : ""}`;
}, fetcher);

function homeworkClick(homework: Homework) {
  router.push({ path: `/homeworks/${homework.id}` });
}

async function changeHomeworkStatus(homework: Homework, done: boolean) {
  await updateHomeworkStatus(homework.id, done);

  mutate();
}
</script>
