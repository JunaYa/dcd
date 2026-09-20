<script setup lang="ts">
defineProps<{ name: string }>()
const paths: Record<string, string> = {
  palette:
    'M12 3a9 9 0 1 0 0 18h1a2 2 0 0 0 1.6-3.2 1.5 1.5 0 0 1 1.2-2.4H17a4 4 0 0 0 4-4C21 6.8 17 3 12 3ZM7.5 10h.01M10 6.5h.01M15 6.5h.01M17.5 10h.01',
  sun: 'M12 8a4 4 0 1 0 0 8 4 4 0 0 0 0-8M12 2v1M12 21v1M2 12h1M21 12h1M5 5l1 1M18 18l1 1M5 19l1-1M18 6l1-1',
  chart: 'M3 11h4v10H3zM10 6h4v15h-4zM17 2h4v19h-4z',
  clock: 'M12 3a9 9 0 1 0 0 18 9 9 0 0 0 0-18M12 6v7H7',
  settings:
    'M10 2h4l.7 3 2 .9 2.7-1.4 2 3.5-2.2 2v2l2.2 2-2 3.5-2.7-1.4-2 .9-.7 3h-4l-.7-3-2-.9-2.7 1.4-2-3.5 2.2-2v-2l-2.2-2 2-3.5 2.7 1.4 2-.9zM12 8a4 4 0 1 0 0 8 4 4 0 0 0 0-8',
  eye: 'M3 8c4 0 6-5 10-4 3 0 5 3 8 4M3 14c5 5 13 5 18 0M5 17l-1 3M10 19v3M15 19v3M19 17l1 3',
  moon: 'M20 15A9 9 0 0 1 9 3a9 9 0 1 0 11 12Z',
  help: 'M12 3a9 9 0 1 0 0 18 9 9 0 0 0 0-18M9.5 9a2.5 2.5 0 1 1 4 2c-1 .6-1.5 1-1.5 2M12 16v.1',
  info: 'M12 3a9 9 0 1 0 0 18 9 9 0 0 0 0-18M12 10v7M12 7v.1',
  calendar: 'M4 4h16v16H4zM4 8h16M8 2v4M16 2v4M8 12h.1M12 12h.1M16 12h.1M8 16h.1M12 16h.1M16 16h.1',
  list: 'M8 5h13M8 12h13M8 19h13M3 5h.1M3 12h.1M3 19h.1',
  download: 'M4 11v9h16v-9M12 2v13M8 11l4 4 4-4',
  window: 'M3 4h18v16H3zM3 7h18',
  share: 'M12 16V2M8 6l4-4 4 4M5 10H3v11h18V10h-2',
  power: 'M12 2v10M6 5a9 9 0 1 0 12 0',
  check: 'M5 12l4 4L19 6',
  close: 'M6 6l12 12M6 18L18 6',
}
</script>

<template>
  <svg
    class="icon"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    stroke-width="1.7"
    stroke-linecap="round"
    stroke-linejoin="round"
    aria-hidden="true"
  >
    <path :d="paths[name] || paths.help" />
  </svg>
</template>
