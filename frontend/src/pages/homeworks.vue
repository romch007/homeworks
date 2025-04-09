<template>
  <v-text-field
    v-model="searchQuery"
    placeholder="Search for homework..."
    variant="outlined"
    prepend-inner-icon="mdi-magnify"
  ></v-text-field>

  <v-skeleton-loader v-if="data === undefined" type="card" />

  <v-empty-state
    v-else-if="data?.length === 0"
    title="No homework to display"
  ></v-empty-state>

  <div class="d-flex flex-column flex-wrap ga-5" v-else>
    <v-card v-for="homework in data">
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
import type { Homework } from "@/api";
import useSWRV from "swrv";
import { ref } from "vue";

definePage({
  meta: { title: "Homeworks" },
});

const searchQuery = ref<string>("");

const { data, error, isValidating, mutate } = useSWRV<Homework[]>(
  () =>
    searchQuery.value.length === 0
      ? "/api/homeworks"
      : `/api/homeworks?search=${searchQuery.value}`,
  fetcher,
);
</script>
