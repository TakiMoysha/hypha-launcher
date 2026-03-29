<script setup lang="ts"></script>

<template>
  <div class="flex flex-col h-screen">
    <header class="p-4 bg-base-300 shrink-0">
      <div class="flex justify-between">
        Header ?
        <div>
          <div class="hidden sm:hidden md:hidden lg:block">large size</div>
          <div class="hidden sm:hidden md:block lg:hidden">medium size</div>
          <div class="hidden sm:block md:hidden lg:hidden">small size</div>
        </div>
      </div>
    </header>

    <div class="flex flex-1 min-h-0">
      <main class="flex-1 p-4 bg-base-300 overflow-auto mx-1 my-2">
        <slot name="content"> Container Area </slot>
      </main>
      <aside class="w-1/6 bg-base-300 p-4 shrink-0 overflow-auto mx-1 my-2 flex flex-col gap-2">
        <router-link class="hover:underline border-b-2 p-2" to="/dev/style-preview">style</router-link>
        <router-link class="hover:underline border-b-2 p-2" to="/dev">dev</router-link>
        <router-link class="hover:underline border-b-2 p-2" to="/not-found-page">404</router-link>
      </aside>
    </div>

    <footer class="p-4 bg-base-300 shrink-0 min-h-16">
      <div class="flex justify-between">
        <div role="button" class="">ProfileSelect</div>
        <div role="button">Run Game</div>
      </div>
    </footer>
  </div>
</template>
