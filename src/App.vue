<script setup lang="ts">
import ScreenLayout from "./layouts/ScreenLayout.vue";
</script>

<template>
  <ScreenLayout>
    <template #content>
      <RouterView />
    </template>
  </ScreenLayout>
</template>
