<script setup lang="ts">
const props = withDefaults(defineProps<{
  value: string,
  type?: 'outlined'
}>(), {
  type: 'outlined'
})

const typeClasses = computed(() => {
  return {
    'material-symbols-outlined': props.type === 'outlined'
  }
});
</script>

<template>
  <i :class="typeClasses">{{ value }}</i>
</template>

<style scoped>

</style>