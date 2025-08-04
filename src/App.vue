<template>
  <router-view></router-view>
</template>

<script setup>

//一启动，读取数据库autostart，根据其值设置autostart
import {invoke} from "@tauri-apps/api/core";

import {getOrInitAutoStart} from "@/db/modules/autostart.js";

async function initAutostart() {
  try {
    const res1 = await getOrInitAutoStart()

    if (res1 === 'true') {
      await invoke('change_autostart',{open: true})
    }else if (res1 === 'false') {
      await invoke('change_autostart',{open: false})
    }

  } catch (error) {
    console.log(error)
  }
}
initAutostart();

document.addEventListener('contextmenu', (e) => {
  e.preventDefault()
  return false
})
document.addEventListener('dragover', (e) => {
  e.preventDefault();
  return false;
});

document.addEventListener('drop', (e) => {
  e.preventDefault();
  return false;
});
</script>

<style>
/* 基础重置 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, 
               Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
  color: #2D3748;
  line-height: 1.5;
  background: #F8F9FA;
  overflow: hidden; /* 防止滚动条出现 */
}

/* 拖拽区域 */
[data-tauri-drag-region] {
  -webkit-app-region: drag;
}

/* 非拖拽区域 */
button {
  -webkit-app-region: no-drag;
}

/* 滚动条美化 */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

::-webkit-scrollbar-thumb {
  background: #CBD5E0;
  border-radius: 4px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

/* 过渡效果 */
a, button {
  transition: all 0.2s ease;
}

:root {
  --bg-main: #fdfcf7;
  --bg-nav: #fff9f5;
  --bg-subnav: #fffff4;
  --bg-hover: #feeeff;
  --bg-active: #e0f2fe;
  
  --primary: #60a5fa;
  --secondary: #f9a8d4;
  --text: #374151;
  --text-light: #6b7280;
}
body {
  background: var(--bg-main);
  color: var(--text);
}

</style>