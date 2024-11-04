<script setup lang="ts">
import type {IEmulatorData} from "~/data/models/IEmulatorData";

const {fileSystemCommands} = useTauriCommands();

const easyRoomPath = ref<string>();
const emulatorsData = ref<IEmulatorData[]>();

{
  const path = await fileSystemCommands.getEasyRomsDevicePath();
  if (path) {
    easyRoomPath.value = path;
    emulatorsData.value = await fileSystemCommands.getEmulatorsData(path);
  }
}

</script>

<template>
  <div class="flex flex-col gap-2">
    <label for="easy-dir" class="block font-bold">EASYROOM</label>
    <InputGroup>
      <InputText placeholder="Seleccione la ruta o letra de unidad de EASYROOM..." :value="easyRoomPath"/>
      <Button icon="material-symbols-outlined before:icon-[folder\_open]" label="Abrir"/>
    </InputGroup>
    <small class="text-neutral-500">
      <strong>Nota:</strong> Normalmente la letra del dispositivo con nombre EASYROOM sera determinada de forma
      automática, de no ser asi, debes seleccionar su ubicación.
    </small>

    <div>
      <ul v-if="emulatorsData">
        <li v-for="emulator in emulatorsData" :key="emulator.name">
          {{ emulator.name }}
        </li>
      </ul>
    </div>
  </div>
</template>

<style scoped>

</style>