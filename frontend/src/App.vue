<template>
  <v-app>
    <v-app-bar class="px-3">
      <template v-slot:prepend>
        <v-app-bar-nav-icon
          v-if="!$vuetify.display.mobile"
          @click.stop="showDrawer = !showDrawer"
        ></v-app-bar-nav-icon>
      </template>

      <v-app-bar-title v-if="appBarTitle">{{ t(appBarTitle) }}</v-app-bar-title>

      <v-spacer></v-spacer>

      <v-btn
        variant="tonal"
        color="info"
        prepend-icon="mdi-plus"
        @click="
          resetFields();
          showHomeworkDialog = true;
        "
      >
        {{ t("newHomework") }}
      </v-btn>
    </v-app-bar>

    <v-navigation-drawer
      v-model="showDrawer"
      v-if="!$vuetify.display.mobile"
      temporary
    >
      <v-list nav>
        <v-list-item
          v-for="item in navigationItems"
          :prepend-icon="item.icon"
          :title="item.title"
          :to="item.to"
        ></v-list-item>
      </v-list>
    </v-navigation-drawer>

    <homework-dialog
      :loading="homeworkDialogLoading"
      v-model:show="showHomeworkDialog"
      v-model:title="homeworkTitle"
      v-model:description="homeworkDescription"
      v-model:dueDate="homeworkDueDate"
      v-model:subject="homeworkSubject"
      @submit="submitCreateHomework"
    ></homework-dialog>

    <v-bottom-navigation v-if="$vuetify.display.mobile">
      <v-btn v-for="item in navigationItems" :to="item.to">
        <v-icon>{{ item.icon }}</v-icon>

        <span>{{ item.title }}</span>
      </v-btn>
    </v-bottom-navigation>

    <v-main>
      <v-container fluid>
        <router-view v-slot="{ Component }">
          <v-fade-transition hide-on-leave>
            <component :is="Component" />
          </v-fade-transition>
        </router-view>
      </v-container>
    </v-main>
  </v-app>
</template>

<script lang="ts" setup>
import { computed, ref } from "vue";
import { useRoute } from "vue-router";
import { createHomework, fetcher, type Subject } from "./api";
import { mutate } from "swrv";
import { useI18n } from "vue-i18n";

const { t } = useI18n();

const navigationItems = [
  { icon: "mdi-view-dashboard", title: t("dashboard"), to: "/" },
  { icon: "mdi-calendar-month", title: t("schedule"), to: "/schedule" },
  { icon: "mdi-notebook", title: t("homeworks"), to: "/homeworks" },
  { icon: "mdi-tag-multiple", title: t("subjects"), to: "/subjects" },
];

const showDrawer = ref<boolean>(false);
const showHomeworkDialog = ref<boolean>(false);

const homeworkDialogLoading = ref<boolean>(false);

const homeworkTitle = ref<string>("");
const homeworkDescription = ref<string>("");
const homeworkSubject = ref<Subject>();
const homeworkDueDate = ref<Date>(new Date());

const route = useRoute();
const appBarTitle = computed(() => route.meta.title as string | null);

async function submitCreateHomework() {
  homeworkDialogLoading.value = true;

  await createHomework(
    homeworkTitle.value,
    homeworkDescription.value,
    homeworkDueDate.value,
    homeworkSubject.value,
  );

  mutate("/api/homeworks", fetcher("/api/homeworks"));

  homeworkDialogLoading.value = false;
  showHomeworkDialog.value = false;

  resetFields();
}

function resetFields() {
  homeworkTitle.value = "";
  homeworkDescription.value = "";
  homeworkDueDate.value = new Date();
  homeworkSubject.value = undefined;
}
</script>
