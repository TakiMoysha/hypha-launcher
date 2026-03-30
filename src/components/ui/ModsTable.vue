<script setup lang="ts">
const props = defineProps({
  provider: String,
});
</script>

<template>
  <div>
    <div>ModsTable</div>
    <div>{{ provider }}</div>
  </div>
</template>
