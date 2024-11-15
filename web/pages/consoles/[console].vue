<script setup lang="ts">
const route = useRoute();
const code = route.params.console as string;

const {emulator, roms} = useRomCoversPage(code);
</script>

<template>
  <div class="flex flex-col h-full">
    <div class="flex gap-5 border-b pb-5">
      <figure class="block size-32">
        <figcaption class="sr-only">Console:</figcaption>

        <img :src="`/img/emulators/${code}.svg`" :alt="`Logo ${code}`"
             class="h-full object-scale-down"/>
      </figure>
      <div class="grid gap-y-2 gap-x-5">
        <div class="flex flex-col items-start gap-2">
          <h1 class="text-3xl font-bold ">
            {{ $t(`emulators.${code}.name`) }}
          </h1>
          <Chip v-if="roms" :label="`${roms.length} juegos disponibles`" class="font-medium grow-0"/>
        </div>
        <div>
          <div class="grid grid-cols-2 gap-2 text-neutral-500">
            <strong>Extensiones soportadas:</strong>
            <span>{{ emulator.formats.join(', ') }}</span>
          </div>
        </div>
      </div>
    </div>

    <div class="grow">
      <NuxtPage/>
    </div>
  </div>
</template>

<style scoped>

</style>