// e2e_tests/global.d.ts
// This file provides global type definitions for the E2E test environment.

// Import the main WASM app type from the generated declaration file.
// The path is relative to this file.
import type { AppHandle } from '../static/pkg/app';

declare global {
  interface Window {
    /**
     * The main WASM application instance, attached to the window for E2E testing and debugging.
     * This type is imported from the wasm-pack generated `app.d.ts` file.
     */
    appInstance?: AppHandle;
  }
}

// This export {} is important to make this file a module,
// which allows augmenting global types.
export {};
