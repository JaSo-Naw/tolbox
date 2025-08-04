<template>
  <img :style="{height: size+ 'px'}" :src="iconMap(name)" :alt="{name}">
</template>

<script setup>

defineProps({
  name: {
    type: String,
    required: true
  },
  size: {
    type: [Number, String],
    default: 20
  }
})
const iconMap = (name) => {
  return new URL(`../../assets/icons/${name}.png`, import.meta.url).href
}
</script>

<style>

</style>