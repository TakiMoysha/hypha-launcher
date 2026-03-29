<script setup lang="ts"></script>

<template>
  <div class="m-auto grid grid-cols-12 gap-1">
    <header class="col-span-full p-4 bg-base-300">
      <div class="flex justify-between">
        Header ?
        <div>
          <div class="hidden sm:hidden md:hidden lg:block">large size</div>
          <div class="hidden sm:hidden md:block lg:hidden">medium size</div>
          <div class="hidden sm:block md:hidden lg:hidden">small size</div>
        </div>
      </div>
    </header>

    <main class="col-span-10 p-4 bg-base-300 max-h-screen">
      <slot name="content"> Container Area </slot>
    </main>
    <aside class="col-span-2 bg-base-300 p-4">aside</aside>

    <footer class="col-span-full bg-base-300 p-4 min-h-16">
      <div class="flex justify-between">
        <div role="button" class="">ProfileSelect</div>
        <div role="button">Run Game</div>
      </div>
    </footer>
  </div>
</template>
