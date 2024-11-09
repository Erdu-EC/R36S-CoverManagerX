<script setup lang="ts">
withDefaults(defineProps<{
  icon: string;
  iconType?: 'outlined';
  iconClass?: string;

  label: string;
  description?: string;
}>(), {
  iconType: 'outlined'
})
</script>

<template>
  <div class="flex flex-col items-center justify-center gap-2">
    <Icon v-if="icon" :type="iconType" :value="icon" :class="iconClass"/>
    <div class="text-2xl">{{ label }}</div>
    <span v-if="description" class="text-pretty text-neutral-400">{{ description }}</span>
  </div>
</template>

<style scoped>

</style>