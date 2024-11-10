<script setup lang="ts">
const props = defineProps<{
  src: string | null,
  alt: string
}>();

const image = ref<HTMLImageElement>();
const loading = ref(!!props.src);
const error = ref(false);

const onLoad = () => {
  loading.value = false;
}

const onError = () => {
  loading.value = false;
  error.value = true;
}
</script>

<template>
  <div>
    <Skeleton v-if="loading" height="100%"></Skeleton>

    <div v-if="!src || error" class="bg-surface-200 dark:bg-surface-700 size-full flex items-center justify-center">
      <Icon value="image" class="text-surface-400 dark:text-surface-400 !text-5xl"/>
    </div>

    <img ref="image" v-if="src && !error" :src="src" :alt="alt" @load="onLoad" @error="onError"
         :class="{'opacity-0': loading}">

  </div>
</template>

<style scoped>

</style>