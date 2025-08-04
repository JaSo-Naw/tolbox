<template>
  <div class="app-container">
    <MainNav data-tauri-drag-region/>
    <div class="content-container">
      <SubNav />

      <router-view v-slot="{ Component }">
        <keep-alive>
          <component :is="Component" />
        </keep-alive>
      </router-view>

    </div>
  </div>
</template>

<script setup>
import MainNav from '@/components/layout/MainNav.vue'
import SubNav from '@/components/layout/SubNav.vue'

</script>

<style>
.app-container {
  display: flex;
  height: 100vh;
  overflow: hidden; /* 防止滚动条出现在错误的位置 */
}
.content-container {
  display: flex;
  flex: 1;
  overflow: hidden;
  height: 100%; /* 确保高度继承 */
}
</style>
