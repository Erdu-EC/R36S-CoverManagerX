<script setup lang="ts">

</script>

<template>
  <div class="flex flex-col gap-2">
    <label for="easy-dir" class="block font-bold">EASYROOM</label>
    <InputGroup>
      <InputText placeholder="Seleccione la ruta o letra de unidad de EASYROOM..."/>
      <Button icon="material-symbols-outlined before:icon-[folder\_open]" label="Abrir"/>
    </InputGroup>
    <small class="text-neutral-500">
      <strong>Nota:</strong> Normalmente la letra del dispositivo con nombre EASYROOM sera determinada de forma
      automática, de no ser asi, debes seleccionar su ubicación.
    </small>
  </div>
</template>

<style scoped>

</style>