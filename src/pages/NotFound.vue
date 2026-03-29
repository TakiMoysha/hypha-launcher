<script setup lang="ts">
import { ExclamationTriangleIcon } from "@heroicons/vue";
import { RouterLink } from "vue-router";
</script>

<template>
  <div class="max-w-md text-center">
    <ExclamationTriangleIcon></ExclamationTriangleIcon>

    <h2 class="mt-6 text-2xl font-bold">Page Not Found</h2>

    <p class="mt-4 text-pretty">
      Sorry, we couldn't find the page you're looking for.
    </p>

    <RouterLink to="/"><button type="button"></button></RouterLink>

    <button type="button"
      class="mt-6 block w-full rounded-lg bg-primary px-6 py-3 text-sm font-medium text-primary-content transition-colors hover:bg-accent">
      Go to Home
    </button>

    <p class="mt-6 text-sm text-gray-700 dark:text-gray-200">
      If You think this is a bug, you can create issue on
      <a href="#" class="underline hover:text-neutral">github page</a>.
    </p>
  </div>
</template>
