<script setup lang="ts">
defineProps<{ value: string; animated: boolean }>()
</script>
<template>
  <div class="countdown" role="timer" :aria-label="value" aria-live="off" dir="ltr">
    <template v-if="animated">
      <span
        v-for="(character, index) in value"
        :key="index"
        class="countdown-cell"
        :class="{ separator: character === ':' }"
        aria-hidden="true"
      >
        <span :key="character" class="pixel-reveal">{{ character }}</span>
      </span>
    </template>
    <template v-else>{{ value }}</template>
  </div>
</template>
<style scoped>
.countdown {
  display: flex;
  justify-content: center;
}
.countdown-cell {
  display: inline-grid;
  place-items: center;
  width: 0.65em;
}
.countdown-cell.separator {
  width: 0.3em;
}
.countdown-cell > span {
  display: block;
}
</style>
