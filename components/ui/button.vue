<script lang="ts" setup>

</script>

<template>
  <button
    un-flex="~ gap-2" un-items-center
    un-p="x-2 y-1"
    un-rounded-md
    un-bg="zinc-600 hover:zinc-700 disabled:zinc-800 active:zinc-800"
    un-text="sm disabled:neutral-500"
  >
    <div><slot /></div>
  </button>
</template>

<style>

</style>
