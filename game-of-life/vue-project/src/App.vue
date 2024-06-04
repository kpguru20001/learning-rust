<script setup lang="ts">
import { Universe } from "wasm-game-of-life";
import { onMounted, ref } from "vue";

const text = ref('')
const universe = Universe.new();

const renderLoop = () => {
  text.value = universe.render();
  universe.tick();

  setTimeout(() => requestAnimationFrame(renderLoop), 200);

};

onMounted(() => {
  requestAnimationFrame(renderLoop);
})
</script>

<template>
  <pre>
    {{ text }}
  </pre>
</template>

<style>
body {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

pre {}
</style>
