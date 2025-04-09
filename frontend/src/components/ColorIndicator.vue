<template>
  <div
    class="badge"
    :style="{
      backgroundColor: props.color,
      width: size + 'px',
      height: size + 'px',
    }"
  ></div>
</template>

<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{ color: string; size?: number }>();

const size = computed(() => props.size ?? 20);
</script>

<style scoped>
.badge {
  margin: 0 8px;
  border-radius: 50%;
}
</style>
