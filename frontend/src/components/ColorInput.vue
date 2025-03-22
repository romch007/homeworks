<template>
  <v-text-field
    v-model="color"
    hide-details
    placeholder="Color"
    prepend-inner-icon="mdi-palette"
  >
    <template v-slot:append-inner>
      <v-menu
        v-model="menu"
        location="top"
        :close-on-content-click="false"
        offset="-145 16"
      >
        <template v-slot:activator="{ props }">
          <div :style="swatchStyle" v-bind="props" />
        </template>
        <v-card>
          <v-card-text class="pa-0">
            <v-color-picker v-model="color" flat />
          </v-card-text>
        </v-card>
      </v-menu>
    </template>
  </v-text-field>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";

const color = defineModel<string>();
const menu = ref(false);

const swatchStyle = computed(() => ({
  backgroundColor: color.value,
  cursor: "pointer",
  height: "30px",
  width: "30px",
  borderRadius: menu.value ? "50%" : "4px",
  transition: "border-radius 200ms ease-in-out",
}));
</script>

<style scoped></style>
