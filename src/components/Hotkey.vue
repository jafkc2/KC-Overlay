<template>
  <input type="text" v-model="hotkey" @keydown.prevent="handleKeyDown" placeholder="Crie um atalho" readonly />
</template>

<script setup lang="ts">
import { ref } from "vue";

interface Props {
  hotkey: String
}
const props = defineProps<Props>();
const hotkey = ref(props.hotkey);
const emit = defineEmits(["hotkeyChange"]);

const handleKeyDown = (event: KeyboardEvent) => {
  let keys = [];
  if (event.ctrlKey || event.metaKey) keys.push("Ctrl");
  if (event.shiftKey) keys.push("Shift");
  if (event.altKey) keys.push("Alt");

  if (event.key.length === 1 || /^[A-Z]$/.test(event.key.toUpperCase())) {
    keys.push(event.key.toUpperCase());
  }

  hotkey.value = keys.join("+");
  emit("hotkeyChange", hotkey.value);
};


</script>

<style scoped>
input{
  text-align: center;
  width: 150px;
  display: flex;
}
</style>