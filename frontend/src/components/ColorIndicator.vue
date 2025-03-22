<template>
  <div class="badge" :style="{ backgroundColor: color }"></div>
</template>

<script setup lang="ts">
const { color } = defineProps<{ color: string }>();
</script>

<style scoped>
.badge {
  margin: 0 8px;
  min-width: 20px;
  min-height: 20px;
  border-radius: 50%;
}
</style>
