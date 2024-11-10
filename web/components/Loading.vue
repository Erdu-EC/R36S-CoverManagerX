<script setup lang="ts">
withDefaults(defineProps<{
  label: string,
  size?: string,
  strokeWidth?: number
}>(), {
  size: '80px',
  strokeWidth: 4
})
</script>

<template>
  <div class="flex flex-col items-center justify-center gap-4">
    <ProgressSpinner v-once :style="{width: size, height: size}" :strokeWidth="strokeWidth.toString()"
                     class="fill-surface-0 dark:fill-surface-800"
                     animationDuration=".5s"/>
    <span class="text-2xl">{{ label }}
      <span class="animate-blink-1">. </span>
      <span class="animate-blink-2">. </span>
      <span class="animate-blink-3">.</span>
    </span>
  </div>
</template>

<style scoped lang="scss">

</style>