<script setup lang="ts">
const props = withDefaults(defineProps<{
  value: string,
  type?: 'outline'
}>(), {
  type: 'outline'
})

const typeClasses = computed(() => {
  return {
    'material-symbols-outlined': props.type === 'outline'
  }
});
</script>

<template>
  <i :class="typeClasses">{{ value }}</i>
</template>

<style scoped>

</style>