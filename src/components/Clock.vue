<script setup lang="ts">
</script>

<template>
  <div class="wrapper">
    <div class="clock">
      <div class="arrow" />
      <div class="arrow" />
      <div class="arrow" />
      <div class="marks">
        <div class="mark" />
        <div class="mark" />
        <div class="mark" />
        <div class="mark" />
        <div class="mark" />
        <div class="mark" />
        <div class="mark" />
        <div class="mark" />
        <div class="mark" />
        <div class="mark" />
        <div class="mark" />
        <div class="mark" />
      </div>
    </div>

    <svg class="filter" xmlns="http://www.w3.org/2000/svg" version="1.1">
      <filter id="blurFilter">
        <feGaussianBlur stdDeviation="10" />
        <feColorMatrix
          type="matrix" values="1 0 0 0 0
                0 1 0 0 0
                0 0 1 0 0
                0 0 0 19 -9"
        />
      </filter>
    </svg>
  </div>
</template>

<style scoped>
.clock {
  position: relative;
}
.clock:after {
  content: "";
  position: absolute;
  width: 5rem;
  aspect-ratio: 1;
  translate: -50% -50%;
  border-radius: 1.5rem;
  background: conic-gradient(#ff2c74, #43202c);
}

.arrow {
  position: absolute;
  width: 20rem;
  height: 4rem;
  border-bottom-right-radius: 10rem;
  --duration: 5s;
  translate: 0 -50%;
  transform-origin: left;
  animation: clock calc(var(--duration) * 60) linear infinite;
  background: linear-gradient(to right, #e91e63, #00bcd4);
}
.arrow:first-of-type {
  width: 25rem;
  height: 1rem;
  background: linear-gradient(to right, #e91e63, #00bcd4);
  animation-duration: var(--duration);
}
.arrow:nth-child(2) {
  width: 14rem;
  background: linear-gradient(to right, #e91e63, #00bcd4);
  animation-duration: calc(var(--duration) * 60 * 60);
}

.marks {
  --size: 1.5rem;
  translate: calc(var(--size) * -0.5) calc(var(--size) * -0.5);
}

.mark {
  width: var(--size);
  aspect-ratio: 1 / 2.5;
  position: absolute;
  background: #e91e63;
  --r: 20rem;
  --ai: calc(360deg / 12 * var(--i));
  translate: calc(cos(var(--ai)) * var(--r)) calc(sin(var(--ai)) * var(--r) * -1);
  rotate: calc(90deg - 360deg / 12 * var(--i));
}
.mark:nth-child(1) {
  --i: 0;
}
.mark:nth-child(2) {
  --i: 1;
}
.mark:nth-child(3) {
  --i: 2;
}
.mark:nth-child(4) {
  --i: 3;
}
.mark:nth-child(5) {
  --i: 4;
}
.mark:nth-child(6) {
  --i: 5;
}
.mark:nth-child(7) {
  --i: 6;
}
.mark:nth-child(8) {
  --i: 7;
}
.mark:nth-child(9) {
  --i: 8;
}
.mark:nth-child(10) {
  --i: 9;
}
.mark:nth-child(11) {
  --i: 10;
}
.mark:nth-child(12) {
  --i: 11;
}
.mark:nth-child(13) {
  --i: 12;
}

@keyframes clock {
  to {
    rotate: 360deg;
  }
}

.wrapper {
  overflow: hidden;
  filter: url(#blurFilter);
  display: grid;
  place-items: center;
  height: 100dvh;
  width: 100dvw;
  --c1: rgba(255, 255, 255, 0.04);
  --c2: transparent;
  background: conic-gradient(var(--c1), var(--c2)), conic-gradient(from 180deg, var(--c1), var(--c2)), #111;
}
.filter {
  position: absolute;
  visibility: hidden;
  pointer-events: none;
}
* {
  box-sizing: border-box;
}
</style>
