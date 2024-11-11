<script setup lang="ts">
const {easyRomsPath, emulatorsData} = storeToRefs(useAppStore());
const {fileSystemCommands} = useTauriCommands();

fileSystemCommands.openDirectoryDialog();
</script>

<template>
  <div class="flex flex-col gap-5 h-full">
    <div class="flex flex-col gap-2">
      <label for="easy-dir" class="block font-bold">{{ $t('console.storageDeviceName') }}</label>
      <InputGroup>
        <InputText placeholder="Seleccione la ruta o letra de unidad de EASYROOM..." :value="easyRomsPath"/>
        <Button icon="material-symbols-outlined before:icon-[folder\_open]" label="Abrir"/>
      </InputGroup>
      <small class="text-neutral-500">
        <strong>{{ $t('ui.note') }}</strong> {{ $t('pages.main.selectPathNote') }}
      </small>
    </div>

    <div>
      <ul v-if="emulatorsData?.length" class="grid grid-cols-3 items-end gap-5">
        <li v-for="emulator in emulatorsData" :key="emulator.name">
          <NuxtLinkLocale :to="`/consoles/${emulator.name}`">
            <EmulatorGridItem :emulator="emulator"/>
          </NuxtLinkLocale>
        </li>
      </ul>
      <Card v-else class="shrink-0">
        <template #content>
          <MessageWithIcon icon="info" label="No se encontraron emuladores"
                           class="text-neutral-500" iconClass="!text-6xl"/>
        </template>
      </Card>
    </div>
  </div>
</template>

<style scoped>

</style>