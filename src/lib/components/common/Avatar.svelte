<script lang="ts">
  type Theme = "dark" | "light" | "glass" | "custom";

  interface Props {
    src?: string;
    name?: string;
    size?: "xs" | "sm" | "md" | "lg" | "xl";
    shape?: "circle" | "squircle";
    color?: string;
    theme?: Theme;
    customColor?: string;
  }

  let {
    src,
    name,
    size = "md",
    shape = "squircle",
    color,
    theme,
    customColor,
  }: Props = $props();

  const sizeMap = { xs: 20, sm: 24, md: 32, lg: 40, xl: 48 };
  const px = $derived(sizeMap[size]);

  const initials = $derived(() => {
    if (!name) return "";
    const words = name.trim().split(/\s+/);
    if (words.length >= 2) return (words[0][0] + words[1][0]).toUpperCase();
    return name.slice(0, 2).toUpperCase();
  });

  const fontSize = $derived(Math.floor(px * 0.38));
  const bgColor = $derived(color || "var(--surface-raised)");
</script>

<div
  class="avatar avatar--{size} avatar--{shape}"
  data-theme={theme}
  style={[
    `width: ${px}px;`,
    `height: ${px}px;`,
    `font-size: ${fontSize}px;`,
    !src ? `background-color: ${bgColor};` : "",
    theme === "custom" && customColor ? `--custom-color: ${customColor};` : "",
  ].filter(Boolean).join(" ")}
>
  {#if src}
    <img class="avatar-img" src={src} alt={name || "avatar"} />
  {:else}
    <span class="avatar-initials">{initials()}</span>
  {/if}
</div>

<style>
  .avatar {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    flex-shrink: 0;
    user-select: none;
    -webkit-user-select: none;
  }

  .avatar--circle  { border-radius: var(--sq-full); }
  .avatar--squircle { border-radius: var(--sq-icon); }

  .avatar-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .avatar-initials {
    font-weight: 600;
    color: var(--text-secondary);
    line-height: 1;
    letter-spacing: 0.02em;
  }
</style>
