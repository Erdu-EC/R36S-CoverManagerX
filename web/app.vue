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

const {fileSystemCommands} = useTauriCommands();
const path = await fileSystemCommands.getEasyRomsDevicePath();

if (path) {
  const appStore = useAppStore();
  appStore.setEasyRomsPath(path);
  appStore.fetchEmulators();
}
</script>

<style lang="scss">

</style>