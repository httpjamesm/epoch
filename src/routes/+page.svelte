<script lang="ts">
  import { onDestroy, onMount } from "svelte";
  import { writeText } from "@tauri-apps/plugin-clipboard-manager";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";

  interface EpochTimestamp {
    components: {
      seconds: string;
      milliseconds: string;
      microseconds: string;
      nanoseconds: string;
    };
  }

  let timestamp: EpochTimestamp | null = $state(null);

  const updateTimestamp = (currentNs: number) => {
    // Convert the input nanoseconds to BigInt
    const totalNs = BigInt(currentNs);

    // Define BigInt constants for calculations
    const ONE_BILLION = BigInt(1_000_000_000); // 1 second in nanoseconds
    const ONE_MILLION = BigInt(1_000_000); // 1 millisecond in nanoseconds
    const ONE_THOUSAND = BigInt(1_000); // 1 microsecond in nanoseconds

    // Calculate each component using BigInt division and modulo operations
    const seconds = totalNs / ONE_BILLION;
    const milliseconds = (totalNs % ONE_BILLION) / ONE_MILLION;
    const microseconds = (totalNs % ONE_MILLION) / ONE_THOUSAND;
    const nanoseconds = totalNs % ONE_THOUSAND;

    // Update the timestamp object with string representations of each component
    timestamp = {
      components: {
        seconds: seconds.toString(),
        milliseconds: milliseconds.toString().padStart(3, "0"),
        microseconds: microseconds.toString().padStart(3, "0"),
        nanoseconds: nanoseconds.toString().padStart(3, "0"),
      },
    };
  };

  let unlisten: () => void | null = null;

  const startEmitter = async () => {
    try {
      await invoke("start_precise_timestamp_emitter");
    } catch (error) {
      console.error(error);
    }
  };

  const startListener = async () => {
    unlisten = await listen<number>("precise_timestamp", (event) => {
      updateTimestamp(event.payload);
    });
  };

  onMount(() => {
    startEmitter();
    startListener();
  });

  onDestroy(() => {
    unlisten?.();
  });

  const copyTimestamp = async (
    component: keyof EpochTimestamp["components"]
  ) => {
    try {
      let copiedTimestamp = timestamp?.components.seconds;
      // incrementally concatenate the timestamp component. e.g. if nanoseconds is requested, then start with seconds, then milliseconds, then microseconds, then nanoseconds
      if (component === "nanoseconds") {
        copiedTimestamp =
          (timestamp?.components.seconds ?? "") +
          (timestamp?.components.milliseconds ?? "") +
          (timestamp?.components.microseconds ?? "") +
          (timestamp?.components.nanoseconds ?? "");
      } else if (component === "microseconds") {
        copiedTimestamp =
          (timestamp?.components.seconds ?? "") +
          (timestamp?.components.milliseconds ?? "") +
          (timestamp?.components.microseconds ?? "");
      } else if (component === "milliseconds") {
        copiedTimestamp =
          (timestamp?.components.seconds ?? "") +
          (timestamp?.components.milliseconds ?? "");
      }
      await writeText(copiedTimestamp ?? "");
      title = "✅ Copied!";
      setTimeout(() => {
        title = "Click to copy timestamp";
      }, 1000);
    } catch (error) {
      title = "Failed to copy timestamp";
    }
  };

  let title = $state("Click to copy timestamp");
</script>

<div class="wrapper">
  <div class="container">
    <p>{title}</p>

    <div class="timestamp-container">
      <button
        class="timestamp-component"
        onclick={() => copyTimestamp("seconds")}
      >
        <h1 class="timestamp">
          {(timestamp?.components.seconds ?? "0").padStart(10, "0")}
        </h1>
        <span>secs</span>
      </button>
      <button
        class="timestamp-component"
        onclick={() => copyTimestamp("milliseconds")}
      >
        <h1 class="timestamp">{timestamp?.components.milliseconds ?? "000"}</h1>
        <span>ms</span>
      </button>
      <button
        class="timestamp-component"
        onclick={() => copyTimestamp("microseconds")}
      >
        <h1 class="timestamp">{timestamp?.components.microseconds ?? "000"}</h1>
        <span>µs</span>
      </button>
      <button
        class="timestamp-component"
        onclick={() => copyTimestamp("nanoseconds")}
      >
        <h1 class="timestamp">{timestamp?.components.nanoseconds ?? "000"}</h1>
        <span>ns</span>
      </button>
    </div>
  </div>
</div>

<style lang="scss">
  .wrapper {
    height: 100vh;
    width: 100%;
    display: flex;
    justify-content: center;
    align-items: center;

    .container {
      text-align: center;

      .timestamp-container {
        display: flex;
        gap: 0.5rem;

        .timestamp-component {
          border-radius: 0.5rem;
          padding: 0.5rem;
          background: none;
          border: none;
          cursor: pointer;
          color: var(--text-primary);
          min-width: 3rem;

          .timestamp {
            border-bottom: 5px dotted var(--text-secondary);
            font-family: monospace;
            font-variant-numeric: tabular-nums;
          }

          &:hover {
            background-color: var(--card-bg);
          }
        }
      }
    }
  }
</style>
