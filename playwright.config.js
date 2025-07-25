// @ts-check
import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './e2e_tests',
  fullyParallel: true,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 2 : 0,
  reporter: 'html',
  use: {
    baseURL: 'http://127.0.0.1:8080',
    trace: 'on-first-retry',
  },
  projects: [
    {
      name: 'chromium',
      use: { ...devices['Desktop Chrome'], hasTouch: true },
    },
    {
      name: 'firefox',
      use: { ...devices['Desktop Firefox'], hasTouch: true },
    },
    ...(process.platform === 'darwin' ? [{
      name: 'webkit',
      use: { ...devices['Desktop Safari'], hasTouch: true },
    }] : []),
  ],
  webServer: {
    command: 'cargo run --bin cli -- dev --port 8080 --host 127.0.0.1 --wasm-debug',
    url: 'http://127.0.0.1:8080',
    reuseExistingServer: !process.env.CI,
    timeout: 120 * 1000,
  },
});
