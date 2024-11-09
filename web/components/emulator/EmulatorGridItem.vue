<script setup lang="ts">
import type {IEmulatorData} from "~/data/models/IEmulatorData";

defineProps<{
  emulator: IEmulatorData
}>();
</script>

<template>
  <Card v-ripple class="group cursor-pointer transition-all hover:scale-105 hover:duration-300">
    <template #content>
      <div class="flex flex-col items-center gap-2 h-[10.5rem]">
        <picture class="flex items-center w-32 max-h-32 grow">
          <img :src="`/img/emulators/${emulator.name}.svg`" :alt="`${emulator.name} logo`" height="128">
        </picture>
        <div class="text-pretty text-center">
          <span class="text-xl text-neutral-500 group-hover:text-black">
            {{ $t(`emulators.${emulator.name}.name`) }}
          </span>
        </div>
      </div>
    </template>
  </Card>
</template>

<style scoped>

</style>