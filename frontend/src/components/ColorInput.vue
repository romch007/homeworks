<template>
  <v-menu
    transition="scale-transition"
    location="center"
    location-strategy="connected"
    target="cursor"
  >
    <template v-slot:activator="{ props }">
      <v-sheet
        class="cursor-pointer d-flex justify-start"
        height="50"
        :color="color"
        border
        rounded
        v-bind="props"
      >
        <v-label class="mx-4 cursor-pointer" :style="{ opacity: 1 }"
          >Color</v-label
        >
      </v-sheet>
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

function colorClick(btnColor: string) {
  color.value = btnColor;
}
</script>
