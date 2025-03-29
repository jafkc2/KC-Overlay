<template>
    <input
      type="text"
      v-model="hotkey"
      @keydown.prevent="handleKeyDown"
      placeholder="Press a shortcut"
      readonly
    />
  </template>
  
  <script setup>
  import { ref } from "vue";
  
  const hotkey = ref("");
  
  const emit = defineEmits(["hotkeyChange"]);
  
  const handleKeyDown = (event) => {
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
  input {
    width: 200px;
    padding: 5px;
    text-align: center;
  }
  </style>