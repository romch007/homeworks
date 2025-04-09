<template>
  <v-app>
    <v-app-bar>
      <template v-slot:prepend>
        <v-app-bar-nav-icon
          v-if="!$vuetify.display.mobile"
          @click.stop="drawer = !drawer"
        ></v-app-bar-nav-icon>
      </template>

      <v-app-bar-title>{{ appBarTitle }}</v-app-bar-title>
    </v-app-bar>

    <v-navigation-drawer
      v-model="drawer"
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

    <v-bottom-navigation v-if="$vuetify.display.mobile">
      <v-btn v-for="item in navigationItems" :to="item.to">
        <v-icon>{{ item.icon }}</v-icon>

        <span>{{ item.title }}</span>
      </v-btn>
    </v-bottom-navigation>

    <v-main>
      <v-container fluid>
        <router-view />
      </v-container>
    </v-main>

    <v-footer
      app
      v-if="!$vuetify.display.mobile"
      class="d-flex align-center justify-center ga-2 flex-wrap flex-grow-1 py-3"
    >
      © 2025 Romain Chardiny —
      <a href="https://www.gnu.org/licenses/agpl-3.0.html" target="_blank"
        >AGPLv3 License</a
      >
      —
      <a href="https://github.com/romch007/homeworks" target="_blank"
        >View Source</a
      >
    </v-footer>
  </v-app>
</template>

<script lang="ts" setup>
import { computed, ref } from "vue";
import { useRoute } from "vue-router";

const navigationItems = [
  { icon: "mdi-view-dashboard", title: "Dashboard", to: "/" },
  { icon: "mdi-notebook", title: "Homeworks", to: "/homeworks" },
  { icon: "mdi-tag-multiple", title: "Subjects", to: "/subjects" },
];

const drawer = ref<boolean>(false);

const route = useRoute();
const appBarTitle = computed(() => route.meta.title);
</script>
