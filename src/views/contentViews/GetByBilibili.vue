<template>
  <div class="processor-container">
    <!-- 输入BV号和标题 -->
    <div class="input-section folder-input">
      <div class="drop-zone">
        <textarea
            v-model="bv"
            placeholder="输入B站视频BV号"
            class="input-box textarea-box"
            rows="1"
        ></textarea>
      </div>
    </div>

    <div class="input-section folder-input">
      <div class="drop-zone">
        <textarea
            v-model="title"
            placeholder="输入视频标题"
            class="input-box textarea-box"
            rows="1"
        ></textarea>
      </div>
    </div>

    <!-- 选择保存目录 -->
    <div class="input-section folder-input">
      <div class="drop-zone" @click="selectFolder">
        <div class="input-value save-path">{{ savePath || "点击选择保存目录" }}</div>
      </div>
    </div>

    <!-- 下载结果 -->
    <div class="input-section output-input">
      <div class="drop-zone">
        <div v-if="loading" class="loading-container">
          <el-icon class="loading-icon" :size="30" color="#d4a017">
            <Loading/>
          </el-icon>
          <span>正在下载...</span>
        </div>
        <pre v-else class="input-value">{{ result || "下载状态显示区" }}</pre>
      </div>
    </div>

    <!-- 操作按钮 -->
    <div class="action-buttons">
      <button @click="reset" class="action-btn reset-btn" :disabled="loading">↻</button>
      <button @click="download" class="action-btn process-btn" :disabled="loading">→</button>
    </div>
  </div>
</template>

<script setup>
import {ref} from "vue"
import {invoke} from "@tauri-apps/api/core"
import {open} from "@tauri-apps/plugin-dialog"
import {ElMessage, ElIcon} from "element-plus"
import {Loading} from "@element-plus/icons-vue"

const bv = ref("")
const title = ref("")
const savePath = ref("")
const result = ref("")
const loading = ref(false)

async function selectFolder() {
  const folder = await open({
    directory: true,
    multiple: false
  })
  if (folder) {
    savePath.value = folder
  }
}

async function download() {
  if (!bv.value || !title.value || !savePath.value) {
    ElMessage.error("请输入BV号、标题并选择保存目录")
    return
  }
  loading.value = true
  result.value = ""
  try {
    const res = await invoke("download_bilibili", {
      bv: bv.value,
      title: title.value,
      path: savePath.value
    })
    result.value = res
    ElMessage.success(res)
  } catch (e) {
    result.value = e
  } finally {
    loading.value = false
  }
}


function reset() {
  bv.value = ""
  title.value = ""
  savePath.value = ""
  result.value = ""
}
</script>

<style scoped>
/* 直接复用你翻译器的样式 */
.processor-container {
  min-height: 100vh;
  overflow-y: auto;
  width: 100vw;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;
  background-color: #fff9f0;
  gap: 20px;
  padding: 20px 0;
}

.input-section {
  width: 60%;
  max-width: 500px;
  display: flex;
  align-items: center;
  gap: 20px;
  padding: 25px;
  background: rgba(255, 255, 255, 0.7);
  border-radius: 16px;
  box-shadow: 0 6px 18px rgba(0, 0, 0, 0.05);
  transition: all 0.3s ease;
}

.drop-zone {
  flex-grow: 1;
  text-align: center;
  padding: 15px;
  border: 2px dashed rgba(212, 160, 23, 0.3);
  border-radius: 8px;
  transition: all 0.3s;
  word-break: break-word;
}

.drop-zone:hover {
  border-color: rgba(212, 160, 23, 0.6);
}

.input-box {
  width: 100%;
  border: none;
  outline: none;
  background: transparent;
  font-size: 1.2rem;
  color: #333;
  font-family: 'Maple Mono Normal NF CN', sans-serif;
  text-align: left;
  resize: none;
}

.textarea-box {
  min-height: 40px;
}

.input-value {
  font-size: 1.2rem;
  color: #333;
  font-family: 'Maple Mono Normal NF CN', sans-serif;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
  white-space: pre-line;
  text-align: left;
}
.save-path:hover {
  cursor: pointer;
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

.action-buttons {
  display: flex;
  gap: 30px;
}

.action-btn {
  width: 60px;
  height: 60px;
  border-radius: 50%;
  border: none;
  font-size: 1.8rem;
  cursor: pointer;
  transition: all 0.3s;
  display: flex;
  align-items: center;
  justify-content: center;
}

.reset-btn {
  background: rgba(212, 160, 23, 0.1);
  color: #8a6d0b;
}

.process-btn {
  background: #d4a017;
  color: white;
}

.action-btn:hover {
  transform: scale(1.1);
}

.action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
  color: #ccc;
}

.input-section,
.action-buttons {
  animation: fadeInUp 0.8s ease-out;
}

@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.folder-input {
  animation-delay: 0.1s;
}

.output-input {
  animation-delay: 0.2s;
}

.action-buttons {
  animation-delay: 0.3s;
}
</style>
