<template>
  <div>
    <div>
      <NuxtRouteAnnouncer/>

      <NuxtLayout>
        <NuxtPage/>
      </NuxtLayout>
    </div>
  </div>
</template>

<script setup lang="ts">
import "./assets/vendor/material-symbols/outlined.css";

const appStore = useAppStore();
const {easyRomsPath} = storeToRefs(appStore);

if (!easyRomsPath.value) {
  const {fileSystemCommands} = useTauriCommands();
  const path = await fileSystemCommands.getEasyRomsDevicePath();

  if (path)
    appStore.setEasyRomsPath(path);
}

if (easyRomsPath.value)
  await appStore.fetchEmulators();
</script>

<style lang="scss">

</style>