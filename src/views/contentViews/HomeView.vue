<template>
  <div class="logo-container">
    <div class="logo-content">
      <img src="@/assets/logo.png" alt="Logo" class="logo-image">
      <div class="logo-text">tolbox</div>
    </div>
  </div>
</template>

<script setup>

</script>

<style scoped>
.logo-container {
  height: 100vh;
  width: 100vw;
  display: flex;
  justify-content: center;
  align-items: center;
  background-color: #fff9f0; /* 浅黄色背景 */
}

.logo-content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 20px; /* 图片和文字之间的间距 */
}

.logo-image {
  max-width: 80%;
  max-height: 60%; /* 为文字留出空间 */
  object-fit: contain;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
}

.logo-text {
  font-size: 3rem;
  font-weight: bold;
  color: #333;
  font-family: 'Maple Mono Normal NF CN','Helvetica Neue', Arial, sans-serif;
  text-shadow: 2px 2px 2px rgba(0, 0, 0, 0.3);
  animation: fadeIn 0.8s ease-out 0.2s both; /* 延迟0.2秒出现 */
}

/* 动画效果 */
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(10px); }
  to { opacity: 1; transform: translateY(0); }
}

.logo-image {
  animation: fadeIn 0.8s ease-out;
}

@media (max-width: 768px) {
  .logo-text {
    font-size: 1.5rem;
  }
}
</style>