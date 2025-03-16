<template>
  <div class="custom-select" ref="dropdown" @click="toggleDropdown">
    <div class="selected-option">
      {{ selectedOption || placeholder }}
    </div>
    <ul v-if="isOpen" class="options-list">
      <li
        v-for="(option, index) in options"
        :key="index"
        @click="selectOption(option)"
        :class="{ 'selected': option === selectedOption }"
      >
        {{ option }}
      </li>
    </ul>
  </div>
</template>

<script>
export default {
  props: {
    options: {
      type: Array,
      required: true,
    },
    placeholder: {
      type: String,
      },
    value: {
      type: String,
      default: null,
      },
    },
  data() {
    return {
      isOpen: false,
      selectedOption: this.value,
    };
  },
  watch: {
    value(newValue) {
      this.selectedOption = newValue;
    },
  },
  methods: {
    toggleDropdown() {
      this.isOpen = !this.isOpen;
    },
    selectOption(option) {
      this.selectedOption = option;
      this.isOpen = false;
      this.$emit('input', option);
    },

    handleClickOutside(event) {
      if (this.$refs.dropdown && !this.$refs.dropdown.contains(event.target)) {
        this.isOpen = false;
      }
    },
  },

  mounted() {
    document.addEventListener("click", this.handleClickOutside);
  },
  beforeDestroy() {
    document.removeEventListener("click", this.handleClickOutside);
  },
};
</script>

<style scoped>
.custom-select {
  position: relative;
  width: 200px;
  cursor: pointer;
  border: 1px solid #ccc;
  border-radius: 4px;
  padding: 8px;
  background-color: rgb(49, 50, 68);
}

.selected-option {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.options-list {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  border: 1px solid #ccc;
  border-top: none;
  border-radius: 0 0 4px 4px;
  background-color: rgb(49, 50, 68);
  list-style: none;
  padding: 0;
  margin: 0;
  z-index: 1000;
}

.options-list li {
  padding: 8px;
  cursor: pointer;
}

.options-list li:hover {
  background-color: #f0f0f0;
}

.options-list li.selected {
  background-color: #e0e0e0;
}
</style>