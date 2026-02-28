<script lang="ts">
  interface Props {
    src: string;
    class?: string;
  }

  let { src, class: className = "" }: Props = $props();

  function pdfCleanup(node: HTMLIFrameElement) {
    return {
      destroy() {
        node.src = "about:blank";
        node.removeAttribute("src");
      }
    };
  }
</script>

<div class="pdf-wrapper {className}">
  <iframe use:pdfCleanup {src} title="PDF Preview" class="pdf-frame"></iframe>
</div>

<style>
  .pdf-wrapper {
    width: 100%;
    height: 100%;
    flex: 1;
    display: flex;
    min-height: 0;
  }

  .pdf-frame {
    width: 100%;
    height: 100%;
    flex: 1;
    border: none;
    background: var(--bg, #1a1a1a);
  }
</style>
