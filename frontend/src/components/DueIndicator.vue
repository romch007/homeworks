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

const isPasted = computed(
  () => dayjs(props.dueDate).isBefore(dayjs()) && !props.done,
);

const dueText = computed(() => {
  return props.done ? "Done" : "Due " + dayjs(props.dueDate).fromNow();
});

const dueTextStyle = computed(() => {
  let color = "";

  if (props.done) {
    color = "rgb(var(--v-theme-success)) !important";
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
  return isPasted.value ? "error" : undefined;
});
</script>
