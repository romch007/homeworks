<template>
  <div>
    <v-skeleton-loader v-if="homeworks === undefined"> </v-skeleton-loader>

    <div v-if="homeworks">
      <v-sheet>
        <v-calendar
          v-if="homeworks"
          view-mode="month"
          :events="calendarEvents"
        ></v-calendar>
      </v-sheet>
    </div>
  </div>
</template>

<script setup lang="ts">
import { fetcher, type Homework } from "@/api";
import useSWRV from "swrv";
import { computed } from "vue";
import dayjs from "@/dayjs";

definePage({
  meta: { title: "schedule" },
});

const {
  data: homeworks,
  error,
  mutate,
} = useSWRV<Homework[]>("/api/homeworks", fetcher);

const calendarEvents = computed(() =>
  homeworks.value
    ?.filter((homework) => homework.due_date)
    .map((homework) => {
      const dueDateStart = dayjs(homework.due_date);
      const dueDateEnd = dueDateStart.add(1, "hour");

      return {
        title: homework.title,
        start: dueDateStart.toDate(),
        end: dueDateEnd.toDate(),
        color: homework.subject?.hex_color,
        allDay: true,
      };
    }),
);
</script>
