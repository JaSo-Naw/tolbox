<template>
  <div class="processor-container">
    <!-- 输入 -->
    <div class="input-section folder-input">
      <div class="drop-zone">
        <textarea
            v-model="input"
            @keydown.enter.exact.prevent="translate"
            placeholder="输入单词或句子"
            class="input-box textarea-box"
            rows="3"
        ></textarea>
      </div>
    </div>

    <!-- 翻译结果 -->
    <div class="input-section output-input">
      <div class="drop-zone">
        <div v-if="loading" class="loading-container">
          <el-icon class="loading-icon" :size="30" color="#d4a017">
            <Loading />
          </el-icon>
          <span>正在翻译...</span>
        </div>
        <pre v-else class="input-value">{{ result || "翻译结果将在这里显示" }}</pre>
      </div>
    </div>
    <!-- 操作按钮 -->
    <div class="action-buttons">
      <button @click="reset" class="action-btn reset-btn">↻</button>
      <button @click="translate" class="action-btn process-btn">→</button>
    </div>

  </div>
</template>

<script setup>
import {ref} from "vue"
import { invoke } from "@tauri-apps/api/core"
import { ElMessage } from "element-plus"
import { ElIcon } from "element-plus"
import { Loading } from "@element-plus/icons-vue"

const input = ref("")
const result = ref("")
const loading = ref(false)


async function translate() {
  if (!input.value) {
    ElMessage.error("请输入单词或句子")
    return
  }
  loading.value = true
  result.value = ""
  try {
    // 先试单词翻译
    const res = await invoke("translate_word", { word: input.value })
    if (Array.isArray(res) && res.length > 0) {
      result.value = res.join("\n")
    } else {
      // 没结果就试句子翻译
      const sentence = await invoke("translate_sentence", { sentence: input.value })
      result.value = "【" + sentence + "】"
    }
    ElMessage.success("翻译成功")
  } catch (e) {
    ElMessage.error(e)
    result.value = "无翻译结果"
  } finally {
    loading.value = false
  }
}

function reset() {
  input.value = ""
  result.value = ""
}
</script>

<style scoped>
.processor-container {
  min-height: 100vh;
  overflow-y: auto;
  width: 100vw;
  display: flex;
  flex-direction: column;
  justify-content: flex-start;
  align-items: center;
  background-color: #fff9f0;
  gap: 40px;
  padding: 60px 0;
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
  min-height: 80px;
}

.input-value {
  font-size: 1.2rem;
  color: #333;
  font-family: 'Maple Mono Normal NF CN', sans-serif;
  text-shadow: 1px 1px 2px rgba(0, 0, 0, 0.1);
  white-space: pre-line; /* 保留换行 */
  text-align: left;
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


.reset-btn:disabled {
  color: #ccc; /* 可选：改变禁用时的颜色 */
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
