<template>
  <v-menu
    transition="scale-transition"
    location="center"
    location-strategy="connected"
    target="cursor"
  >
    <template v-slot:activator="{ props }">
      <v-btn
        :color="displayColor"
        v-bind="props"
        block
        prepend-icon="mdi-palette"
        size="large"
      >
        Change color
      </v-btn>
    </template>

    <v-sheet border rounded elevation="10" max-height="150" max-width="320">
      <v-container fluid>
        <v-row>
          <v-col
            v-for="btnColor in availableColors"
            cols="3"
            class="d-flex align-center justify-center"
          >
            <v-sheet
              class="cursor-pointer"
              :color="btnColor"
              border
              rounded
              width="40"
              height="40"
              @click="colorClick(btnColor)"
            ></v-sheet>
          </v-col>
        </v-row>
      </v-container>
    </v-sheet>
  </v-menu>
</template>

<script setup lang="ts">
import { randomChoice } from "@/utils";
import { computed } from "vue";

const availableColors = [
  "#E57373", // Soft Red
  "#64B5F6", // Soft Blue
  "#81C784", // Soft Green
  "#BA68C8", // Soft Purple
  "#FFD54F", // Soft Yellow
  "#FFB74D", // Soft Orange
  "#4DB6AC", // Teal
  "#A1887F", // Soft Brown/Neutral
];

const color = defineModel<string>();

const displayColor = computed(
  () => color.value ?? randomChoice(availableColors),
);

function colorClick(btnColor: string) {
  color.value = btnColor;
}
</script>
