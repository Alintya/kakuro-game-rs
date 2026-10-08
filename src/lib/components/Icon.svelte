<script lang="ts" module>
const PATHS = {
  plus: ['M12 5v14', 'M5 12h14'],
  undo: ['M9 14 4 9l5-5', 'M4 9h11a5 5 0 0 1 0 10h-3'],
  redo: ['m15 14 5-5-5-5', 'M20 9H9a5 5 0 0 0 0 10h3'],
  pen: ['M4 7V4h16v3', 'M9 20h6', 'M12 4v16'],
  pencil: ['M12 20h9', 'M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4Z'],
  eraser: [
    'm7 21-4.3-4.3a1 1 0 0 1 0-1.4l9.6-9.6a1 1 0 0 1 1.4 0l5.6 5.6a1 1 0 0 1 0 1.4L13 21',
    'M22 21H7',
    'm5 11 9 9',
  ],
  sun: [
    'M16 12a4 4 0 1 1-8 0 4 4 0 0 1 8 0',
    'M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M6.34 17.66l-1.41 1.41M19.07 4.93l-1.41 1.41',
  ],
  moon: ['M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z'],
  monitor: [
    'M4 3h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2Z',
    'M8 21h8M12 17v4',
  ],
  clock: ['M22 12a10 10 0 1 1-20 0 10 10 0 0 1 20 0', 'M12 6v6l4 2'],
  pause: ['M10 4H6v16h4zM18 4h-4v16h4z'],
  check: ['M20 6 9 17l-5-5'],
  x: ['M18 6 6 18M6 6l12 12'],
  settings: [
    'M20 7h-9',
    'M14 17H5',
    'M20 17a3 3 0 1 1-6 0 3 3 0 0 1 6 0',
    'M10 7a3 3 0 1 1-6 0 3 3 0 0 1 6 0',
  ],
  'arrow-right': ['M5 12h14M12 5l7 7-7 7'],
  'arrow-down': ['M12 5v14M19 12l-7 7-7-7'],
} satisfies Record<string, string[]>;

export type IconName = keyof typeof PATHS;
</script>

<script lang="ts">
let { name, size = 18 }: { name: IconName; size?: number } = $props();
</script>

<svg
  width={size}
  height={size}
  viewBox="0 0 24 24"
  fill="none"
  stroke="currentColor"
  stroke-width="2"
  stroke-linecap="round"
  stroke-linejoin="round"
  aria-hidden="true"
>
  {#each PATHS[name] as d (d)}
    <path {d} />
  {/each}
</svg>
