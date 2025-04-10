<template>
  <v-skeleton-loader
    v-if="homework === undefined"
    type="text"
  ></v-skeleton-loader>

  <div v-else>
    <h1>{{ homework.title }}</h1>
  </div>
</template>

<script setup lang="ts">
import { fetcher } from "@/api";
import useSWRV from "swrv";
import { computed } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();

const currentHomeworkId = computed(() => Number(route.params.id));

const { data: homework } = useSWRV(
  `/api/homeworks/${currentHomeworkId.value}`,
  fetcher,
);
</script>
