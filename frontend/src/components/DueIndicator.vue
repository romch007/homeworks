<template>
  <div class="d-flex ga-2 justify-start align-center">
    <v-icon :icon="icon" :color="iconColor"></v-icon>

    Due {{ dueText }}
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import dayjs from "@/dayjs";

const props = defineProps<{ dueDate: string }>();

const isPasted = computed(() => dayjs(props.dueDate).isBefore(dayjs()));
const dueText = computed(() => dayjs(props.dueDate).fromNow());
const icon = computed(() => (isPasted.value ? "mdi-clock-alert" : "mdi-clock"));
const iconColor = computed(() => (isPasted.value ? "error" : undefined));
</script>
