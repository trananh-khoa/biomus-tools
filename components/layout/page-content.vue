<script lang="ts" setup>

</script>

<template>
  <div un-h-full un-flex="~ col">
    <!-- Title -->
    <h1 class="text-rainbow-gradient" un-font-700 un-mb-4 un-text="3xl">
      <slot name="title" />
    </h1>
    <!-- Description -->
    <p un-mb-8 un-text-neutral-400>
      <slot name="description" />
    </p>
    <!-- Content -->
    <slot />
  </div>
</template>

<style>

</style>
