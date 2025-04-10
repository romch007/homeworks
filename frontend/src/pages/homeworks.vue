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
        :loading="isSubjectsLoading"
        :disabled="isSubjectsLoading"
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

  <v-skeleton-loader v-if="isHomeworksLoading" type="card" />

  <v-empty-state
    v-else-if="homeworks?.length === 0"
    title="No homework to display"
  ></v-empty-state>

  <div class="d-flex flex-column flex-wrap ga-5" v-else>
    <v-card v-for="homework in homeworks">
      <subject-indicator
        class="px-2 mt-3"
        v-if="!!homework.subject"
        :subject="homework.subject"
      />

      <v-card-title>{{ homework.title }}</v-card-title>

      <v-card-subtitle
        :style="{ whiteSpace: 'preserve' }"
        v-if="!!homework.description"
        >{{ homework.description }}</v-card-subtitle
      >

      <v-card-subtitle class="font-italic" v-else
        >No description provided</v-card-subtitle
      >

      <v-spacer></v-spacer>

      <v-card-actions class="px-3">
        <due-indicator
          v-if="!!homework.due_date"
          :dueDate="homework.due_date"
        />
      </v-card-actions>
    </v-card>
  </div>
</template>

<script lang="ts" setup>
import { fetcher } from "@/api";
import type { Homework, Subject } from "@/api";
import useSWRV from "swrv";
import { computed, ref } from "vue";

definePage({
  meta: { title: "Homeworks" },
});

const { data: subjects, isLoading: isSubjectsLoading } = useSWRV<Subject[]>(
  "/api/subjects",
  fetcher,
);

const selectedSubjects = ref<Subject[]>([]);

function subjectFilterItemProps(item: Subject) {
  return { title: item.name };
}

const searchQuery = ref<string>("");
const completionFilter = ref<"All" | "Unfinished" | "Done">("All");

const {
  data: homeworks,
  isLoading: isHomeworksLoading,
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
</script>
