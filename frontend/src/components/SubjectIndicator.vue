<template>
  <div class="d-inline-flex justify-start align-center">
    <color-indicator :color="props.subject.hex_color" :size="18" />

    {{ props.subject.name }}
  </div>
</template>

<script setup lang="ts">
import type { Subject } from "@/api";

const props = defineProps<{ subject: Subject }>();
</script>
