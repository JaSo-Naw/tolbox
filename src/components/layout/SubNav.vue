<template>
  <div class="sub-nav" v-if="layoutStore.activeNav !== 'home'">
<!--    用来占空，显得好看-->
    <div class="sub-nav-header"></div>

    <div class="sub-nav-content">
      <div
          v-for="item in subNavItems"
          :key="item.id"
          class="sub-nav-item"
          :class="{ active: activeSubNav === item.id }"
          @click="switchSubNav(item.id)"
      >
        <Icon :name="item.icon" size="18" />
        <span class="sub-nav-text">{{ item.text }}</span>
        <span v-if="item.badge" class="sub-nav-badge">{{ item.badge }}</span>
      </div>
    </div>

  </div>
</template>

<script setup>
import {computed, ref, watch} from 'vue'
import { useLayoutStore } from '@/stores'
import Icon from '@/components/common/Icon.vue'
import {subNav} from "@/common/layout.js";

const layoutStore = useLayoutStore()


const subNavItems = computed(() => {
  return subNav[layoutStore.activeNav] || []
})

const activeSubNav = ref('')

const switchSubNav = (id) => {
  activeSubNav.value = id
  layoutStore.setActiveSubNav(id)
}
watch(()=>layoutStore.activeSubNav,(val)=> {
  activeSubNav.value = val
},{})
</script>

<style scoped>


.sub-nav {
  width: 21vw;
  flex-shrink: 0;
  min-height: 100%;
  background: var(--bg-subnav);
  border-right: 1px solid #e5e7eb;
  display: flex;
  flex-direction: column;
  transition: all 0.3s ease;
  overflow-y: auto; /* 可选，如果内容超出时添加滚动条 */
}

.sub-nav-header {
  margin-bottom: 15px; /* 与内容区的间距 */
}


.sub-nav-content {
  display: flex;
  flex-direction: column;
  gap: 6px; /* 项与项之间的间隙 */
  padding: 0 8px; /* 左右内边距 */
}
.sub-nav-item {
  padding: 10px 16px;
  border-radius: 8px;
  display: flex;
  align-items: center;
  transition: all 0.2s ease;
  cursor: pointer;
  color: var(--text-light);
  margin: 2px 0; /* 增加上下外边距 */
}

.sub-nav-item:hover {
  background: var(--bg-hover);
}

.sub-nav-item.active {
  background: var(--bg-active);
  color: var(--primary);
  font-weight: 600;
}

.sub-nav-text {
  margin-left: 10px;
  font-size: 14px;
}

.sub-nav-badge {
  background: var(--secondary);
  color: white;
  font-size: 11px;
  padding: 2px 6px;
  border-radius: 12px;
  margin-left: auto;
}
</style>

