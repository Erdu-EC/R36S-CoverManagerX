<script setup lang="ts">
import type {NuxtError} from "#app";

defineProps<{
  error: NuxtError<any>,
  title?: string
}>();

const emit = defineEmits<{
  (ev: 'refresh'): void
}>();

</script>

<template>
  <Card>
    <template v-if="title" #title>
      {{ title }}
    </template>
    <template #content>
      <div class="flex flex-col items-center pt-5 pb-6">
        <MessageWithIcon :label="$t('ui.somethingWentWrongRetryAgain')"
                         :description="error.message"
                         icon="error" iconClass="!text-7xl" class="h-100"/>
        <Button outlined :label="$t('ui.tryAgain')" class="mt-5" @click="emit('refresh')"/>
      </div>
    </template>
  </Card>
</template>

<style scoped lang="scss">

</style>