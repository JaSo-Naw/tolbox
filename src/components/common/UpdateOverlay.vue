<template>
  <div v-if="show" class="update-overlay">
    <div class="update-container">
      <div class="drop-zone">

        <div v-if="!finished" class="loading-container" @mouseenter="showTooltip = true" @mouseleave="showTooltip = false">
          <el-icon class="loading-icon" :size="30" color="#d4a017">
            <Loading />
          </el-icon>
          <span>Updating... {{ progress }}%</span>
        </div>
        <div v-else class="loading-container">
          <el-icon class="loading-icon" :size="30" color="#d4a017">
            <Loading />
          </el-icon>
          <span>Restarting...</span>
        </div>
      </div>

      <div
          v-if="showTooltip"
          class="custom-tooltip"
          v-html="tooltipContent"
      ></div>
    </div>
  </div>
</template>

<script setup>
import {ref, onMounted, computed} from "vue"
import { check } from "@tauri-apps/plugin-updater"
import { relaunch } from "@tauri-apps/plugin-process"
import {ElIcon, ElMessage} from "element-plus"
import { Loading } from "@element-plus/icons-vue"


const show = ref(false)
const progress = ref(0)
const finished = ref(false)
const version = ref("")
const currentVersion = ref("")
const notes = ref("")
const date = ref("")
const showTooltip = ref(false)

const tooltipContent = computed(() => {
  return `
    <div style="
      min-width:240px;
      background:rgba(255,255,255,0.95);
      border-radius:8px;
      padding:12px 16px;
      box-shadow:0 4px 12px rgba(0,0,0,0.08);
      font-size:13px;
      line-height:1.5;
      color:#333;
      position: absolute;
      top: 100%;
      left: 50%;
      transform: translate(-50%, calc(-100% - 20px)); /* 上浮20px */
      z-index: 10000;
      pointer-events: none; /* 防止遮挡鼠标事件 */
    ">
      <div style="font-weight:600;color:#d4a017;margin-bottom:6px;">
        新版本: ${version.value}
        <span style="font-size:12px;color:#999;">(${date.value})</span>
      </div>
      <div style="margin-bottom:6px;">
        <b>当前版本:</b> ${currentVersion.value}
      </div>
      <div style="white-space:normal;word-break:break-word;">
        <b>更新内容:</b> ${notes.value}
      </div>
    </div>
  `
})



async function checkUpdate() {

  let update = null;

  try {
    update = await check()
  }catch (e) {
    update = null
  }

    if (update) {
      version.value = update.version
      notes.value = update.body || ""
      currentVersion.value = update.currentVersion
      date.value = update.date
      progress.value = 0
      let downloaded = 0
      let contentLength = 0
      show.value = true

      try {
        await update.downloadAndInstall(event => {
          switch (event.event) {
            case "Started":
              contentLength = event.data.contentLength
              progress.value = 0
              break
            case "Progress":
              downloaded += event.data.chunkLength
              if (contentLength > 0) {
                progress.value = Math.min(100, Math.round((downloaded / contentLength) * 100))
              }
              break
            case "Finished":
              progress.value = 100
              finished.value = true
              break
          }
        })

        try {
          await relaunch()
        }catch (e){
          show.value = false
        }

      }catch (e) {
        show.value = false
      }finally {
        show.value = false
      }

    }


}

onMounted(() => {
  // 应用启动时检查更新
  checkUpdate()
})
</script>

<style scoped>
.update-overlay {
  position: fixed;
  inset: 0;
  background: rgba(255, 249, 240, 0.9);
  display: flex;
  justify-content: center;
  align-items: center;
  z-index: 9999;
}

.update-container {
  width: 60%;
  max-width: 500px;
  background: rgba(255, 255, 255, 0.9);
  border-radius: 16px;
  box-shadow: 0 6px 18px rgba(0, 0, 0, 0.05);
  padding: 40px;
  text-align: center;
}

.drop-zone {
  border: 2px dashed rgba(212, 160, 23, 0.3);
  border-radius: 8px;
  padding: 30px;
}


.loading-container {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 12px;
  font-size: 1.2rem;
  color: #d4a017;
}

.loading-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  100% {
    transform: rotate(360deg);
  }
}
</style>