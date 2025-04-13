<template>
  <div>
    <v-text-field
      v-model="searchQuery"
      :placeholder="t('searchForHomework')"
      variant="outlined"
      prepend-inner-icon="mdi-magnify"
    ></v-text-field>

    <v-row>
      <v-col cols="2">
        <v-select
          v-model="completionFilter"
          :label="t('filterByHomeworkCompletion')"
          :items="['all', 'unfinished', 'done']"
          :item-props="completionFilterItemProps"
          prepend-inner-icon="mdi-list-status"
        ></v-select>
      </v-col>

      <v-col cols="4">
        <v-autocomplete
          v-model="selectedSubjects"
          :loading="subjects === undefined"
          :disabled="subjects === undefined"
          :items="subjects"
          :item-props="subjectFilterItemProps"
          :label="t('filterBySubjects')"
          prepend-inner-icon="mdi-tag-multiple"
          multiple
          chips
          closable-chips
        >
          <template v-slot:chip="{ props, item }">
            <v-chip v-bind="props" :color="item.raw.hex_color" variant="flat">{{
              item.title
            }}</v-chip>
          </template>

          <template v-slot:item="{ props, item }">
            <v-list-item v-bind="props">
              <template v-slot:prepend>
                <v-icon icon="mdi-tag" :color="item.raw.hex_color"></v-icon>
              </template>
            </v-list-item>
          </template>
        </v-autocomplete>
      </v-col>
    </v-row>

    <div v-if="homeworks === undefined" class="d-flex flew-wrap ga-4">
      <v-skeleton-loader
        width="510"
        type="card"
        v-for="i in [...Array(3).keys()]"
        :key="i"
      />
    </div>

    <v-empty-state
      v-else-if="homeworks?.length === 0"
      :title="t('noHomework')"
    ></v-empty-state>

    <div class="d-flex flex-row flex-wrap ga-3" v-else>
      <homework-card
        v-for="homework in homeworks"
        :homework="homework"
        @mutate="mutate"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { fetcher } from "@/api";
import type { Homework, Subject } from "@/api";
import useSWRV from "swrv";
import { ref } from "vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

definePage({
  meta: { title: "homeworks" },
});

const { data: subjects } = useSWRV<Subject[]>("/api/subjects", fetcher);

const selectedSubjects = ref<Subject[]>([]);

function subjectFilterItemProps(item: Subject) {
  return { title: item.name };
}

function completionFilterItemProps(item: string) {
  return { title: t(item) };
}

const searchQuery = ref<string>("");
const completionFilter = ref<"all" | "unfinished" | "done">("all");

const {
  data: homeworks,
  error,
  mutate,
} = useSWRV<Homework[]>(() => {
  const params = new URLSearchParams();

  if (searchQuery.value.length > 0) {
    params.append("search", searchQuery.value);
  }

  if (selectedSubjects.value.length > 0) {
    params.append(
      "subject_ids",
      selectedSubjects.value.map((s) => s.id).join(","),
    );
  }

  if (completionFilter.value !== "all") {
    params.append("done", completionFilter.value === "done" ? "true" : "false");
  }

  const queryString = params.toString();
  return `/api/homeworks${queryString ? "?" + queryString : ""}`;
}, fetcher);
</script>
