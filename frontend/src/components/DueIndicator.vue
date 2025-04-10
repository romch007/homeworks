<template>
  <div class="d-flex ga-2 justify-start align-center">
    <v-icon :icon="icon" :color="iconColor"></v-icon>

    <span :style="dueTextStyle">{{ dueText }}</span>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import dayjs from "@/dayjs";

const props = defineProps<{ dueDate: string; done: boolean }>();

const dueDate = computed(() => dayjs(props.dueDate));

const isPasted = computed(() => dueDate.value.isBefore(dayjs()) && !props.done);
const isClose = computed(
  () =>
    dueDate.value.isAfter(dayjs()) &&
    dueDate.value.diff(dayjs(), "day", true) < 2,
);

const dueText = computed(() => {
  return props.done ? "Done" : "Due " + dueDate.value.fromNow();
});

const dueTextStyle = computed(() => {
  let color = "";

  if (props.done) {
    color = "rgb(var(--v-theme-success)) !important";
  }

  if (isClose.value) {
    color = "rgb(var(--v-theme-warning)) !important";
  }

  if (isPasted.value) {
    color = "rgb(var(--v-theme-error)) !important";
  }

  return { color };
});

const icon = computed(() => {
  if (props.done) return "mdi-clock-check";
  return isPasted.value ? "mdi-clock-alert" : "mdi-clock";
});

const iconColor = computed(() => {
  if (props.done) return "success";
  if (isClose.value) return "warning";
  return isPasted.value ? "error" : undefined;
});
</script>
